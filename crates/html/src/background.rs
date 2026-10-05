//! Section 4 of `cli/engine/rules/checks.mjs`: the unified background walk
//! (`readOwnBackgroundColor`, `readCascadeBackgroundColor`,
//! `resolveBackgroundInfo`, `resolveBackground`, `resolveGradientStops`,
//! `compositeGradientStops`, `resolveBorderRadiusPx`), static-engine
//! branches only (`DETECTOR_IS_BROWSER === false`).

use crate::cascade::csstree::strings::{decode_string, decode_url};
use crate::cascade::values::{css_call_end, css_string_end};
use crate::cascade::StyleValues;
use crate::dom::StaticElement;
use ego_tree::NodeId;
use impeccable_core::checks::measures::{parse_color_resolved, parse_radius_to_px, CustomProps};
use impeccable_core::checks::sampled_contrast::uniform_wash;
use impeccable_core::color::{
    composite_color_over, is_no_paint_color_value, parse_any_color, parse_gradient_colors,
    parse_rgb, split_top_level_commas, Rgba,
};
use impeccable_core::js;
use once_cell::sync::Lazy;
use regex::Regex;
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// `style.x || ''` on a computed style map.
pub fn sv<'a>(style: &'a StyleValues, key: &str) -> &'a str {
    style.get(key).map(|s| s.as_str()).unwrap_or("")
}

/// `style.x` as JS sees it: `None` for a key the style object never had.
pub fn sv_opt<'a>(style: &'a StyleValues, key: &str) -> Option<&'a str> {
    style.get(key).map(|s| s.as_str())
}

/// JS `c.a >= x` where `a` may be undefined (then false).
pub fn a_ge(c: &Rgba, x: f64) -> bool {
    c.a.is_some_and(|a| a >= x)
}
/// JS `c.a > x`.
pub fn a_gt(c: &Rgba, x: f64) -> bool {
    c.a.is_some_and(|a| a > x)
}
/// JS `c.a < x`.
pub fn a_lt(c: &Rgba, x: f64) -> bool {
    c.a.is_some_and(|a| a < x)
}

/// The static engine's `customPropMap` is always `null`; kept as a
/// parameter so the port mirrors the JS signatures.
pub type CustomPropMap<'a> = Option<&'a dyn CustomProps>;

static INLINE_BG_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&format!(
        r"(?i)background(?:-color)?{ws}*:{ws}*([^;]+)",
        ws = js::WS
    ))
    .expect("INLINE_BG_RE")
});
static INLINE_BG_IMAGE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&format!(
        r"(?i)background(?:-image)?{ws}*:{ws}*([^;]+)",
        ws = js::WS
    ))
    .expect("INLINE_BG_IMAGE_RE")
});
static GRADIENT_RE: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)gradient").expect("GRADIENT_RE"));
static GRADIENT_CALL_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&format!(r"(?i)gradient{ws}*\(", ws = js::WS)).expect("GRADIENT_CALL_RE")
});
static URL_CALL_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(&format!(r"(?i)url{ws}*\(", ws = js::WS)).expect("URL_CALL_RE"));
static URL_START_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(&format!(r"(?i)^{ws}*url{ws}*\(", ws = js::WS)).expect("URL_START_RE"));
static HEX_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)#([0-9a-f]{6}|[0-9a-f]{3})(?-u:\b)").expect("HEX_RE"));
static CURRENTCOLOR_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new("(?i)^currentcolor$").expect("CURRENTCOLOR_RE"));

/// `rawStyle.match(/background(?:-color)?\s*:\s*([^;]+)/i)` → trimmed value.
fn inline_bg(el: &StaticElement<'_>) -> String {
    let raw = el.get_attribute("style").unwrap_or("");
    INLINE_BG_RE
        .captures(raw)
        .map(|m| js::trim(&m[1]).to_string())
        .unwrap_or_default()
}

fn hex_to_rgba(h: &str) -> Rgba {
    let p = |s: &str| js::parse_int(s, 16);
    if h.len() == 6 {
        Rgba::new(p(&h[0..2]), p(&h[2..4]), p(&h[4..6]), 1.0)
    } else {
        let d = |i: usize| {
            let c = &h[i..i + 1];
            p(&format!("{c}{c}"))
        };
        Rgba::new(d(0), d(1), d(2), 1.0)
    }
}

/// JS: checks.mjs#readOwnBackgroundColor(el, computedStyle)
pub fn read_own_background_color(el: &StaticElement<'_>, style: &StyleValues) -> Option<Rgba> {
    let bgc = sv_opt(style, "backgroundColor");
    let bg = parse_rgb(bgc).or_else(|| parse_any_color(bgc));
    if bg.as_ref().is_some_and(|c| a_ge(c, 0.1)) {
        return bg;
    }
    let inline = inline_bg(el);
    if inline.is_empty() {
        return bg;
    }
    if GRADIENT_RE.is_match(&inline) || URL_CALL_RE.is_match(&inline) {
        return bg;
    }
    if let Some(from_rgb) = parse_rgb(Some(&inline)) {
        return Some(from_rgb);
    }
    if let Some(m) = HEX_RE.captures(&inline) {
        return Some(hex_to_rgba(&m[1]));
    }
    bg
}

/// JS: checks.mjs#readCascadeBackgroundColor(current, style, customPropMap)
pub fn read_cascade_background_color(
    current: &StaticElement<'_>,
    style: &StyleValues,
    custom_props: CustomPropMap<'_>,
) -> Option<Rgba> {
    let bgc = sv_opt(style, "backgroundColor");
    let mut bg = parse_rgb(bgc).or_else(|| parse_any_color(bgc));
    if bg.is_none() || bg.as_ref().is_some_and(|c| a_lt(c, 0.1)) {
        if let Some(map) = custom_props {
            bg = parse_color_resolved(bgc, Some(map));
        }
        if bg.is_none() || bg.as_ref().is_some_and(|c| a_lt(c, 0.1)) {
            let inline = inline_bg(current);
            if !inline.is_empty()
                && !GRADIENT_RE.is_match(&inline)
                && !URL_CALL_RE.is_match(&inline)
            {
                bg = parse_color_resolved(Some(&inline), custom_props)
                    .or_else(|| parse_any_color(Some(&inline)));
            }
        }
    }
    bg
}

/// `{ color, unresolved }` from `resolveBackgroundInfo`.
#[derive(Debug, Clone, PartialEq)]
pub struct BackgroundInfo {
    pub color: Option<Rgba>,
    pub unresolved: bool,
}

fn flatten(overlays: &[Rgba], base: Rgba) -> Rgba {
    let mut acc = base;
    for o in overlays.iter().rev() {
        acc = composite_color_over(o, &acc);
    }
    acc
}

/// One element's own background color as both ancestor walks read it: the
/// cascade color, or the text color when the declared value is
/// `currentcolor`.
fn level_background_color(
    cur: &StaticElement<'_>,
    style: &StyleValues,
    custom_props: CustomPropMap<'_>,
) -> Option<Rgba> {
    let mut bg = read_cascade_background_color(cur, style, custom_props);
    if (bg.is_none() || bg.as_ref().is_some_and(|c| a_lt(c, 0.1)))
        && CURRENTCOLOR_RE.is_match(js::trim(sv(style, "backgroundColor")))
    {
        let color = sv_opt(style, "color");
        bg = parse_rgb(color).or_else(|| parse_color_resolved(color, custom_props));
    }
    bg
}

/// JS: checks.mjs#resolveBackgroundInfo(el, win, customPropMap)
pub fn resolve_background_info(
    el: &StaticElement<'_>,
    custom_props: CustomPropMap<'_>,
) -> BackgroundInfo {
    let mut current = Some(*el);
    let mut overlays: Vec<Rgba> = Vec::new();
    while let Some(cur) = current {
        let style = cur.style();
        let bg_image = sv(style, "backgroundImage");
        let has_gradient_or_url = !bg_image.is_empty()
            && bg_image != "none"
            && (GRADIENT_RE.is_match(bg_image) || URL_CALL_RE.is_match(bg_image));

        let bg = level_background_color(&cur, style, custom_props);

        if let Some(c) = bg.as_ref().filter(|c| a_gt(c, 0.1)) {
            if a_ge(c, 0.99) {
                return BackgroundInfo {
                    color: Some(flatten(&overlays, *c)),
                    unresolved: false,
                };
            }
            overlays.push(*c);
        } else if bg.is_none() && !is_no_paint_color_value(sv_opt(style, "backgroundColor")) {
            return BackgroundInfo {
                color: None,
                unresolved: true,
            };
        }
        if has_gradient_or_url {
            let layers = split_top_level_commas(bg_image);
            let top_paint_layer = layers
                .iter()
                .find(|layer| GRADIENT_CALL_RE.is_match(layer) || URL_CALL_RE.is_match(layer));
            let gradient_on_top = top_paint_layer.is_some_and(|layer| {
                GRADIENT_CALL_RE.is_match(layer) && !URL_START_RE.is_match(layer)
            });
            if !gradient_on_top {
                return BackgroundInfo {
                    color: None,
                    unresolved: true,
                };
            }
            let top = top_paint_layer.unwrap();
            let url_beneath = layers
                .iter()
                .any(|layer| layer != top && URL_CALL_RE.is_match(layer));
            if url_beneath {
                let top_stops = parse_gradient_colors(Some(top));
                let provably_opaque =
                    !top_stops.is_empty() && top_stops.iter().all(|s| s.alpha_or_one() >= 0.99);
                if !provably_opaque {
                    return BackgroundInfo {
                        color: None,
                        unresolved: true,
                    };
                }
            }
            return BackgroundInfo {
                color: None,
                unresolved: false,
            };
        }
        current = cur.parent_element();
    }
    BackgroundInfo {
        color: Some(flatten(&overlays, Rgba::new(255.0, 255.0, 255.0, 1.0))),
        unresolved: false,
    }
}

/// JS: checks.mjs#resolveBackground(el, win, customPropMap)
pub fn resolve_background(el: &StaticElement<'_>, custom_props: CustomPropMap<'_>) -> Option<Rgba> {
    resolve_background_info(el, custom_props).color
}

/// JS: checks.mjs#resolveGradientStops(el, win, customPropMap)
pub fn resolve_gradient_stops(
    el: &StaticElement<'_>,
    custom_props: CustomPropMap<'_>,
) -> Option<Vec<Rgba>> {
    let mut current = Some(*el);
    let mut overlays: Vec<Rgba> = Vec::new();
    while let Some(cur) = current {
        let style = cur.style();
        let bg_image = sv(style, "backgroundImage");
        if !bg_image.is_empty() && bg_image != "none" && URL_CALL_RE.is_match(bg_image) {
            return None;
        }
        let mut stops: Option<Vec<Rgba>> = None;
        if !bg_image.is_empty() && bg_image != "none" && GRADIENT_RE.is_match(bg_image) {
            let parsed = parse_gradient_colors(Some(bg_image));
            if !parsed.is_empty() {
                stops = Some(parsed);
            }
        }
        if stops.is_none() {
            let raw = cur.get_attribute("style").unwrap_or("");
            if let Some(m) = INLINE_BG_IMAGE_RE.captures(raw) {
                if GRADIENT_RE.is_match(&m[1]) {
                    let parsed = parse_gradient_colors(Some(&m[1]));
                    if !parsed.is_empty() {
                        stops = Some(parsed);
                    }
                }
            }
        }
        if let Some(stops) = stops {
            let composited = composite_gradient_stops(&stops, &cur, custom_props);
            let Some(composited) = composited else {
                return None;
            };
            if overlays.is_empty() {
                return Some(composited);
            }
            return Some(
                composited
                    .into_iter()
                    .map(|stop| flatten(&overlays, stop))
                    .collect(),
            );
        }
        let bg = read_cascade_background_color(&cur, style, custom_props);
        if let Some(c) = bg.filter(|c| a_gt(c, 0.1)) {
            if a_ge(&c, 0.99) {
                return None;
            }
            overlays.push(c);
        }
        current = cur.parent_element();
    }
    None
}

/// JS: checks.mjs#compositeGradientStops(stops, gradientEl, win, customPropMap)
pub fn composite_gradient_stops(
    stops: &[Rgba],
    gradient_el: &StaticElement<'_>,
    custom_props: CustomPropMap<'_>,
) -> Option<Vec<Rgba>> {
    let has_alpha = stops.iter().any(|s| s.alpha_or_one() < 0.99);
    if !has_alpha {
        return Some(stops.to_vec());
    }
    let base_el = gradient_el.parent_element().unwrap_or(*gradient_el);
    let base = resolve_background(&base_el, custom_props);
    let mut out: Vec<Rgba> = Vec::new();
    for s in stops {
        let a = s.alpha_or_one();
        if a >= 0.99 {
            out.push(*s);
            continue;
        }
        if let Some(base) = base.as_ref() {
            out.push(composite_color_over(s, base));
        }
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

/// JS: checks.mjs#resolveBorderRadiusPx(el, style, widthPx, win)
pub fn resolve_border_radius_px(style: &StyleValues, width_px: f64) -> f64 {
    parse_radius_to_px(sv_opt(style, "borderRadius"), width_px).unwrap_or(0.0)
}

// ─── the image ground (#560) ────────────────────────────────────────────────

static GRADIENT_PRELUDE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&format!(
        r"(?i)^(?:to{ws}|from{ws}|at{ws}|in{ws}|[-+]?[0-9.]+(?:deg|grad|rad|turn)(?-u:\b)|circle(?-u:\b)|ellipse(?-u:\b)|closest-|farthest-)|{ws}at{ws}",
        ws = js::WS
    ))
    .expect("GRADIENT_PRELUDE_RE")
});
static STOP_POSITION_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)^[-+]?[0-9.]+(?:%|[a-z]+)?$").expect("STOP_POSITION_RE"));

/// The first call at the top level of `text[from..to]` whose name `wanted`
/// accepts, as `(start of the name, index of its "(")`. A call nested in
/// another (`image-set(url(a.png) 1x)`, `cross-fade(..)`) is that function's
/// argument, not the layer's image, and a string hides whatever it quotes.
fn top_level_call(
    text: &str,
    from: usize,
    to: usize,
    wanted: impl Fn(&str) -> bool,
) -> Option<(usize, usize)> {
    let bytes = text.as_bytes();
    let is_name = |b: u8| b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b >= 0x80;
    let mut i = from;
    while i < to {
        match bytes[i] {
            b'\\' => i += 2,
            b'"' | b'\'' => i = css_string_end(text, i),
            b'(' => {
                let mut name = i;
                while name > from && is_name(bytes[name - 1]) {
                    name -= 1;
                }
                if wanted(&text[name..i]) {
                    return Some((name, i));
                }
                i = css_call_end(text, i);
            }
            _ => i += 1,
        }
    }
    None
}

fn is_gradient_name(name: &str) -> bool {
    name.len() >= 8 && name.as_bytes()[name.len() - 8..].eq_ignore_ascii_case(b"gradient")
}

/// A `url()` call inside a background value, located the way the CSS
/// tokenizer would: `url(` and then a quoted string, or an unquoted run to
/// the first unescaped `)`. Spans are byte offsets into the value it was
/// found in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UrlCall {
    arg_start: usize,
    arg_end: usize,
    quoted: bool,
    escaped: bool,
}

impl UrlCall {
    /// The `url()` call at the top level of `text[from..to]`. `url(` inside
    /// a longer identifier (`myurl(`) is a custom function, not this token.
    fn find(text: &str, from: usize, to: usize) -> Option<UrlCall> {
        let (_, open) = top_level_call(text, from, to, |name| name.eq_ignore_ascii_case("url"))?;
        let close = css_call_end(text, open).min(to);
        let closed = close > open + 1 && text.as_bytes()[close - 1] == b')';
        let inner_end = if closed { close - 1 } else { close };
        let inner = &text[open + 1..inner_end];
        let lead = inner.len() - inner.trim_start().len();
        let trimmed = inner.trim();
        let mut arg_start = open + 1 + lead;
        let mut arg_end = arg_start + trimmed.len();
        let bytes = trimmed.as_bytes();
        let quoted = bytes.len() >= 2
            && (bytes[0] == b'"' || bytes[0] == b'\'')
            && bytes[bytes.len() - 1] == bytes[0];
        if quoted {
            arg_start += 1;
            arg_end -= 1;
        }
        Some(UrlCall {
            arg_start,
            arg_end,
            quoted,
            escaped: text[arg_start..arg_end].contains('\\'),
        })
    }

    /// The url the call names, decoded. Borrowed from `text` unless it
    /// carries escapes, so a multi-megabyte data URI is never copied.
    pub fn value<'t>(&self, text: &'t str) -> Cow<'t, str> {
        let raw = &text[self.arg_start..self.arg_end];
        if !self.escaped {
            Cow::Borrowed(raw)
        } else if self.quoted {
            Cow::Owned(decode_string(&text[self.arg_start - 1..self.arg_end + 1]))
        } else {
            Cow::Owned(decode_url(&format!("url({raw})")))
        }
    }
}

/// The byte spans of a background list's top-level layers, trimmed. Commas
/// inside calls and strings do not split.
fn layer_spans(text: &str) -> Vec<(usize, usize)> {
    let bytes = text.as_bytes();
    let mut spans: Vec<(usize, usize)> = Vec::new();
    let mut start = 0usize;
    let mut i = 0usize;
    while i < bytes.len() {
        match bytes[i] {
            b'\\' => i += 1,
            b'(' => {
                i = css_call_end(text, i);
                continue;
            }
            b'"' | b'\'' => {
                i = css_string_end(text, i);
                continue;
            }
            b',' => {
                spans.push((start, i));
                start = i + 1;
            }
            _ => {}
        }
        i += 1;
    }
    spans.push((start.min(text.len()), text.len()));
    spans
        .into_iter()
        .map(|(from, to)| {
            let slice = &text[from..to];
            let lead = slice.len() - slice.trim_start().len();
            (from + lead, from + lead + slice.trim().len())
        })
        .collect()
}

/// One entry of a comma-separated background list, with the CSS wraparound
/// for a list shorter than the image list.
fn layer_entry(value: &str, index: usize) -> String {
    let parts = split_top_level_commas(value);
    if parts.is_empty() {
        return String::new();
    }
    js::trim(&parts[index % parts.len()]).to_string()
}

/// The stops of a gradient call (`linear-gradient(..)`, name to closing
/// paren), when every one of them was read. `parse_gradient_colors` reads
/// color functions and hex only, so a `transparent`, named, `currentcolor`,
/// or `var()` stop goes missing from its result, and a wash judged on the
/// stops that remain is not the wash that paints. `None` for that case, and
/// for a gradient with no stops.
fn gradient_stops(call: &str) -> Option<Vec<Rgba>> {
    let stops = parse_gradient_colors(Some(call));
    let args = &call[call.find('(')? + 1..];
    let args = args.strip_suffix(')').unwrap_or(args);
    let expected = split_top_level_commas(args)
        .iter()
        .enumerate()
        .filter(|(index, segment)| {
            let segment = js::trim(segment);
            let prelude = *index == 0 && GRADIENT_PRELUDE_RE.is_match(segment);
            !segment.is_empty() && !prelude && !STOP_POSITION_RE.is_match(segment)
        })
        .count();
    (!stops.is_empty() && stops.len() == expected).then_some(stops)
}

/// What one element paints behind its descendants' text, as the sampled
/// path reads it.
#[derive(Debug, Clone, PartialEq)]
pub enum LevelPaint {
    /// Translucent paint only (uniform washes, then the element's own
    /// color), top to bottom. The walk goes on to the parent.
    Through(Vec<Rgba>),
    /// The analytic walk owns this level: an opaque color with no image over
    /// it, an opaque gradient, a scrim that varies or has a stop the engine
    /// cannot read, an unparseable color, or an image function it cannot
    /// read.
    Stop,
    /// A `url()` layer, with what the element paints over and under it.
    Image(ImageLayer),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImageLayer {
    /// The `url()` call inside the element's `backgroundImage` value.
    pub call: UrlCall,
    /// The layer's own `background-repeat` and `background-size` entries.
    pub repeat: String,
    pub size: String,
    /// Uniform washes listed above the image in the same declaration.
    pub washes: Vec<Rgba>,
    /// The element's own background color, painted beneath the image.
    pub color: Option<Rgba>,
    /// Another layer is listed beneath the image, so a translucent pixel has
    /// unknown paint under it.
    pub layers_beneath: bool,
    /// The element's font size in px, for a size given in `em`.
    pub font_size: f64,
}

/// The paint of one element, in isolation from its ancestors.
pub fn analyze_level(cur: &StaticElement<'_>) -> LevelPaint {
    let style = cur.style();
    let bg = level_background_color(cur, style, None);
    if bg.is_none() && !is_no_paint_color_value(sv_opt(style, "backgroundColor")) {
        return LevelPaint::Stop;
    }
    let color = bg.filter(|c| a_gt(c, 0.1));
    let bg_image = sv(style, "backgroundImage");
    let mut washes: Vec<Rgba> = Vec::new();
    if !bg_image.is_empty() {
        let spans = layer_spans(bg_image);
        for (index, &(from, to)) in spans.iter().enumerate() {
            let layer = &bg_image[from..to];
            if layer.is_empty() || layer.eq_ignore_ascii_case("none") {
                continue;
            }
            if let Some((name, open)) = top_level_call(bg_image, from, to, is_gradient_name) {
                // An opaque gradient is the ground; a scrim that varies is
                // placed under the text on purpose, and without layout the
                // engine cannot say which stop the text sits on.
                let call = &bg_image[name..css_call_end(bg_image, open)];
                let Some(stops) = gradient_stops(call) else {
                    return LevelPaint::Stop;
                };
                if stops.iter().all(|s| s.alpha_or_one() >= 0.99) {
                    return LevelPaint::Stop;
                }
                let Some(wash) = uniform_wash(&stops) else {
                    return LevelPaint::Stop;
                };
                washes.push(wash);
                continue;
            }
            // A shorthand layer may put color, position, size, or repeat
            // tokens around the image call; the call is what matters.
            let Some(call) = UrlCall::find(bg_image, from, to) else {
                return LevelPaint::Stop;
            };
            let layers_beneath = spans[index + 1..].iter().any(|&(from, to)| {
                let rest = &bg_image[from..to];
                !rest.is_empty() && !rest.eq_ignore_ascii_case("none")
            });
            let font_size = js::parse_float(sv(style, "fontSize"));
            return LevelPaint::Image(ImageLayer {
                call,
                repeat: layer_entry(sv(style, "backgroundRepeat"), index),
                size: layer_entry(sv(style, "backgroundSize"), index),
                washes,
                color,
                layers_beneath,
                font_size: if font_size > 0.0 { font_size } else { 16.0 },
            });
        }
    }
    if color.as_ref().is_some_and(|c| a_ge(c, 0.99)) {
        return LevelPaint::Stop;
    }
    washes.extend(color);
    LevelPaint::Through(washes)
}

/// [`analyze_level`] remembered per element for one document: a page's text
/// elements share their ancestors, and an ancestor's background (a data URI
/// can run to megabytes) is read once.
#[derive(Default)]
pub struct LevelCache(RefCell<HashMap<NodeId, Rc<LevelPaint>>>);

impl LevelCache {
    fn get(&self, el: &StaticElement<'_>) -> Rc<LevelPaint> {
        if let Some(hit) = self.0.borrow().get(&el.id()) {
            return hit.clone();
        }
        let paint = Rc::new(analyze_level(el));
        self.0.borrow_mut().insert(el.id(), paint.clone());
        paint
    }
}

/// The image a text element sits on: the nearest level that paints a `url()`
/// layer, with everything painted between the text and it.
#[derive(Debug, Clone, PartialEq)]
pub struct ImageGround {
    /// The element that paints the image.
    pub node: NodeId,
    pub layer: ImageLayer,
    /// Translucent paint between the text and the image, top to bottom.
    pub overlays: Vec<Rgba>,
    /// What a translucent pixel composites over, when the walk can say.
    pub under: Option<Rgba>,
    /// The element also declares an opaque color, so the image replaces it
    /// as the ground only if it provably covers the box.
    pub over_opaque_color: bool,
}

/// The walk the sampled path takes in place of [`resolve_background_info`]:
/// up from the text to the first `url()` layer. `None` when an element on
/// the way is one the analytic walk owns (see [`LevelPaint::Stop`]) or when
/// no image is reached.
pub fn find_image_ground(el: &StaticElement<'_>, levels: &LevelCache) -> Option<ImageGround> {
    let mut current = Some(*el);
    let mut overlays: Vec<Rgba> = Vec::new();
    while let Some(cur) = current {
        match &*levels.get(&cur) {
            LevelPaint::Stop => return None,
            LevelPaint::Through(paint) => overlays.extend(paint.iter().copied()),
            LevelPaint::Image(layer) => {
                overlays.extend(layer.washes.iter().copied());
                let over_opaque_color = layer.color.as_ref().is_some_and(|c| a_ge(c, 0.99));
                // A layer beneath the image is unknown paint. Otherwise the
                // element's own color, over its parent's ground when it is
                // translucent, white at the root like the analytic walk.
                let under = if layer.layers_beneath {
                    None
                } else if over_opaque_color {
                    layer.color
                } else {
                    let parent_ground = match cur.parent_element() {
                        Some(p) => resolve_background(&p, None),
                        None => Some(Rgba::new(255.0, 255.0, 255.0, 1.0)),
                    };
                    match layer.color {
                        Some(c) => parent_ground.map(|p| composite_color_over(&c, &p)),
                        None => parent_ground,
                    }
                };
                return Some(ImageGround {
                    node: cur.id(),
                    layer: layer.clone(),
                    overlays,
                    under,
                    over_opaque_color,
                });
            }
        }
        current = cur.parent_element();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cascade::{build_static_style_map, collect_static_css_text};
    use crate::dom::StaticDocument;
    use std::path::Path;

    fn url_in(text: &str) -> Option<Cow<'_, str>> {
        UrlCall::find(text, 0, text.len()).map(|call| call.value(text))
    }

    #[test]
    fn a_url_call_ends_where_the_tokenizer_says() {
        assert_eq!(url_in("url(x.png)").as_deref(), Some("x.png"));
        assert_eq!(
            url_in("URL( 'x y.png' ) no-repeat").as_deref(),
            Some("x y.png")
        );
        // The generator's unquoted-escaped form: an escaped paren is part
        // of the url, not its end.
        assert_eq!(
            url_in(r"url(a\).png)center/cover").as_deref(),
            Some("a).png")
        );
        assert_eq!(url_in(r"url(a\ b\(1\).png)").as_deref(), Some("a b(1).png"));
        // A style attribute keeps its quotes: an escaped quote does not
        // close the string, and a paren inside it does not close the call.
        assert_eq!(
            url_in(r#"url("a\"b.png") no-repeat"#).as_deref(),
            Some(r#"a"b.png"#)
        );
        assert_eq!(
            url_in(r#"url("a).png") no-repeat"#).as_deref(),
            Some("a).png")
        );
        // Tokens may come first, and a color function ahead of the image is
        // not the call.
        assert_eq!(
            url_in("rgba(0, 0, 0, 0.5) url(x.png)center/cover").as_deref(),
            Some("x.png")
        );
        assert_eq!(url_in("myurl(x.png)"), None);
        assert_eq!(url_in("linear-gradient(red, blue)"), None);
        // A url that is another function's argument is not the layer's
        // image, and neither is one a string merely mentions.
        assert_eq!(url_in("image-set(url(x.png) 1x, url(x2.png) 2x)"), None);
        assert_eq!(url_in("cross-fade(url(a.png), url(b.png), 50%)"), None);
        assert_eq!(url_in(r#"image-set("url(x.png)" 1x)"#), None);
        // No escapes means no copy, whatever the length.
        let uri = format!("url(data:image/png;base64,{})", "A".repeat(4096));
        assert!(matches!(url_in(&uri), Some(Cow::Borrowed(_))));
        // A call that never closes still yields what is there.
        assert_eq!(url_in("url(x.png").as_deref(), Some("x.png"));
    }

    #[test]
    fn layers_split_on_top_level_commas_only() {
        let text = r#"linear-gradient(red, blue) , url("a,b.png") center, none"#;
        let layers: Vec<&str> = layer_spans(text)
            .into_iter()
            .map(|(from, to)| &text[from..to])
            .collect();
        assert_eq!(
            layers,
            [
                "linear-gradient(red, blue)",
                r#"url("a,b.png") center"#,
                "none"
            ]
        );
        assert_eq!(layer_spans("none"), vec![(0, 4)]);
    }

    #[test]
    fn a_gradient_is_read_only_when_every_stop_is() {
        let uniform =
            gradient_stops("linear-gradient(to right, rgba(0,0,0,.4) 0%, rgba(0,0,0,.4) 100%)");
        assert_eq!(uniform.map(|s| s.len()), Some(2));
        assert_eq!(
            gradient_stops("radial-gradient(circle at top, #000, 40%, #fff)").map(|s| s.len()),
            Some(2)
        );
        assert_eq!(
            gradient_stops("conic-gradient(from 90deg, #000 0deg, #fff 180deg)").map(|s| s.len()),
            Some(2)
        );
        // A stop the color parser does not read is a stop all the same.
        for scrim in [
            "linear-gradient(rgba(0,0,0,.8), transparent)",
            "linear-gradient(to top, rgba(0,0,0,.75), transparent)",
            "linear-gradient(transparent, rgba(0,0,0,.8))",
            "linear-gradient(black, rgba(0,0,0,.8))",
            "linear-gradient(rgba(0,0,0,.8), currentcolor)",
            "linear-gradient(rgba(0,0,0,.8), var(--end))",
            "linear-gradient(transparent, transparent)",
        ] {
            assert_eq!(gradient_stops(scrim), None, "{scrim}");
        }
    }

    fn document(html: &str) -> StaticDocument {
        let mut doc = StaticDocument::parse(html);
        let css = collect_static_css_text(&doc, Path::new("/nonexistent"), None, "x.html", None);
        build_static_style_map(&mut doc, &css, None, "x.html");
        doc
    }

    /// The two ancestor walks, side by side: the analytic one (`unresolved`)
    /// and the sampled one (an image ground, and whether it sits over an
    /// opaque color).
    fn walks(css: &str) -> (bool, Option<bool>) {
        let doc = document(&format!(
            "<style>{css}</style><section><div class=x><p id=t>Text</p></div></section>"
        ));
        let el = doc.query_selector("#t").expect("text element");
        let levels = LevelCache::default();
        (
            resolve_background_info(&el, None).unresolved,
            find_image_ground(&el, &levels).map(|g| g.over_opaque_color),
        )
    }

    #[test]
    fn the_sampled_walk_finds_an_image_only_where_the_analytic_walk_cannot_see_past_one() {
        // Wherever the analytic walk reports `unresolved` because of a
        // `url()` layer, the sampled walk finds that layer, and nowhere else
        // does it claim one without an opaque color to say so.
        for (css, unresolved, ground) in [
            (".x { background: url(a.png); }", true, Some(false)),
            (
                ".x { background-image: none, url(a.png); }",
                true,
                Some(false),
            ),
            (
                ".x { background: center / cover no-repeat url(a.png); }",
                true,
                Some(false),
            ),
            (
                ".x { background: rgba(0,0,0,.4) url(a.png); }",
                true,
                Some(false),
            ),
            (
                ".x { background: linear-gradient(rgba(0,0,0,.4), rgba(0,0,0,.4)), url(a.png); }",
                true,
                Some(false),
            ),
            (
                "section { background: url(a.png); } .x { background: rgba(0,0,0,.4); }",
                true,
                Some(false),
            ),
            // A scrim, a stop it cannot read, an image function it cannot
            // read: both walks give up.
            (
                ".x { background: linear-gradient(rgba(0,0,0,.8), transparent), url(a.png); }",
                true,
                None,
            ),
            (
                ".x { background: linear-gradient(rgba(0,0,0,.8), rgba(0,0,0,0)), url(a.png); }",
                true,
                None,
            ),
            (
                ".x { background-image: image-set(url(a.png) 1x); }",
                true,
                None,
            ),
            // An opaque color: the analytic walk resolves it. The sampled
            // walk agrees unless an image is painted over that color.
            (".x { background: #111; }", false, None),
            (
                "section { background: url(a.png); } .x { background: #111; }",
                false,
                None,
            ),
            (".x { background: #111 url(a.png); }", false, Some(true)),
            (".x { background: url(a.png) #111; }", false, Some(true)),
            // A color-only shorthand clears the image an earlier rule set.
            (
                ".x { background: url(a.png); } div.x { background: transparent; }",
                false,
                None,
            ),
            // No image anywhere.
            (".x { color: red; }", false, None),
        ] {
            assert_eq!(walks(css), (unresolved, ground), "{css}");
        }
    }
}
