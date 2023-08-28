use crate::domain::model::ClusterIdSettings;

#[async_trait::async_trait]
pub trait ClusterIdSettingsRepository: Send + Sync {
    async fn get_by_cluster_id(&self, id: &str) -> anyhow::Result<ClusterIdSettings>;
}
