use futures_util::TryStreamExt;
use ollama_rs::generation::chat::{
    ChatMessage, ChatMessageResponseStream, request::ChatMessageRequest,
};
use sakura_ai_rs::SakuraAI;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, stdout};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sakura = SakuraAI::default();
    let history = Arc::new(Mutex::new(vec![]));
    let mut stdout = stdout();

    let mut commands = std::pin::pin!({
        let stdin_reader = tokio::io::BufReader::new(tokio::io::stdin());
        let line_stream = tokio_stream::wrappers::LinesStream::new(stdin_reader.lines());
        line_stream.try_take_while(|x| std::future::ready(Ok(!x.eq_ignore_ascii_case("exit"))))
    });

    loop {
        stdout.write_all(b"\n> ").await?;
        stdout.flush().await?;

        let input = match commands.next().await {
            Some(Ok(x)) => x,
            Some(Err(e)) => return Err(e.into()),
            None => break,
        };

        let mut stream: ChatMessageResponseStream = sakura
            .send_chat_messages_with_history_stream(
                history.clone(),
                ChatMessageRequest::new("gpt-oss-120b".to_string(), vec![ChatMessage::user(input)]),
            )
            .await?;

        while let Some(res) = stream.next().await {
            let res = res.map_err(|_| "Error during stream")?;
            stdout.write_all(res.message.content.as_bytes()).await?;
            stdout.flush().await?;
        }
    }

    dbg!(&history.lock().unwrap());

    Ok(())
}
