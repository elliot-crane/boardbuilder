use reqwest::blocking::{Client, ClientBuilder};
use ril::{Image, ImageFormat, Rgba};
use std::{
    env,
    ffi::OsString,
    fs::DirBuilder,
    path::{Path, PathBuf},
};
use url::Url;

use crate::error::AppError;

pub struct ImageLoaderOptions {
    pub cache_dir: PathBuf,
}

impl Default for ImageLoaderOptions {
    fn default() -> Self {
        Self {
            cache_dir: PathBuf::from(".cache/images"),
        }
    }
}

/// Loads images from various sources and caches to the filesystem as necessary.
pub struct ImageLoader {
    options: ImageLoaderOptions,
    // TODO: move this out of `new`
    http_client: Client,
}

impl ImageLoader {
    pub fn new(options: ImageLoaderOptions) -> Result<Self, AppError> {
        let pwd = env::current_dir().map_err(AppError::FilesystemError)?;
        if options.cache_dir == pwd {
            return Err(AppError::InvalidConfigError(
                "Image loader may not cache to the current directory.",
            ));
        }
        // ensure cache directory exists
        let cache_path = options.cache_dir.as_path();
        DirBuilder::new()
            .recursive(true)
            .create(cache_path)
            .map_err(AppError::FilesystemError)?;
        // http client
        let http_client = ClientBuilder::new()
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .build()
            .map_err(AppError::HttpError)?;
        Ok(Self {
            options,
            http_client,
        })
    }

    /// Makes a best effort to load from any given source, it will attempt to interpret as
    /// a URL first, and then fall back to a path otherwise.
    pub fn load<S: AsRef<str>>(&self, location: S) -> Result<Image<Rgba>, AppError> {
        if let Ok(url) = Url::parse(location.as_ref()) {
            if url.scheme() == "http" || url.scheme() == "https" {
                return self.load_from_url(location);
            }
        }
        let path = Path::new(location.as_ref());
        return self.load_from_file(path);
    }

    pub fn load_from_url<U: AsRef<str>>(&self, url: U) -> Result<Image<Rgba>, AppError> {
        let (url, partial_cache_path) = parse_web_url_and_cache_path(url)?;
        let mut cache_path = self.options.cache_dir.clone();
        cache_path.push(partial_cache_path);
        let cache_path = cache_path.as_path();
        let encoding = get_encoding_from_extension(cache_path);
        // the easy path - file exists on disk, just return it
        if cache_path.is_file() {
            println!("returning image from filesystem cache");
            return self.load_from_file(cache_path);
        }
        // otherwise, we need to load it
        println!("loading image from URL: {}", url);
        DirBuilder::new()
            .recursive(true)
            .create(cache_path.parent().expect("bad cache path"))
            .map_err(AppError::FilesystemError)?;
        let bytes = self
            .http_client
            .get(url)
            .send()
            .and_then(|r| r.bytes())
            .map_err(AppError::HttpError)?;
        let ril_image = ril_image_from_bytes(&bytes)?;
        // cache it for next time
        ril_image
            .save(encoding, cache_path)
            .map_err(AppError::RILError)?;
        println!("cached imaged to filesystem: {}", cache_path.display());
        Ok(ril_image)
    }

    pub fn load_from_file<P: AsRef<Path>>(&self, path: P) -> Result<Image<Rgba>, AppError> {
        Image::open(path).map_err(AppError::RILError)
    }
}

fn parse_web_url_and_cache_path<U: AsRef<str>>(url: U) -> Result<(Url, PathBuf), AppError> {
    let mut url = Url::parse(url.as_ref()).map_err(AppError::UrlParseError)?;
    if url.scheme() == "http" {
        url.set_scheme("https").unwrap();
    }
    if url.scheme() != "https" {
        return Err(AppError::InvalidArgumentError(
            "url",
            "must be an http or https URL",
        ));
    }
    let host = match url.host_str() {
        Some(host) => host,
        None => {
            return Err(AppError::InvalidArgumentError(
                "url",
                "must specify a hostname",
            ));
        }
    };
    let path = Path::new(url.path());
    if url.path() == "/" || path.extension().is_none() {
        return Err(AppError::InvalidArgumentError(
            "url",
            "must specify a path to a file",
        ));
    }

    let mut cache_path = PathBuf::new();
    for fragment in host.split(".") {
        cache_path.push(fragment);
    }
    // SAFETY: unwrap is safe here because the check for host above fails on data:, mailto:, and file:
    // and the check for an empty path above guarantees this iterator is non-empty
    for fragment in url.path_segments().unwrap() {
        cache_path.push(fragment);
    }

    Ok((url, cache_path))
}

fn ril_image_from_bytes<B: AsRef<[u8]>>(bytes: B) -> Result<Image<Rgba>, AppError> {
    let loaded = image::load_from_memory(bytes.as_ref())
        .map_err(AppError::ImageError)?
        .to_rgba8();
    Ok(Image::from_fn(loaded.width(), loaded.height(), |x, y| {
        let p = loaded.get_pixel(x, y);
        Rgba {
            r: p[0],
            g: p[1],
            b: p[2],
            a: p[3],
        }
    }))
}

fn get_encoding_from_extension<P: AsRef<Path>>(path: P) -> ImageFormat {
    let extension = path
        .as_ref()
        .extension()
        .map(|e| e.to_owned().to_ascii_lowercase())
        .unwrap_or(OsString::new());
    match extension.to_str() {
        Some("jpg") | Some("jpeg") => ImageFormat::Jpeg,
        Some("png") => ImageFormat::Png,
        _ => ImageFormat::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses() {
        let (_, cache_path) = parse_web_url_and_cache_path(
            "https://oldschool.runescape.wiki/images/Serpentine_helm_detail.png",
        )
        .expect("expected parse to succeed");
        let actual = cache_path.as_path();
        let expected = {
            let mut buf = PathBuf::new();
            buf.push("oldschool");
            buf.push("runescape");
            buf.push("wiki");
            buf.push("images");
            buf.push("Serpentine_helm_detail.png");
            buf
        };
        let expected = expected.as_path();
        assert_eq!(expected, actual);
    }
}
