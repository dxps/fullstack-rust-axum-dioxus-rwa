use crate::{
    domain::model::Article,
    repos::{ArticlesRepo, UsersRepo},
    web_api::respond_internal_server_error,
    AppError,
};
use serde::Deserialize;
use slug::slugify;
use std::sync::Arc;

#[derive(Clone)]
pub struct ArticlesMgr {
    articles_repo: ArticlesRepo,
    user_repo: Arc<UsersRepo>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateArticleInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
    #[serde(rename = "tagList")]
    pub tag_list: Option<Vec<String>>,
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

    pub async fn get_article(&self, slug: String) -> Result<Option<Article>, AppError> {
        self.articles_repo.get_article(slug).await
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

    pub async fn delete_article(&self, slug: String) -> Result<(), AppError> {
        //
        self.articles_repo.delete(slug).await
    }

    pub async fn update_article(
        &self,
        slug: String,
        input: UpdateArticleInput,
    ) -> Result<Article, AppError> {
        //
        log::debug!("update_article >> input={:?}", input);
        let res = self.get_article(slug).await?;
        if res.is_none() {
            return Err(AppError::NothingFound);
        }
        let mut a = res.unwrap();

        // Fill-in any of the input's elements.
        if let Some(title) = input.title {
            a.title = title;
        }
        if let Some(description) = input.description {
            a.description = description;
        }
        if let Some(body) = input.body {
            a.body = body;
        }
        if let Some(tag_list) = input.tag_list {
            a.tag_list = tag_list;
        }
        // Persist the changes.

        self.articles_repo.update(a.clone()).await.map(|_| a)
    }
}
