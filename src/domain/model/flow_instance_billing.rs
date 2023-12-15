use alice_architecture::model::AggregateRoot;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 工作流实例
/// 工作流实例是工作流草稿提交之后解析形成的，其中记录的数据有恢复回工作流草稿的能力。
#[derive(Debug, Clone, Serialize, Deserialize, Default, AggregateRoot)]
pub struct FlowInstanceBilling {
    pub id: Uuid,
    pub flow_instance_id: Uuid,
    pub cpu: i32,
    pub memory: i64,
    pub storage: i64,
    pub wall_time: i64,
    pub total_price: Decimal,
    pub user_id: Uuid,
}

impl From<database_model::flow_instance_billing::Model> for FlowInstanceBilling {
    fn from(model: database_model::flow_instance_billing::Model) -> Self {
        let database_model::flow_instance_billing::Model {
            id,
            flow_instance_id,
            cpu,
            memory,
            storage,
            wall_time,
            total_price,
            user_id,
            created_time: _,
            modified_time: _,
        } = model;

        Self {
            id,
            flow_instance_id,
            cpu,
            memory,
            storage,
            wall_time,
            total_price,
            user_id,
        }
    }
}
