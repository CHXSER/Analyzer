use image_hasher::{HasherConfig, ImageHash};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct Photo {
    pub path: PathBuf,
    hash: ImageHash,
}

impl Photo {
    pub fn new(file_path: &PathBuf) -> Option<Photo> {
        let hasher = HasherConfig::new().to_hasher();
        match image::open(file_path) {
            Ok(image) => {
                let h = hasher.hash_image(&image);
                Some(Photo {
                    path: file_path.clone(),
                    hash: h,
                })
            }
            Err(_e) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct PhotoMatchGroup {
    pub images: Vec<Photo>,
}

#[allow(dead_code)]
pub fn find_similar_images(fotos: &[Photo], threshold: u32) -> Vec<PhotoMatchGroup> {
    let seen = std::sync::Mutex::new(HashMap::new());

    let groups: Vec<PhotoMatchGroup> = fotos
        .par_iter()
        .filter_map(|foto| {
            let mut local_seen = seen.lock().unwrap();
            if local_seen.contains_key(&foto.path) {
                return None;
            }

            let mut group = vec![foto.clone()];
            local_seen.insert(foto.path.clone(), true);

            for other in fotos {
                if local_seen.contains_key(&other.path) {
                    continue;
                }
                if foto.hash.dist(&other.hash) <= threshold {
                    group.push(other.clone());
                    local_seen.insert(other.path.clone(), true);
                }
            }

            if group.len() > 1 {
                Some(PhotoMatchGroup { images: group })
            } else {
                None
            }
        })
        .collect();

    groups
}
