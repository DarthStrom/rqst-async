use miniserve::{http::StatusCode, Content, Request, Response};
use tokio::join;

async fn index(_req: Request) -> Response {
    let content = include_str!("../index.html").to_string();
    Ok(Content::Html(content))
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Conversation {
    messages: Vec<String>,
}

async fn chat(req: Request) -> Response {
    if let Request::Post(body) = req {
        let mut conversation: Conversation =
            serde_json::from_str(&body).unwrap_or(Conversation { messages: vec![] });

        let get_rand = chatbot::gen_random_number();
        let get_responses = chatbot::query_chat(&conversation.messages);
        let (rand, responses) = join!(get_rand, get_responses);

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
