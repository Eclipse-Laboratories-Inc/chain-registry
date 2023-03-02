use async_trait::async_trait;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use sea_orm::ConnectOptions;
use sea_orm_rocket::{rocket::figment::Figment, Config, Database};
use std::env;
use std::time::Duration;

#[derive(Database, Debug)]
#[database("chain_registry")]
pub struct Db(SeaOrmPool);

#[derive(Debug, Clone)]
pub struct SeaOrmPool {
    pub conn: sea_orm::DatabaseConnection,
}

/// <https://url.spec.whatwg.org/#query-percent-encode-set>
const QUERY: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'#').add(b'<').add(b'>');

/// <https://url.spec.whatwg.org/#path-percent-encode-set>
const PATH: &AsciiSet = &QUERY.add(b'?').add(b'`').add(b'{').add(b'}');

/// <https://url.spec.whatwg.org/#userinfo-percent-encode-set>
const USERINFO: &AsciiSet = &PATH
    .add(b'/')
    .add(b':')
    .add(b';')
    .add(b'=')
    .add(b'@')
    .add(b'[')
    .add(b'\\')
    .add(b']')
    .add(b'^')
    .add(b'|');

fn postgres_url() -> String {
    let user = env::var("POSTGRES_USER").expect("POSTGRES_USER not set");
    let password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD not set");
    let host = env::var("POSTGRES_HOST").expect("POSTGRES_HOST not set");
    let db_name = env::var("POSTGRES_DB").expect("POSTGRES_DB not set");

    format!(
        "postgresql://{user}:{password}@{host}/{db_name}",
        user = utf8_percent_encode(&user, USERINFO),
        password = utf8_percent_encode(&password, USERINFO),
        host = utf8_percent_encode(&host, CONTROLS),
        db_name = utf8_percent_encode(&db_name, PATH),
    )
}

#[async_trait]
impl sea_orm_rocket::Pool for SeaOrmPool {
    type Error = sea_orm::DbErr;

    type Connection = sea_orm::DatabaseConnection;

    async fn init(_figment: &Figment) -> Result<Self, Self::Error> {
        let config: Config = sea_orm_rocket::Config {
            url: postgres_url(),
            min_connections: None,
            max_connections: 1024,
            connect_timeout: 3,
            idle_timeout: None,
            sqlx_logging: true,
        };

        let mut options: ConnectOptions = config.url.into();
        options
            .max_connections(config.max_connections as u32)
            .min_connections(config.min_connections.unwrap_or_default())
            .connect_timeout(Duration::from_secs(config.connect_timeout));
        if let Some(idle_timeout) = config.idle_timeout {
            options.idle_timeout(Duration::from_secs(idle_timeout));
        }
        let conn = sea_orm::Database::connect(options).await?;

        Ok(SeaOrmPool { conn })
    }

    fn borrow(&self) -> &Self::Connection {
        &self.conn
    }
}
