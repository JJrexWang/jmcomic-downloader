use std::{
    io::Write,
    path::Path,
    sync::{atomic::AtomicU32, Arc},
};

use anyhow::{anyhow, Context};

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tauri::AppHandle;
use tauri_specta::Event;
use zip::{write::SimpleFileOptions, ZipWriter};

use crate::{
    events::ExportCbzEvent,
    export::{get_downloaded_chapters, get_image_paths, ExportArchive},
    extensions::AnyhowErrorToStringChain,
    types::{Comic, ComicInfo},
};

struct CbzErrorEventGuard {
    uuid: String,
    app: AppHandle,
    success: bool,
}

impl Drop for CbzErrorEventGuard {
    fn drop(&mut self) {
        if self.success {
            return;
        }

        let uuid = self.uuid.clone();
        let _ = ExportCbzEvent::Error { uuid }.emit(&self.app);
    }
}

#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::too_many_lines)]
pub fn cbz(app: &AppHandle, comic: &Comic) -> anyhow::Result<()> {
    let downloaded_chapter_infos = get_downloaded_chapters(&comic.chapter_infos);
    // 生成格式化的xml
    let cfg = yaserde::ser::Config {
        perform_indent: true,
        ..Default::default()
    };
    let event_uuid = uuid::Uuid::new_v4().to_string();
    // 发送开始导出cbz事件
    let _ = ExportCbzEvent::Start {
        uuid: event_uuid.clone(),
        comic_title: comic.name.clone(),
        total: downloaded_chapter_infos.len() as u32,
    }
    .emit(app);
    // 如果success为false，drop时发送Error事件
    let mut error_event_guard = CbzErrorEventGuard {
        uuid: event_uuid.clone(),
        app: app.clone(),
        success: false,
    };
    // 用来记录导出进度
    let current = Arc::new(AtomicU32::new(0));

    let extension = ExportArchive::Cbz.extension();
    let comic_export_dir = comic
        .get_comic_export_dir(app)
        .context("获取导出目录失败")?;
    let chapter_export_dir = comic_export_dir.join(extension);
    // 保证导出目录存在
    std::fs::create_dir_all(&chapter_export_dir)
        .context(format!("创建目录`{}`失败", chapter_export_dir.display()))?;
    // 先把封面拷贝到导出目录(如果有)
    if let Err(err) = copy_cover(comic, &chapter_export_dir) {
        let comic_title = &comic.name;
        let err_title = format!("`{comic_title}`导出cbz时，将封面拷贝到导出目录失败");
        let string_chain = err.to_string_chain();
        tracing::error!(err_title, message = string_chain);
    }
    // 并发处理
    let downloaded_chapter_infos = downloaded_chapter_infos.into_par_iter();
    downloaded_chapter_infos.try_for_each(|chapter_info| -> anyhow::Result<()> {
        let chapter_title = chapter_info.chapter_title.clone();
        // 生成ComicInfo
        let comic_info = ComicInfo::from(comic, &chapter_info);
        // 序列化ComicInfo为xml
        let comic_info_xml =
            yaserde::ser::to_string_with_config(&comic_info, &cfg).map_err(|err_msg| {
                anyhow!("章节`{chapter_title}`序列化`ComicInfo.xml`失败: {err_msg}")
            })?;
        // 创建cbz文件
        let chapter_download_dir_name = &chapter_info
            .get_chapter_download_dir_name()
            .context(format!("章节`{chapter_title}`获取章节下载目录名失败"))?;
        let save_path = chapter_export_dir.join(format!("{chapter_download_dir_name}.{extension}"));
        let zip_file = std::fs::File::create(&save_path).context(format!(
            "章节`{chapter_title}`创建文件`{}`失败",
            save_path.display()
        ))?;
        let mut zip_writer = ZipWriter::new(zip_file);
        // 把ComicInfo.xml写入cbz
        zip_writer
            .start_file("ComicInfo.xml", SimpleFileOptions::default())
            .context(format!(
                "章节`{chapter_title}`在`{}`创建`ComicInfo.xml`失败",
                save_path.display()
            ))?;
        zip_writer
            .write_all(comic_info_xml.as_bytes())
            .context(format!("章节`{chapter_title}`写入`ComicInfo.xml`失败"))?;

        let chapter_download_dir = chapter_info.chapter_download_dir.as_ref().context(format!(
            "章节`{chapter_title}`的`chapter_download_dir`字段为`None`"
        ))?;

        let image_paths = get_image_paths(chapter_download_dir, false).context(format!(
            "获取`{}`中的图片失败",
            chapter_download_dir.display()
        ))?;

        for image_path in image_paths {
            let filename = image_path
                .file_name()
                .and_then(|name: &std::ffi::OsStr| name.to_str())
                .context(format!("获取`{}`的文件名失败", image_path.display()))?;
            // 将文件写入cbz
            zip_writer
                .start_file(filename, SimpleFileOptions::default())
                .context(format!(
                    "章节`{chapter_title}`在`{}`创建`{filename}`失败",
                    save_path.display()
                ))?;
            let mut file = std::fs::File::open(&image_path)
                .context(format!("打开`{}`失败", image_path.display()))?;
            std::io::copy(&mut file, &mut zip_writer).context(format!(
                "章节`{chapter_title}`将`{}`写入`{}`失败",
                image_path.display(),
                save_path.display()
            ))?;
        }

        zip_writer.finish().context(format!(
            "章节`{chapter_title}`关闭`{}`失败",
            save_path.display()
        ))?;
        // 更新导出cbz的进度
        let current = current.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
        // 发送导出cbz进度事件
        let _ = ExportCbzEvent::Progress {
            uuid: event_uuid.clone(),
            current,
        }
        .emit(app);
        Ok(())
    })?;
    // 标记为成功，后面drop时就不会发送Error事件
    error_event_guard.success = true;
    // 发送导出cbz完成事件
    let _ = ExportCbzEvent::End {
        uuid: event_uuid,
        chapter_export_dir,
    }
    .emit(app);

    Ok(())
}

fn copy_cover(comic: &Comic, chapter_export_dir: &Path) -> anyhow::Result<()> {
    let src_cover_path = comic.get_cover_path().context("获取封面路径失败")?;
    let cover_filename = src_cover_path.file_name().context("获取封面的文件名失败")?;

    if src_cover_path.exists() {
        let dst_cover_path = chapter_export_dir.join(cover_filename);
        std::fs::copy(src_cover_path, dst_cover_path)?;
    }

    Ok(())
}
