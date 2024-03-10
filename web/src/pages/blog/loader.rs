use std::path::Path;

use anyhow::{Context, Result};
use chrono::NaiveDate;
use tracing::{info, warn};

use super::entry::Blog;

pub struct BlogLoader;

impl BlogLoader {
    pub async fn read(path: impl AsRef<Path>) -> Option<Blog> {
        let file_name = path.as_ref().file_name()?.to_string_lossy();
        let (date, title) = file_name.trim_end_matches(".md").split_once(':')?;

        let content = tokio::fs::read_to_string(&path).await.ok()?;
        let ast = markdown::to_mdast(&content, &markdown::ParseOptions::gfm()).unwrap();

        Some(Blog {
            title: title.to_owned(),
            date: NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()?,
            ast,
        })
    }

    pub async fn read_dir(blogs_dir: impl AsRef<Path>) -> Result<Vec<Blog>> {
        let mut blogs = Vec::new();

        let mut entries = tokio::fs::read_dir(&blogs_dir).await?;
        while let Some(file) = entries.next_entry().await? {
            if !file.file_type().await?.is_file() {
                warn!(?file, "Unrecognized file type");
                continue;
            }

            let blog = Self::read(blogs_dir.as_ref().join(file.file_name()))
                .await
                .context("Invalid blog file")?;

            info!(title = blog.title, date = %blog.date, "Found blog");

            blogs.push(blog);
        }

        info!(count = blogs.len(), "Done loading blogs");

        Ok(blogs)
    }
}
