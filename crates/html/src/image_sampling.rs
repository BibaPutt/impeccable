//! The pixels behind image-backed text for the static engine (#560). A
//! `url()` resolves to bytes from a local file relative to the page (a
//! stylesheet linked from another directory has its urls restated relative
//! to the page as its rules are read, see `cascade::UrlBase`) or from a
//! base64 data URI, never from the network. The pure-Rust decoders turn them
//! into a raster no larger than the browser overlay's 640px canvas.
//!
//! The hook runs this on every edit, so nothing is done twice: an element's
//! image is located and its header read once per document, a url that cannot
//! be read is remembered as such, and a decoded raster is shared for the rest
//! of the process, so a directory scan decodes each hero once. Anything
//! unreadable, remote, oversized, or undecodable is `None`, and the caller
//! keeps today's skip.

use crate::background::{ImageLayer, LevelCache};
use crate::cascade::resolve_linked_css_path;
use crate::dom::StaticDocument;
use base64::Engine;
use ego_tree::NodeId;
use impeccable_common::jsp;
use impeccable_core::color::Rgba;
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, Cursor, Seek};
use std::rc::Rc;
use std::thread::LocalKey;

/// The browser overlay draws to a canvas no larger than this on a side.
const MAX_RASTER_SIDE: u32 = 640;
/// Files above this are not read: a hero is a few megabytes.
const MAX_FILE_BYTES: u64 = 24 * 1024 * 1024;
/// The base64 payload length that decodes to [`MAX_FILE_BYTES`], so a data
/// URI is refused before its payload is copied or decoded.
const MAX_DATA_URI_CHARS: usize = (MAX_FILE_BYTES as usize / 3) * 4 + 4;
const MAX_IMAGE_SIDE: u32 = 8192;
/// Pixels an image may hold before it is decoded. `image`'s own allocation
/// limit binds PNG and GIF buffers only, so the budget is checked against
/// the header for every format: 24 megapixels is 72 MiB of RGB.
const MAX_PIXELS: u64 = 24_000_000;
/// Entries kept per process cache; a cache is cleared when full.
const CACHE_ENTRIES: usize = 64;

/// A decoded image, RGBA8, at most [`MAX_RASTER_SIDE`] on a side.
pub struct Raster {
    pub width: u32,
    pub height: u32,
    rgba: Vec<u8>,
}

impl Raster {
    pub fn pixel(&self, x: usize, y: usize) -> Rgba {
        let i = (y.min(self.height as usize - 1) * self.width as usize
            + x.min(self.width as usize - 1))
            * 4;
        let p = &self.rgba[i..i + 4];
        Rgba::new(p[0] as f64, p[1] as f64, p[2] as f64, p[3] as f64 / 255.0)
    }
}

/// An image the engine can read: where its bytes are and its intrinsic
/// size, known from the header alone. The size is what `background-size:
/// auto` paints, and it decides whether the layer is decoration before any
/// pixel is decoded.
pub struct ImageSource {
    key: String,
    /// The file to read, or `None` for a data URI.
    path: Option<String>,
    pub width: u32,
    pub height: u32,
}

type Cache<T> = RefCell<HashMap<String, Option<Rc<T>>>>;

thread_local! {
    static SOURCES: Cache<ImageSource> = RefCell::new(HashMap::new());
    static RASTERS: Cache<Raster> = RefCell::new(HashMap::new());
}

/// `make()` once per key for the rest of the process, misses included.
fn cached<T>(
    cache: &'static LocalKey<Cache<T>>,
    key: &str,
    make: impl FnOnce() -> Option<T>,
) -> Option<Rc<T>> {
    if let Some(hit) = cache.with(|c| c.borrow().get(key).cloned()) {
        return hit;
    }
    let value = make().map(Rc::new);
    cache.with(|c| {
        let mut cache = c.borrow_mut();
        if cache.len() >= CACHE_ENTRIES {
            cache.clear();
        }
        cache.insert(key.to_string(), value.clone());
    });
    value
}

/// Locates and decodes the `url()` grounds of one document, relative to the
/// document's directory.
pub struct ImageSampler {
    base: String,
    levels: LevelCache,
    /// The source behind each image-painting element, misses included, so a
    /// page's text elements ask about their shared ancestor once.
    sources: RefCell<HashMap<NodeId, Option<Rc<ImageSource>>>>,
}

impl ImageSampler {
    pub fn new(html_dir: &str) -> Self {
        ImageSampler {
            base: html_dir.to_string(),
            levels: LevelCache::default(),
            sources: RefCell::new(HashMap::new()),
        }
    }

    pub fn levels(&self) -> &LevelCache {
        &self.levels
    }

    /// The image behind an element's `url()` layer, or `None` when it cannot
    /// be read here: a remote URL, a missing or oversized file, a format the
    /// decoders do not cover (SVG), or a text data URI.
    pub fn source(
        &self,
        doc: &StaticDocument,
        node: NodeId,
        layer: &ImageLayer,
    ) -> Option<Rc<ImageSource>> {
        if let Some(hit) = self.sources.borrow().get(&node) {
            return hit.clone();
        }
        let found = self.find_source(&layer_url(doc, node, layer));
        self.sources.borrow_mut().insert(node, found.clone());
        found
    }

    /// The decoded raster of a source.
    pub fn raster(
        &self,
        doc: &StaticDocument,
        node: NodeId,
        layer: &ImageLayer,
        source: &ImageSource,
    ) -> Option<Rc<Raster>> {
        cached(&RASTERS, &source.key, || {
            decode(&source_bytes(source, &layer_url(doc, node, layer))?)
        })
    }

    fn find_source(&self, url: &str) -> Option<Rc<ImageSource>> {
        let url = url.trim();
        if url.is_empty() {
            return None;
        }
        if is_data_uri(url) {
            // Refused before anything is allocated: a project file can carry
            // any size of data URI.
            if url.len() > MAX_DATA_URI_CHARS + 256 {
                return None;
            }
            let key = data_uri_key(url);
            return cached(&SOURCES, &key, || {
                let (width, height) = dimensions(Cursor::new(data_uri_bytes(url)?))?;
                Some(ImageSource {
                    key: key.clone(),
                    path: None,
                    width,
                    height,
                })
            });
        }
        if url.starts_with("//") || url.contains("://") {
            return None;
        }
        // Keyed on the url as written, so a url that does not resolve costs
        // its directory walk once, not once per text element.
        let lookup = format!("{}\u{0}{}", self.base, url);
        cached(&SOURCES, &lookup, || {
            let path = resolve_linked_css_path(&self.base, url);
            let meta = std::fs::metadata(&path).ok()?;
            if !meta.is_file() || meta.len() > MAX_FILE_BYTES {
                return None;
            }
            let file = std::io::BufReader::new(std::fs::File::open(&path).ok()?);
            let (width, height) = dimensions(file)?;
            Some(ImageSource {
                key: path.clone(),
                path: Some(path),
                width,
                height,
            })
        })
    }
}

/// The url of an element's image layer, borrowed from its computed style.
pub fn layer_url<'d>(doc: &'d StaticDocument, node: NodeId, layer: &ImageLayer) -> Cow<'d, str> {
    let bg_image = doc
        .get_style(node)
        .get("backgroundImage")
        .map(|s| s.as_str())
        .unwrap_or("");
    layer.call.value(bg_image)
}

fn is_data_uri(url: &str) -> bool {
    // `get`, not a byte slice: a url may open with a multibyte character.
    url.get(..5)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("data:"))
}

/// The cache key of a data URI: its length and a hash of bounded windows of
/// it (head, tail, and evenly spaced slices), so a multi-megabyte inlined
/// image costs a few kilobytes of hashing and the cache never holds a copy.
fn data_uri_key(url: &str) -> String {
    const EDGE: usize = 2048;
    const SLICES: usize = 16;
    const SLICE: usize = 64;
    let bytes = url.as_bytes();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    if bytes.len() <= EDGE * 4 {
        bytes.hash(&mut hasher);
    } else {
        bytes[..EDGE].hash(&mut hasher);
        bytes[bytes.len() - EDGE..].hash(&mut hasher);
        let step = bytes.len() / (SLICES + 1);
        for i in 1..=SLICES {
            bytes[i * step..i * step + SLICE].hash(&mut hasher);
        }
    }
    format!("data:{}:{:016x}", bytes.len(), hasher.finish())
}

/// The name a finding gives the image: the file name, or `data:<mime>`.
pub fn ground_label(url: &str) -> String {
    let url = url.trim();
    if is_data_uri(url) {
        let header = url[5..].split(',').next().unwrap_or("");
        let mime = header
            .split(';')
            .next()
            .unwrap_or("")
            .trim()
            .to_ascii_lowercase();
        return if mime.is_empty() {
            "data:image".into()
        } else {
            format!("data:{mime}")
        };
    }
    let stripped = url.split(['?', '#']).next().unwrap_or("");
    jsp::basename(stripped)
}

fn source_bytes(source: &ImageSource, url: &str) -> Option<Vec<u8>> {
    match &source.path {
        Some(path) => std::fs::read(path).ok(),
        None => data_uri_bytes(url.trim()),
    }
}

fn data_uri_bytes(url: &str) -> Option<Vec<u8>> {
    let (header, payload) = url[5..].split_once(',')?;
    if !header
        .split(';')
        .any(|p| p.trim().eq_ignore_ascii_case("base64"))
    {
        return None;
    }
    if payload.len() > MAX_DATA_URI_CHARS {
        return None;
    }
    let compact: Cow<str> = if payload.chars().any(char::is_whitespace) {
        Cow::Owned(payload.chars().filter(|c| !c.is_whitespace()).collect())
    } else {
        Cow::Borrowed(payload)
    };
    base64::engine::general_purpose::STANDARD
        .decode(compact.as_ref())
        .or_else(|_| base64::engine::general_purpose::STANDARD_NO_PAD.decode(compact.as_ref()))
        .ok()
}

/// An image's size from its header, when it is one the engine will decode:
/// no pixel is read here.
fn dimensions<R: BufRead + Seek>(reader: R) -> Option<(u32, u32)> {
    let (width, height) = image::ImageReader::new(reader)
        .with_guessed_format()
        .ok()?
        .into_dimensions()
        .ok()?;
    let readable = width > 0
        && height > 0
        && width <= MAX_IMAGE_SIDE
        && height <= MAX_IMAGE_SIDE
        && u64::from(width) * u64::from(height) <= MAX_PIXELS;
    readable.then_some((width, height))
}

fn decode(bytes: &[u8]) -> Option<Raster> {
    let decoded = image::ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .ok()?
        .decode()
        .ok()?;
    // `thumbnail` also upscales, so only images past the budget go through it.
    let scaled = if decoded.width().max(decoded.height()) > MAX_RASTER_SIDE {
        decoded.thumbnail(MAX_RASTER_SIDE, MAX_RASTER_SIDE)
    } else {
        decoded
    };
    let rgba = scaled.to_rgba8();
    Some(Raster {
        width: rgba.width(),
        height: rgba.height(),
        rgba: rgba.into_raw(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn png_bytes(width: u32, height: u32, rgba: [u8; 4]) -> Vec<u8> {
        let img = image::RgbaImage::from_pixel(width, height, image::Rgba(rgba));
        let mut out = Cursor::new(Vec::new());
        image::DynamicImage::ImageRgba8(img)
            .write_to(&mut out, image::ImageFormat::Png)
            .unwrap();
        out.into_inner()
    }

    fn raster(sampler: &ImageSampler, url: &str) -> Option<Rc<Raster>> {
        let source = sampler.find_source(url)?;
        cached(&RASTERS, &source.key, || {
            decode(&source_bytes(&source, url)?)
        })
    }

    #[test]
    fn data_uri_decodes_and_labels() {
        let bytes = png_bytes(8, 8, [240, 236, 228, 255]);
        let uri = format!(
            "data:image/png;base64,{}",
            base64::engine::general_purpose::STANDARD.encode(&bytes)
        );
        let sampler = ImageSampler::new("/nonexistent");
        let source = sampler
            .find_source(&uri)
            .expect("png data uri has a header");
        assert_eq!((source.width, source.height), (8, 8));
        let pixels = raster(&sampler, &uri).expect("png data uri decodes");
        assert_eq!((pixels.width, pixels.height), (8, 8));
        assert_eq!(pixels.pixel(3, 3).r, 240.0);
        assert_eq!(ground_label(&uri), "data:image/png");
        assert!(sampler
            .find_source("data:image/svg+xml;utf8,<svg/>")
            .is_none());
        assert!(sampler
            .find_source("https://example.com/hero.jpg")
            .is_none());
        assert!(sampler.find_source("//cdn.example.com/hero.jpg").is_none());
        assert_eq!(ground_label("img/hero.jpg?v=3"), "hero.jpg");
        // A url that opens with a multibyte character is a local path, not
        // a panic.
        assert!(!is_data_uri("dat\u{20ac}"));
        assert!(sampler.find_source("abcd\u{20ac}.png").is_none());
        assert_eq!(ground_label("abcd\u{20ac}.png"), "abcd\u{20ac}.png");
        // A payload past the byte budget is refused before it is decoded.
        let huge = format!(
            "data:image/png;base64,{}",
            "A".repeat(MAX_DATA_URI_CHARS + 1)
        );
        assert!(sampler.find_source(&huge).is_none());
    }

    #[test]
    fn a_long_data_uri_is_fingerprinted_from_bounded_windows() {
        let head = "data:image/png;base64,";
        let a = format!("{head}{}", "A".repeat(60_000));
        let mut b = a.clone().into_bytes();
        // A change inside one of the sampled slices changes the key.
        let step = b.len() / 17;
        b[step + 3] = b'B';
        let b = String::from_utf8(b).unwrap();
        assert_ne!(data_uri_key(&a), data_uri_key(&b));
        assert_ne!(data_uri_key(&a), data_uri_key(&format!("{a}A")));
        assert_eq!(data_uri_key(&a), data_uri_key(&a.clone()));
    }

    #[test]
    fn files_resolve_against_the_page_dir_and_downscale() {
        let dir = std::env::temp_dir().join(format!("impeccable-sampler-{}", std::process::id()));
        std::fs::create_dir_all(dir.join("img")).unwrap();
        std::fs::write(
            dir.join("img").join("wide.png"),
            png_bytes(1280, 320, [20, 20, 20, 255]),
        )
        .unwrap();
        let sampler = ImageSampler::new(&dir.to_string_lossy());
        assert!(sampler.find_source("wide.png").is_none());
        // The miss is remembered: the file appearing later does not change
        // the answer within one process.
        std::fs::write(dir.join("wide.png"), png_bytes(4, 4, [0, 0, 0, 255])).unwrap();
        assert!(sampler.find_source("wide.png").is_none());
        let source = sampler
            .find_source("img/wide.png")
            .expect("resolves against the page dir");
        // The intrinsic size comes from the header; the raster is scaled.
        assert_eq!((source.width, source.height), (1280, 320));
        let pixels = raster(&sampler, "img/wide.png").unwrap();
        assert_eq!((pixels.width, pixels.height), (640, 160));
        assert_eq!(pixels.pixel(639, 159).g, 20.0);
        assert!(Rc::ptr_eq(
            &pixels,
            &raster(&sampler, "img/wide.png").unwrap()
        ));
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn an_image_past_the_pixel_budget_is_refused_from_its_header() {
        // 6000 x 6000 is 36 megapixels: over the budget, and never decoded.
        let big = png_bytes(6000, 6000, [0, 0, 0, 255]);
        assert!(dimensions(Cursor::new(&big)).is_none());
        let ok = png_bytes(4000, 3000, [0, 0, 0, 255]);
        assert_eq!(dimensions(Cursor::new(&ok)), Some((4000, 3000)));
    }
}
