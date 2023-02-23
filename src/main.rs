#[macro_use]
extern crate rocket;

use std::env;
use eclipse_chain_registry::entity::evm_chain::Model as EvmChain;
use eclipse_chain_registry::entity::evm_chain;
use evm_chain::Entity as EvmChainEntity;
use eclipse_chain_registry::pool::Db;
use rocket::request::Outcome;
use rocket::serde::json::Json;
use sea_orm::ActiveModelTrait;
use sea_orm_rocket::Connection;
use sea_orm_rocket::Database;
use sea_orm::Set;
use sea_orm::EntityTrait;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

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
    let chains = EvmChainEntity::find().all(db).await.expect("couldnt load evm chains");
    Ok(Json(chains))
}

#[post("/evm_chains", data = "<evm_chain>")]
async fn add_evm_chain(conn: Connection<'_, Db>, evm_chain: Json<EvmChain>, _key: ApiKey) -> Status {


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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![add_evm_chain, evm_chains])
}
