use async_trait::async_trait;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use sea_orm::ConnectOptions;
use sea_orm_rocket::{rocket::figment::Figment, Database};
use std::env;
use std::time::Duration;
use dotenv::dotenv;

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

/// Add `/` and `:` to support Unix socket hosts in Cloud SQL:
/// <https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING>
const HOST: &AsciiSet = &CONTROLS.add(b'/').add(b':');

fn postgres_url(user: &str, password: &str, host: &str, db_name: &str) -> String {
    format!(
        "postgresql://{user}:{password}@{host}/{db_name}",
        user = utf8_percent_encode(user, USERINFO),
        password = utf8_percent_encode(password, USERINFO),
        host = utf8_percent_encode(host, HOST),
        db_name = utf8_percent_encode(db_name, PATH),
    )
}

fn postgres_url_from_env() -> String {
    let user = env::var("POSTGRES_USER").expect("POSTGRES_USER not set");
    let password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD not set");
    let host = env::var("POSTGRES_HOST").expect("POSTGRES_HOST not set");
    let db_name = env::var("POSTGRES_DB").expect("POSTGRES_DB not set");

    postgres_url(&user, &password, &host, &db_name)
}

#[async_trait]
impl sea_orm_rocket::Pool for SeaOrmPool {
    type Error = sea_orm::DbErr;

    type Connection = sea_orm::DatabaseConnection;

    async fn init(_figment: &Figment) -> Result<Self, Self::Error> {
        dotenv().ok();
        let mut options = ConnectOptions::new(postgres_url_from_env());
        options
            .max_connections(1024)
            .connect_timeout(Duration::from_secs(3));
        let conn = sea_orm::Database::connect(options).await?;

        Ok(SeaOrmPool { conn })
    }

    fn borrow(&self) -> &Self::Connection {
        &self.conn
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn postgres_url_to_string() {
        assert_eq!(
            postgres_url("user", "p@ssw0rd!", "127.0.0.1", "db"),
            "postgresql://user:p%40ssw0rd!@127.0.0.1/db",
        );
        assert_eq!(
            postgres_url(
                "user",
                "p@ssw0rd!",
                "/cloudsql/eclipse-123456:us-central1:eclipse",
                "db",
            ),
            "postgresql://user:p%40ssw0rd!@%2Fcloudsql%2Feclipse-123456%3Aus-central1%3Aeclipse/db",
        );
    }
}
