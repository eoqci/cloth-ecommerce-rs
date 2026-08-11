// use std::sync::Arc;

// use crate::shared::ports::mail::{EmailPayload, MailService};

// pub struct EmailConfig {
//     pub email_to_send: String,
//     pub name_to_send: String,
//     pub email_service: Arc<dyn MailService>,
//     pub domain_url: String,
//     pub verification_token: String,
// }

// // 1. First send email (Register)
// pub fn send_register_verification(config: EmailConfig) {
//     tokio::spawn(async move {
//         send_verification_logic(
//             config,
//             "Kích hoạt tài khoản".to_string(),
//             "Chào mừng bạn! Đây là link kích hoạt tài khoản:".to_string(),
//         )
//         .await;
//     });
// }

// // 2. Second send email (Resend)
// pub fn send_resend_verification(config: EmailConfig) {
//     tokio::spawn(async move {
//         send_verification_logic(
//             config,
//             "Gửi lại mã kích hoạt".to_string(),
//             "Bạn vừa yêu cầu gửi lại email kích hoạt:".to_string(),
//         )
//         .await;
//     });
// }
// fn generate_verification_html(name: &str, link: &str, intro_text: &str) -> String {
//     format!(
//         r#"<!DOCTYPE html>
//             <html>
//               <body style="margin:0; padding:0; font-family:Arial, sans-serif;">
//                 <div style="background:#f4f4f4; padding:20px;">
//                   <div style="max-width:600px; margin:0 auto; background:#fff; border-radius:8px; overflow:hidden;">
//                     <div style="padding:20px; text-align:center; background:#808080; color:#fff;">
//                       <h2>Chào {}!</h2>
//                     </div>
//                     <div style="padding:20px; color:#333; text-align:center;">
//                       <h2>{}</h2>
//                       <p><a href="{}" style="display:inline-block; background:#808080; color:#fff; padding:12px 24px; text-decoration:none; border-radius:4px;">Kích hoạt ngay</a></p>
//                       <p style="font-size:12px; color:#777;">Link hết hạn sau 15 phút. Vui lòng không chia sẻ link này.</p>
//                     </div>
//                   </div>
//                 </div>
//               </body>
//             </html>"#,
//         name, intro_text, link
//     )
// }

// // Send email logic
// async fn send_verification_logic(config: EmailConfig, subject: String, intro_text: String) {
//     let link = format!(
//         "http://{}/verify?token={}&email={}",
//         config.domain_url, config.verification_token, config.email_to_send
//     );

//     let html_body = generate_verification_html(&config.name_to_send, &link, &intro_text);

//     let payload = EmailPayload {
//         to: vec![config.email_to_send.clone()],
//         subject,
//         html_body,
//         text_body: None,
//         cc: None,
//         bcc: None,
//     };

//     if let Err(e) = config.email_service.send_email(payload).await {
//         tracing::error!("Failed to send email: {:?}", e);
//     }
// }
