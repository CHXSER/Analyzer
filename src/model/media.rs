use super::photo::Photo;
use super::video::Video;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Media {
    Photo(Photo),
    Video(Video),
}
