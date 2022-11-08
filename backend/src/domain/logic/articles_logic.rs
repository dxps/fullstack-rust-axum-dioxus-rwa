use log::debug;

use crate::{domain::model::Article, repos::ArticlesRepo, AppError};

pub struct ArticlesMgr {
    articles_repo: ArticlesRepo,
}

impl ArticlesMgr {
    /// Create a new instance of `ArticlesMgr`.
    pub fn new(repo: ArticlesRepo) -> Self {
        Self {
            articles_repo: repo,
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
        match self
            .articles_repo
            .add(title, description, body, tag_list, author_id)
            .await
        {
            Ok(id) => {
                debug!("Created article with id {}.", id);
                // ...
                todo!()
            }
            Err(err) => Err(AppError::from(err)),
        }
    }
}
