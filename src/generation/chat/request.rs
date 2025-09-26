use std::ops::Not;

use serde::Serialize;

use ollama_rs::generation::chat::{
    ChatMessage as OllamaChatMessage, request::ChatMessageRequest as OllamaChatMessageRequest,
};

/// A chat message request to Sakura AI Engine
#[derive(Debug, Clone, Serialize)]
pub struct ChatMessageRequest {
    #[serde(rename = "model")]
    pub model_name: String,

    pub messages: Vec<OllamaChatMessage>,

    #[serde(skip_serializing_if = "Not::not")]
    pub(crate) stream: bool,
}

impl ChatMessageRequest {
    pub fn new(model_name: String, messages: Vec<OllamaChatMessage>) -> Self {
        Self {
            model_name,
            messages,
            stream: false,
        }
    }
}

impl From<OllamaChatMessageRequest> for ChatMessageRequest {
    fn from(value: OllamaChatMessageRequest) -> Self {
        Self {
            model_name: value.model_name,
            messages: value.messages,
            stream: false, // value.stream is private
        }
    }
}
