use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn run(input_path: &Path, output_path: &Path, delete_originals: bool) -> Result<()> {
    //tofo
    todo!()
}

/// Check if ffmpeg is installed and working properly
fn check_ffmpeg() -> Result<()> {
    let output = Command::new("ffmpeg")
        .arg("-version")
        .output()
        .context("Failed to run ffmpeg. Is it installed?")?;

    if !output.status.success() {
        bail!("ffmpeg is working properly");
    }

    Ok(())
}

/// Find all AIFF files in a directory (non-recursive)
fn find_aiff_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    let entries = std::fs::read_dir(dir)
        .with_context(|| format!("Failed to read dir {:?}", dir.display()))?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        // Check if it's a file with .aiff or .aif extension
        if path.is_file() {
            if let Some(ext) = path.extension() {
                let ext_lower = ext.to_string_lossy().to_lowercase();
                if ext_lower == "aiff" || ext_lower == "aif" {
                    files.push(path);
                }
            }
        }
    }

    Ok(files)
}
