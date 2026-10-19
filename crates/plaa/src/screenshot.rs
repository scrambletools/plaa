//! Saving the picture shown as a PNG.

use std::io::BufWriter;
use std::path::{Path, PathBuf};

/// Writes `picture` (width, height and RGBA) to `path`, or beside it with
/// a number when a file has that name. Runs on a thread of its own.
pub async fn save(picture: (u32, u32, Vec<u8>), path: PathBuf) -> Result<PathBuf, String> {
    let (sender, receiver) = iced::futures::channel::oneshot::channel();
    std::thread::Builder::new()
        .name("plaa-screenshot".into())
        .spawn(move || {
            let _ = sender.send(write(&picture, &path));
        })
        .map_err(|error| error.to_string())?;
    receiver
        .await
        .unwrap_or_else(|_| Err("the picture was not saved".to_owned()))
}

fn write((width, height, rgba): &(u32, u32, Vec<u8>), path: &Path) -> Result<PathBuf, String> {
    if let Some(folder) = path.parent() {
        std::fs::create_dir_all(folder).map_err(|error| error.to_string())?;
    }
    let path = unused(path);
    let file = std::fs::File::create_new(&path).map_err(|error| error.to_string())?;
    let mut encoder = png::Encoder::new(BufWriter::new(file), *width, *height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().map_err(|error| error.to_string())?;
    writer
        .write_image_data(rgba)
        .map_err(|error| error.to_string())?;
    writer.finish().map_err(|error| error.to_string())?;
    Ok(path)
}

/// `path`, or "name 2.png", "name 3.png" and so on when it is taken,
/// with the path's own ending.
pub(crate) fn unused(path: &Path) -> PathBuf {
    if !path.exists() {
        return path.to_path_buf();
    }
    let stem = path
        .file_stem()
        .map(|stem| stem.to_string_lossy().into_owned())
        .unwrap_or_default();
    let ending = path
        .extension()
        .map(|extension| format!(".{}", extension.to_string_lossy()))
        .unwrap_or_default();
    (2..)
        .map(|number| path.with_file_name(format!("{stem} {number}{ending}")))
        .find(|candidate| !candidate.exists())
        .unwrap_or_else(|| path.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saves_without_overwriting() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("shot.png");
        let picture = (2, 1, vec![255, 0, 0, 255, 0, 0, 255, 255]);
        assert_eq!(write(&picture, &path).unwrap(), path);
        assert_eq!(
            write(&picture, &path).unwrap(),
            dir.path().join("shot 2.png")
        );
    }
}
