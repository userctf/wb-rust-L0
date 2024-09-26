use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::sync::futures;

#[derive(Clone)]
pub(crate) struct DataBase {
    pool: Pool<Postgres>
}


impl DataBase {
    pub(crate) async fn connect(username: String, password: String, db_name: String) -> Self {
        let url = format!("postgres://{}:{}@localhost/{}", username, password, db_name);
        let max_connections = 5;

        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(&url)
            .await
            .expect("Couldn't connect to the database");

        let databse = Self {
            pool,
        };

        databse.init().await;
        databse

    }

    pub(crate) fn is_ok(&self) -> bool {
        true
    }

    pub(crate) async fn init(&self) {
        let create_delivery_db_request = r#"
        CREATE TABLE IF NOT EXISTS delivery (
            delivery_id BIGSERIAL PRIMARY KEY,
            name VARCHAR(255),
            phone VARCHAR(255),
            zip VARCHAR(255),
            city VARCHAR(255),
            address VARCHAR(255),
            region VARCHAR(255),
            email VARCHAR(255)
        );
        "#;
        sqlx::query(create_delivery_db_request)
        .fetch_optional(&self.pool).await.expect("Can't init DB tables");

        let create_payment_db_request = r#"
        CREATE TABLE IF NOT EXISTS payment (
            transaction VARCHAR(255) PRIMARY KEY,
            request_id VARCHAR(255),
            currency VARCHAR(255),
            provider VARCHAR(255),
            amount INTEGER,
            payment_dt BIGINT,
            bank VARCHAR(255),
            delivery_cost INTEGER,
            goods_total INTEGER,
            custom_fee INTEGER
        );
        "#;
        sqlx::query(create_payment_db_request)
        .fetch_optional(&self.pool).await.expect("Can't init DB tables");

        let create_item_db_request = r#"
        CREATE TABLE IF NOT EXISTS item (
            chrt_id BIGINT PRIMARY KEY,
            track_number VARCHAR(255),
            price INTEGER,
            rid VARCHAR(255),
            name VARCHAR(255),
            sale INTEGER,
            size VARCHAR(255),
            total_price INTEGER,
            nm_id BIGINT,
            brand VARCHAR(255),
            status INTEGER
        );
        "#;
        sqlx::query(create_item_db_request)
        .fetch_optional(&self.pool).await.expect("Can't init DB tables");

        let create_map_items_db_request = r#"
        CREATE TABLE IF NOT EXISTS map_items (
            item_id BIGSERIAL PRIMARY KEY,
            map_item_id BIGINT REFERENCES item ON DELETE SET NULL
        );
        "#;
        sqlx::query(create_map_items_db_request)
        .fetch_optional(&self.pool).await.expect("Can't init DB tables");

        let create_orders_db_request = r#"
        CREATE TABLE IF NOT EXISTS orders (
            order_uid VARCHAR(255) PRIMARY KEY,
            track_number VARCHAR(255),
            entry VARCHAR(255),
            delivery_id INT REFERENCES delivery ON DELETE SET NULL,
            payment_id VARCHAR(255) REFERENCES payment ON DELETE SET NULL,
            items INT,
            locale VARCHAR(255),
            internal_signature VARCHAR(255),
            customer_id VARCHAR(255),
            delivery_service VARCHAR(255),
            shardkey VARCHAR(255),
            sm_id BIGINT,
            date_created VARCHAR(255),
            oof_shard VARCHAR(255)
        );
        "#;
        sqlx::query(create_orders_db_request)
        .fetch_optional(&self.pool).await.expect("Can't init DB tables");
    }

}