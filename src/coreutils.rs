use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use color_eyre::eyre::Context;
use color_eyre::Result;

/// Returns the contents of a file, returning an Error if the file does not exist.
pub fn cat<P: AsRef<Path>>(path: P) -> Result<String> {
    Ok(std::fs::read_to_string(path)?)
}

/// Redirects `new_data` to the file `path`. Exhibits the same behavior as the `>` Unix shell operator.
pub fn redirect<P: AsRef<Path>, S: Display>(path: P, new_data: S) -> Result<()> {
    let mut file = File::create(path)?;
    writeln!(file, "{}", new_data)?;
    Ok(())
}

/// Appends `data` to the file `path`. Exhibits the same behavior as the `>>` Unix shell operator.
pub fn append<P: AsRef<Path>, S: Display>(path: P, data: S) -> Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{}", data)?;
    Ok(())
}

/// Returns the current working directory.
pub fn pwd() -> Result<PathBuf> {
    Ok(std::env::current_dir()?)
}

/// Changes the directory to the path specified. Works with relative and absolute directories.
pub fn cd<P: AsRef<Path>>(path: P) -> Result<()> {
    Ok(std::env::set_current_dir(path)?)
}

/// Remove a file or an empty directory.
///
/// If the given path is a directory and it is not empty, an error is returned.
pub fn rm<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    if path.is_file() {
        std::fs::remove_file(path).context("failed to remove file")?;
    } else if path.is_dir() {
        std::fs::remove_dir(path).context("failed to remove directory")?;
    } else {
        return Err(color_eyre::eyre::eyre!(
            "path does not exist or is not a file or directory"
        ));
    }
    Ok(())
}

/// Recursively remove a directory and all its contents.
pub fn rm_rf<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    if path.is_dir() {
        std::fs::remove_dir_all(path).context("failed to remove directory recursively")?;
    } else if path.is_file() {
        std::fs::remove_file(path).context("failed to remove file")?;
    } else {
        return Err(color_eyre::eyre::eyre!(
            "path does not exist or is not a file or directory"
        ));
    }
    Ok(())
}

/// Create a new directory at the given path.
///
/// If the directory already exists, an error is returned.
pub fn mkdir<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    std::fs::create_dir(path)
        .with_context(|| format!("failed to create directory '{}'", path.display()))?;
    Ok(())
}

/// Copy a file from the source path to the destination path.
///
/// If the destination file already exists, it will be overwritten.
pub fn cp<P, Q>(src: P, dst: Q) -> Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let src = src.as_ref();
    let dst = dst.as_ref();
    std::fs::copy(src, dst).with_context(|| {
        format!(
            "failed to copy file from '{}' to '{}'",
            src.display(),
            dst.display()
        )
    })?;
    Ok(())
}

/// Move a file from the source path to the destination path.
///
/// If the destination file already exists, it will be overwritten.
pub fn mv<P, Q>(src: P, dst: Q) -> Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let src = src.as_ref();
    let dst = dst.as_ref();
    std::fs::rename(src, dst).with_context(|| {
        format!(
            "failed to move file from '{}' to '{}'",
            src.display(),
            dst.display()
        )
    })?;
    Ok(())
}

/// Copy a file or a directory from the source path to the destination path.
///
/// If the source is a directory, its contents will be copied recursively to the destination.
/// If the destination already exists and is a directory, the source will be copied inside it.
pub fn cp_r<P, Q>(src: P, dst: Q) -> Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let src = src.as_ref();
    let dst = dst.as_ref();

    if src.is_file() {
        // If the source is a file, use the `fs::copy` function to copy it to the destination.
        std::fs::copy(src, dst).with_context(|| {
            format!(
                "failed to copy file from '{}' to '{}'",
                src.display(),
                dst.display()
            )
        })?;
    } else if src.is_dir() {
        // If the source is a directory, use `fs::create_dir` to create the destination directory if it does not exist.
        if !dst.exists() {
            std::fs::create_dir(dst)
                .with_context(|| format!("failed to create directory '{}'", dst.display()))?;
        }

        // Iterate over the entries in the source directory and copy them recursively to the destination directory.
        for entry in std::fs::read_dir(src)
            .with_context(|| format!("failed to read directory '{}'", src.display()))?
        {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            cp_r(src_path, dst_path)?;
        }
    } else {
        // If the source is neither a file nor a directory, return an error.
        return Err(color_eyre::eyre::eyre!(
            "source path '{}' is neither a file nor a directory",
            src.display()
        ));
    }

    Ok(())
}
