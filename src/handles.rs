use core::sync;

use actix_web::{body, web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use bson::doc;
use mongodb::Collection;

use crate::{auth::create_jwt, models::{AuthRequest, Claims, User}};




pub async fn register(users : web::Data<Collection<User>> , data : web::Json<AuthRequest> , secret : web::Data<String>)-> impl Responder{
               //1. check if email allready exists

               if users.find_one(doc! {"email" : &data.email}, None).await.unwrap().is_some(){
                       return  HttpResponse::Conflict().body("User allready exists");

               }
               //Hash password 

               let hashed = hash(&data.password, DEFAULT_COST).unwrap();  

               //Create User structure 

               let new_user = User {
                id : None,
                email : String,
                password : hashed ,
                name : String 
               };
               

               //insert into the MongoDb 

               let insert_user = users.insert_one(new_user , None).await.unwrap();
               let id = insert_user.inserted_id.as_object_id().unwrap().to_hex();

               //create a Jwt 
               let claims = Claims::new(id.clone() , 7 );
               let token = create_jwt(&claims, &secret);

               HttpResponse::Created().json(serde_json::json!({
                "id" : id ,
                "token" : token
               }))

}


pub async  fn login(users : web::Data<Collection<User>> , data : web::Json<AuthRequest> , secret : web::Data<String>) -> impl Responder{

       // 1 . Find User by Email
       let user = match users.find_one(doc! {"email" : &data.email}).await.unwrap() {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().body("Invalid creadentioals")
           };
      //verify password 
      if verify(&data.password, &user.password).unwrap(){
           return  HttpResponse::Unauthorized().body("Invalid Credentials");
      }
      //3. JWT Password create
      let id = user.id.unwrap().to_hex();
      let claims = Claims::new(id.clone() , 7);
      let token = create_jwt(&claims, &secret);

      HttpResponse::Ok().json(serde_json::json!({
        "id" : id ,
        "token" : token
      }

      ))

}