use super::UserProfile;
use chrono::{DateTime, Utc};
use serde::Serialize;

/// `Article` domain model is what a `User` can read or write.
#[derive(Clone, Debug, Serialize)]
pub struct Article {
    #[serde(skip_serializing)]
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub favorited: bool,
    pub favorites_count: i32,
    pub author: UserProfile,
}

impl Article {
    //
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: i64,
        slug: String,
        title: String,
        description: String,
        body: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        author: UserProfile,
    ) -> Self {
        Self {
            id,
            slug,
            title,
            description,
            body,
            tag_list: vec![],
            created_at,
            updated_at,
            favorited: false,
            favorites_count: 0,
            author,
        }
    }

    /// Create a new `Article` with the basic (minimal) attributes.
    pub fn new_basic(
        slug: String,
        title: String,
        description: String,
        body: String,
        tag_list: Vec<String>,
        author_id: i64,
    ) -> Self {
        Self {
            id: 0,
            slug,
            title,
            description,
            body,
            tag_list,
            created_at: DateTime::default(),
            updated_at: DateTime::default(),
            favorited: false,
            favorites_count: 0,
            author: UserProfile::new_basic(author_id),
        }
    }
}
