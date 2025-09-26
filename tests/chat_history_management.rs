use ollama_rs::generation::chat::{ChatMessage, MessageRole, request::ChatMessageRequest};
use sakura_ai_rs::SakuraAI;

#[tokio::test]
async fn test_chat_history_accumulated() {
    let sakura = SakuraAI::default();

    let mut history = vec![];

    assert!(
        sakura
            .send_chat_messages_with_history(
                &mut history,
                ChatMessageRequest::new(
                    "gpt-oss-120b".into(),
                    vec![ChatMessage::new(
                        MessageRole::User,
                        "Why is the sky blue?".into(),
                    )],
                ),
            )
            .await
            .is_ok()
    );

    assert!(
        sakura
            .send_chat_messages_with_history(
                &mut history,
                ChatMessageRequest::new(
                    "gpt-oss-120b".into(),
                    vec![ChatMessage::new(
                        MessageRole::User,
                        "But, why is the sky blue?".into()
                    )]
                ),
            )
            .await
            .is_ok()
    );

    assert_eq!(history.len(), 4)
}
