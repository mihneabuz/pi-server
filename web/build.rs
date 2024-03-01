use anyhow::{Context, Result};
use std::process::Command;

fn main() -> Result<()> {
    Command::new("tailwindcss")
        .args(["--input", "config/styles.css"])
        .args(["--output", "public/styles.css"])
        .args(["--config", "tailwind.config.js"])
        .arg("--minify")
        .output()
        .context("Please ensure `tailwindcss` is installed")?;

    println!("cargo:rerun-if-changed=tailwind.config.js");
    println!("cargo:rerun-if-changed=config/styles.css");

    println!("cargo:rerun-if-changed=src/pages/");
    println!("cargo:rerun-if-changed=src/components/");

    Ok(())
}
