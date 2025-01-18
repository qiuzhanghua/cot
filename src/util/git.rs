use anyhow::{Context, Result};
use semver::Version;
use std::process::Command;
use std::str;
pub fn git_installed() -> bool {
    let output = Command::new("git").arg("--version").output();
    output.is_ok()
}
pub fn git_head_hash() -> Option<String> {
    let output = Command::new("git").arg("rev-parse").arg("HEAD").output();

    match output {
        Ok(output) => {
            if !output.status.success() {
                return None;
            }
            let s = str::from_utf8(&output.stdout).ok()?;
            Some(s.trim().to_string())
        }
        Err(_) => None,
    }
}
pub fn git_dir_is_repo(path: &str) -> bool {
    let output = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("rev-parse")
        .output();
    output.is_ok()
}

pub fn git_add_tag(tag: &str) -> Result<String> {
    let output = Command::new("git").arg("tag").arg(tag).output()?;
    let s = String::from_utf8(output.stdout)?;
    Ok(s.trim().to_string())
}

pub fn git_rev_of_tag(tag: &str) -> Result<String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--short")
        .arg(tag)
        .output()?;
    let s = String::from_utf8(output.stdout)?;
    Ok(s.trim().to_string())
}

pub fn git_hash_of_tag(tag: &str) -> Result<String> {
    let output = Command::new("git").arg("rev-parse").arg(tag).output()?;
    let s = String::from_utf8(output.stdout)?;
    Ok(s.trim().to_string())
}

pub fn git_date_of_hash(hash: &str) -> Result<String> {
    let output = Command::new("git")
        .arg("show")
        .arg("-s")
        .arg("--format=%cd")
        .arg("--date=format:%Y-%m-%d")
        .arg(hash)
        .output()
        .with_context(|| format!("Failed to run git command for hash: {}", hash))?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "Failed to get date of hash {}: exit code {}",
            hash,
            output.status.code().unwrap_or(-1)
        ));
    }

    let s = String::from_utf8(output.stdout)
        .with_context(|| format!("Invalid UTF-8 output from git command for hash: {}", hash))?;
    Ok(s.trim().to_string())
}

pub fn git_all_tags() -> Result<Vec<String>> {
    let output = Command::new("git").arg("tag").output()?;
    let s = String::from_utf8(output.stdout)?;
    Ok(s.trim().lines().map(|line| line.to_string()).collect())
}

pub fn git_latest_tag() -> Result<String> {
    match git_all_tags()? {
        arr if arr.is_empty() || (arr.len() == 1 && arr[0].is_empty()) => Ok("0.0.0".to_string()),
        arr => {
            let mut vs: Vec<Version> = vec![];
            for tag in &arr {
                match Version::parse(tag) {
                    Ok(v) => vs.push(v),
                    Err(_) => continue,
                }
            }
            // vs.sort_by(|a, b| a.cmp(b));
            vs.sort();

            if arr[0].starts_with("v") {
                Ok(format!("v{}", vs.last().unwrap()))
            } else {
                Ok(vs.last().unwrap().to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_installed() {
        assert!(git_installed());
    }

    #[test]
    fn test_git_head_hash() {
        assert!(git_head_hash().is_some());
    }

    #[test]
    fn test_git_dir_is_repo() {
        assert!(git_dir_is_repo("."));
    }

    #[test]
    fn test_git_get_all_tags() {
        let tags = git_all_tags().unwrap();
        assert_ne!(tags, Vec::<String>::new());
        assert!(tags.len() > 0);
    }

    #[test]
    fn test_git_latest_tag() {
        let tag = git_latest_tag().unwrap();
        assert_ne!(tag, "0.0.0");
    }

    #[test]
    fn test_git_date_of_latest() {
        let tag = git_latest_tag().unwrap();
        let date = git_date_of_hash(&tag).unwrap();
        println!("Date of latest tag: {}", date);
        assert_ne!(date, "2025-01-18");
    }
}
