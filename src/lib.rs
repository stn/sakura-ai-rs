//! Sakura-AI-rs
//!
//! ### [さくらのAI Engine](https://www.sakura.ad.jp/aipf/ai-engine/) を簡単に使うための非公式Rustクライアント
//!
//! [ollama-rs](https://github.com/pepperoni21/ollama-rs)と互換性を持つことで既存のコードを最小限の変更で再利用できるようにしています。
//!
//! ※現在はChat Completionのみをサポート
//!
//! ## インストール
//!
//! ollama-rsとsakura-ai-rsをCargo.tomlのdependenciesに追加
//!
//!```
//!cargo add sakura-ai-rs
//!cargo add ollama-rs
//!```
//!
//! ## 使い方
//!
//! [アカウントトークン](https://secure.sakura.ad.jp/ai/account-tokens)を作成し、環境変数`SAKURA_AI_ENGINE_API_KEY`に設定します。
//!
//!```
//!# macOS / Linux
//!export SAKURA_AI_ENGINE_API_KEY='...'
//!```
//!
//!```
//!# Windows PowerShell
//!$env:SAKURA_AI_ENGINE_API_KEY = '...'
//!```
//!
//! ### SakuraAIの初期化
//!
//!```rust
//!use sakura_ai_rs::SakuraAI;
//!
//!let sakura = SakuraAI::default();
//!```
//!
//! ### Chat Completion API
//!
//! `send_chat_messages`を用いてSakura AI Engineの[Chat Completion](https://manual.sakura.ad.jp/api/cloud/ai-engine/inference.html#operation/createChatCompletion)を呼び出す。
//!
//!```rust
//!# tokio_test::block_on(async {
//!use ollama_rs::generation::chat::{ChatMessage, request::ChatMessageRequest};
//!use ollama_rs::history::ChatHistory;
//!use sakura_ai_rs::SakuraAI;
//!
//!let sakura = SakuraAI::default();
//!
//!let model = "gpt-oss-120b".to_string();
//!let prompt = "Why is the sky blue?".to_string();
//!
//!let res = sakura
//!    .send_chat_messages(
//!        ChatMessageRequest::new(
//!            model,
//!            vec![ChatMessage::user(prompt)],
//!        ),
//!    )
//!    .await;
//!
//!if let Ok(res) = res {
//!    println!("{}", res.message.content);
//!}
//!# });
//!```
//!
//! `send_chat_messages_stream`の使用例は[examples/chat_api_chatbot.rs](https://github.com/stn/sakura-ai-rs/blob/main/examples/chat_api_chatbot.rs)を参考に。
//!
//!```
//!> cargo run --example chat_api_chatbot        
//!    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
//!     Running `target\debug\examples\chat_api_chatbot.exe`
//!
//!> こんにちは
//!こんにちは！今日はどのようなお手伝いが必要ですか？
//!>
//!```
//!
//! ### Chat Completion API with History
//!
//! `send_chat_messages_with_history`を用いると会話履歴も管理される。
//!
//!```rust
//!# tokio_test::block_on(async {
//!use ollama_rs::generation::chat::{ChatMessage, request::ChatMessageRequest};
//!use ollama_rs::history::ChatHistory;
//!use sakura_ai_rs::SakuraAI;
//!
//!let sakura = SakuraAI::default();
//!
//!let model = "llama2:latest".to_string();
//!let prompt = "Why is the sky blue?".to_string();
//!// `Vec<ChatMessage>` implements `ChatHistory`,
//!// but you could also implement it yourself on a custom type
//!let mut history = vec![];
//!
//!let res = sakura
//!    .send_chat_messages_with_history(
//!        &mut history, // <- messages will be saved here
//!        ChatMessageRequest::new(
//!            model,
//!            vec![ChatMessage::user(prompt)], // <- You should provide only one message
//!        ),
//!    )
//!    .await;
//!
//!if let Ok(res) = res {
//!    println!("{}", res.message.content);
//!}
//!# });
//!```
//!
//! [examples/chat_with_history.rs](https://github.com/stn/sakura-ai-rs/blob/main/examples/chat_with_history.rs)と[examples/chat_with_history_stream.rs](https://github.com/stn/sakura-ai-rs/blob/main/examples/chat_with_history_stream.rs)を参考に。
//!
//! ## Ollama-rsからの移行
//!
//! 上の例を[ollama-rs](https://github.com/pepperoni21/ollama-rs)と比較すると分かる通り、`ollama_rs::Ollama`の代わりに`sakura_ai_rs::SakuraAI`を使用すればよい。
//!
//! 入出力のオブジェクトには同じものを使用しているため、それ以外のコードは同じ。
//!
//! 現在、`send_chat_messages`, `send_chat_messages_stream`, `send_chat_messages_with_history`, `send_chat_messages_with_history_stream` がサポートされている。
//!
//! ## 公式ドキュメント
//!
//! - [さくらのAI Engine](https://manual.sakura.ad.jp/cloud/manual-ai-engine.html)
//! - [チャット補完・音声文字起こし用API](https://manual.sakura.ad.jp/api/cloud/ai-engine/inference.html)
//! - [ドキュメント・RAGAPI](https://manual.sakura.ad.jp/api/cloud/ai-engine/rag.html)
//!
#![cfg_attr(docsrs, feature(doc_cfg))]

use ollama_rs::IntoUrl;
use secrecy::SecretString;
use url::Url;

pub mod generation;

#[derive(Debug, Clone)]
pub struct SakuraAI {
    pub(crate) url: Url,
    pub(crate) reqwest_client: reqwest::Client,
    #[cfg(feature = "headers")]
    pub(crate) request_headers: reqwest::header::HeaderMap,
    pub(crate) api_key: SecretString,
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
            ..Default::default()
        }
    }

    /// To use a different API key different from default SAKURA_AI_ENGINE_API_KEY env var
    pub fn with_api_key<S: Into<String>>(mut self, api_key: S) -> Self {
        self.api_key = SecretString::from(api_key.into());
        self
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
            api_key: SecretString::from(
                std::env::var("SAKURA_AI_ENGINE_API_KEY").unwrap_or_default(),
            ),
        }
    }
}
