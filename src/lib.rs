use std::cmp::Ordering;

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

    pub fn from(s: &str) -> Result<Self, String> {
        let parts: Vec<_> = s
            .split('.')
            .take(3)
            .map(|part| {
                part.parse()
                    .or_else(|_| Err(format!("cannot parse `{}` as u32", part)))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let major = parts.get(0).ok_or("cannot extract the major part")?;
        let minor = parts.get(1).unwrap_or(&0);
        let patch = parts.get(2).unwrap_or(&0);

        Ok(Self::new(*major, *minor, *patch))
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
    fn can_instantiate_from_string() {
        let v = SemVer::from("1.8.9").unwrap();

        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 8);
        assert_eq!(v.patch, 9);
    }

    #[test]
    fn can_instantiate_with_fewer_parts() {
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
    fn error_on_empty_string_instantiation() {
        SemVer::from("").unwrap();
    }

    #[test]
    #[should_panic]
    fn error_on_giving_characters_as_version_parts() {
        SemVer::from("hi.there").unwrap();
    }
}
