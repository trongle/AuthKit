use hands_on_maud::http;
use sqlx::{migrate, mysql::MySqlPoolOptions};

#[tokio::main]
async fn main() {
    // Setup Mysql pool connections
    // for the applcation.
    let db = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://localhost:3306/hands_on_maud")
        .await
        .unwrap();

    // Run database migrations so we
    // can ensure the database is correctly
    // startup. If we used sqlx-cli to run
    // the migrations, then this action
    // will not re-run the migrations. So
    // it is safe.
    migrate!().run(&db).await.unwrap();

    http::server(db).await;
}
