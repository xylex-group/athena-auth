use uuid::Uuid;

pub fn generate_api_key() -> String {
    format!("ak_{}", Uuid::new_v4().to_string().replace("-", ""))
}
