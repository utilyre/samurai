use std::cmp::Ordering;

type Result<T> = std::result::Result<T, String>;

#[derive(Eq)]
pub struct SemVer {
    major: u32,
    minor: u32,
    patch: u32,
}

impl SemVer {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub fn from(s: &str) -> Result<Self> {
        let parts: Vec<_> = s
            .split('.')
            .take(3)
            .map(|part| {
                part.parse()
                    .or_else(|_| Err(format!("cannot parse `{}` as u32", part)))
            })
            .collect::<Result<Vec<_>>>()?;

        let major = parts.get(0).ok_or("cannot extract the major part")?;
        let minor = parts.get(1).unwrap_or(&0);
        let patch = parts.get(2).unwrap_or(&0);

        Ok(Self::new(*major, *minor, *patch))
    }

    pub fn check(&self, pattern: &str) -> Result<bool> {
        let Some(version_start) = pattern.find(|ch: char| ch.is_numeric()) else {
            return Err(format!("cannot extract the major part"));
        };

        let operator = &pattern[..version_start];
        let other = Self::from(&pattern[version_start..])?;

        match operator {
            "=" => Ok(self == &other),
            "<" => Ok(self < &other),
            ">" => Ok(self > &other),
            "<=" => Ok(self <= &other),
            ">=" => Ok(self >= &other),
            _ => Err(format!("operator `{}` not found", operator)),
        }
    }
}

impl PartialEq for SemVer {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }
}

impl PartialOrd for SemVer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SemVer {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.major > other.major {
            return Ordering::Greater;
        }

        if self.major == other.major {
            if self.minor > other.minor {
                return Ordering::Greater;
            }

            if self.minor == other.minor {
                if self.patch > other.patch {
                    return Ordering::Greater;
                }

                if self.patch == other.patch {
                    return Ordering::Equal;
                }
            }
        }

        Ordering::Less
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_string() {
        let v = SemVer::from("1.8.9").unwrap();

        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 8);
        assert_eq!(v.patch, 9);
    }

    #[test]
    fn from_less_parts() {
        let v1 = SemVer::from("10").unwrap();

        assert_eq!(v1.major, 10);
        assert_eq!(v1.minor, 0);
        assert_eq!(v1.patch, 0);

        let v2 = SemVer::from("6.9").unwrap();

        assert_eq!(v2.major, 6);
        assert_eq!(v2.minor, 9);
        assert_eq!(v2.patch, 0);
    }

    #[test]
    #[should_panic]
    fn from_empty_string_panics() {
        SemVer::from("").unwrap();
    }

    #[test]
    #[should_panic]
    fn from_non_version_panics() {
        SemVer::from("hi.there").unwrap();
    }

    #[test]
    fn ord_two_instances() {
        let v1 = SemVer::new(7, 8, 9);
        let v2 = SemVer::new(8, 5, 8);
        assert!(v1 < v2);

        let v1 = SemVer::new(5, 2, 8);
        let v2 = SemVer::new(5, 1, 9);
        assert!(v1 > v2);
        assert!(v1 >= v2);

        let v1 = SemVer::new(1, 2, 7);
        let v2 = SemVer::new(1, 2, 5);
        assert!(v1 > v2);

        let v1 = SemVer::new(6, 9, 9);
        let v2 = SemVer::new(6, 9, 9);
        assert!(v1 == v2);

        let v1 = SemVer::new(8, 1, 20);
        let v2 = SemVer::new(8, 81, 20);
        assert!(v1 != v2);
    }

    #[test]
    fn check_against_pattern() {
        let v = SemVer::from("7.8.9").unwrap();
        assert!(v.check("<8.5.8").unwrap());

        let v = SemVer::from("5.2.8").unwrap();
        assert!(v.check(">5.1.9").unwrap());
        assert!(v.check(">=5.1.9").unwrap());

        let v = SemVer::from("1.2.7").unwrap();
        assert!(v.check(">1.2.5").unwrap());

        let v = SemVer::from("6.9.9").unwrap();
        assert!(v.check("=6.9.9").unwrap());
    }

    #[test]
    #[should_panic]
    fn check_against_invalid_pattern_panics() {
        let v = SemVer::new(1, 0, 69);
        v.check("seeya5.8.10").unwrap();
    }
}
