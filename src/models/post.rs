use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

// ====================================
// DATABASE MODEL
// ====================================
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
    pub slug: Option<String>,
    pub status: String,
    pub featured_image_url: Option<String>,
    pub views_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
}

// ====================================
// REQUEST DTOs (with validation)
// ====================================
#[derive(Debug, Deserialize, Validate)]
pub struct CreatePostRequest {
    #[validate(length(
        min = 3,
        max = 255,
        message = "Title must be between 3 and 255 characters"
    ))]
    pub title: String,

    #[validate(length(min = 10, message = "Content must be at least 10 characters"))]
    pub content: String,

    #[validate(custom(function = "validate_post_status"))]
    pub status: Option<String>,

    #[validate(url(message = "Featured image must be a valid URL"))]
    pub featured_image_url: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePostRequest {
    #[validate(length(
        min = 3,
        max = 255,
        message = "Title must be between 3 and 255 characters"
    ))]
    pub title: Option<String>,

    #[validate(length(min = 10, message = "Content must be at least 10 characters"))]
    pub content: Option<String>,

    #[validate(custom(function = "validate_post_status"))]
    pub status: Option<String>,

    #[validate(url(message = "Featured image must be a valid URL"))]
    pub featured_image_url: Option<String>,
}

// Custom validator for post status
fn validate_post_status(status: &str) -> Result<(), validator::ValidationError> {
    let valid_statuses = ["draft", "published", "archived"];
    if valid_statuses.contains(&status) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("invalid_status"))
    }
}

// ====================================
// RESPONSE DTOs
// ====================================
#[derive(Debug, Serialize)]
pub struct PostResponse {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub slug: Option<String>,
    pub status: String,
    pub featured_image_url: Option<String>,
    pub views_count: i32,
    pub author: AuthorInfo,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct PostWithStatsResponse {
    #[serde(flatten)]
    pub post: PostResponse,
    pub likes_count: i64,
    pub comments_count: i64,
    pub user_liked: bool, // Whether current user liked this post
                          // pub user_bookmarked: bool, // Whether current user bookmarked this post
}

#[derive(Debug, Serialize)]
pub struct PostListResponse {
    pub posts: Vec<PostWithStatsResponse>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub total_pages: i64,
}

#[derive(Debug, Serialize, Clone, FromRow)]
pub struct AuthorInfo {
    pub id: Uuid,
    pub username: String,
    pub avatar_url: Option<String>,
}

// ====================================
// IMPLEMENTATION - Conversions
// ====================================
impl Post {
    pub fn to_response(self, author: AuthorInfo) -> PostResponse {
        PostResponse {
            id: self.id,
            title: self.title,
            content: self.content,
            slug: self.slug,
            status: self.status,
            featured_image_url: self.featured_image_url,
            views_count: self.views_count,
            author,
            created_at: self.created_at,
            updated_at: self.updated_at,
            published_at: self.published_at,
        }
    }
}

impl PostResponse {
    pub fn with_stats(
        self,
        likes_count: i64,
        comments_count: i64,
        user_liked: bool,
        // user_bookmarked: bool,
    ) -> PostWithStatsResponse {
        PostWithStatsResponse {
            post: self,
            likes_count,
            comments_count,
            user_liked,
            // user_bookmarked,
        }
    }
}

// ====================================
// QUERY PARAMETERS
// ====================================
#[derive(Debug, Deserialize)]
pub struct PostQueryParams {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub status: Option<String>,
    pub user_id: Option<Uuid>,
    pub tag: Option<String>,
    pub search: Option<String>, // For searching in title/content
}

impl Default for PostQueryParams {
    fn default() -> Self {
        Self {
            page: Some(1),
            page_size: Some(10),
            status: None,
            user_id: None,
            tag: None,
            search: None,
        }
    }
}

impl PostQueryParams {
    pub fn page(&self) -> i64 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn page_size(&self) -> i64 {
        self.page_size.unwrap_or(10).clamp(1, 100)
    }

    pub fn offset(&self) -> i64 {
        (self.page() - 1) * self.page_size()
    }
}
