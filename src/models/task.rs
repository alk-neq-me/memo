#[derive(Debug)]
pub struct Task {
    pub id: Option<u32>,
	pub title: String,
	pub is_completed: bool,
	pub book_id: Option<u32>,
}
