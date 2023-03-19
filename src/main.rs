#[macro_use]
extern crate rocket;

use eclipse_chain_registry::entity::evm_chain;
use eclipse_chain_registry::entity::evm_chain::Model as EvmChain;
use eclipse_chain_registry::pool::Db;
use evm_chain::Entity as EvmChainEntity;
use migration::MigratorTrait;
use rocket::fairing;
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::request::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::serde::json::Json;
use rocket::Build;
use rocket::Rocket;
use sea_orm::ActiveModelTrait;
use sea_orm::EntityTrait;
use sea_orm::Set;
use sea_orm_rocket::Connection;
use sea_orm_rocket::Database;
use std::env;


struct ApiKey(String);

/// Returns true if `key` is a valid API key string.
fn is_valid(key: &str) -> bool {
    key == env::var("API_KEY").expect("api key env var not set!")
}

#[derive(Debug)]
enum ApiKeyError {
    BadCount,
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = req.headers().get("x-api-key").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            1 if is_valid(keys[0]) => Outcome::Success(ApiKey(keys[0].to_string())),
            1 => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
            _ => Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount)),
        }
    }
}


#[get("/evm_chains")]
async fn evm_chains(conn: Connection<'_, Db>) -> Result<Json<Vec<EvmChain>>, Status> {
    let db = conn.into_inner();
    let chains = EvmChainEntity::find()
        .all(db)
        .await
        .expect("couldnt load evm chains");
    Ok(Json(chains))
}

#[delete("/evm_chains/<chain_id>")]
async fn remove_evm_chain(conn: Connection<'_, Db>, chain_id: String, _key: ApiKey) -> Status {
    let db = conn.into_inner();
    let res = EvmChainEntity::delete_by_id(chain_id).exec(db).await;

    match res {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}

#[post("/evm_chains", data = "<evm_chain>")]
async fn add_evm_chain(
    conn: Connection<'_, Db>,
    evm_chain: Json<EvmChain>,
    _key: ApiKey,
) -> Status {
    let db = conn.into_inner();
    let chain = evm_chain.into_inner();

    let new_chain: evm_chain::ActiveModel = evm_chain::ActiveModel {
        chain_id: Set(chain.chain_id),
        rpc_urls: Set(chain.rpc_urls),
        block_explorer_urls: Set(chain.block_explorer_urls),
        icon_urls: Set(chain.icon_urls),
        chain_name: Set(chain.chain_name),
        native_currency_name: Set(chain.native_currency_name),
        native_currency_decimals: Set(chain.native_currency_decimals),
        native_currency_symbol: Set(chain.native_currency_symbol),
        data_availability: Set(chain.data_availability),
    };
    match new_chain.insert(db).await {
        Ok(_) => Status::Created,
        Err(_) => Status::UnprocessableEntity,
    }
}

#[get("/health")]
fn health_check() -> Status {
    Status::Ok
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}

#[launch]
fn rocket() -> _ {

    let cors = rocket_cors::CorsOptions::default()
    .to_cors().unwrap();

    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .attach(cors)
        .mount(
            "/",
            routes![add_evm_chain, evm_chains, remove_evm_chain, health_check],
        )
}
