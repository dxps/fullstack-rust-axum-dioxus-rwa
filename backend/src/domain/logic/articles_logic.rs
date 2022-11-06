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
}
