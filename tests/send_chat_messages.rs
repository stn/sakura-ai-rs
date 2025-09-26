use ollama_rs::generation::chat::{ChatMessage, request::ChatMessageRequest};
use sakura_ai_rs::SakuraAI;
use std::sync::{Arc, Mutex};
use tokio_stream::StreamExt;

const PROMPT: &str = "Why is the sky blue?";

#[tokio::test]
async fn test_send_chat_messages_stream() {
    let sakura = SakuraAI::default();

    let messages = vec![ChatMessage::user(PROMPT.to_string())];
    let mut res = sakura
        .send_chat_messages_stream(ChatMessageRequest::new(
            "gpt-oss-120b".to_string(),
            messages,
        ))
        .await
        .unwrap();

    let mut done = false;
    while let Some(res) = res.next().await {
        let res = res.unwrap();
        dbg!(&res);
        if res.done {
            done = true;
            break;
        }
    }

    assert!(done);
}

#[tokio::test]
async fn test_send_chat_messages() {
    let sakura = SakuraAI::default();

    let messages = vec![ChatMessage::user(PROMPT.to_string())];
    let res = sakura
        .send_chat_messages(ChatMessageRequest::new(
            "gpt-oss-120b".to_string(),
            messages,
        ))
        .await
        .unwrap();
    dbg!(&res);

    assert!(res.done);
}

#[tokio::test]
async fn test_send_chat_messages_with_history_stream() {
    let sakura = SakuraAI::default();
    let history = Arc::new(Mutex::new(vec![]));

    let messages = vec![ChatMessage::user(PROMPT.to_string())];

    let mut done = false;

    let mut res = sakura
        .send_chat_messages_with_history_stream(
            history.clone(),
            ChatMessageRequest::new("gpt-oss-120b".to_string(), messages),
        )
        .await
        .unwrap();

    while let Some(res) = res.next().await {
        let res = res.unwrap();

        if res.done {
            done = true;
            break;
        }
    }

    assert!(done);
    // Should have user's message as well as AI's response
    dbg!(&history.lock().unwrap());
    assert_eq!(history.lock().unwrap().len(), 2);
}

#[tokio::test]
async fn test_send_chat_messages_with_history() {
    let sakura = SakuraAI::default();
    let mut history = vec![];
    let second_message = vec![ChatMessage::user("Second message".to_string())];

    let messages = vec![ChatMessage::user(PROMPT.to_string())];
    let res = sakura
        .send_chat_messages_with_history(
            &mut history,
            ChatMessageRequest::new("gpt-oss-120b".to_string(), messages.clone()),
        )
        .await
        .unwrap();

    dbg!(&res);
    assert!(res.done);
    // Should have user's message as well as AI's response
    assert_eq!(history.len(), 2);

    let res = sakura
        .send_chat_messages_with_history(
            &mut history,
            ChatMessageRequest::new("gpt-oss-120b".to_string(), second_message.clone()),
        )
        .await
        .unwrap();

    dbg!(&res);
    assert!(res.done);

    // Should now have 2 user messages as well as AI's responses
    assert_eq!(history.len(), 4);

    let second_user_message_in_history = history.get(2);

    assert!(second_user_message_in_history.is_some());
    assert_eq!(
        second_user_message_in_history.unwrap().content,
        "Second message".to_string()
    );
}
