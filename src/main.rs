use containerd_client::{
    connect,
    with_namespace,
    services::v1::{
        containers_client::ContainersClient,
        version_client::VersionClient
    }
};
use nanologger::{LogLevel, LoggerBuilder};
use pounce_daemon::config::{node::NodeConfig, pounce::PounceConfig};

#[tokio::main]
async fn main() {
    // Logger
    LoggerBuilder::new()
        .thread_info(true)
        .timestamps(true)
        .level(LogLevel::Debug)
        .init().unwrap();

    let config = PounceConfig::new().unwrap();
    let node_config = NodeConfig::new(&config.main.node_cfg_file).unwrap();
    
    let channel = connect(&config.containerd.socket_file).await.unwrap();

    let mut client = VersionClient::new(channel);
    let resp = client.version(()).await.unwrap();

    println!("Response: {:?}", resp.get_ref());
}
