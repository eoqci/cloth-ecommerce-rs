use crate::{
    error::AppError,
    shared::ports::mail::{EmailPayload, EmailResponse, MailService},
};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

#[derive(Clone)]
pub struct ResendMailService {
    client: Client,
    api_key: String,
    from_email: String,
}

impl ResendMailService {
    pub fn new(api_key: String, from_email: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            from_email,
        }
    }
}

#[async_trait]
impl MailService for ResendMailService {
    async fn send_email(&self, payload: EmailPayload) -> Result<EmailResponse, AppError> {
        // 1. Tạo URL
        let url = "https://api.resend.com/emails";

        // 2. Tạo Body JSON thủ công (Sướng nhất đoạn này, muốn thêm trường gì cũng được)
        // serde_json sẽ tự động bỏ qua các trường None (null)
        let body = json!({
            "from": self.from_email,
            "to": payload.to,
            "subject": payload.subject,
            "html": payload.html_body,
            "text": payload.text_body,
            "cc": payload.cc,   // Nếu là None, serde tự bỏ qua
            "bcc": payload.bcc, // Nếu là None, serde tự bỏ qua
            "tags": [
                { "name": "environment", "value": "production" },
                { "name": "app", "value": "axum-ecommerce" }
            ]
        });

        // 3. Bắn Request
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key)) // Nhét Key vào Header
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::EmailService(format!("Failed to send request: {}", e)))?;

        // 4. Kiểm tra xem Resend có trả về 200 OK không
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            tracing::error!("Resend API Error: {}", error_text);
            return Err(AppError::EmailService(format!(
                "Resend API error: {}",
                error_text
            )));
        }

        // 5. Parse ID trả về
        // Resend trả về: { "id": "re_123..." }
        let res_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::EmailService(format!("Failed to parse response: {}", e)))?;

        let id = res_json["id"].as_str().unwrap_or("unknown").to_string();

        Ok(EmailResponse {
            id,
            status: "sent".to_string(),
        })
    }
}
