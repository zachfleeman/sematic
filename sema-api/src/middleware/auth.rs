use actix_web::dev::ServiceRequest;
use actix_web::{Error};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::{AuthenticationError};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
  sub: String,
  exp: usize
}

pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
  let key = req.app_data::<DecodingKey>().unwrap();

  let mut validation = Validation::new(Algorithm::HS256);
  validation.leeway = 120;

  match decode::<Claims>(credentials.token(), &key, &validation) {
    Ok(_) => Ok(req),
    Err(err) => {
      println!("valdation err: {:?}", err);
      // TODO: figure out how to better handle this error;
      let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);
      Err(AuthenticationError::from(config).into())
    }
  }
}