# vrcapi-proxy

VRChat REST API / WebSocket APIのプロキシサーバーです。jsonデータは信用できない値、自分が使用していない値が含まれていない場合があります。

## 使い方

### 既存のリリースが実行環境に対応している場合

1. [Releasesページ](https://github.com/yu256/vrcapi-proxy/releases)にアクセスします。
2. 環境に合ったリリースをダウンロードします。

### 対応していない場合

1. Rust環境をセットアップします。
2. `rustup default nightly` を実行して Nightly に切り替えます。
3. このリポジトリを `git clone https://github.com/yu256/vrcapi-proxy.git` でクローンします。
4. クローンしたリポジトリに移動し、`cargo build --release` を実行してバイナリを生成します。生成されたバイナリは `/target/release` ディレクトリに保存されます。

初回起動時（すぐに停止します）、ホームディレクトリの `/vrcapi-proxy` フォルダにjsonが生成されます。認証ID, CORS, listenするポートの設定を行ってください。設定が完了したら再度起動します。
