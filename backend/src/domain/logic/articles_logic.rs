use std::sync::Arc;

use slug::slugify;

use crate::{
    domain::model::Article,
    repos::{ArticlesRepo, UsersRepo},
    AppError,
};

#[derive(Clone)]
pub struct ArticlesMgr {
    articles_repo: ArticlesRepo,
    user_repo: Arc<UsersRepo>,
}

impl ArticlesMgr {
    /// Create a new instance of `ArticlesMgr`.
    pub fn new(articles_repo: ArticlesRepo, user_repo: Arc<UsersRepo>) -> Self {
        Self {
            articles_repo,
            user_repo,
        }
    }

    pub async fn get_articles(&self) -> Result<Vec<Article>, AppError> {
        self.articles_repo.get_articles().await
    }

    pub async fn create_article(
        &self,
        title: String,
        description: String,
        body: String,
        tag_list: Vec<String>,
        author_id: i64,
    ) -> Result<Article, AppError> {
        //
        let slug = slugify(&title);
        match self
            .articles_repo
            .add(&slug, &title, &description, &body, &tag_list, author_id)
            .await
        {
            Ok(created_at) => {
                // TODO: Check if it's reliable.
                let author_user_profile = self.user_repo.get_profile_by_id(author_id).await?;
                // match self.user_repo.get_profile_by_id(author_id).await {
                //     Ok(profile) => author_user_profile = profile,
                //     Err(err) => return Err(AppError::from(err)),
                // };
                Ok(Article {
                    slug,
                    title,
                    description,
                    body,
                    tag_list,
                    created_at,
                    updated_at: created_at,
                    favorited: false,
                    favorites_count: 0,
                    author: author_user_profile,
                })
            }
            Err(err) => Err(AppError::from(err)),
        }
    }
}
