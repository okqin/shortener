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

    pub async fn create_short_url(&self, url: &str) -> Result<Url, Error> {
        let id = nanoid::nanoid!(6);
        let mut url = Url {
            id,
            url: url.to_string(),
        };

        loop {
            match self.repo.create_url(&self.db, &url).await {
                Ok(row) => {
                    return Ok(row);
                }
                Err(_) => {
                    url.id = nanoid::nanoid!(6);
                }
            }
        }
    }

    pub async fn get_original_url(&self, id: &str) -> Result<Url, Error> {
        let url = self.repo.get_url_by_id(&self.db, id).await?;
        Ok(url)
    }
}
