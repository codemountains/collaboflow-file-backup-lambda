use collaboflow_rs::bytes::Bytes;
use collaboflow_rs::{Authorization, CollaboflowClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

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

    let fid =
        env::var("BACKUP_FID_KEY").unwrap_or_else(|_| panic!("{} is undefined.", "BACKUP_FID_KEY"));

    // ファイルIDを取得する
    let id = match resp.contents.get(&fid) {
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
    let name = match resp.contents.get(&fid) {
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
    match client.files.get(id).await {
        Ok(resp) => Ok(resp.body),
        Err(_) => Err(()),
    }
}

pub fn collaboflow_client() -> CollaboflowClient {
    // コラボフローに接続するための情報を環境変数から取得する
    let authorization = Authorization::with_api_key(
        env::var("CF_USER_ID")
            .unwrap_or_else(|_| panic!("{} is undefined.", "CF_USER_ID"))
            .as_str(),
        env::var("CF_API_KEY")
            .unwrap_or_else(|_| panic!("{} is undefined.", "CF_API_KEY"))
            .as_str(),
    );

    // コラボフロー REST API クライアントを生成する
    CollaboflowClient::new(
        env::var("CF_BASE_URL")
            .unwrap_or_else(|_| panic!("{} is undefined.", "CF_BASE_URL"))
            .as_str(),
        authorization,
    )
}
