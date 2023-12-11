use collaboflow_rs::bytes::Bytes;
use collaboflow_rs::{Authorization, CollaboflowClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use tracing::info;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CollaboflowWebhookResponse {
    contents: Value,
}

#[derive(Clone, Debug)]
pub struct BackupFile {
    pub id: String,
    pub name: String,
}

pub fn get_file_info(s: &str) -> Result<BackupFile, ()> {
    let resp = match serde_json::from_str::<CollaboflowWebhookResponse>(s) {
        Ok(resp) => resp,
        Err(_) => {
            return Err(());
        }
    };

    let file_id = resp
        .contents
        .get("fidFile")
        .unwrap()
        .get("value")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();
    let file_name = resp
        .contents
        .get("fidFile")
        .unwrap()
        .get("label")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    Ok(BackupFile {
        id: file_id,
        name: file_name,
    })
}

pub async fn download_file(id: &str) -> Result<Bytes, ()> {
    let client = collaboflow_client();
    info!("File ID: {}", id);
    match client.files.get(id).await {
        Ok(resp) => Ok(resp.body),
        Err(_) => Err(()),
    }
}

pub fn collaboflow_client() -> CollaboflowClient {
    // コラボフローに接続するための情報を環境変数から取得する
    let authorization = Authorization::with_api_key(
        env::var("USER_ID")
            .unwrap_or_else(|_| panic!("{} is undefined.", "USER_ID"))
            .as_str(),
        env::var("API_KEY")
            .unwrap_or_else(|_| panic!("{} is undefined.", "API_KEY"))
            .as_str(),
    );

    // コラボフロー REST API クライアントを生成する
    CollaboflowClient::new(
        env::var("BASE_URL")
            .unwrap_or_else(|_| panic!("{} is undefined.", "BASE_URL"))
            .as_str(),
        authorization,
    )
}
