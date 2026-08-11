// use crate::error::AppError;
// use async_trait::async_trait;

// #[derive(Debug, Clone)]
// pub struct EmailPayload {
//     pub to: Vec<String>,
//     pub subject: String,
//     pub html_body: String,
//     pub text_body: Option<String>,
//     pub cc: Option<Vec<String>>,
//     pub bcc: Option<Vec<String>>,
// }

// #[derive(Debug, Clone)]
// pub struct EmailResponse {
//     pub id: String,
//     pub status: String,
// }

// #[async_trait]
// pub trait MailService: Send + Sync {
//     async fn send_email(&self, payload: EmailPayload) -> Result<EmailResponse, AppError>;
// }
