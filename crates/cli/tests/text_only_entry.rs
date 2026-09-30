//! A first viewport built entirely in code (no raster region) through the real
//! browser: native capture, the hero gate's readings, and an accepted review.
use impeccable::entry_capture::CdpEntryRenderer;
use impeccable::reviewed_entry::ReviewedEntryRenderer;
use impeccable_comp::{png_io, raster};
use impeccable_comp_verbs::asset_capture::capture_sha256;
use impeccable_comp_verbs::entry_capture::{EntryRenderer, EntryRequest, EntryStage};
use serde_json::{Value, json};
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

static NEXT: AtomicUsize = AtomicUsize::new(0);
const SPEC: &str = ".impeccable/build/spec.json";
const PAGE: &str = "<!doctype html><style>html,body{margin:0;background:#f4f4f0;font:16px/1.2 sans-serif}\
header{position:absolute;left:16px;top:16px;width:208px;height:24px;background:#181c24}\
h1{position:absolute;left:16px;top:64px;width:208px;height:40px;margin:0;background:#1e5ac8}</style>\
<header></header><h1></h1>";

struct Fixture {
    dir: PathBuf,
    project: PathBuf,
    home: PathBuf,
}
impl Fixture {
    fn new() -> Self {
        let dir = std::env::temp_dir().join(format!(
            "text-only-entry-{}-{}-{}",
            std::process::id(),
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let project = dir.join("project");
        let home = dir.join("home");
        fs::create_dir_all(project.join(".impeccable/build")).unwrap();
        fs::create_dir_all(project.join(".impeccable/review")).unwrap();
        fs::create_dir_all(&home).unwrap();
        fs::write(project.join("index.html"), PAGE).unwrap();
        let comp = png_io::encode_png(&raster::create_image(240, 160, [244, 244, 240, 255]), &[]).unwrap();
        fs::write(project.join("comp.png"), comp).unwrap();
        let spec = json!({"comp":"comp.png","compSize":{"width":240,"height":160},"regions":[
            {"id":"topbar","kind":"text","medium":"semantic","note":"dark top bar with the product name","type":{},
             "box":{"x":16./240.,"y":0.1,"w":208./240.,"h":0.15},"px":{"x":16,"y":16,"w":208,"h":24}},
            {"id":"headline","kind":"text","medium":"semantic","note":"blue headline block","type":{},
             "box":{"x":16./240.,"y":0.4,"w":208./240.,"h":0.25},"px":{"x":16,"y":64,"w":208,"h":40}}]});
        fs::write(project.join(SPEC), serde_json::to_vec_pretty(&spec).unwrap()).unwrap();
        Self { dir, project, home }
    }
    fn request(&self, stage: EntryStage) -> EntryRequest {
        EntryRequest {
            root: self.project.clone(),
            artifact: "index.html".into(),
            spec: SPEC.into(),
            reference: "comp.png".into(),
            stage,
        }
    }
    fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_impeccable"))
            .args(args)
            .current_dir(&self.project)
            .env("HOME", &self.home)
            .env("USERPROFILE", &self.home)
            .env("IMPECCABLE_NATIVE_CAPTURE", "1")
            .env_remove("IMPECCABLE_COMPONENT_REVIEW_TOOL")
            .env_remove("IMPECCABLE_COMPONENT_REVIEW_SESSIONS")
            .env_remove("IMPECCABLE_CAPTURE_PORT")
            .env_remove("IMPECCABLE_CAPTURE_CAPABILITY")
            .output()
            .unwrap()
    }
    fn record_hero(&self) -> (bool, String, Value) {
        let out = self.run(&["build-phase", "record", "hero", "--min", "0.95"]);
        let text = format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
        let report = fs::read(self.project.join(".impeccable/review/diff/hero/report.json"))
            .ok()
            .and_then(|b| serde_json::from_slice(&b).ok())
            .unwrap_or(Value::Null);
        (out.status.code() == Some(0), text, report)
    }
    /// The local review store entry for an approved assembled first viewport,
    /// in the shape the component-review capture writes.
    fn approve(&self, screenshot: &[u8]) {
        fs::write(self.project.join(".impeccable/review/hero.json"), br#"{"id":"hero"}"#).unwrap();
        let project = self.project.canonicalize().unwrap();
        let key = capture_sha256(format!("{}\0hero", project.display()).as_bytes());
        let session = self.home.join(".impeccable/component-reviews").join(key);
        fs::create_dir_all(session.join("blobs")).unwrap();
        let png = capture_sha256(screenshot);
        fs::write(session.join("blobs").join(&png), screenshot).unwrap();
        let sources = json!({
            "index.html": capture_sha256(&fs::read(self.project.join("index.html")).unwrap()),
            "comp.png": capture_sha256(&fs::read(self.project.join("comp.png")).unwrap()),
        });
        let capture = json!({"schema":"native-component-previews-v1","components":[{"views":{"preview":{"kind":"assembled-page","entry":"index.html","viewport":{"width":240,"height":160,"dpr":1},"screenshotSha256":png}}}]});
        let state = json!({"sources":sources,"files":{"preview.png":png},"capture":capture,
            "packet":{"stage":"hero","id":"hero","revision":"rev","comp":{"width":240,"height":160,"url":"/files/rev/comp.png"},
                "components":[{"box":{"x":0,"y":0,"w":1,"h":1},"preview":{"sourceKind":"page","url":"/files/rev/preview.png"}}]},
            "receipt":{"visualDecision":"approved","captureVerified":true,"capture":capture,"submission":{"requestId":"hero","packetRevision":"rev"}}});
        fs::write(session.join("current.json"), serde_json::to_vec(&state).unwrap()).unwrap();
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

fn browser_available() -> bool {
    if impeccable_browser::discovery::find_browser(&std::env::vars().collect()).is_err() {
        eprintln!("skip: browser unavailable");
        return false;
    }
    true
}

#[test]
fn text_only_entry_captures_every_frame_without_region_receipts() {
    if !browser_available() {
        return;
    }
    let f = Fixture::new();
    for stage in [EntryStage::Hero, EntryStage::Responsive] {
        let captured = CdpEntryRenderer.capture_entry(&f.request(stage)).unwrap();
        captured.verify_current().unwrap();
        let evidence = captured.evidence();
        assert_eq!(evidence.report["captureMethod"], "assembled-page-viewport");
        let names: Vec<_> = evidence.frames.iter().map(|f| f.name.as_str()).collect();
        match stage {
            EntryStage::Hero => assert_eq!(names, ["hero"]),
            EntryStage::Responsive => assert_eq!(names, ["desktop", "mobile"]),
        }
        for frame in &evidence.frames {
            assert!(frame.regions.is_empty());
            let image = png_io::decode_png(&frame.png).unwrap().image;
            let expected = match frame.name.as_str() {
                "hero" => (240, 160),
                "desktop" => (1440, 960),
                "mobile" => (390, 844),
                _ => unreachable!(),
            };
            assert_eq!((image.width, image.height), expected);
            assert_eq!(evidence.report["frameProofs"][&frame.name]["kind"], "assembled-page");
            if frame.name == "hero" {
                // The headline block is drawn where the page puts it.
                let p = (80 * image.width + 120) * 4;
                assert_eq!(&image.data[p..p + 3], &[0x1e, 0x5a, 0xc8]);
            }
        }
        // Any bound input that changes invalidates the capture.
        fs::write(f.project.join("index.html"), format!("{PAGE}<p>edited</p>")).unwrap();
        assert!(captured.verify_current().is_err());
        fs::write(f.project.join("index.html"), PAGE).unwrap();
    }
}

#[test]
fn hero_gate_reads_a_text_only_first_viewport_and_honours_an_accepted_review() {
    if !browser_available() {
        return;
    }
    let f = Fixture::new();
    // Comp and build both set the top bar and headline as lettering, the comp in
    // horizontal strokes and the build in vertical ones, so the build's text
    // regions read as contradicted while no ink is invented.
    let strokes = |angle: &str| PAGE
        .replace("background:#181c24", &format!("background:repeating-linear-gradient({angle},#181c24 0 2px,#f4f4f0 2px 4px)"))
        .replace("background:#1e5ac8", &format!("background:repeating-linear-gradient({angle},#1e5ac8 0 2px,#f4f4f0 2px 4px)"));
    fs::write(f.project.join("index.html"), strokes("180deg")).unwrap();
    let comp = CdpEntryRenderer.capture_entry(&f.request(EntryStage::Hero)).unwrap().evidence().frames[0].png.clone();
    fs::write(f.project.join("comp.png"), comp).unwrap();
    let lettered = strokes("90deg");
    fs::write(f.project.join("index.html"), &lettered).unwrap();
    let hero = CdpEntryRenderer.capture_entry(&f.request(EntryStage::Hero)).unwrap().evidence().frames[0].png.clone();
    let start = f.run(&["build-phase", "start", "--comp", "comp.png", "--artifact", "index.html"]);
    assert!(start.status.success(), "{}", String::from_utf8_lossy(&start.stderr));

    // The gate measures the page and fails on its readings, not on capture.
    let (ok, text, report) = f.record_hero();
    assert!(!ok, "{text}");
    assert!(!text.contains("capture unavailable"), "{text}");
    assert!(report["regions"].as_array().is_some_and(|r| r.iter().any(|r| r["id"] == "headline")), "{report}");
    assert_eq!(report["nativeCapture"]["inputs"]["captureMethod"], "assembled-page-viewport");

    // The same page accepted by the user in the first-viewport review binds by
    // pixels and ends the numeric fight; the material gates still ran.
    f.approve(&hero);
    let (ok, text, report) = f.record_hero();
    assert!(ok, "{text}");
    assert_eq!(report["humanHeroReview"]["viewportAccepted"], true, "{report}");
    assert_eq!(report["nativeCapture"]["inputs"]["humanTextReview"]["schema"], "human-assembled-reference-v1");

    // A visible change after acceptance lapses it again.
    fs::write(f.project.join("index.html"), lettered.replace("top:64px", "top:112px")).unwrap();
    let (ok, text, report) = f.record_hero();
    assert!(!ok, "{text}");
    assert_eq!(report["humanHeroReview"]["viewportAccepted"], false, "{report}");
}

#[test]
fn reviewed_renderer_binds_an_approved_text_only_viewport() {
    if !browser_available() {
        return;
    }
    let f = Fixture::new();
    let hero = CdpEntryRenderer.capture_entry(&f.request(EntryStage::Hero)).unwrap().evidence().frames[0].png.clone();
    f.approve(&hero);
    let renderer = ReviewedEntryRenderer::local(&f.project, Some(&f.home));
    let captured = renderer.capture_entry(&f.request(EntryStage::Hero)).unwrap();
    let approved = captured.approved_reference().expect("approved viewport binds");
    assert_eq!(approved.proof["schema"], "human-assembled-reference-v1");
    // Same capture method, same page: the approved pixels are the current frame's.
    let a = png_io::decode_png(&approved.png).unwrap().image;
    let b = png_io::decode_png(&captured.evidence().frames[0].png).unwrap().image;
    assert_eq!((a.width, a.height, &a.data), (b.width, b.height, &b.data));
    captured.verify_current().unwrap();
}
