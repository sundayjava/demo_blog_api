use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

// ====================================
// DATABASE MODEL
// ====================================
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Comment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub parent_comment_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ====================================
// REQUEST DTOs
// ====================================
#[derive(Debug, Deserialize, Validate)]
pub struct CreateCommentRequest {
    #[validate(length(
        min = 1,
        max = 2000,
        message = "Comment must be between 1 and 2000 characters"
    ))]
    pub content: String,

    pub parent_comment_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCommentRequest {
    #[validate(length(
        min = 1,
        max = 2000,
        message = "Comment must be between 1 and 2000 characters"
    ))]
    pub content: String,
}

// ====================================
// RESPONSE DTOs
// ====================================
#[derive(Debug, Serialize)]
pub struct CommentResponse {
    pub id: Uuid,
    pub post_id: Uuid,
    pub content: String,
    pub parent_comment_id: Option<Uuid>,
    pub author: CommentAuthor,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub replies_count: i64,
}

#[derive(Debug, Serialize, Clone)]
pub struct CommentAuthor {
    pub id: Uuid,
    pub username: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct _CommentWithRepliesResponse {
    #[serde(flatten)]
    pub comment: CommentResponse,
    pub replies: Vec<CommentResponse>,
}

#[derive(Debug, Serialize)]
pub struct CommentsListResponse {
    pub comments: Vec<CommentResponse>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub total_pages: i64,
}

#[derive(Debug, Serialize)]
pub struct CommentStatsResponse {
    pub post_id: Uuid,
    pub total_comments: i64,
    pub top_level_comments: i64,
}

// ====================================
// QUERY PARAMETERS
// ====================================
#[derive(Debug, Deserialize)]
pub struct CommentQueryParams {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub parent_id: Option<Uuid>, // Filter by parent comment (for replies)
}

impl Default for CommentQueryParams {
    fn default() -> Self {
        Self {
            page: Some(1),
            page_size: Some(20),
            parent_id: None,
        }
    }
}

impl CommentQueryParams {
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
// HELPER STRUCTS
// ====================================
#[derive(Debug, FromRow)]
pub struct CommentWithAuthor {
    // Comment fields
    pub id: Uuid,
    pub post_id: Uuid,
    pub _user_id: Uuid,
    pub content: String,
    pub parent_comment_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Author fields
    pub author_id: Uuid,
    pub author_username: String,
    pub author_avatar_url: Option<String>,
}

impl CommentWithAuthor {
    pub fn to_response(self, replies_count: i64) -> CommentResponse {
        CommentResponse {
            id: self.id,
            post_id: self.post_id,
            content: self.content,
            parent_comment_id: self.parent_comment_id,
            author: CommentAuthor {
                id: self.author_id,
                username: self.author_username,
                avatar_url: self.author_avatar_url,
            },
            created_at: self.created_at,
            updated_at: self.updated_at,
            replies_count,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct CommentCount {
    pub comment_id: Uuid,
    pub count: i64,
}
