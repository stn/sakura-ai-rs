# Sakura-AI-rs

### [さくらのAI Engine](https://www.sakura.ad.jp/aipf/ai-engine/) を簡単に使うための非公式Rustクライアント

[ollama-rs](https://github.com/pepperoni21/ollama-rs)と互換性を持つことで既存のコードを最小限の変更で再利用できるようにしています。

※現在はChat Completionのみをサポート

## インストール

ollama-rsとsakura-ai-rsをCargo.tomlのdependenciesに追加

```bash
cargo add sakura-ai-rs
cargo add ollama-rs
```

## 使い方

[アカウントトークン](https://secure.sakura.ad.jp/ai/account-tokens)を作成し、環境変数`SAKURA_AI_ENGINE_API_KEY`に設定します。

```bash
# macOS / Linux
export SAKURA_AI_ENGINE_API_KEY='...'
```

```bash
# Windows PowerShell
$env:SAKURA_AI_ENGINE_API_KEY = '...'
```

### SakuraAIの初期化

```rust
use sakura_ai_rs::SakuraAI;

let sakura = SakuraAI::default();
```

### Chat Completion API

`send_chat_messages`を用いてSakura AI Engineの[Chat Completion](https://manual.sakura.ad.jp/api/cloud/ai-engine/inference.html#operation/createChatCompletion)を呼び出します。

```rust
use ollama_rs::generation::chat::{ChatMessage, ChatMessageRequest};
use ollama_rs::history::ChatHistory;
use sakura_ai_rs::SakuraAI;

let sakura = SakuraAI::default();

let model = "gpt-oss-120b".to_string();
let prompt = "Why is the sky blue?".to_string();

let res = sakura
    .send_chat_messages(
        ChatMessageRequest::new(
            model,
            vec![ChatMessage::user(prompt)],
        ),
    )
    .await;

if let Ok(res) = res {
    println!("{}", res.message.content);
}
```

`send_chat_messages_stream`の使用例は[examples/chat_api_chatbot.rs]()を参考に。

```bash
> cargo run --example chat_api_chatbot        
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
     Running `target\debug\examples\chat_api_chatbot.exe`

> こんにちは
こんにちは！今日はどのようなお手伝いが必要ですか？
>
```

### Chat Completion API with History

`send_chat_messages_with_history`を用いると会話履歴も管理されます。

```rust
use ollama_rs::generation::chat::{ChatMessage, ChatMessageRequest};
use ollama_rs::history::ChatHistory;

let model = "llama2:latest".to_string();
let prompt = "Why is the sky blue?".to_string();
// `Vec<ChatMessage>` implements `ChatHistory`,
// but you could also implement it yourself on a custom type
let mut history = vec![];

let res = ollama
    .send_chat_messages_with_history(
        &mut history, // <- messages will be saved here
        ChatMessageRequest::new(
            model,
            vec![ChatMessage::user(prompt)], // <- You should provide only one message
        ),
    )
    .await;

if let Ok(res) = res {
    println!("{}", res.message.content);
}
```

## Ollama-rsからの移行

上の例を[ollama-rs](https://github.com/pepperoni21/ollama-rs)と比較すると分かる通り、`ollama_rs::Ollama`の代わりに`sakura_ai_rs::SakuraAI`を使用するだけです。

入出力のオブジェクトには同じものを使用しています。

`send_chat_messages`, `send_chat_messages_stream`, `send_chat_messages_with_history`, `send_chat_messages_with_history_stream` がサポートされています。

## 公式ドキュメント

- [さくらのAI Engine](https://manual.sakura.ad.jp/cloud/manual-ai-engine.html)
- [チャット補完・音声文字起こし用API](https://manual.sakura.ad.jp/api/cloud/ai-engine/inference.html)
- [ドキュメント・RAGAPI](https://manual.sakura.ad.jp/api/cloud/ai-engine/rag.html)
