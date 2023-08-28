use std::sync::Arc;

use alice_architecture::hosting::IBackgroundService;
use alice_di::build_container;
use alice_di::IServiceProvider;
use alice_infrastructure::data::db::Database;
use alice_infrastructure::message_queue::{
    InternalMessageQueueProducer, KafkaMessageQueue, KafkaSingleTopicMessageQueueConsumer,
};
use alice_infrastructure::ConsumerFn;

use super::database::SeaOrmDbRepository;
use super::service::UserWebhookServiceImpl;
use crate::api;
use crate::domain::service::*;
use crate::service::FlowNodeBillingServiceImpl;

build_container! {
    #[derive(Clone)]
    pub struct ServiceProvider;
    params(config: config::Config)
    scoped_params(_user_info: Option<alice_architecture::authorization::UserInfo>)
    common_config: alice_infrastructure::config::CommonConfig {
        build {
            let common_config: alice_infrastructure::config::CommonConfig = config.get("common")?;
            common_config
        }
    }
    database: Arc<Database> {
        build async {
            Arc::new(Database::new(common_config.db().url()).await)
        }
    }
    scoped sea_orm_repository: Arc<SeaOrmDbRepository> {
        build {
            Arc::new(SeaOrmDbRepository::new(sp.provide()))
        }
    }
    background_services: Vec<Arc<dyn IBackgroundService + Send + Sync>> {
        build {
            let result: Vec<Arc<dyn IBackgroundService + Send + Sync>> =
                vec![

                ];
            result
        }
    }
    scoped http_client: Arc<reqwest::Client> {
        build {
            Arc::new(reqwest::Client::builder().connect_timeout(std::time::Duration::from_secs(2)).build()?)
        }
    }

    scoped user_webhook_service: Arc<dyn UserWebhookService>{
        build {
            let repo = sea_orm_repository.clone();
            Arc::new(UserWebhookServiceImpl::new(repo,http_client.clone()))
        }
    }
    scoped billing_service: Arc<dyn FlowNodeBillingService>{
        build {
            let repo = sea_orm_repository.clone();
            let service = user_webhook_service.clone();
            Arc::new(FlowNodeBillingServiceImpl::new(repo.clone(), repo.clone(), repo.clone(),repo.clone(),repo,service))
        }
    }

    internal_message_queue_producer: Arc<InternalMessageQueueProducer> {
        build{
            Arc::new(InternalMessageQueueProducer::new())
        }
    }
    message_queue:Arc<KafkaMessageQueue>{
        build{
            let client_options = common_config.mq().client_options();
            Arc::new(KafkaMessageQueue::new(client_options))
        }
    }
    after_build {
        let arc_sp = Arc::new(sp.clone());
        let consumer: ConsumerFn<ServiceProvider> = api::controller::bill_consumer;
        let consumers = vec![consumer];
        let config: alice_infrastructure::config::CommonConfig = arc_sp.provide();
        let client_options = config.mq().client_options().clone();
        let topics = config.mq().topics();
        let message_queue = Arc::new(KafkaSingleTopicMessageQueueConsumer::new(topics,client_options, arc_sp, consumers));
        sp.background_services.push(message_queue)
    }
}
