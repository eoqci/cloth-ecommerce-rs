use std::{collections::HashMap, sync::Arc};

use crate::{
    error::AppError,
    modules::category::{
        dto::{CategoryTreeResponse, CreateCategoryRequest},
        model::Category,
        repository::CategoryRepository,
    },
};

#[derive(Clone)]
pub struct CategoryService {
    repo: Arc<CategoryRepository>,
}

impl CategoryService {
    pub fn new(repo: Arc<CategoryRepository>) -> Self {
        Self { repo }
    }

    pub async fn get_category_tree(&self) -> Result<Vec<CategoryTreeResponse>, AppError> {
        let categories = self.repo.get_all().await?;

        let mut children_map: HashMap<Option<i32>, Vec<Category>> = HashMap::new();
        for cat in categories {
            children_map
                .entry(cat.parent_id)
                .or_insert_with(Vec::new)
                .push(cat);
        }

        let tree = Self::build_tree(None, &children_map);
        Ok(tree)
    }

    pub async fn create_category(&self, req: CreateCategoryRequest) -> Result<Category, AppError> {
        // Chỗ này sau này có thể làm thêm logic check xem 'slug' đã tồn tại chưa nha.
        self.repo.create(&req.name, &req.slug, req.parent_id).await
    }

    // Recursive helper
    fn build_tree(
        parent_id: Option<i32>,
        map: &HashMap<Option<i32>, Vec<Category>>,
    ) -> Vec<CategoryTreeResponse> {
        let mut result = Vec::new();

        //Checkin parent_id if there is any child
        if let Some(children) = map.get(&parent_id) {
            for child in children {
                // on every single child -> checkin if there is any other child of that child
                let node = CategoryTreeResponse {
                    id: child.id,
                    name: child.name.clone(),
                    slug: child.slug.clone(),
                    //callback
                    children: Self::build_tree(Some(child.id), map),
                };
                result.push(node);
            }
        }
        result
    }
}

//========================| UNIT TEST |========================
#[cfg(test)]
mod tests {
    use super::*; // Import toàn bộ code ở file hiện tại vào khu vực test
    use chrono::Utc;
    use std::collections::HashMap;

    // Hàm tạo Category giả (Mock data) để test cho lẹ
    fn mock_category(id: i32, name: &str, parent_id: Option<i32>) -> Category {
        Category {
            id,
            name: name.to_string(),
            slug: name.to_lowercase().replace(" ", "-"),
            parent_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn test_category_build_tree_logic() {
        // 1. Chuẩn bị dữ liệu giả (Mock Data)
        let mut map: HashMap<Option<i32>, Vec<Category>> = HashMap::new();

        // Nhánh 1: Thời trang Nam (id: 1) -> Áo Nam (id: 2) -> Áo Thun (id: 3)
        map.insert(None, vec![mock_category(1, "Thời trang Nam", None)]);
        map.insert(Some(1), vec![mock_category(2, "Áo Nam", Some(1))]);
        map.insert(Some(2), vec![mock_category(3, "Áo Thun", Some(2))]);

        // 2. Chạy hàm cần test (gọi hàm private thoải mái vì test nằm chung file)
        let tree = CategoryService::build_tree(None, &map);

        // 3. Kiểm tra kết quả (Assert)
        // - Cây phải có đúng 1 nhánh gốc (Thời trang Nam)
        assert_eq!(tree.len(), 1);
        assert_eq!(tree[0].name, "Thời trang Nam");

        // - Thời trang Nam phải có 1 đứa con (Áo Nam)
        assert_eq!(tree[0].children.len(), 1);
        assert_eq!(tree[0].children[0].name, "Áo Nam");

        // - Áo Nam phải có 1 đứa cháu (Áo Thun)
        assert_eq!(tree[0].children[0].children.len(), 1);
        assert_eq!(tree[0].children[0].children[0].name, "Áo Thun");

        // - Áo Thun không được có con (rỗng)
        assert_eq!(tree[0].children[0].children[0].children.len(), 0);
    }
}
