use super::photo::Photo;
use super::video::Video;

#[derive(Debug, Clone)]
pub enum Media {
    Photo(Photo),
    Video(Video),
}
