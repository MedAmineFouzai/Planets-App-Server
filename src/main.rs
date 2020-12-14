mod controller;
mod model;
use actix_web::{App,HttpServer};
use actix_cors::Cors;
use model::{UserModel};
use mongodb::{options::ClientOptions, Client};
pub struct ModelsContainer {
   user:UserModel,
 
}
impl ModelsContainer {
    pub fn new(user:UserModel) -> ModelsContainer {
        ModelsContainer {user}
    }
  }

pub struct AppState{
    models_container:ModelsContainer,

}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  
    const DB:&str="users";
    let client_options = ClientOptions::parse("mongodb://localhost:27017").unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database(DB);
    let user_collection = db.collection("user");

    HttpServer::new(move || {
        let models_container = ModelsContainer::new(
            
            UserModel::new(user_collection.clone()) 
        );

        let cors=Cors::default()
        .allow_any_origin()
        .allow_any_header()
        .allow_any_method()
        .supports_credentials();


        App::new()
        .wrap(cors)
        .data(AppState{models_container})
        .service(controller::plants)
        .service(controller::search)
        .service(controller::login)
        .service(controller::signup)
        .service(controller::delete_account)
        .service(controller::update_account)
        .service(controller::update_password)
        .service(controller::add_favorite)
        .service(controller::delete_favorite)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}