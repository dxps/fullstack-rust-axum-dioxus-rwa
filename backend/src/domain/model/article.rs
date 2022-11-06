use chrono::{DateTime, Utc};
use serde::Serialize;

use super::UserProfile;

/// `Article` domain model is what a User can read or write.
#[derive(Serialize)]
pub struct Article {
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
    pub fn new(
        slug: String,
        title: String,
        description: String,
        body: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        author: UserProfile,
    ) -> Self {
        Self {
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
}
