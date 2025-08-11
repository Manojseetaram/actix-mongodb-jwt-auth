use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use bson::doc;
use mongodb::Collection;

use crate::{auth::create_jwt, models::{AuthRequest, Claims, User}};

pub async fn register(
    users: web::Data<Collection<User>>,
    data: web::Json<AuthRequest>,
    secret: web::Data<String>
) -> impl Responder {
    // 1. Check if email already exists
    if let Ok(Some(_)) = users.find_one(doc! { "email": &data.email }).await {
        return HttpResponse::Conflict().body("User already exists");
    }

    // 2. Hash password
    let hashed = hash(&data.password, DEFAULT_COST).unwrap();

    // 3. Create user
    let new_user = User {
        id: None,
        email: data.email.clone(),
        password: hashed,
        name: data.name.clone(),
    };

    // 4. Insert into MongoDB
    let insert_user = users.insert_one(new_user).await.unwrap();
    let id = insert_user
        .inserted_id
        .as_object_id()
        .unwrap()
        .to_hex();

    // 5. Create JWT
    let claims = Claims::new(id.clone(), 7);
    let token = create_jwt(&claims, &secret);

    HttpResponse::Created().json(serde_json::json!({
        "id": id,
        "token": token
    }))
}


pub async fn login(
    users: web::Data<Collection<User>>,
    data: web::Json<AuthRequest>,
    secret: web::Data<String>
) -> impl Responder {
    // 1. Find user by email
    let user = match users.find_one(doc! { "email": &data.email }).await.unwrap() {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().body("Invalid credentials"),
    };

    // 2. Verify password
    if !verify(&data.password, &user.password).unwrap() {
    return HttpResponse::Unauthorized().body("Invalid Credentials");
}


    // 3. Create JWT
    let id = user.id.unwrap().to_hex();
    let claims = Claims::new(id.clone(), 7);
    let token = create_jwt(&claims, &secret);

    HttpResponse::Ok().json(serde_json::json!({
        "id": id,
        "token": token
    }))
}
