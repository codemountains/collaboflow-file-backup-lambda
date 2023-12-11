use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::{config::Region, Client};
use collaboflow_rs::bytes::Bytes;
use std::error::Error;
use tracing::info;

pub async fn put_object(bucket: &str, key: &str, data: Bytes) -> Result<(), Box<dyn Error>> {
    let region = Some("ap-northeast-1");
    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("ap-northeast-1"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    let body = ByteStream::from(data.to_vec());
    let request = client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(body)
        .send()
        .await?;

    info!("{:?}", request);

    Ok(())
}
