use mongodb::{options::ClientOptions, Client, Collection};

use crate::models::User;


//Create the mongodb and get user connections below the code 
pub async fn get_user_connection(uri : &str , data_name : &str)-> Collection<User>{
    let mut options = ClientOptions::parse(uri).await.expect("Invalid url");
    options.app_name = Some("actix-mongo-auth".to_string());
    let client = Client::with_options(options).expect("Mongo Db connections Failed");
    client.database(data_name).collection::<User>("users")
}   