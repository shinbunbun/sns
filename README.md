# sns

株式会社ゆめみのインターンシップ課題で製作した、Rust製SNSアプリ

# 動かし方

1. `export DATABASE_URL=mysql://user:password@db:3306/local_db`で環境変数を設定する
2. migrationディレクトリで`cargo run`を実行してDBのmigrationをする
3. リポジトリ直下で`cargo run`を実行してアプリケーションを起動する
