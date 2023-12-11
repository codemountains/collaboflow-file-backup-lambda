mod collaboflow;
mod s3;

use crate::collaboflow::{download_file, get_file_info};
use crate::s3::put_object;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use std::env;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let body = event.body();
    let s = std::str::from_utf8(body).expect("invalid utf-8 sequence");

    // Serialize JSON into struct.
    // If JSON is incorrect, send back 400 with error.
    let backup_file = match get_file_info(s) {
        Ok(info) => info,
        Err(_) => {
            let resp = Response::builder()
                .status(400)
                .header("content-type", "text/html")
                .body("Error...".into())
                .map_err(Box::new)?;
            return Ok(resp);
        }
    };

    if !backup_file.id.is_empty() {
        let bucket = env::var("BACKUP_BUCKET_NAME")
            .unwrap_or_else(|_| panic!("{} is undefined.", "BACKUP_BUCKET_NAME"));

        // コラボフローからファイルを取得する
        if let Ok(data) = download_file(&backup_file.id).await {
            // ファイルIDでフォルダを作成し、その中にファイルを保存する
            let key = format!("{}/{}", &backup_file.id, &backup_file.name);
            let _ = put_object(&bucket, &key, data).await;
        }
    }

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body("Success!".into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
