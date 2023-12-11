use collaboflow_rs::bytes::Bytes;
use collaboflow_rs::{Authorization, CollaboflowClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use tracing::info;

const FID_KEY: &str = "fidFile";

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

    // ファイルIDを取得する
    let id = match resp.contents.get(FID_KEY) {
        None => "".to_string(),
        Some(fid_file) => match fid_file.get("value") {
            None => "".to_string(),
            Some(value) => match value.as_str() {
                None => "".to_string(),
                Some(v) => v.to_string(),
            },
        },
    };

    // ファイル名を取得する
    let name = match resp.contents.get(FID_KEY) {
        None => "".to_string(),
        Some(fid_file) => match fid_file.get("label") {
            None => "".to_string(),
            Some(value) => match value.as_str() {
                None => "".to_string(),
                Some(v) => v.to_string(),
            },
        },
    };

    Ok(BackupFile { id, name })
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
