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
        let versions: Vec<_> = s
            .split('.')
            .map(|part| part.parse::<u32>().unwrap())
            .collect();

        let major = *versions
            .get(0)
            .ok_or("couldn't extract the major version")?;
        let minor = *versions.get(1).unwrap_or(&0);
        let patch = *versions.get(2).unwrap_or(&0);

        Ok(Self {
            major,
            minor,
            patch,
        })
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
}
