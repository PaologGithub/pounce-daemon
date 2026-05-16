use containerd_client::{
    connect,
    with_namespace,
    services::v1::{
        containers_client::ContainersClient,
        version_client::VersionClient
    }
};

#[tokio::main]
async fn main() {
    let channel = connect("/run/containerd/containerd.sock").await.unwrap();

    let mut client = VersionClient::new(channel);
    let resp = client.version(()).await.unwrap();

    println!("Response: {:?}", resp.get_ref());
}
