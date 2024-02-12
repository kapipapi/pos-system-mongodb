use dotenvy::dotenv;
use mongodb::{Client, Database};
use mongodb::bson::{doc, Uuid};
use mongodb::options::{ClientOptions, Credential};
use serde::de::DeserializeOwned;
use serde::{Serialize};
use crate::services::waiters::create_waiter_code_index;
use futures::TryStreamExt;
use crate::models::CollectionName;
use crate::repo::error::RepoError;

#[derive(Clone, Debug)]
pub struct Repository {
    database: Database,
}

impl Repository {
    pub async fn connect() -> Self {
        dotenv().expect(".env file not found");

        let uri = dotenvy::var("DB_URI").expect("DB_URI must be set");
        let username = dotenvy::var("DB_USERNAME").expect("DB_USERNAME must be set");
        let password = dotenvy::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
        let db_name = dotenvy::var("DB_NAME").expect("DB_NAME must be set");

        let mut client_options = ClientOptions::parse_async(uri).await.unwrap();
        let default_cred = Credential::builder()
            .username(username)
            .password(password)
            .source(db_name.clone())
            .build();
        client_options.credential = Some(default_cred);
        let client = Client::with_options(client_options).unwrap();
        let db = client.database(&*db_name);

        create_waiter_code_index(&db).await;

        Self {
            database: db
        }
    }

    pub fn get_collection<T>(&self) -> mongodb::Collection<T>
        where
            T: DeserializeOwned + Send + Sync + CollectionName,
    {
        self.database.collection::<T>(T::collection_name())
    }

    pub async fn insert_one<T>(&self, document: T) -> Result<(), RepoError>
        where
            T: Serialize + DeserializeOwned + Unpin + Send + Sync + CollectionName,
    {
        match self.get_collection::<T>().insert_one(document, None).await {
            Ok(_) => Ok(()),
            Err(err) => Err(RepoError::MongoDBError(err)),
        }
    }

    pub async fn query_one<T>(&self, id: &Uuid) -> Result<T, RepoError>
        where
            T: Serialize + DeserializeOwned + Unpin + Send + Sync + CollectionName,
    {
        let bson_id = Uuid::parse_str(&id.to_string()).unwrap();

        match self.get_collection::<T>()
            .find_one(Some(doc! {"_id": bson_id}), None)
            .await? {
            Some(result) => Ok(result),
            None => Err(RepoError::IdNotFound(id.clone())),
        }
    }

    pub async fn query_many<T>(&self, ids: &Vec<Uuid>) -> Result<Vec<T>, RepoError>
        where
            T: Serialize + DeserializeOwned + Unpin + Send + Sync + CollectionName,
    {
        let bson_ids: Vec<Uuid> = ids.iter().map(|id| Uuid::parse_str(&id.to_string()).unwrap()).collect();

        let mut cursor = self.get_collection::<T>()
            .find(Some(doc! {"_id": {"$in": bson_ids}}), None)
            .await?;

        let mut results: Vec<T> = Vec::new();
        while let Some(result) = cursor.try_next().await? {
            results.push(result)
        }

        if results.len() == ids.len() {
            Ok(results)
        } else {
            Err(RepoError::IdsNotFound(ids.clone()))
        }
    }

    pub async fn query_all<T>(&self) -> Result<Vec<T>, RepoError>
        where
            T: Serialize + DeserializeOwned + Unpin + Send + Sync + CollectionName,
    {
        let mut cursor = self.get_collection::<T>()
            .find(None, None)
            .await?;

        let mut results: Vec<T> = Vec::new();
        while let Some(result) = cursor.try_next().await? {
            results.push(result)
        }
        Ok(results)
    }

    pub async fn delete_one<T>(&self, id: &Uuid) -> Result<(), RepoError>
        where
            T: Serialize + DeserializeOwned + Unpin + Send + Sync + CollectionName,
    {
        let bson_id = Uuid::parse_str(&id.to_string()).unwrap();

        match self.get_collection::<T>()
            .delete_one(doc! {"_id": bson_id}, None)
            .await {
            Ok(_) => Ok(()),
            Err(err) => Err(RepoError::MongoDBError(err)),
        }
    }
}