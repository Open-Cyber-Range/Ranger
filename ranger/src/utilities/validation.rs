use actix_web::ResponseError;

pub trait Validation {
    fn validate(&self) -> Result<(), Box<dyn ResponseError>>
    where
        Self: Sized;
}
