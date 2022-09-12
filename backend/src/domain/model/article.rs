use chrono::{DateTime, Utc};

use super::UserProfile;

/// `Article` domain model is what a User can read or write.
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
