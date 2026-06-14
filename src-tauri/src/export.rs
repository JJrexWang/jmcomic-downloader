mod cbz;
mod pdf;

use std::path::{Path, PathBuf};

pub use cbz::cbz;
use eyre::Context;
pub use pdf::pdf;

use crate::{extensions::PathIsImg, types::ChapterInfo};

pub enum ExportArchive {
    Cbz,
    Pdf,
}

impl ExportArchive {
    pub fn extension(&self) -> &str {
        match self {
            ExportArchive::Cbz => "cbz",
            ExportArchive::Pdf => "pdf",
        }
    }
}

/// 获取已下载的章节
fn get_downloaded_chapters(chapter_infos: &[ChapterInfo]) -> Vec<ChapterInfo> {
    chapter_infos
        .iter()
        .filter(|chapter_info| chapter_info.is_downloaded.unwrap_or(false))
        .cloned()
        .collect()
}

fn get_image_paths(images_dir: &Path, must_be_common_img: bool) -> eyre::Result<Vec<PathBuf>> {
    let mut image_paths: Vec<PathBuf> = std::fs::read_dir(images_dir)
        .wrap_err(format!("读取目录`{}`失败", images_dir.display()))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            if must_be_common_img {
                path.is_common_img()
            } else {
                path.is_img()
            }
        })
        .collect();
    image_paths.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    Ok(image_paths)
}
