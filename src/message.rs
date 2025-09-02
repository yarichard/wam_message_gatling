use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub text: String,
    pub user_id: i32,
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Message {{ id: {}, text: {}, user_id: {} }}", self.id, self.text, self.user_id)
    }
}