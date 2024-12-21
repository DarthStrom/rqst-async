use miniserve::{http::StatusCode, Content, Request, Response};
use tokio::join;

async fn index(_req: Request) -> Response {
    let content = include_str!("../index.html").to_string();
    Ok(Content::Html(content))
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct Conversation {
    messages: Vec<String>,
}

async fn chat(req: Request) -> Response {
    if let Request::Post(body) = req {
        let mut conversation: Conversation =
            serde_json::from_str(&body).unwrap_or(Conversation { messages: vec![] });

        let rand_handle = tokio::spawn(chatbot::gen_random_number());

        let conversation_export = conversation.clone();
        let responses_handle =
            tokio::spawn(async move { chatbot::query_chat(&conversation_export.messages).await });
        let (rand_result, responses_result) = join!(rand_handle, responses_handle);

        let rand = rand_result.unwrap();
        let responses = responses_result.unwrap();

        let index = rand % responses.len();
        let random_message = &responses[index];

        conversation.messages.push(random_message.to_string());

        Ok(Content::Json(serde_json::to_string(&conversation).unwrap()))
    } else {
        Err(StatusCode::METHOD_NOT_ALLOWED)
    }
}

#[tokio::main]
async fn main() {
    miniserve::Server::new()
        .route("/", index)
        .route("/chat", chat)
        .run()
        .await
}
