use deadpool_postgres::{Config, ManagerConfig, Pool, PoolConfig, RecyclingMethod, Runtime};
use pg::NoTls;
use std::env;

pub async fn get_pool() -> Pool {
    dotenv::dotenv().ok(); // .env dosyasındaki parametreleri yüklüyor
                           // böylelikle std::env::var'dan env değişkeni olarak alabiliyoruz

    let mut cfg = Config::new();

    cfg.host = Some(env::var("DB_HOST").unwrap_or_else(|_| String::from("localhost")));
    cfg.dbname = Some(env::var("DB_NAME").unwrap_or_else(|_| String::from("test")));

    cfg.port = Some(
        env::var("DB_PORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(5432),
    );
    cfg.user = Some(env::var("DB_USER").unwrap_or_else(|_| String::from("postgres")));
    cfg.password = Some(env::var("DB_PASSWORD").unwrap_or_else(|_| String::from("secret")));
    let max_pool_size: usize = env::var("DB_POOL_SIZE")
        .unwrap_or_else(|_| String::from("50"))
        .parse()
        .unwrap();
    cfg.pool = Some(PoolConfig::new(max_pool_size));

    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    //TODO: Result dönmeli, buda ? olmalı
    cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap()
}
