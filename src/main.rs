use std::{backtrace::Backtrace, panic::{self, PanicHookInfo}};

use containerd_client::{
    connect,
    with_namespace,
    services::v1::{
        containers_client::ContainersClient,
        version_client::VersionClient
    }
};
use nanologger::{LogLevel, LoggerBuilder, error};
use pounce_daemon::config::{node::NodeConfig, pounce::PounceConfig};

fn panic_handler(panic_info: &PanicHookInfo) {
    println!("");
    error!("PANIC OCCURED: ");
    
    if let Some(location) = panic_info.location() {
        error!("Location  : {} at line {}:{}",
            location.file(),
            location.line(), location.column()
        );
    }

    if let Some(message) = panic_info.payload().downcast_ref::<&str>() {
        error!("Details   : {}", message);
    } else if let Some(message) = panic_info.payload().downcast_ref::<String>() {
        error!("Details   : {}", message);
    } else {
        error!("Details   : COULDN'T PARSE PANIC DETAILS");
    }
    
    let backtrace = Backtrace::capture();
    error!("Backtrace : {}", backtrace);
}

#[tokio::main]
async fn main() {
    // Logger
    LoggerBuilder::new()
        .thread_info(true)
        .timestamps(true)
        .level(LogLevel::Debug)
        .init().unwrap();

    panic::set_hook(Box::new(|info| {
        panic_handler(info);
    }));

    let config = PounceConfig::new().unwrap();
    let node_config = NodeConfig::new(&config.main.node_cfg_file).unwrap();

    for server in node_config.servers {
        println!("{}: {:?}", server.0, server.1);
    }
    
    let channel = connect(&config.containerd.socket_file).await.unwrap();

    let mut client = VersionClient::new(channel);
    let resp = client.version(()).await.unwrap();

    println!("Response: {:?}", resp.get_ref());
}
