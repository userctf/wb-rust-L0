use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Error};
use sqlx::Row;

use crate::model::{Item, WBmodel};

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

    pub(crate) async fn add_order(&self, order: &WBmodel) -> Result<(), Error> {
        let insert_delivery = r#"
            INSERT INTO Delivery (name, phone, zip, city, address, region, email) 
                VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING delivery_id;
        "#;
        let insert_payment = r#"
        INSERT INTO payment (transaction, request_id, currency, provider, amount, payment_dt, bank, delivery_cost, goods_total, custom_fee)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        "#;
        let insert_orders = r#"
        INSERT INTO orders (order_uid, track_number, entry, delivery_id, payment_id, locale, internal_signature, customer_id, delivery_service, shardkey, sm_id, date_created, oof_shard)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        "#;

        let mut tx = self.pool.begin().await?;

        let delivery_id: i64 = sqlx::query(insert_delivery)
            .bind(&order.delivery.name)
            .bind(&order.delivery.phone)
            .bind(&order.delivery.zip)
            .bind(&order.delivery.city)
            .bind(&order.delivery.address)
            .bind(&order.delivery.region)
            .bind(&order.delivery.email)
            .fetch_one(&mut *tx)
            .await?.get(0);

        sqlx::query(insert_payment)
            .bind(&order.payment.transaction)
            .bind(&order.payment.request_id)
            .bind(&order.payment.currency)
            .bind(&order.payment.provider)
            .bind(order.payment.amount)
            .bind(order.payment.payment_dt)
            .bind(&order.payment.bank)
            .bind(order.payment.delivery_cost)
            .bind(order.payment.goods_total)
            .bind(order.payment.custom_fee)
            .execute(&mut *tx)
            .await?;

        sqlx::query(insert_orders)
            .bind(&order.order_uid)
            .bind(&order.track_number)
            .bind(&order.entry)
            .bind(delivery_id)
            .bind(&order.payment.transaction)
            .bind(&order.locale)
            .bind(&order.internal_signature)
            .bind(&order.customer_id)
            .bind(&order.delivery_service)
            .bind(&order.shardkey)
            .bind(order.sm_id)
            .bind(&order.date_created)
            .bind(&order.oof_shard)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        self.add_items(&order.order_uid, &order.items).await?;
        Ok(())
    }

    async fn add_items(&self, order_uid: &String, items: &Vec<Item>) -> Result<(), Error> {
        let add_item = r#"
            INSERT INTO item (chrt_id, track_number, price, rid, name, sale, size, total_price, nm_id, brand, status)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11);
        "#;
        let add_item_mapping = r#"
        INSERT INTO map_items (map_order_uid, map_chrt_id)
            VALUES ($1, $2);
        "#;
        let mut tx = self.pool.begin().await?;

        for i in items {
            sqlx::query(add_item)
            .bind(i.chrt_id)
            .bind(&i.track_number)
            .bind(i.price)
            .bind(&i.rid)
            .bind(&i.name)
            .bind(i.sale)
            .bind(&i.size)
            .bind(i.total_price)
            .bind(i.nm_id)
            .bind(&i.brand)
            .bind(i.status)
            .execute(& mut *tx).await?;

            sqlx::query(add_item_mapping)
            .bind(order_uid)
            .bind(i.chrt_id)
            .execute(& mut *tx).await?;
        }

        tx.commit().await?;
        Ok(())
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

        let create_orders_db_request = r#"
        CREATE TABLE IF NOT EXISTS orders (
            order_uid VARCHAR(255) PRIMARY KEY,
            track_number VARCHAR(255),
            entry VARCHAR(255),
            delivery_id INT REFERENCES delivery ON DELETE SET NULL,
            payment_id VARCHAR(255) REFERENCES payment ON DELETE SET NULL,
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

        let create_map_items_db_request = r#"
        CREATE TABLE IF NOT EXISTS map_items (
            map_order_uid VARCHAR(255) REFERENCES orders ON DELETE CASCADE,
            map_chrt_id BIGINT REFERENCES item ON DELETE CASCADE,
            PRIMARY KEY(map_order_uid, map_chrt_id)
        );
        "#;
        sqlx::query(create_map_items_db_request)
        .fetch_optional(&self.pool).await.expect("Can't init DB tables");
    }

}