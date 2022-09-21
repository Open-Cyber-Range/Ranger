mod validation;

use uuid::Uuid;

pub use validation::*;
pub fn default_uuid() -> Uuid {
    Uuid::new_v4()
}
