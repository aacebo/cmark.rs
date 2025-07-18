use std::{
    fs, io,
    path::{Path, PathBuf},
    str::FromStr,
};

use cmark::{ParseOptions, Render, RenderOptions};

fn test_dir(path: &Path) -> Result<(), io::Error> {
    println!("{}", path.display());

    for dir in fs::read_dir(path)? {
        let entry = dir?;
        let p = entry.path();

        if entry.file_type()?.is_dir() {
            test_dir(p.as_path())?;
            continue;
        }

        let is_md = match p.extension() {
            None => continue,
            Some(v) => v == "md",
        };

        if !is_md {
            continue;
        }

        test_file(&p)?;
    }

    return Ok(());
}

fn test_file(path: &Path) -> Result<(), io::Error> {
    println!("{}", path.display());

    let mut html_path = trim_suffix(path, ".md");
    html_path.set_extension("html");

    let md = fs::read(path)?;
    let html = fs::read_to_string(html_path)?;
    let node = match cmark::parse(md.clone(), &ParseOptions::default()) {
        Ok(v) => v,
        Err(err) => panic!("{}", err),
    };

    let rendered = match node.render(&RenderOptions::default()) {
        Ok(v) => v,
        Err(err) => panic!("{}", err),
    };

    assert_eq!(html, rendered);
    println!("{}", rendered);
    return Ok(());
}

fn trim_suffix(path: &Path, suffix: &str) -> PathBuf {
    let p = path.to_str().unwrap_or_default();
    let trimmed = p.strip_suffix(suffix).unwrap_or_default();
    return PathBuf::from_str(trimmed).unwrap_or_default();
}

#[test]
pub fn main() -> Result<(), io::Error> {
    env_logger::init();
    let cwd = std::env::current_dir()?;
    return test_dir(&cwd.join(&Path::new("tests/markdown_test/testcases").iter().as_path()));
}
