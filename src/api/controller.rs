use std::sync::Arc;

use actix_web::web;
use actix_web::web::Path;
use actix_web::{get, post};
use alice_architecture::base_dto::ResponseBase;
use alice_di::{actix_auto_inject, IServiceProvider};

use super::dto::*;
use crate::domain::services::*;
use crate::infrastructure::ServiceProvider;

#[actix_auto_inject(ServiceProvider, scoped = "None")]
#[alice_web::http_request]
#[alice_web::authorize]
#[post("billing-system/WebhookSubscribe")]
pub async fn webhook_subscribe(
    web::Json(url): web::Json<Url>,
    #[inject] service: Arc<dyn UserWebhookService>,
) -> web::Json<ResponseBase<String>> {
    let user_id = url.user_id;
    let url = url.url;
    match service.register_webhook(&user_id, &url).await {
        Ok(_) => web::Json(ResponseBase::ok(None)),
        Err(_) => web::Json(ResponseBase::err(500, "Interval error")),
    }
}

#[actix_auto_inject(ServiceProvider, scoped = "user_info")]
#[alice_web::http_request]
#[alice_web::authorize]
#[get("billing-system/GetFlowNodesBill/{flow_instance_id}")]
pub async fn get_flow_nodes_bill(
    flow_instance_id: Path<String>,
    #[inject] service: Arc<dyn FlowNodeBillingService>,
) -> web::Json<ResponseBase<FlowBillResponse>> {
    match service.get_bill(&flow_instance_id).await {
        Ok(el) => {
            let response = FlowBillResponse::from(el);
            web::Json(ResponseBase::ok(Some(response)))
        }
        Err(e) => {
            tracing::error!("{e}");
            web::Json(ResponseBase::err(500, "Interval error"))
        }
    }
}

#[alice_di::auto_inject(ServiceProvider, scoped = "None")]
#[alice_web::message_consumer]
pub async fn bill_consumer(
    #[inject] service: Arc<dyn FlowNodeBillingService>,
    #[serialize] node_instance_id: String,
) -> anyhow::Result<()> {
    tracing::info!("Receive msg: {node_instance_id:#?}");
    service.record_bill(&node_instance_id).await
}
