use semver::{Prerelease, Version};

//	a.b.c => a.b.(c+1)-alpha.0
//	a.b.c-alpha.x => a.b.c-beta.0
//	a.b.c-beta.x => a.b.c-rc.0
//	a.b.c-rc.x => a.b.c
pub fn next_phase(mut v: Version) -> Version {
    let pre = v.pre.clone();
    if pre == Prerelease::EMPTY {
        v.pre = Prerelease::new("alpha.0").unwrap();
        v.patch += 1;
        return v;
    }
    match pre.split('.').next().unwrap() {
        "alpha" => {
            v.pre = Prerelease::new("beta.0").unwrap();
        }
        "beta" => {
            v.pre = Prerelease::new("rc.0").unwrap();
        }
        "rc" => {
            v.pre = Prerelease::EMPTY;
        }
        _ => {
            // any other prerelease => release
            v.pre = Prerelease::EMPTY;
        }
    }

    v
}

// a.b.c-alpha.x => a.b.c-alpha.(x+1)
// a.b.c-beta.x => a.b.c-beta.(x+1)
// a.b.c-rc.x => a.b.c-rc.(x+1)
// a.b.c => a.b.(c+1)-alpha.0
pub fn next_pre(mut v: Version) -> Version {
    let pre = v.pre.clone();
    if pre == Prerelease::EMPTY {
        return next_phase(v);
    }
    let mut parts = pre.split('.').collect::<Vec<&str>>();
    let mut digit: u64 = 0;
    if parts.len() == 2 {
        digit = parts[1].parse::<u64>().unwrap_or(0);
    }
    v.pre = Prerelease::new(&format!("{}.{}", parts[0], digit + 1)).unwrap();
    v
}

pub fn next_patch(mut v: Version) -> Version {
    v.patch += 1;
    v.pre = Prerelease::EMPTY;
    v
}

pub fn next_minor(mut v: Version) -> Version {
    v.minor += 1;
    v.patch = 0;
    v.pre = Prerelease::EMPTY;
    v
}

pub fn next_major(mut v: Version) -> Version {
    v.major += 1;
    v.minor = 0;
    v.patch = 0;
    v.pre = Prerelease::EMPTY;
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_phase() {
        let v = Version::parse("0.0.0").unwrap();
        let v = next_phase(v);
        assert_eq!(v.to_string(), "0.0.1-alpha.0");
        let v = next_phase(v);
        assert_eq!(v.to_string(), "0.0.1-beta.0");
        let v = next_phase(v);
        assert_eq!(v.to_string(), "0.0.1-rc.0");
        let v = next_phase(v);
        assert_eq!(v.to_string(), "0.0.1");
    }

    #[test]
    fn test_next_pre() {
        let v = Version::parse("0.0.1-alpha.0").unwrap();
        let v = next_pre(v);
        assert_eq!(v.to_string(), "0.0.1-alpha.1");
        let v = next_pre(v);
        assert_eq!(v.to_string(), "0.0.1-alpha.2");
        let v = next_pre(v);
        assert_eq!(v.to_string(), "0.0.1-alpha.3");
        let v = next_phase(v);
        assert_eq!(v.to_string(), "0.0.1-beta.0");
        let v = Version::parse("0.0.1").unwrap();
        let v = next_pre(v);
        assert_eq!(v.to_string(), "0.0.2-alpha.0");
    }

    #[test]
    fn test_next_patch() {
        let v = Version::parse("0.0.0").unwrap();
        let v = next_patch(v);
        assert_eq!(v.to_string(), "0.0.1");
        let v = Version::parse("0.0.1-alpha.0").unwrap();
        let v = next_patch(v);
        assert_eq!(v.to_string(), "0.0.2");
    }

    #[test]
    fn test_next_minor() {
        let v = Version::parse("0.0.0").unwrap();
        let v = next_minor(v);
        assert_eq!(v.to_string(), "0.1.0");
        let v = Version::parse("0.1.0-beta.1").unwrap();
        let v = next_minor(v);
        assert_eq!(v.to_string(), "0.2.0");
    }

    #[test]
    fn test_next_major() {
        let v = Version::parse("0.0.0").unwrap();
        let v = next_major(v);
        assert_eq!(v.to_string(), "1.0.0");
        let v = Version::parse("1.0.0-rc.1").unwrap();
        let v = next_major(v);
        assert_eq!(v.to_string(), "2.0.0");
    }
}
