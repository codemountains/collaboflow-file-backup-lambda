# Collaboflow File Backup Lambda

コラボフロー Webhook を起点に添付ファイルを Amazon S3 にバックアップとして保存する AWS Lambda 関数

<div align="center">
  <img src="assets/collaboflow_file_bk_lambda.png" alt=""/>
</div>

## Deploy

### Build and Deploy

ビルド後、関数 URL を有効にしてデプロイします。

```shell
cargo lambda build --release
cargo lambda deploy --enable-function-url
```

### 環境変数

- `BACKUP_BUCKET_NAME`: バックアップを格納する S3 バケット名
- `BACKUP_FID_KEY`: 添付ファイルパーツのパーツ ID
- `CF_API_KEY`: コラボフロー API キー
- `CF_BASE_URL`: コラボフロー API URL (`https://{hostname}/{instance}/api/index.cfm`)
- `CF_USER_ID`: コラボフローの管理者ユーザー ID

## Setting up

Lambda 関数をデプロイ後、コラボフローで以下の設定をおこなう。

1. フォームを作成する
2. 添付ファイルパーツのパーツ ID を `fidFile` とする
3. 経路を作成する
4. 経路設定から Webhook に Lambda 関数 URL を登録する
5. 「申請書の経路終了時に通知する」を有効にする

## LICENSE

This project is licensed under the [MIT license](LICENSE).
