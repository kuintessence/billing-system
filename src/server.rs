use std::sync::Arc;

use actix_easy_multipart::MultipartFormConfig;
use alice_architecture::background_service::BackgroundService;
use alice_di::IServiceProvider;
use alice_infrastructure::config::build_config;
use alice_infrastructure::config::CommonConfig;
use alice_infrastructure::message_queue::KafkaSingleTopicMessageQueueConsumer;
use alice_infrastructure::telemetry::init_telemetry;
use alice_infrastructure::ConsumerFn;
use colored::Colorize;

use crate::api;
use crate::infrastructure::ServiceProvider;

pub fn run() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_run());
}

pub async fn async_run() {
    let config = match build_config() {
        Ok(x) => x,
        Err(e) => {
            return eprintln!("{}: {}", "Cannot build config".red(), e);
        }
    };

    let service_provider = match ServiceProvider::build(config).await {
        Ok(x) => Arc::new(x),
        Err(e) => {
            return eprintln!("{}: {}", "Cannot build Service Provider".red(), e);
        }
    };

    let common_config: alice_infrastructure::config::CommonConfig = service_provider.provide();
    if let Err(e) = init_telemetry(&common_config.telemetry) {
        return eprintln!("{}: {}", "Cannot build logger".red(), e);
    }

    let background_services = async {
        let arc_sp = service_provider.clone();
        let consumer: ConsumerFn<ServiceProvider> = api::controller::bill_consumer;
        let consumers = vec![consumer];
        let client_options = common_config.mq.consumer.clone();
        let topics = common_config.mq.topics;
        let message_queue =
            KafkaSingleTopicMessageQueueConsumer::new(&topics, client_options, arc_sp, consumers);
        let services = [tokio::spawn(async move { message_queue.run().await })];
        Result::<_, anyhow::Error>::Ok(services)
    }
    .await;

    if let Err(e) = background_services {
        return eprintln!("{}: {}", "Cannot start background services".red(), e);
    }

    tokio::select! {
        _ = initialize_web_host(service_provider) => (),
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("Stoping Services (ctrl-c handling).");
            for handle in background_services.unwrap() {
                handle.abort()
            }
            std::process::exit(0);
        }
    }
}

pub async fn initialize_web_host(sp: Arc<ServiceProvider>) {
    let common_config: CommonConfig = sp.provide();
    match actix_web::HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            .max_age(86400);

        actix_web::App::new()
            .wrap(cors)
            .app_data(MultipartFormConfig::default().total_limit(100 * 1024 * 1024))
            .app_data(actix_web::web::Data::from(sp.clone()))
            .wrap(tracing_actix_web::TracingLogger::default())
            .service(api::controller::get_flow_nodes_bill)
        // .service(api::controller::webhook_subscribe)
    })
    .bind((
        common_config.host.bind_address.to_owned(),
        common_config.host.bind_port,
    ))
    .unwrap()
    .disable_signals()
    .run()
    .await
    {
        Ok(_) => tracing::info!("Web server stopped successfully."),
        Err(e) => tracing::error!("Web server into erorr: {}", e),
    }
}
