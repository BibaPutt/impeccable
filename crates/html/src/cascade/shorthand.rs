//! Shorthand expansion of the static cascade.
//!
//! JS: css-cascade.mjs#expandStaticBoxValues, #parseStaticBorder,
//! #parseStaticFont, #parseStaticTransition, #parseStaticAnimation,
//! #expandStaticDeclaration

use super::defaults::{is_static_inherited_prop, static_default_style};
use super::values::{css_prop_to_camel, extract_static_color, split_css_list, split_css_tokens};
use impeccable_core::js;
use once_cell::sync::Lazy;
use regex::Regex;

/// A `[prop, value]` pair as emitted by `expandStaticDeclaration`.
pub type Expanded = (String, String);

/// JS: css-cascade.mjs#expandStaticBoxValues(tokens)
pub fn expand_static_box_values(tokens: &[String]) -> [String; 4] {
    match tokens.len() {
        0 => ["0px".into(), "0px".into(), "0px".into(), "0px".into()],
        1 => [
            tokens[0].clone(),
            tokens[0].clone(),
            tokens[0].clone(),
            tokens[0].clone(),
        ],
        2 => [
            tokens[0].clone(),
            tokens[1].clone(),
            tokens[0].clone(),
            tokens[1].clone(),
        ],
        3 => [
            tokens[0].clone(),
            tokens[1].clone(),
            tokens[2].clone(),
            tokens[1].clone(),
        ],
        _ => [
            tokens[0].clone(),
            tokens[1].clone(),
            tokens[2].clone(),
            tokens[3].clone(),
        ],
    }
}

/// `{ width, color }` from `parseStaticBorder`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StaticBorder {
    pub width: String,
    pub color: String,
}

static BORDER_WIDTH_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^-?[0-9.]+(?:px|rem|em|%)$").expect("BORDER_WIDTH_RE"));

/// JS: css-cascade.mjs#parseStaticBorder(value)
pub fn parse_static_border(value: &str) -> StaticBorder {
    let mut out = StaticBorder::default();
    for token in split_css_tokens(value) {
        if out.width.is_empty() && BORDER_WIDTH_RE.is_match(&token) {
            out.width = token.clone();
        }
        if out.color.is_empty() {
            out.color = extract_static_color(&token);
        }
    }
    out
}

static FONT_SIZE_SLASH_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&format!(
        r"(?:^|{ws})([0-9.]+(?:px|rem|em|%))(?:/([^{wsc}]+))?",
        ws = js::WS,
        wsc = js::WS_CHARS
    ))
    .expect("FONT_SIZE_SLASH_RE")
});
static ITALIC_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)(?-u:\b)italic(?-u:\b)").expect("ITALIC_RE"));
static FONT_WEIGHT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)(?-u:\b)([1-9]00|bold|normal|lighter|bolder)(?-u:\b)").expect("FONT_WEIGHT_RE")
});

/// JS: css-cascade.mjs#parseStaticFont(value)
pub fn parse_static_font(value: &str) -> Vec<Expanded> {
    let mut out: Vec<Expanded> = Vec::new();
    let slash_parts = FONT_SIZE_SLASH_RE.captures(value);
    if ITALIC_RE.is_match(value) {
        out.push(("fontStyle".into(), "italic".into()));
    }
    if let Some(w) = FONT_WEIGHT_RE.captures(value) {
        out.push(("fontWeight".into(), w[1].to_string()));
    }
    if let Some(m) = slash_parts {
        out.push(("fontSize".into(), m[1].to_string()));
        if let Some(lh) = m.get(2) {
            if !lh.as_str().is_empty() {
                out.push(("lineHeight".into(), lh.as_str().to_string()));
            }
        }
        let whole = m.get(0).unwrap().as_str();
        // JS: value.indexOf(slashParts[0]) + slashParts[0].length
        let family_start = match value.find(whole) {
            Some(idx) => idx + whole.len(),
            // indexOf returned -1 in JS: -1 + length; unreachable since the
            // match text is a substring of value.
            None => whole.len().saturating_sub(1),
        };
        let family = js::trim(&value[family_start.min(value.len())..]);
        if !family.is_empty() {
            out.push(("fontFamily".into(), family.to_string()));
        }
    }
    out
}

/// `{ property, timing }` from `parseStaticTransition`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StaticTransition {
    pub property: String,
    pub timing: String,
}

/// `{ name, timing }` from `parseStaticAnimation`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StaticAnimation {
    pub name: String,
    pub timing: String,
}

static TIMING_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)^(?:ease|linear|step-|cubic-bezier\()").expect("TIMING_RE"));
static TRANSITION_PROP_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)^[a-z-]+$").expect("TRANSITION_PROP_RE"));
static TRANSITION_KEYWORD_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?:ease|linear|infinite|alternate|forwards|backwards|both|normal|none)$")
        .expect("TRANSITION_KEYWORD_RE")
});
static ENDS_WITH_S_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"s$").expect("ENDS_WITH_S_RE"));

/// JS: css-cascade.mjs#parseStaticTransition(value)
pub fn parse_static_transition(value: &str) -> StaticTransition {
    let mut props: Vec<String> = Vec::new();
    let mut timings: Vec<String> = Vec::new();
    for item in split_css_list(value) {
        let tokens = split_css_tokens(&item);
        if let Some(timing) = tokens.iter().find(|t| TIMING_RE.is_match(t)) {
            timings.push(timing.clone());
        }
        if let Some(prop) = tokens.iter().find(|t| {
            TRANSITION_PROP_RE.is_match(t)
                && !TRANSITION_KEYWORD_RE.is_match(t)
                && !ENDS_WITH_S_RE.is_match(t)
        }) {
            props.push(prop.clone());
        }
    }
    StaticTransition {
        property: props.join(", "),
        timing: timings.join(", "),
    }
}

static ANIMATION_NAME_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)^[a-z_-][0-9A-Za-z_-]*$").expect("ANIMATION_NAME_RE"));
static ANIMATION_KEYWORD_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^(?:ease|linear|infinite|alternate|forwards|backwards|both|normal|none|running|paused)$",
    )
    .expect("ANIMATION_KEYWORD_RE")
});

/// JS: css-cascade.mjs#parseStaticAnimation(value)
pub fn parse_static_animation(value: &str) -> StaticAnimation {
    let mut names: Vec<String> = Vec::new();
    let mut timings: Vec<String> = Vec::new();
    for item in split_css_list(value) {
        let tokens = split_css_tokens(&item);
        if let Some(timing) = tokens.iter().find(|t| TIMING_RE.is_match(t)) {
            timings.push(timing.clone());
        }
        if let Some(name) = tokens
            .iter()
            .find(|t| ANIMATION_NAME_RE.is_match(t) && !ANIMATION_KEYWORD_RE.is_match(t))
        {
            names.push(name.clone());
        }
    }
    StaticAnimation {
        name: names.join(", "),
        timing: timings.join(", "),
    }
}

static BG_IMAGE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)gradient|url\(").expect("BG_IMAGE_RE"));
static BG_IMAGE_SPLIT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)(?:repeating-)?(?:linear|radial|conic)-gradient\(|url\(")
        .expect("BG_IMAGE_SPLIT_RE")
});
static VAR_ANYWHERE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)var\(").expect("VAR_ANYWHERE_RE"));
static OUTLINE_STYLE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)^(none|hidden|solid|dashed|dotted|double|groove|ridge|inset|outset)$")
        .expect("OUTLINE_STYLE_RE")
});
static ZERO_LENGTH_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^0(?:px|rem|em|%)?$").expect("ZERO_LENGTH_RE"));
static BORDER_SIDE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^border-(top|right|bottom|left)$").expect("BORDER_SIDE_RE"));

fn box4(names: [&str; 4], vals: [String; 4]) -> Vec<Expanded> {
    let [a, b, c, d] = vals;
    vec![
        (names[0].to_string(), a),
        (names[1].to_string(), b),
        (names[2].to_string(), c),
        (names[3].to_string(), d),
    ]
}

static BG_FUNC_START_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)(?:repeating-)?(?:linear|radial|conic)-gradient\s*\(|url\s*\(")
        .expect("BG_FUNC_START_RE")
});

/// `bytes[i]` starts a UTF-8 sequence; the byte length of a `\` escape's
/// target must be counted in characters, or a later slice lands mid-char.
fn utf8_char_len(first: u8) -> usize {
    if first < 0x80 {
        1
    } else if first >> 5 == 0b110 {
        2
    } else if first >> 4 == 0b1110 {
        3
    } else if first >> 3 == 0b11110 {
        4
    } else {
        1
    }
}

/// Index just past a backslash escape at `i` (`bytes[i] == b'\\'`).
fn skip_css_escape(bytes: &[u8], i: usize) -> usize {
    let after = i + 1;
    if after >= bytes.len() {
        return after;
    }
    after + utf8_char_len(bytes[after])
}

/// Index just past the quoted string opening at `i` (its closing quote
/// included); an unterminated string ends at the last byte.
fn skip_css_string(bytes: &[u8], i: usize) -> usize {
    let quote = bytes[i];
    let mut j = i + 1;
    while j < bytes.len() && bytes[j] != quote {
        if bytes[j] == b'\\' {
            j = skip_css_escape(bytes, j);
        } else {
            j += 1;
        }
    }
    (j + 1).min(bytes.len())
}

/// `value` with every `url(...)` and gradient function removed, parens
/// balanced, so a color can be read wherever it sits outside them, before or
/// after the image. Quotes and backslash escapes count: in
/// `url("photo)red.png")` and the parser-emitted `url(photo\)red.png)` the
/// inner `)` belongs to the filename, not to the function boundary. Byte
/// indexing is safe: every slice point is an ASCII paren, quote or the end
/// of the string.
fn strip_background_image_functions(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut rest = value;
    while let Some(m) = BG_FUNC_START_RE.find(rest) {
        out.push_str(&rest[..m.start()]);
        let bytes = rest.as_bytes();
        let mut depth = 0usize;
        let mut idx = m.start();
        while idx < bytes.len() {
            match bytes[idx] {
                b'\\' => {
                    idx = skip_css_escape(bytes, idx);
                    continue;
                }
                b'"' | b'\'' => {
                    idx = skip_css_string(bytes, idx);
                    continue;
                }
                b'(' => depth += 1,
                b')' => {
                    if depth > 0 {
                        depth -= 1;
                    }
                    if depth == 0 {
                        idx += 1;
                        break;
                    }
                }
                _ => {}
            }
            idx += 1;
        }
        rest = &rest[idx.min(rest.len())..];
    }
    out.push_str(rest);
    out
}

/// A shorthand's layers: split on commas outside parens and quotes, so
/// `url(a,b.png) center / cover, var(--bg)` is two layers and the comma in
/// the filename is not one.
fn split_background_layers(value: &str) -> Vec<&str> {
    let bytes = value.as_bytes();
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut depth = 0usize;
    let mut i = 0usize;
    while i < bytes.len() {
        match bytes[i] {
            b'\\' => {
                i = skip_css_escape(bytes, i);
                continue;
            }
            b'"' | b'\'' => {
                i = skip_css_string(bytes, i);
                continue;
            }
            b'(' => depth += 1,
            b')' => {
                if depth > 0 {
                    depth -= 1;
                }
            }
            b',' if depth == 0 => {
                parts.push(&value[start..i]);
                start = i + 1;
            }
            _ => {}
        }
        i += 1;
    }
    parts.push(&value[start..]);
    parts
}

/// Index just past the balanced `var(...)` whose `(` sits at `open_paren`;
/// the end of the string when the parens never close.
fn var_span_end(bytes: &[u8], open_paren: usize) -> usize {
    let mut depth = 0usize;
    let mut i = open_paren;
    while i < bytes.len() {
        match bytes[i] {
            b'\\' => {
                i = skip_css_escape(bytes, i);
                continue;
            }
            b'"' | b'\'' => {
                i = skip_css_string(bytes, i);
                continue;
            }
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return i + 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    bytes.len()
}

/// `rem` with every `var(...)` span blanked, so a color read from it can
/// never be plucked out of a custom-property name (`var(--red)` reads as
/// `red`, which the author never wrote).
fn without_var_spans(rem: &str) -> String {
    let mut out = String::with_capacity(rem.len());
    let mut rest = rem;
    while let Some(m) = VAR_ANYWHERE_RE.find(rest) {
        out.push_str(&rest[..m.start()]);
        out.push(' ');
        let end = var_span_end(rest.as_bytes(), m.end() - 1);
        rest = &rest[end.min(rest.len())..];
    }
    out.push_str(rest);
    out
}

/// The var() span whose own resolved value reads as a color — the token
/// that actually produced the layer's color. With several var()s in one
/// layer the first is often a position or size, and storing that as
/// `backgroundColor` would make compute resolve a non-color. Returns the
/// span's start offset alongside it so callers can check its slot.
fn color_var_span(
    rem: &str,
    root: &impeccable_core::checks::css_scan::CustomProps,
) -> Option<(usize, String)> {
    let lookup = |name: &str| root.get(name).cloned();
    let mut rest = rem;
    let mut base = 0usize;
    while let Some(m) = VAR_ANYWHERE_RE.find(rest) {
        let end = var_span_end(rest.as_bytes(), m.end() - 1).min(rest.len());
        let span = &rest[m.start()..end];
        let value = impeccable_core::checks::measures::resolve_var_refs(span, &lookup, 0);
        if !extract_static_color(&value).is_empty() {
            return Some((base + m.start(), span.to_string()));
        }
        base += end;
        rest = &rest[end..];
    }
    None
}

/// Whether the var() starting at `start` occupies the layer's size slot —
/// right after the first `/`, with only whitespace before it. A color there
/// (`center / red`) makes the browser drop the declaration rather than
/// paint it.
fn var_in_size_slot(rem: &str, start: usize) -> bool {
    match rem.find('/') {
        Some(slash) if start > slash => rem[slash + 1..start].trim().is_empty(),
        _ => false,
    }
}

/// Whether a bare token after the layer's slash could be part of a
/// background size (a length, a percentage, or a size keyword).
fn looks_like_bg_size(tok: &str) -> bool {
    const UNITS: [&str; 15] = [
        "px", "rem", "em", "ex", "ch", "vw", "vh", "vmin", "vmax", "cm", "mm", "in", "pt", "pc",
        "q",
    ];
    if matches!(
        tok,
        "auto" | "cover" | "contain" | "min-content" | "max-content"
    ) {
        return true;
    }
    let t = tok.trim_start_matches('+').trim_start_matches('-');
    if let Some(n) = t.strip_suffix('%') {
        return n.parse::<f64>().is_ok();
    }
    UNITS
        .iter()
        .any(|u| t.strip_suffix(u).is_some_and(|n| n.parse::<f64>().is_ok()))
}

/// Whether every var() in `rem` sits within the layer's size — a size is
/// one or two tokens after the `/`, so `center / var(--w) var(--h)` counts
/// both, while a third token has left the size for the color region. No
/// var may sit in the position slot before the slash. Such vars carry no
/// surface of their own.
fn vars_only_in_size_slot(rem: &str) -> bool {
    let Some(slash) = rem.find('/') else {
        return false;
    };
    if VAR_ANYWHERE_RE.find_iter(rem).any(|m| m.start() < slash) {
        return false;
    }
    let after = &rem[slash + 1..];
    let mut i = 0usize;
    let mut tokens = 0usize;
    while i < after.len() {
        i += after[i..].len() - after[i..].trim_start().len();
        if i >= after.len() {
            break;
        }
        let rest = &after[i..];
        let tok_end = match VAR_ANYWHERE_RE.find(rest) {
            Some(m) if m.start() == 0 => {
                var_span_end(after.as_bytes(), i + m.end() - 1).min(after.len())
            }
            _ => {
                let end = rest.find(char::is_whitespace).unwrap_or(rest.len());
                if !looks_like_bg_size(&rest[..end]) {
                    return false;
                }
                i + end
            }
        };
        tokens += 1;
        if tokens > 2 {
            return false;
        }
        i = tok_end;
    }
    tokens > 0
}

/// The `backgroundColor` a `background` shorthand with an image implies
/// (issue #964): `.hero { background: #111 }` then
/// `.hero.photo { background: url(photo.jpg) center / cover }` paints
/// transparent under the image in a browser, but the static cascade kept
/// `#111`. Entered beside the expansion in `apply_static_declaration`, never
/// inside `expand_static_declaration` (pinned by recorded vectors), carrying
/// the shorthand's cascade metadata so later rules still win.
///
/// Returns `[prop, value, isReset]`:
/// - a color the author wrote — before the image, after it, or in another
///   layer — is a real surface; it enters without the reset flag, so the
///   hover pass never treats it as auto-cleared.
/// - only the implied `transparent` carries `isReset`.
///
/// `root` supplies custom properties for a decision-time substitution
/// (`var()` colors are recognized instead of vetoing the declaration), but
/// the author's spelling is what gets stored: the compute pass re-resolves
/// a stored `var()` against the element's own custom properties, which may
/// scope a different value than the stylesheet-wide first one seen here.
/// A var() that survives resolution anywhere in a layer may still be a
/// color or a size — the surface is unknown, so the cascade is left alone.
pub fn expand_background_color_reset(
    prop: &str,
    value: &str,
    root: &impeccable_core::checks::css_scan::CustomProps,
) -> Vec<(String, String, bool)> {
    if js::to_lower_case(prop) != "background" {
        return Vec::new();
    }
    let v = js::trim(value);
    if v.is_empty() || !BG_IMAGE_RE.is_match(v) {
        return Vec::new();
    }
    let resolved = if VAR_ANYWHERE_RE.is_match(v) {
        let lookup = |name: &str| root.get(name).cloned();
        impeccable_core::checks::measures::resolve_var_refs(v, &lookup, 0)
    } else {
        v.to_string()
    };
    let layers_v = split_background_layers(v);
    let layers_r = split_background_layers(&resolved);
    // A custom property whose value itself contains a comma changes the
    // layer count; mapping a color back to its layer stops being reliable,
    // so leave the cascade alone.
    if layers_v.len() != layers_r.len() {
        return Vec::new();
    }
    let mut found: Option<String> = None;
    for (layer_v, layer_r) in layers_v.iter().zip(layers_r.iter()) {
        let rem_r = strip_background_image_functions(layer_r);
        if VAR_ANYWHERE_RE.is_match(&rem_r) {
            return Vec::new();
        }
        let rem_v = strip_background_image_functions(layer_v);
        let color_r = extract_static_color(&rem_r);
        if !VAR_ANYWHERE_RE.is_match(&rem_v) {
            if color_r.is_empty() {
                continue;
            }
            found = Some(color_r);
            continue;
        }
        // The layer holds var()s; none survive resolution (checked above).
        if color_r.is_empty() {
            if vars_only_in_size_slot(&rem_v) {
                // A size-only var resolved to a non-color: the layer carries
                // no surface, so the implied transparent stands.
                continue;
            }
            // The element may scope a color here (`.hero { --bg: red }` over
            // a stylesheet `--bg: cover`), and a leading var() already
            // survives in the frozen expansion — keep what the cascade has.
            return Vec::new();
        }
        // Store a color only in the spelling the author wrote: a literal
        // outside the var()s, else the var() that actually carries the
        // color — never a token read from inside a custom-property name,
        // and never a position or size var that happens to come first.
        let literal = extract_static_color(&without_var_spans(&rem_v));
        if !literal.is_empty() {
            found = Some(literal);
        } else {
            match color_var_span(&rem_v, root) {
                // A color in the size slot makes the browser drop the
                // declaration rather than paint it; the prior rule stands.
                Some((start, _)) if var_in_size_slot(&rem_v, start) => return Vec::new(),
                Some((_, span)) => found = Some(span),
                // No var() carries a color after all; leave the cascade
                // alone rather than guess.
                None => return Vec::new(),
            }
        }
    }
    match found {
        // A color after the image (`url(photo.jpg) #111`) never reaches the
        // frozen expansion, which reads only before the first image; write
        // it here so a prior rule's surface does not outlive it.
        Some(color) => vec![("backgroundColor".into(), color, false)],
        None => vec![("backgroundColor".into(), "rgba(0, 0, 0, 0)".into(), true)],
    }
}

/// JS: css-cascade.mjs#expandStaticDeclaration(prop, value)
pub fn expand_static_declaration(prop: &str, value: &str) -> Vec<Expanded> {
    let p = js::to_lower_case(prop);
    let v = js::trim(value);
    if v.is_empty() {
        return Vec::new();
    }
    if p.starts_with("--") {
        return vec![(p, v.to_string())];
    }
    if p == "background" {
        let mut out: Vec<Expanded> = Vec::new();
        let has_image = BG_IMAGE_RE.is_match(v);
        if has_image {
            out.push(("backgroundImage".into(), v.to_string()));
        }
        let before_image: &str = if has_image {
            match BG_IMAGE_SPLIT_RE.find(v) {
                Some(m) => &v[..m.start()],
                None => v,
            }
        } else {
            v
        };
        let color = extract_static_color(if has_image { before_image } else { v });
        if !color.is_empty() {
            out.push(("backgroundColor".into(), color.clone()));
        }
        // The `background` shorthand resets every longhand it does not set.
        // Without this, `pre code { background: none }` leaves an earlier
        // `background: var(--surface)` color standing and the contrast checks
        // measure text against a surface the browser never paints. var() values
        // stay untouched: they may resolve to a color later in the pipeline.
        if color.is_empty() && !has_image && !VAR_ANYWHERE_RE.is_match(v) {
            out.push(("backgroundColor".into(), "rgba(0, 0, 0, 0)".into()));
            out.push(("backgroundImage".into(), "none".into()));
        }
        return out;
    }
    if p == "border" {
        let parsed = parse_static_border(v);
        let mut out: Vec<Expanded> = Vec::new();
        for side in ["Top", "Right", "Bottom", "Left"] {
            if !parsed.width.is_empty() {
                out.push((format!("border{}Width", side), parsed.width.clone()));
            }
            if !parsed.color.is_empty() {
                out.push((format!("border{}Color", side), parsed.color.clone()));
            }
        }
        return out;
    }
    if p == "outline" {
        // `outline` shorthand: width | style | color, in any order. Reuse the
        // border parser for width + color, then sniff a style keyword from the
        // tokens (solid|dashed|...). `outline: 0` (single-token zero) zeros
        // the width and effectively hides the outline.
        let tokens = split_css_tokens(v);
        let parsed = parse_static_border(v);
        let style_token = tokens.iter().find(|t| OUTLINE_STYLE_RE.is_match(t));
        let mut out: Vec<Expanded> = Vec::new();
        if !parsed.width.is_empty() {
            out.push(("outlineWidth".into(), parsed.width.clone()));
        }
        if !parsed.color.is_empty() {
            out.push(("outlineColor".into(), parsed.color.clone()));
        }
        if let Some(st) = style_token {
            out.push(("outlineStyle".into(), js::to_lower_case(st)));
        }
        // `outline: 0` with no other tokens: explicit zero width.
        if parsed.width.is_empty() && ZERO_LENGTH_RE.is_match(js::trim(v)) {
            out.push(("outlineWidth".into(), "0px".into()));
        }
        return out;
    }
    if let Some(m) = BORDER_SIDE_RE.captures(&p) {
        let parsed = parse_static_border(v);
        let raw_side = &m[1];
        let mut side = String::new();
        let mut chars = raw_side.chars();
        if let Some(first) = chars.next() {
            side.push_str(&first.to_uppercase().to_string());
            side.push_str(chars.as_str());
        }
        let mut out: Vec<Expanded> = Vec::new();
        if !parsed.width.is_empty() {
            out.push((format!("border{}Width", side), parsed.width.clone()));
        }
        if !parsed.color.is_empty() {
            out.push((format!("border{}Color", side), parsed.color.clone()));
        }
        return out;
    }
    if p == "border-width" {
        let vals = expand_static_box_values(&split_css_tokens(v));
        return box4(
            [
                "borderTopWidth",
                "borderRightWidth",
                "borderBottomWidth",
                "borderLeftWidth",
            ],
            vals,
        );
    }
    if p == "border-color" {
        let vals = expand_static_box_values(&split_css_tokens(v));
        return box4(
            [
                "borderTopColor",
                "borderRightColor",
                "borderBottomColor",
                "borderLeftColor",
            ],
            vals,
        );
    }
    if p == "padding" {
        let vals = expand_static_box_values(&split_css_tokens(v));
        return box4(
            ["paddingTop", "paddingRight", "paddingBottom", "paddingLeft"],
            vals,
        );
    }
    if p == "margin" {
        let vals = expand_static_box_values(&split_css_tokens(v));
        return box4(
            ["marginTop", "marginRight", "marginBottom", "marginLeft"],
            vals,
        );
    }
    if p == "font" {
        return parse_static_font(v);
    }
    if p == "transition" {
        let parsed = parse_static_transition(v);
        let mut out: Vec<Expanded> = Vec::new();
        if !parsed.property.is_empty() {
            out.push(("transitionProperty".into(), parsed.property));
        }
        if !parsed.timing.is_empty() {
            out.push(("transitionTimingFunction".into(), parsed.timing));
        }
        return out;
    }
    if p == "animation" {
        let parsed = parse_static_animation(v);
        let mut out: Vec<Expanded> = Vec::new();
        if !parsed.name.is_empty() {
            out.push(("animationName".into(), parsed.name));
        }
        if !parsed.timing.is_empty() {
            out.push(("animationTimingFunction".into(), parsed.timing));
        }
        return out;
    }
    let mapped = css_prop_to_camel(&p);
    if static_default_style(&mapped).is_some() || is_static_inherited_prop(&mapped) {
        return vec![(mapped, v.to_string())];
    }
    Vec::new()
}
