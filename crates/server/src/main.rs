use miniserve::{http::StatusCode, Content, Request, Response};

fn index(_req: Request) -> Response {
    let content = include_str!("../index.html").to_string();
    Ok(Content::Html(content))
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Conversation {
    messages: Vec<String>,
}

fn chat(req: Request) -> Response {
    if let Request::Post(body) = req {
        let mut conversation: Conversation =
            serde_json::from_str(&body).unwrap_or(Conversation { messages: vec![] });
        conversation
            .messages
            .push("And how does that make you feel?".to_string());
        Ok(Content::Json(serde_json::to_string(&conversation).unwrap()))
    } else {
        Err(StatusCode::METHOD_NOT_ALLOWED)
    }
}

fn main() {
    miniserve::Server::new()
        .route("/", index)
        .route("/chat", chat)
        .run()
}
