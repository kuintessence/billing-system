use std::sync::Arc;

use alice_di::build_container;
use alice_di::IServiceProvider;
use alice_infrastructure::config::CommonConfig;
use alice_infrastructure::data::Database;
use alice_infrastructure::http_client::build_http_client;
use alice_infrastructure::middleware::authorization::AliceScopedConfig;
use uuid::Uuid;

use super::database::OrmRepo;
// use super::service::UserWebhookServiceImpl;
use crate::domain::service::*;
use crate::service::FlowNodeBillingServiceImpl;

build_container! {
    #[derive(Clone)]
    pub struct ServiceProvider;

    params(config: config::Config)

    scoped_params(scoped_config: AliceScopedConfig)

    scoped user_id: Option<Uuid> {
        build {
            scoped_config.user_info.map(|u| u.id)
        }
    }

    common_config: CommonConfig {
        build {
            let common_config: alice_infrastructure::config::CommonConfig = config.try_deserialize()?;
            common_config
        }
    }

    database: Arc<Database> {
        build async {
            Arc::new(Database::new(&common_config.db.url).await)
        }
    }

    scoped sea_orm_repository: Arc<OrmRepo> {
        build {
            Arc::new(OrmRepo::new(sp.provide()))
        }
    }

    scoped http_client: Arc<reqwest::Client> {
        build {
            build_http_client(&self.common_config.http_client)?
        }
    }

    // scoped user_webhook_service: Arc<dyn UserWebhookService>{
    //     build {
    //         let repo = sea_orm_repository.clone();
    //         Arc::new(UserWebhookServiceImpl::new(repo,http_client.clone()))
    //     }
    // }

    scoped billing_service: Arc<dyn FlowNodeBillingService>{
        build {
            let repo = sea_orm_repository.clone();
            // let service = user_webhook_service.clone();
            Arc::new(FlowNodeBillingServiceImpl::new(repo.clone(), repo.clone(), repo.clone(), repo.clone(), repo))
        }
    }

    after_build {
        // let arc_sp = Arc::new(sp.clone());
        // let consumer: ConsumerFn<ServiceProvider> = api::controller::bill_consumer;
        // let consumers = vec![consumer];
        // let config: alice_infrastructure::config::CommonConfig = arc_sp.provide();
        // let client_options = config.mq.consumer.clone();
        // let topics = config.mq.topics;
        // let message_queue = Arc::new(KafkaSingleTopicMessageQueueConsumer::new(&topics, client_options, arc_sp, consumers));
    }
}
