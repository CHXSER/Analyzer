use std::path::{Path, PathBuf};
use vid_dup_finder_lib::{
    ffmpeg_builder::VideoHashBuilder, CreationOptions, Cropdetect, VideoHash,
};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct Video {
    path: PathBuf,
    pub hash: VideoHash,
}

impl Video {
    pub fn new(file_path: &Path) -> Option<Video> {
        let options = CreationOptions {
            skip_forward_amount: 0.0,
            duration: 4.0,
            cropdetect: Cropdetect::Letterbox,
        };

        let builder = VideoHashBuilder::from_options(options);
        let h = builder.hash(file_path.to_path_buf());

        match h {
            Ok(ha) => Some(Video {
                path: file_path.to_path_buf(),
                hash: ha,
            }),
            Err(_e) => None,
        }
    }
}
