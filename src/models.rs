// use actix_web::cookie::time::Duration;
// use bson::oid::ObjectId;
// use chrono::{Utc , Duration};
// use serde::{Deserialize, Serialize};
use actix_web::cookie::time::Duration as CookieDuration; // for cookie expiry
use bson::oid::ObjectId;
use chrono::{Utc, Duration as ChronoDuration}; // for JWT expiry
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize , Deserialize , Clone)]
pub struct User{
    #[serde(rename ="_id" , skip_serializing_if = "Option::is_none")]
    pub id : Option<ObjectId>,
    pub email : String,
    pub password : String,
    pub name : Option<String>,
}

//Data recived registers and login 
#[derive(Debug , Serialize , Deserialize )]
pub struct AuthRequest {
    pub email : String,
    pub password : String,
    pub name : Option<String>
}
#[derive( Debug , Serialize , Deserialize , Clone)]
pub struct Claims{
    pub sub : String,
    pub exp : usize,
}
impl Claims {
    pub fn new(user_id: String, days: i64) -> Self {
        let exp = (Utc::now() + ChronoDuration::days(days)).timestamp() as usize;
        Claims { sub: user_id, exp }
    }
}



