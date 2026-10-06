//! Unit tests for the cascade helpers without recorded vectors. Expected
//! values were produced by running the JS `css-cascade.mjs` in Node.

use impeccable_core::checks::css_scan::CustomProps as RootProps;
use impeccable_html::cascade::checks_shim::{resolve_length_px, resolve_var_refs, CustomProps};
use impeccable_html::cascade::rules::{
    apply_static_declaration, parse_static_style_attribute, DeclMeta, SpecifiedStore,
};
use impeccable_html::cascade::values::{normalize_color_for_check, unwrap_css_at_layer};

fn root(vars: &[(&str, &str)]) -> RootProps {
    let mut r = RootProps::new();
    for (k, v) in vars {
        r.set(k, v.to_string());
    }
    r
}

#[test]
fn normalize_color_for_check_matches_node() {
    let cases: &[(&str, &str)] = &[
        ("#ffffff", "rgb(255, 255, 255)"),
        ("#FfF", "rgb(255, 255, 255)"),
        ("#abc", "rgb(170, 187, 204)"),
        ("  #ABCDEF  ", "rgb(171, 205, 239)"),
        ("white", "rgb(255, 255, 255)"),
        ("Black", "rgb(0, 0, 0)"),
        ("GRAY", "rgb(128, 128, 128)"),
        ("grey", "rgb(128, 128, 128)"),
        ("silver", "rgb(192, 192, 192)"),
        ("red", "rgb(255, 0, 0)"),
        ("green", "rgb(0, 128, 0)"),
        ("blue", "rgb(0, 0, 255)"),
        ("yellow", "rgb(255, 255, 0)"),
        ("purple", "purple"),
        ("rgb(1, 2, 3)", "rgb(1, 2, 3)"),
        ("oklch(50% 0.1 20)", "oklch(50% 0.1 20)"),
        ("#abcd", "#abcd"),
        ("#12345", "#12345"),
        ("", ""),
        ("   ", ""),
        ("var(--x)", "var(--x)"),
        ("transparent", "transparent"),
    ];
    for (input, expected) in cases {
        assert_eq!(
            normalize_color_for_check(input),
            *expected,
            "input {:?}",
            input
        );
    }
}

fn meta(important: bool, specificity: [u32; 3], order: i64, inline: bool) -> DeclMeta {
    DeclMeta {
        important,
        specificity,
        order,
        inline,
    }
}

#[test]
fn apply_static_declaration_matches_node() {
    let mut specified: SpecifiedStore<&str> = SpecifiedStore::new();
    let node = "n1";
    let root = root(&[]);
    let mut apply = |prop: &str, value: &str, m: DeclMeta| {
        apply_static_declaration(&mut specified, node, prop, value, &m, &root);
    };
    apply("margin", "0", meta(false, [0, 0, 0], 0, false));
    apply("margin-top", "5px", meta(false, [0, 1, 0], 1, false));
    apply("margin", "10px 20px", meta(false, [0, 0, 1], 2, false));
    apply("color", "red", meta(true, [0, 0, 0], 3, false));
    apply("color", "blue", meta(false, [1, 0, 0], 4, true));
    apply("background", "var(--x)", meta(false, [0, 1, 0], 5, false));
    apply("background", "none", meta(false, [0, 1, 0], 6, false));
    apply("--brand", "#fff", meta(false, [0, 1, 0], 7, false));
    apply(
        "font",
        "italic 700 12px/1.4 Inter, sans-serif",
        meta(false, [0, 1, 0], 8, false),
    );
    apply("box-sizing", "border-box", meta(false, [0, 1, 0], 9, false));
    apply("margin-top", "1px", meta(false, [0, 0, 0], 10, false));
    apply("margin-top", "2px", meta(true, [0, 0, 0], 11, false));
    apply("margin-top", "3px", meta(false, [9, 9, 9], 12, true));
    apply("outline", "0", meta(false, [0, 0, 0], 13, false));
    apply(
        "border-left",
        "3px solid teal",
        meta(false, [0, 0, 0], 14, false),
    );

    let map = specified.get(&node).expect("node entry");
    let got: Vec<(String, bool, [u32; 3], i64, bool, String)> = map
        .iter()
        .map(|(k, d)| {
            (
                k.clone(),
                d.meta.important,
                d.meta.specificity,
                d.meta.order,
                d.meta.inline,
                d.value.clone(),
            )
        })
        .collect();
    let s = |x: &str| x.to_string();
    let expected = vec![
        (s("marginTop"), true, [0, 0, 0], 11, false, s("2px")),
        (s("marginRight"), false, [0, 0, 1], 2, false, s("20px")),
        (s("marginBottom"), false, [0, 0, 1], 2, false, s("10px")),
        (s("marginLeft"), false, [0, 0, 1], 2, false, s("20px")),
        (s("color"), true, [0, 0, 0], 3, false, s("red")),
        (
            s("backgroundColor"),
            false,
            [0, 1, 0],
            6,
            false,
            s("rgba(0, 0, 0, 0)"),
        ),
        (s("backgroundImage"), false, [0, 1, 0], 6, false, s("none")),
        (s("--brand"), false, [0, 1, 0], 7, false, s("#fff")),
        (s("fontStyle"), false, [0, 1, 0], 8, false, s("italic")),
        (s("fontWeight"), false, [0, 1, 0], 8, false, s("700")),
        (s("fontSize"), false, [0, 1, 0], 8, false, s("12px")),
        (s("lineHeight"), false, [0, 1, 0], 8, false, s("1.4")),
        (
            s("fontFamily"),
            false,
            [0, 1, 0],
            8,
            false,
            s("Inter, sans-serif"),
        ),
        (s("outlineWidth"), false, [0, 0, 0], 13, false, s("0px")),
        (s("borderLeftWidth"), false, [0, 0, 0], 14, false, s("3px")),
        (s("borderLeftColor"), false, [0, 0, 0], 14, false, s("teal")),
        (
            s("borderLeftStyle"),
            false,
            [0, 0, 0],
            14,
            false,
            s("solid"),
        ),
    ];
    assert_eq!(got, expected);
    // prop is the expanded property name
    assert_eq!(map.get("marginTop").unwrap().prop, "marginTop");
}

#[test]
fn parse_static_style_attribute_edge_cases_match_node() {
    let decls = parse_static_style_attribute(
        ": x; a:b; c: d !important ; e:f!IMPORTANT; g; h:; :i; j:k:l",
        5,
    );
    let got: Vec<(String, String, bool, i64)> = decls
        .into_iter()
        .map(|d| (d.prop, d.value, d.important, d.order))
        .collect();
    let s = |x: &str| x.to_string();
    assert_eq!(
        got,
        vec![
            (s("a"), s("b"), false, 5),
            (s("c"), s("d"), true, 6),
            (s("e"), s("f"), true, 7),
            (s("h"), s(""), false, 8),
            (s(""), s("i"), false, 9),
            (s("j"), s("k:l"), false, 10),
        ]
    );
}

#[test]
fn unwrap_css_at_layer_shapes() {
    assert_eq!(unwrap_css_at_layer(""), "");
    assert_eq!(unwrap_css_at_layer(".a{color:red}"), ".a{color:red}");
    assert_eq!(
        unwrap_css_at_layer("@layer base { .a{color:red} } .b{c:d}"),
        " .a{color:red}  .b{c:d}"
    );
    assert_eq!(
        unwrap_css_at_layer("@layer{ .a{ .n{x:y} } }@layer a.b { .c{d:e} }"),
        " .a{ .n{x:y} }  .c{d:e} "
    );
    // statement form is untouched
    assert_eq!(
        unwrap_css_at_layer("@layer a, b; .x{y:z}"),
        "@layer a, b; .x{y:z}"
    );
    // unbalanced: source unchanged
    assert_eq!(
        unwrap_css_at_layer("@layer x { .a{color:red}"),
        "@layer x { .a{color:red}"
    );
    // `@layered` does not match the word boundary
    assert_eq!(
        unwrap_css_at_layer("@layered x { .a{c:d} }"),
        "@layered x { .a{c:d} }"
    );
}

#[test]
fn checks_shim_helpers() {
    let mut props = CustomProps::new();
    props.insert("--a".into(), "var(--b)".into());
    props.insert("--b".into(), "#fff".into());
    props.insert("--loop".into(), "var(--loop)".into());
    assert_eq!(resolve_var_refs("var(--a)", &props), "#fff");
    assert_eq!(resolve_var_refs("var( --b , red )", &props), "#fff");
    assert_eq!(resolve_var_refs("var(--missing, red )", &props), "red");
    assert_eq!(
        resolve_var_refs("var(--outer, var(--inner, 0))", &props),
        "0"
    );
    assert_eq!(resolve_var_refs("var(--missing)", &props), "var(--missing)");
    assert_eq!(resolve_var_refs("var(--loop)", &props), "var(--loop)");
    assert_eq!(resolve_var_refs("plain", &props), "plain");
    assert_eq!(resolve_length_px("normal", 16.0), None);
    assert_eq!(resolve_length_px("", 16.0), None);
    assert_eq!(resolve_length_px("abc", 16.0), None);
    assert_eq!(resolve_length_px("12px", 16.0), Some(12.0));
    assert_eq!(resolve_length_px("1.5rem", 10.0), Some(24.0));
    assert_eq!(resolve_length_px("2em", 10.0), Some(20.0));
    assert_eq!(resolve_length_px("50%", 10.0), Some(5.0));
    assert_eq!(resolve_length_px("1.5", 10.0), Some(15.0));
}

#[test]
fn bg_image_shorthand_resets_stale_color_964() {
    let mut s: SpecifiedStore<&str> = SpecifiedStore::new();
    let m1 = meta(false, [0, 1, 0], 0, false);
    let m2 = meta(false, [0, 2, 0], 1, false);
    let root = root(&[]);
    apply_static_declaration(&mut s, "n", "background", "#111", &m1, &root);
    apply_static_declaration(
        &mut s,
        "n",
        "background",
        "url(photo.jpg) center / cover",
        &m2,
        &root,
    );
    let map = s.get(&"n").unwrap();
    assert_eq!(
        map.get("backgroundImage").unwrap().value,
        "url(photo.jpg) center / cover"
    );
    assert_eq!(
        map.get("backgroundColor").unwrap().value,
        "rgba(0, 0, 0, 0)"
    );
    // The implied transparent is the auto-reset, not an author color: the
    // hover pass keys on this flag.
    assert!(map.get("backgroundColor").unwrap().shorthand_reset);
}

/// A color after the image is a real surface (`url(photo.jpg) #111` paints
/// the color beneath it) and must reach `backgroundColor` itself — the
/// frozen expansion never reads past the first image, so without this a
/// prior rule's color would outlive the browser's own paint.
#[test]
fn bg_shorthand_trailing_color_reaches_background_color() {
    let mut s: SpecifiedStore<&str> = SpecifiedStore::new();
    let m1 = meta(false, [0, 1, 0], 0, false);
    let m2 = meta(false, [0, 2, 0], 1, false);
    let root = root(&[]);
    apply_static_declaration(&mut s, "n", "background", "#111", &m1, &root);
    apply_static_declaration(&mut s, "n", "background", "url(photo.jpg) #111", &m2, &root);
    let decl = s.get(&"n").unwrap().get("backgroundColor").unwrap();
    assert_eq!(decl.value, "#111");
    // Author-written, so not flagged as an auto-reset.
    assert!(!decl.shorthand_reset);
}

/// Vars are decided with the stylesheet's custom properties: a resolvable
/// size var lets the reset fire, and a resolvable color var in another
/// layer stops it. Anything still var-shaped after substitution (including
/// post-slash vars, which may legally be colors) leaves the cascade alone.
#[test]
fn bg_shorthand_reset_resolves_vars_layer_by_layer() {
    let m1 = meta(false, [0, 1, 0], 0, false);
    let m2 = meta(false, [0, 2, 0], 1, false);

    // --size: cover resolves, so the image-only layer is transparent.
    let mut s: SpecifiedStore<&str> = SpecifiedStore::new();
    let sized = root(&[("--size", "cover")]);
    apply_static_declaration(&mut s, "n", "background", "#111", &m1, &sized);
    apply_static_declaration(
        &mut s,
        "n",
        "background",
        "url(photo.jpg) center / var(--size)",
        &m2,
        &sized,
    );
    assert_eq!(
        s.get(&"n").unwrap().get("backgroundColor").unwrap().value,
        "rgba(0, 0, 0, 0)"
    );

    // An unresolvable size var may still be a color where it sits: keep
    // the earlier surface rather than wiping it.
    let mut s: SpecifiedStore<&str> = SpecifiedStore::new();
    apply_static_declaration(&mut s, "n", "background", "#111", &m1, &root(&[]));
    apply_static_declaration(
        &mut s,
        "n",
        "background",
        "url(photo.jpg) center / var(--size)",
        &m2,
        &root(&[]),
    );
    assert_eq!(
        s.get(&"n").unwrap().get("backgroundColor").unwrap().value,
        "#111"
    );

    // A color-bearing layer: --bg resolves to red, so no reset wipes it.
    let mut s: SpecifiedStore<&str> = SpecifiedStore::new();
    let colored = root(&[("--bg", "red")]);
    apply_static_declaration(&mut s, "n", "background", "#111", &m1, &colored);
    apply_static_declaration(
        &mut s,
        "n",
        "background",
        "url(a.png) center / cover, var(--bg) url(b.png)",
        &m2,
        &colored,
    );
    // The original spelling is stored; the compute pass re-resolves it
    // against the element's own custom properties.
    assert_eq!(
        s.get(&"n").unwrap().get("backgroundColor").unwrap().value,
        "var(--bg)"
    );

    // The same layer without a resolvable var blocks the reset outright.
    let mut s: SpecifiedStore<&str> = SpecifiedStore::new();
    apply_static_declaration(&mut s, "n", "background", "#111", &m1, &root(&[]));
    apply_static_declaration(
        &mut s,
        "n",
        "background",
        "url(a.png) center / cover, var(--bg) url(b.png)",
        &m2,
        &root(&[]),
    );
    assert_eq!(
        s.get(&"n").unwrap().get("backgroundColor").unwrap().value,
        "#111"
    );

    // The frozen expansion already carries a whole-value var through as the
    // color; the reset stays out of its way entirely.
    let mut s: SpecifiedStore<&str> = SpecifiedStore::new();
    apply_static_declaration(&mut s, "n", "background", "#111", &m1, &root(&[]));
    apply_static_declaration(
        &mut s,
        "n",
        "background",
        "var(--bg) url(y.png)",
        &m2,
        &root(&[]),
    );
    assert_eq!(
        s.get(&"n").unwrap().get("backgroundColor").unwrap().value,
        "var(--bg)"
    );
}

/// A `)` inside an image function is part of the filename — quoted
/// (`url("photo)red.png")`) or parser-escaped (`url(photo\)red.png)`) —
/// never the function's end; mistaking one for a boundary reads `red` as
/// the layer's color and skips a reset that should fire.
#[test]
fn bg_shorthand_reset_survives_parens_inside_filenames() {
    let m1 = meta(false, [0, 1, 0], 0, false);
    let m2 = meta(false, [0, 2, 0], 1, false);
    for image in [
        "url(\"photo)red.png\") center / cover",
        "url(photo\\)red.png) center / cover",
    ] {
        let mut s: SpecifiedStore<&str> = SpecifiedStore::new();
        apply_static_declaration(&mut s, "n", "background", "#111", &m1, &root(&[]));
        apply_static_declaration(&mut s, "n", "background", image, &m2, &root(&[]));
        assert_eq!(
            s.get(&"n").unwrap().get("backgroundColor").unwrap().value,
            "rgba(0, 0, 0, 0)",
            "image {:?}",
            image
        );
    }
}
