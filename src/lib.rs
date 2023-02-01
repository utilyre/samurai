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

    pub fn from(s: &str) -> Result<Self, &'static str> {
        let parts: Vec<_> = s
            .split('.')
            .map(|part| part.parse::<u32>().unwrap())
            .take(3)
            .collect();

        let major = parts
            .get(0)
            .ok_or("couldn't extract the major part")?;
        let minor = parts.get(1).unwrap_or(&0);
        let patch = parts.get(2).unwrap_or(&0);

        Ok(Self::new(*major, *minor, *patch))
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
}
