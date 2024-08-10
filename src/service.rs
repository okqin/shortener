use crate::{db::DB, entities::Url, repository::Repository, Error};

#[derive(Debug)]
pub struct Service {
    db: DB,
    repo: Repository,
}

impl Service {
    pub fn new(db: DB) -> Self {
        let repo = Repository::new();
        Self { db, repo }
    }

    pub async fn create_short_url(&self, url: &str) -> Result<String, Error> {
        let id = nanoid::nanoid!(6);
        let mut url = Url {
            id,
            url: url.to_string(),
        };
        while self.repo.create_url(&self.db, &url).await.is_err() {
            url.id = nanoid::nanoid!(6);
        }
        Ok(url.url)
    }

    pub async fn get_original_url(&self, id: &str) -> Result<String, Error> {
        let url = self.repo.get_url_by_id(&self.db, id).await?;
        Ok(url.url)
    }
}
