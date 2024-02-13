use uuid::Uuid;

pub fn generate_user_id() -> String {
	return  Uuid::new_v4().to_string();
}
