use std::sync::Arc;

use actix_web::get;
use actix_web::web::Path;
use alice_di::{actix_auto_inject, IServiceProvider};
use alice_infrastructure::error::{AliceResponder, AliceResponderResult};
use uuid::Uuid;

use super::dto::*;
use crate::api::extract_uuid;
use crate::domain::service::*;
use crate::infrastructure::ServiceProvider;

// #[actix_auto_inject(ServiceProvider, scoped)]
// #[post("billing-system/WebhookSubscribe")]
// pub async fn webhook_subscribe(
//     web::Json(url): web::Json<Url>,
//     #[inject] service: Arc<dyn UserWebhookService>,
// ) -> AliceResponderResult<String> {
//     let user_id = scoped_config.user_info.context("No user_id when subscribe.")?.id;
//     let url = url.url;
//     service.register_webhook(&user_id, &url).await?
// }

#[actix_auto_inject(ServiceProvider, scoped)]
#[get("billing-system/GetFlowNodesBill/{flow_instance_id}")]
pub async fn get_flow_nodes_bill(
    flow_instance_id: Path<String>,
    #[inject] service: Arc<dyn FlowNodeBillingService>,
) -> AliceResponderResult<FlowBillResponse> {
    let id = extract_uuid(&flow_instance_id)?;
    Ok(AliceResponder(service.get_bill(id).await?.into()))
}

#[alice_di::auto_inject(ServiceProvider, scoped)]
#[alice_web::message_consumer]
pub async fn bill_consumer(
    #[inject] service: Arc<dyn FlowNodeBillingService>,
    #[serialize] node_instance_id: Uuid,
) -> anyhow::Result<()> {
    tracing::info!("Receive msg: {}", node_instance_id.to_string());
    service.record_bill(node_instance_id).await
}
