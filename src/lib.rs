#![cfg_attr(docsrs, feature(doc_cfg))]

use ollama_rs::IntoUrl;
use url::Url;

pub mod generation;

#[derive(Debug, Clone)]
pub struct SakuraAI {
    pub(crate) url: Url,
    pub(crate) reqwest_client: reqwest::Client,
    #[cfg(feature = "headers")]
    pub(crate) request_headers: reqwest::header::HeaderMap,
}

/// The main struct representing an Sakura AI Engine client.
///
/// This struct is used to interact with the Sakura AI Engine service.
///
/// # Fields
///
/// * `url` - The base URL of the Sakura AI Engine service.
/// * `reqwest_client` - The HTTP client used for requests.
/// * `request_headers` - Optional headers for requests (enabled with the `headers` feature).
impl SakuraAI {
    /// Creates a new `SakuraAI` instance with the specified host and port.
    ///
    /// # Arguments
    ///
    /// * `host` - The host of the Sakura AI Engine service.
    /// * `port` - The port of the Sakura AI Engine service.
    ///
    /// # Returns
    ///
    /// A new `SakuraAI` instance.
    ///
    /// # Panics
    ///
    /// Panics if the host is not a valid URL or if the URL cannot have a port.
    pub fn new(host: impl IntoUrl, port: u16) -> Self {
        let mut url: Url = host.into_url().unwrap();
        url.set_port(Some(port)).unwrap();

        Self::from_url(url)
    }

    /// Creates a new `SakuraAI` instance with the specified host, port, and `reqwest` client.
    ///
    /// # Arguments
    ///
    /// * `host` - The host of the Sakura AI Engine service.
    /// * `port` - The port of the Sakura AI Engine service.
    /// * `reqwest_client` - The `reqwest` client instance.
    ///
    /// # Returns
    ///
    /// A new `SakuraAI` instance with the specified `reqwest` client.
    ///
    /// # Panics
    ///
    /// Panics if the host is not a valid URL or if the URL cannot have a port.
    pub fn new_with_client(host: impl IntoUrl, port: u16, reqwest_client: reqwest::Client) -> Self {
        let mut url: Url = host.into_url().unwrap();
        url.set_port(Some(port)).unwrap();

        Self {
            url,
            reqwest_client,
            #[cfg(feature = "headers")]
            request_headers: reqwest::header::HeaderMap::new(),
        }
    }

    /// Attempts to create a new `SakuraAI` instance from a URL.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the Sakura AI Engine service.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new `SakuraAI` instance or a `url::ParseError`.
    #[inline]
    pub fn try_new(url: impl IntoUrl) -> Result<Self, url::ParseError> {
        Ok(Self::from_url(url.into_url()?))
    }

    /// Create new instance from a [`Url`].
    #[inline]
    pub fn from_url(url: Url) -> Self {
        Self {
            url,
            ..Default::default()
        }
    }

    /// Returns the URI of the Sakura AI Engine service as a `String`.
    ///
    /// # Panics
    ///
    /// Panics if the URL does not have a host.
    #[inline]
    pub fn uri(&self) -> String {
        self.url.host().unwrap().to_string()
    }

    /// Returns a reference to the URL of the Sakura AI Engine service.
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Returns the URL of the Sakura AI Engine service as a `&str`.
    ///
    /// Syntax in pseudo-BNF:
    ///
    /// ```bnf
    ///   url = scheme ":" [ hierarchical | non-hierarchical ] [ "?" query ]? [ "#" fragment ]?
    ///   non-hierarchical = non-hierarchical-path
    ///   non-hierarchical-path = /* Does not start with "/" */
    ///   hierarchical = authority? hierarchical-path
    ///   authority = "//" userinfo? host [ ":" port ]?
    ///   userinfo = username [ ":" password ]? "@"
    ///   hierarchical-path = [ "/" path-segment ]+
    /// ```
    #[inline]
    pub fn url_str(&self) -> &str {
        self.url.as_str()
    }
}

impl From<Url> for SakuraAI {
    fn from(url: Url) -> Self {
        Self::from_url(url)
    }
}

impl Default for SakuraAI {
    /// Returns a default Sakura instance with the host set to `https://api.ai.sakura.ad.jp`.
    fn default() -> Self {
        Self {
            url: Url::parse("https://api.ai.sakura.ad.jp").unwrap(),
            reqwest_client: reqwest::Client::new(),
            #[cfg(feature = "headers")]
            request_headers: reqwest::header::HeaderMap::new(),
        }
    }
}
