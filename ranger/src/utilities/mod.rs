use uuid::Uuid;

pub fn default_uuid() -> Uuid {
    Uuid::new_v4()
}
