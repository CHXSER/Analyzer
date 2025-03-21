use vid_dup_finder_lib::MatchGroup;

use super::photo::{Photo, PhotoMatchGroup};
use super::video::Video;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Media {
    Photo(Photo),
    Video(Video),
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum DuplicateMedia {
    PhotoMatchGroup(PhotoMatchGroup),
    VideoMatchGroup(MatchGroup),
}
