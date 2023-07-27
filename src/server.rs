use std::sync::Arc;

use actix_easy_multipart::MultipartFormConfig;
use actix_rt::task::JoinHandle;
use alice_architecture::IBackgroundService;
use alice_di::IServiceProvider;
use alice_infrastructure::config::build_config;
use alice_infrastructure::config::CommonConfig;
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
    if let Err(e) = alice_infrastructure::telemetry::initialize_telemetry(common_config.telemetry())
    {
        return eprintln!("{}: {}", "Cannot build logger".red(), e);
    }

    let tasks: Vec<Arc<dyn IBackgroundService + Send + Sync>> = service_provider.provide();

    let handles = tasks
        .into_iter()
        .map(|x| {
            tokio::spawn(async move {
                let task = x.clone();
                task.run().await
            })
        })
        .collect::<Vec<JoinHandle<()>>>();

    tokio::select! {
        _ = initialize_web_host(service_provider) => (),
        _ = tokio::signal::ctrl_c() => {
            log::info!("Stoping Services (ctrl-c handling).");
            for handle in handles {
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
            .service(api::controller::webhook_subscribe)
    })
    .bind((
        common_config.host().bind_address().to_owned(),
        *common_config.host().bind_port(),
    ))
    .unwrap()
    .disable_signals()
    .run()
    .await
    {
        Ok(_) => log::info!("Web server stopped successfully."),
        Err(e) => log::error!("Web server into erorr: {}", e),
    }
}
