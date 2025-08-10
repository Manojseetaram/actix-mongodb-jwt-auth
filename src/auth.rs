//Create a Jwt from Clims

use jsonwebtoken::{encode,decode , DecodingKey, EncodingKey, Header, TokenData, Validation};

use crate::models::Claims;

pub fn create_jwt(claims : &Claims , secret : &str)-> String{
       encode(&Header::default(), claims, &EncodingKey::from_secret(secret.as_ref())).expect("JWT creation failed")

}

//Verify the Jwt return Claims
pub fn verify_jwt (token : &str , secret : &str)-> TokenData<Claims>{

      decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default()
      ).expect("Invalid jwt or expired jwt")

}