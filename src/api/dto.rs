use crate::domain::model::{FlowInstanceBilling, NodeInstanceBilling};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Url {
    pub user_id: String,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowBillResponse {
    pub flow_bill: FlowBill,
    pub node_bill: Vec<NodeBill>,
}

impl From<(FlowInstanceBilling, Vec<NodeInstanceBilling>)> for FlowBillResponse {
    fn from(value: (FlowInstanceBilling, Vec<NodeInstanceBilling>)) -> Self {
        let (flow, nodes) = value;
        Self {
            flow_bill: FlowBill {
                cpu: flow.cpu,
                memory: flow.memory,
                storage: flow.storage,
                cpu_time: flow.cpu_time,
                wall_time: flow.wall_time,
                total_price: flow.total_price,
            },
            node_bill: nodes
                .into_iter()
                .map(|el| NodeBill {
                    id: el.id.to_string(),
                    cpu: el.cpu,
                    memory: el.memory,
                    storage: el.storage,
                    cpu_time: el.cpu_time,
                    wall_time: el.wall_time,
                    price: el.price,
                    formula: el.formula,
                })
                .collect::<Vec<_>>(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowBill {
    pub cpu: i64,
    pub memory: i64,
    pub storage: i64,
    pub cpu_time: i64,
    pub wall_time: i64,
    pub total_price: Decimal,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeBill {
    pub id: String,
    pub cpu: i64,
    pub memory: i64,
    pub storage: i64,
    pub cpu_time: i64,
    pub wall_time: i64,
    pub price: Decimal,
    pub formula: String,
}
