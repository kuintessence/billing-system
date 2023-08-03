use std::str::FromStr;

use database_model::system::prelude::*;
use database_model::utils::WithDecimalFileds;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::domain::models::ClusterIdSettings;
use crate::domain::repositories::ClusterIdSettingsRepository;
use crate::infrastructure::databases::SeaOrmDbRepository;

#[async_trait::async_trait]
impl ClusterIdSettingsRepository for SeaOrmDbRepository {
    async fn get_by_cluster_id(&self, id: &str) -> anyhow::Result<ClusterIdSettings> {
        let mut model = ClusterIdSettingsEntity::find()
            .filter(ClusterIdSettingsColumn::ClusterId.eq(Uuid::from_str(id)?))
            .one(self.db.get_connection())
            .await?
            .ok_or(anyhow::anyhow!("No such cluster"))?;
        model.rescale_all_to(10);
        tracing::debug!("{model:#?}");
        Ok(model.into())
    }
}
