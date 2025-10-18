use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ====================================
// DATABASE MODEL
// ====================================
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Like {
    pub id: Uuid,
    pub user_id: Uuid,
    pub post_id: Uuid,
    pub created_at: DateTime<Utc>,
}

// ====================================
// REQUEST DTOs
// ====================================
#[derive(Debug, Deserialize)]
pub struct LikePostRequest {
    pub post_id: Uuid,
}

// ====================================
// RESPONSE DTOs
// ====================================
#[derive(Debug, Serialize)]
pub struct LikeResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub post_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct LikeStatusResponse {
    pub post_id: Uuid,
    pub liked: bool,
    pub likes_count: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct UserLikeInfo {
    pub id: Uuid,
    pub username: String,
    pub avatar_url: Option<String>,
    pub liked_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct PostLikesResponse {
    pub post_id: Uuid,
    pub likes_count: i64,
    pub users: Vec<UserLikeInfo>,
}

// ====================================
// QUERY PARAMETERS
// ====================================
#[derive(Debug, Deserialize)]
pub struct LikesQueryParams {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

impl Default for LikesQueryParams {
    fn default() -> Self {
        Self {
            page: Some(1),
            page_size: Some(20),
        }
    }
}

impl LikesQueryParams {
    pub fn page(&self) -> i64 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn page_size(&self) -> i64 {
        self.page_size.unwrap_or(20).clamp(1, 100)
    }

    pub fn offset(&self) -> i64 {
        (self.page() - 1) * self.page_size()
    }
}

// ====================================
// IMPLEMENTATION
// ====================================
impl Like {
    pub fn to_response(self) -> LikeResponse {
        LikeResponse {
            id: self.id,
            user_id: self.user_id,
            post_id: self.post_id,
            created_at: self.created_at,
        }
    }
}

// ====================================
// HELPER STRUCTS FOR AGGREGATIONS
// ====================================
#[derive(Debug, FromRow)]
pub struct LikeCount {
    pub post_id: Uuid,
    pub count: i64,
}

#[derive(Debug, FromRow)]
pub struct UserLikeStatus {
    pub post_id: Uuid,
    pub liked: bool,
}
