use aws_sdk_s3::{
    Client,
    config::{Builder, Credentials, Region},
};
use axum::extract::multipart::Multipart;
use uuid::Uuid;

use crate::error::AppError;

#[derive(Clone)]
pub struct UploadService {
    s3_client: Client,
    bucket_name: String,
    public_url: String,
}

impl UploadService {
    // 1. KHỞI TẠO KẾT NỐI ĐẾN CLOUDFLARE R2
    pub fn new(
        endpoint: String,
        access_key: String,
        secret_key: String,
        bucket_name: String,
        public_url: String,
    ) -> Self {
        // Cloudflare R2 đòi hỏi cấu hình custom thay vì dùng mặc định của AWS
        let credentials = Credentials::new(access_key, secret_key, None, None, "cloudflare-r2");

        let config = Builder::new()
            .credentials_provider(credentials)
            .region(Region::new("auto")) // R2 luôn bắt buộc dùng region là "auto"
            .endpoint_url(endpoint)
            .build();

        let s3_client = Client::from_conf(config);

        Self {
            s3_client,
            bucket_name,
            public_url,
        }
    }

    // ==========================================
    // 2. NHẬN FILE TỪ AXUM
    // ==========================================
    pub async fn upload_image(&self, mut multipart: Multipart) -> Result<String, AppError> {
        // Lục lọi trong gói hàng gửi lên xem có cái nào tên là "file" không
        while let Some(field) = multipart.next_field().await.map_err(|e| {
            tracing::error!("Lỗi đọc multipart: {}", e);
            AppError::BadRequest("Dữ liệu form không hợp lệ".to_string())
        })? {
            let name = field.name().unwrap_or("").to_string();

            if name == "file" {
                // Phải gửi file lên qua trường có tên là "file"
                let file_name = field.file_name().unwrap_or("unknown.jpg").to_string();
                let content_type = field.content_type().unwrap_or("image/jpeg").to_string();

                // Hút hết dữ liệu byte của tấm ảnh
                let data = field
                    .bytes()
                    .await
                    .map_err(|_| AppError::BadRequest("Lỗi đọc nội dung file".to_string()))?;

                // Tạo tên file mới bằng UUID để chống trùng lặp (lỡ 2 khách up 2 ảnh cùng tên là tiêu)
                let extension = file_name.split('.').last().unwrap_or("jpg");
                let unique_filename = format!("{}.{}", Uuid::new_v4(), extension);

                // Bắn thẳng lên kho R2
                self.s3_client
                    .put_object()
                    .bucket(&self.bucket_name)
                    .key(&unique_filename)
                    .content_type(content_type)
                    .body(data.into()) // Biến bytes thành luồng data cho SDK S3
                    .send()
                    .await
                    .map_err(|e| {
                        tracing::error!("Lỗi upload lên Cloudflare R2: {:?}", e);
                        AppError::StorageService("Không thể lưu ảnh vào kho".to_string())
                    })?;

                // Nối tên miền vào để trả về URL xịn xò: https://images.domain.com/uuid.jpg
                let image_url = format!("{}/{}", self.public_url, unique_filename);
                return Ok(image_url);
            }
        }

        Err(AppError::BadRequest(
            "Không tìm thấy file ảnh trong request".to_string(),
        ))
    }
}
