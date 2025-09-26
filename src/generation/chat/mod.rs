use chrono::{DateTime, Utc};
use ollama_rs::{
    error::OllamaError,
    generation::{
        chat::{
            ChatMessage as OllamaChatMessage, ChatMessageResponse as OllamaChatMessageResponse,
            MessageRole, request::ChatMessageRequest as OllamaChatMessageRequest,
        },
        tools::ToolCall,
    },
    history::ChatHistory,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg_attr(docsrs, doc(cfg(feature = "stream")))]
#[cfg(feature = "stream")]
use async_stream::stream;

#[cfg_attr(docsrs, doc(cfg(feature = "stream")))]
#[cfg(feature = "stream")]
use ollama_rs::generation::chat::ChatMessageResponseStream;

#[cfg_attr(docsrs, doc(cfg(feature = "stream")))]
#[cfg(feature = "stream")]
use std::sync::{Arc, Mutex};

#[cfg_attr(docsrs, doc(cfg(feature = "stream")))]
#[cfg(feature = "stream")]
use tokio_stream::StreamExt;

use crate::SakuraAI;
use request::ChatMessageRequest;

pub mod request;

impl SakuraAI {
    #[cfg_attr(docsrs, doc(cfg(feature = "stream")))]
    #[cfg(feature = "stream")]
    /// Chat message generation with streaming.
    /// Returns a stream of `ChatMessageResponse` objects
    pub async fn send_chat_messages_stream(
        &self,
        request: OllamaChatMessageRequest,
    ) -> ollama_rs::error::Result<ChatMessageResponseStream> {
        self.send_chat_messages_stream_raw(request.into()).await
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "stream")))]
    #[cfg(feature = "stream")]
    async fn send_chat_messages_stream_raw(
        &self,
        mut request: ChatMessageRequest,
    ) -> ollama_rs::error::Result<ChatMessageResponseStream> {
        request.stream = true;

        let url = format!("{}v1/chat/completions", self.url_str());
        let builder = self.reqwest_client.post(url);

        #[cfg(feature = "headers")]
        let builder = builder.headers(self.request_headers.clone());

        let token = std::env::var("SAKURA_AI_ENGINE_API_KEY").unwrap_or_default();
        let builder = builder.bearer_auth(token);

        let res = builder.json(&request).send().await?;

        if !res.status().is_success() {
            return Err(OllamaError::Other(
                res.text().await.unwrap_or_else(|e| e.to_string()),
            ));
        }

        let s = stream! {
            let mut buffer = String::new();

            let mut stream = res.bytes_stream();
            while let Some(chunk_result) = stream.next().await {
                match chunk_result {
                    Ok(chunk) => {
                        // Convert bytes to string and append to buffer
                        if let Ok(chunk_str) = String::from_utf8(chunk.to_vec()) {
                            buffer.push_str(&chunk_str);

                            // Process complete lines in the buffer
                            let mut lines_to_process = Vec::new();
                            let mut start_pos = 0;

                            // Find all complete lines in the buffer and collect them
                            while let Some(pos) = buffer[start_pos..].find('\n') {
                                let actual_pos = start_pos + pos;
                                let line = buffer[start_pos..actual_pos].trim().to_string();
                                if !line.is_empty() {
                                    if line.starts_with("data: ") {
                                        let json_part = line.trim_start_matches("data: ").trim();
                                        if json_part == "[DONE]" {
                                            // Handle end of stream if necessary
                                            // For now, we just break the loop
                                            break;
                                        } else {
                                            lines_to_process.push(json_part.to_string());
                                        }
                                    } else {
                                        lines_to_process.push(line);
                                    }
                                }
                                start_pos = actual_pos + 1;
                            }

                            // If we processed any lines, truncate the buffer
                            if start_pos > 0 {
                                buffer = buffer[start_pos..].to_string();
                            }

                            // Process all collected lines
                            for line in lines_to_process {
                                // Parse the JSON line
                                match serde_json::from_str::<ChatMessageResponse>(&line) {
                                    Ok(response) => yield Ok(response.into()),
                                    Err(e) => {
                                        eprintln!("Failed to deserialize response: {e}");
                                        // Continue processing other lines even if one fails
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to read response: {e}");
                        yield Err(());
                        break;
                    }
                }
            }

            // Process any remaining data in the buffer
            if !buffer.is_empty() {
                if let Ok(response) = serde_json::from_str::<ChatMessageResponse>(&buffer) {
                    yield Ok(response.into());
                }
            }
        };

        Ok(Box::pin(s))
    }

    /// Chat message generation.
    /// Returns a `ChatMessageResponse` object
    pub async fn send_chat_messages(
        &self,
        request: OllamaChatMessageRequest,
    ) -> ollama_rs::error::Result<OllamaChatMessageResponse> {
        self.send_chat_messages_raw(request.into()).await
    }

    async fn send_chat_messages_raw(
        &self,
        mut request: ChatMessageRequest,
    ) -> ollama_rs::error::Result<OllamaChatMessageResponse> {
        request.stream = false;

        let url = format!("{}v1/chat/completions", self.url_str());
        let builder = self.reqwest_client.post(url);

        let token = std::env::var("SAKURA_AI_ENGINE_API_KEY").unwrap_or_default();
        let builder = builder.bearer_auth(token);

        #[cfg(feature = "headers")]
        let builder = builder.headers(self.request_headers.clone());

        let res = builder.json(&request).send().await?;

        if !res.status().is_success() {
            return Err(OllamaError::Other(
                res.text().await.unwrap_or_else(|e| e.to_string()),
            ));
        }

        let bytes = res.bytes().await?;
        let res = serde_json::from_slice::<ChatMessageResponse>(&bytes)?;

        Ok(res.into())
    }
}

impl SakuraAI {
    #[cfg_attr(docsrs, doc(cfg(feature = "stream")))]
    #[cfg(feature = "stream")]
    pub async fn send_chat_messages_with_history_stream<C: ChatHistory + Send + 'static>(
        &self,
        history: Arc<Mutex<C>>,
        request: OllamaChatMessageRequest,
    ) -> ollama_rs::error::Result<ChatMessageResponseStream> {
        self.send_chat_messages_with_history_stream_raw(history, request.into())
            .await
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "stream")))]
    #[cfg(feature = "stream")]
    async fn send_chat_messages_with_history_stream_raw<C: ChatHistory + Send + 'static>(
        &self,
        history: Arc<Mutex<C>>,
        mut request: ChatMessageRequest,
    ) -> ollama_rs::error::Result<ChatMessageResponseStream> {
        use async_stream::stream;
        use tokio_stream::StreamExt;

        // The request is modified to include the current chat messages
        {
            let mut hist = history.lock().unwrap();
            for m in request.messages {
                hist.push(m);
            }
        }

        request.messages = history.lock().unwrap().messages().to_vec();
        request.stream = true;

        let mut resp_stream: ChatMessageResponseStream =
            self.send_chat_messages_stream_raw(request.clone()).await?;

        let s = stream! {
            let mut result = String::new();

            while let Some(item) = resp_stream.try_next().await.unwrap() {
                let msg_part = item.clone().message.content;

                if item.done {
                    history.lock().unwrap().push(ollama_rs::generation::chat::ChatMessage::assistant(result.clone()));
                } else {
                    result.push_str(&msg_part);
                }

                yield Ok(item);
            }
        };

        Ok(Box::pin(s))
    }

    /// Chat message generation
    /// Returns a `ChatMessageResponse` object
    pub async fn send_chat_messages_with_history<C: ChatHistory>(
        &self,
        history: &mut C,
        request: ollama_rs::generation::chat::request::ChatMessageRequest,
    ) -> ollama_rs::error::Result<OllamaChatMessageResponse> {
        self.send_chat_messages_with_history_raw(history, request.into())
            .await
    }

    async fn send_chat_messages_with_history_raw<C: ChatHistory>(
        &self,
        history: &mut C,
        mut request: ChatMessageRequest,
    ) -> ollama_rs::error::Result<OllamaChatMessageResponse> {
        // The request is modified to include the current chat messages
        for m in request.messages {
            history.push(m);
        }

        request.messages = history.messages().to_vec();

        let result = self.send_chat_messages_raw(request.clone()).await;

        if let Ok(result) = result {
            history.push(result.message.clone());

            return Ok(result);
        }

        result
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessageResponse {
    /// The unique identifier for the completion.
    pub id: String,

    /// The type of the object returned.
    pub object: String,

    /// The creation time of the completion, in such format: `2023-08-04T08:52:19.385406455-07:00`.
    pub created: i64,

    /// The name of the model used for the completion.
    pub model: String,

    /// The list of choices returned by the completion.
    pub choices: Vec<ChatMessageChoice>,

    /// The usage information for the completion.
    pub usage: ChatMessageUsage,

    /// Extra fields returned by the API.
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessageChoice {
    /// The index of the choice.
    pub index: u32,

    /// The full chat message, using in non-streaming.
    pub message: Option<ChatMessage>,

    /// The delta of the chat message, using in streaming.
    pub delta: Option<ChatMessage>,

    /// The reason why the completion finished.
    pub finish_reason: Option<String>,

    /// Extra fields returned by the API.
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<MessageRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_content: Option<String>,
    #[serde(default)]
    pub tool_calls: Vec<ToolCall>,
}

impl ChatMessage {
    pub fn new(role: MessageRole, content: String) -> Self {
        Self {
            role: Some(role),
            content: Some(content),
            reasoning_content: None,
            tool_calls: vec![],
        }
    }

    pub fn user(content: String) -> Self {
        Self::new(MessageRole::User, content)
    }

    pub fn assistant(content: String) -> Self {
        Self::new(MessageRole::Assistant, content)
    }

    pub fn system(content: String) -> Self {
        Self::new(MessageRole::System, content)
    }

    pub fn tool(content: String) -> Self {
        Self::new(MessageRole::Tool, content)
    }
}

impl From<&ChatMessage> for OllamaChatMessage {
    fn from(value: &ChatMessage) -> Self {
        let content = value.content.clone().unwrap_or_default();
        if let Some(role) = value.role.as_ref() {
            return match role {
                MessageRole::User => Self::user(content),
                MessageRole::Assistant => Self::assistant(content),
                MessageRole::System => Self::system(content),
                MessageRole::Tool => {
                    let mut msg = Self::tool(content);
                    if !value.tool_calls.is_empty() {
                        msg.tool_calls = value.tool_calls.clone();
                    }
                    msg
                }
            };
        }
        Self::assistant(content)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessageUsage {
    /// Number of tokens in the prompt
    pub prompt_tokens: u64,
    /// Total number of tokens
    pub total_tokens: u64,
    /// Number of tokens generating the response
    pub completion_tokens: u64,
}

impl From<ChatMessageResponse> for OllamaChatMessageResponse {
    fn from(value: ChatMessageResponse) -> Self {
        let model = value.model.clone();
        let created_at = DateTime::from_timestamp_secs(value.created)
            .unwrap_or_else(|| Utc::now())
            .to_rfc3339();

        if value.choices.len() > 0 {
            let choice = &value.choices[0];
            if let Some(message) = choice.message.as_ref() {
                return Self {
                    model: value.model,
                    created_at,
                    message: message.into(),
                    done: choice.finish_reason.is_some(),
                    final_data: None,
                };
            } else if let Some(delta) = choice.delta.as_ref() {
                return Self {
                    model: value.model,
                    created_at,
                    message: delta.into(),
                    done: choice.finish_reason.is_some(),
                    final_data: None,
                };
            }
        }

        // If neither message nor delta is present, return a default response
        Self {
            model,
            created_at,
            message: OllamaChatMessage::assistant("".to_string()),
            done: false,
            final_data: None,
        }
    }
}
