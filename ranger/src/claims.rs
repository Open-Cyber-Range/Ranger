use actix_http::HttpMessage;
use actix_web::web::Data;
use actix_web::Error;
use actix_web::{dev::ServiceRequest, error::ErrorUnauthorized};
use actix_web_grants::permissions::AttachPermissions;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{self, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RealmAccess {
    roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    pub name: String,
    pub realm_access: RealmAccess,
    pub sub: String,
    exp: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserDetails {
    pub user_id: String,
}

fn decode_jwt(token: &str, pem: &str) -> Result<Claims, Error> {
    let pem_file = format!(
        "-----BEGIN PUBLIC KEY-----
{pem}
-----END PUBLIC KEY-----"
    );
    let decoding_key = DecodingKey::from_rsa_pem(pem_file.as_bytes()).unwrap();
    jsonwebtoken::decode::<Claims>(token, &decoding_key, &Validation::new(Algorithm::RS256))
        .map(|data| data.claims)
        .map_err(|e| {
            log::error!("Failed to decode JWT: {}", e);
            ErrorUnauthorized(e.to_string())
        })
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    if let Some(app_data) = req.app_data::<Data<AppState>>() {
        let result = decode_jwt(
            credentials.token(),
            &app_data.configuration.keycloak.authentication_pem_content,
        );
        return match result {
            Ok(claims) => {
                req.attach(claims.realm_access.roles);
                req.extensions_mut().insert(UserDetails {
                    user_id: claims.sub,
                });

                Ok(req)
            }
            Err(e) => Err((e, req)),
        };
    }
    Err((ErrorUnauthorized("No app data".to_string()), req))
}
