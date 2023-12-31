use alice_architecture::model::AggregateRoot;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, AggregateRoot)]
pub struct NodeInstance {
    pub id: Uuid,
    pub flow_id: Uuid,
    pub resource_meter: TaskUsedResource,
    pub queue_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct TaskUsedResource {
    /// 核心数
    pub cpu: u64,
    /// 平均内存
    pub avg_memory: u64,
    /// 最大内存
    pub max_memory: u64,
    /// 存储空间
    pub storage: u64,
    /// 墙钟时间
    pub wall_time: u64,
    /// 核心时间
    pub cpu_time: u64,
    /// 节点数
    pub node: u64,
    /// 开始时间
    pub start_time: i64,
    /// 结束时间
    pub end_time: i64,
}

impl TryFrom<database_model::node_instance::Model> for NodeInstance {
    type Error = anyhow::Error;

    fn try_from(model: database_model::node_instance::Model) -> Result<Self, Self::Error> {
        let id = model.id;
        Ok(Self {
            id,
            flow_id: model.flow_instance_id,
            resource_meter: match model.resource_meter {
                Some(v) => serde_json::from_value(v)?,
                None => anyhow::bail!("node {id} has no resource meter"),
            },
            queue_id: model.queue_id.ok_or(anyhow!("node {id} didn't be assigned to a queue"))?,
        })
    }
}
