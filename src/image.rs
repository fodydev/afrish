use super::wish;

#[derive(Clone)]
pub struct TkImage {
    pub id: String,
}

pub fn make_image (filename: &str) -> TkImage {
    let id = wish::next_wid(".");
    let msg = format!("image create photo {} -file {}", id, filename);
    wish::tell_wish(&msg);

    TkImage {
        id,
    }
}

