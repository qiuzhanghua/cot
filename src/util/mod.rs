mod git;
mod hf;
mod repl;
mod unzip;
mod ver;
mod xf;

pub use self::git::*;
pub use self::hf::*;
pub use self::repl::*;
pub use self::unzip::*;
pub use self::ver::*;
pub use self::xf::*;

use dirs::home_dir;

fn expand_tilde(path: &str) -> Option<String> {
    let home = home_dir();
    if path == "~" || path == "~/" {
        home.map(|home| format!("{}", home.to_string_lossy()))
    } else if path.starts_with("~/") {
        home.map(|home| format!("{}/{}", home.to_string_lossy(), &path[2..]))
    } else {
        Some(path.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_expand_tilde() {
        let home = home_dir().unwrap().to_string_lossy().to_string();
        assert_eq!(expand_tilde("~/"), Some(home.clone()));
        assert_eq!(expand_tilde("~"), Some(home.clone()));
        assert_eq!(expand_tilde("~/foo"), Some(format!("{}/foo", home)));
        assert_eq!(expand_tilde("~/.config"), Some(format!("{}/.config", home)));
        assert_eq!(expand_tilde("/foo/bar"), Some("/foo/bar".to_string()));
    }
}
