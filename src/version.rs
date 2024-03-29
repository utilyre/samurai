use std::{cmp::Ordering, result, str::FromStr};

type Result<T> = result::Result<T, String>;

#[derive(Eq)]
pub struct Version {
    /// Represents incompatible API changes.
    pub major: u32,

    /// Represents functionality additions in a backwards compatible manner.
    pub minor: u32,

    /// Represents bug fixes in a backwards compatible manner.
    pub patch: u32,
}

impl Version {
    /// Creates a new [`Version`].
    ///
    /// # Examples
    ///
    /// ```
    /// use samurai::Version;
    ///
    /// let version = Version::new(1, 5, 7);
    ///
    /// assert_eq!(version.major, 1);
    /// assert_eq!(version.minor, 5);
    /// assert_eq!(version.patch, 7);
    /// ```
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Checks whether there haven't been any breaking changes since `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use samurai::Version;
    ///
    /// let version = "1.5.7".parse::<Version>().expect("`1.5.7` should be a valid version");
    /// let other1 = "1.2.9".parse::<Version>().expect("`1.2.9` should be a valid version");
    /// let other2 = "0.8.1".parse::<Version>().expect("`0.8.1` should be a valid version");
    ///
    /// assert!(version.is_compatible(&other1));
    /// assert!(!version.is_compatible(&other2));
    /// ```
    pub fn is_compatible(&self, other: &Self) -> bool {
        if self.major == 0 {
            return self.is_featureless(other);
        }

        self >= other && self < &Self::new(other.major + 1, 0, 0)
    }

    /// Checks whether there haven't been any feature implementations since `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use samurai::Version;
    ///
    /// let version = "1.5.7".parse::<Version>().expect("`1.5.7` should be a valid version");
    /// let other1 = "1.5.4".parse::<Version>().expect("`1.5.4` should be a valid version");
    /// let other2 = "1.6.2".parse::<Version>().expect("`1.6.2` should be a valid version");
    ///
    /// assert!(version.is_featureless(&other1));
    /// assert!(!version.is_featureless(&other2));
    /// ```
    pub fn is_featureless(&self, other: &Self) -> bool {
        self >= other && self < &Self::new(other.major, other.minor + 1, 0)
    }

    /// Checks instance of [`Version`] against `pattern`.
    ///
    /// You can find a cheat-sheet of patterns [here](https://devhints.io/semver).
    ///
    /// # Errors
    ///
    /// This function will return an error if it cannot detect a valid pattern.
    ///
    /// # Examples
    ///
    /// ```
    /// use samurai::Version;
    ///
    /// let version = "1.5.7".parse::<Version>().expect("`1.5.7` should be a valid version");
    ///
    /// assert!(version.check("^1.2.9").expect("`^1.2.9` should be a valid pattern"));
    /// assert!(version.check("~1.5.4").expect("`~1.5.4` should be a valid pattern"));
    /// ```
    pub fn check(&self, pattern: &str) -> Result<bool> {
        let Some(version_start) = pattern.find(|ch: char| ch.is_numeric()) else {
            return Err("cannot extract the major part".to_string());
        };

        let operator = &pattern[..version_start];
        let other = &pattern[version_start..].parse::<Self>()?;

        match operator {
            "=" => Ok(self == other),
            "<" => Ok(self < other),
            ">" => Ok(self > other),
            "<=" => Ok(self <= other),
            ">=" => Ok(self >= other),
            "^" => Ok(self.is_compatible(other)),
            "~" => Ok(self.is_featureless(other)),
            _ => Err(format!("operator `{}` not found", operator)),
        }
    }
}

impl FromStr for Version {
    type Err = String;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let parts: Vec<_> = s
            .split('.')
            .map(|part| {
                part.parse()
                    .map_err(|_| format!("cannot parse `{}` as u32", part))
            })
            .collect::<Result<Vec<_>>>()?;

        if parts.len() > 3 {
            return Err("too many parts".to_string());
        }

        let major = parts
            .first()
            .expect("should be available due to previous parsing");
        let minor = parts.get(1).unwrap_or(&0);
        let patch = parts.get(2).unwrap_or(&0);

        Ok(Self::new(*major, *minor, *patch))
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
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
    fn from_string() -> Result<()> {
        let v = "1.8.9".parse::<Version>()?;

        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 8);
        assert_eq!(v.patch, 9);

        Ok(())
    }

    #[test]
    fn from_less_parts() -> Result<()> {
        let v1 = "10".parse::<Version>()?;

        assert_eq!(v1.major, 10);
        assert_eq!(v1.minor, 0);
        assert_eq!(v1.patch, 0);

        let v2 = "6.9".parse::<Version>()?;

        assert_eq!(v2.major, 6);
        assert_eq!(v2.minor, 9);
        assert_eq!(v2.patch, 0);

        Ok(())
    }

    #[test]
    #[should_panic(expected = "too many parts")]
    fn from_too_many_parts_panics() {
        "1.5.7.9".parse::<Version>().unwrap();
    }

    #[test]
    #[should_panic(expected = "cannot parse")]
    fn from_empty_string_panics() {
        "".parse::<Version>().unwrap();
    }

    #[test]
    #[should_panic(expected = "as u32")]
    fn from_non_version_panics() {
        "hi.there".parse::<Version>().unwrap();
    }

    #[test]
    fn ord_two_instances() {
        let v1 = Version::new(7, 8, 9);
        let v2 = Version::new(8, 5, 8);
        assert!(v1 < v2);

        let v1 = Version::new(5, 2, 8);
        let v2 = Version::new(5, 1, 9);
        assert!(v1 > v2);
        assert!(v1 >= v2);

        let v1 = Version::new(1, 2, 7);
        let v2 = Version::new(1, 2, 5);
        assert!(v1 > v2);

        let v1 = Version::new(6, 9, 9);
        let v2 = Version::new(6, 9, 9);
        assert!(v1 == v2);

        let v1 = Version::new(8, 1, 20);
        let v2 = Version::new(8, 81, 20);
        assert!(v1 != v2);
    }

    #[test]
    fn check_against_pattern() -> Result<()> {
        let v = "7.8.9".parse::<Version>()?;
        assert!(v.check("<8.5.8")?);

        let v = "5.2.8".parse::<Version>()?;
        assert!(v.check(">5.1.9")?);
        assert!(v.check(">=5.1.9")?);

        let v = "1.2.7".parse::<Version>()?;
        assert!(v.check(">1.2.5")?);

        let v = "6.9.9".parse::<Version>()?;
        assert!(v.check("=6.9.9")?);

        let v = "8.10.5".parse::<Version>()?;
        assert!(v.check("^8.9.1")?);

        let v = "30.11.21".parse::<Version>()?;
        assert!(v.check("~30.11.20")?);

        Ok(())
    }

    #[test]
    #[should_panic(expected = "not found")]
    fn check_against_invalid_pattern_panics() {
        let v = Version::new(1, 0, 69);
        v.check("seeya5.8.10").unwrap();
    }

    #[test]
    fn is_not_compatible_with_major_bump() {
        let v1 = Version::new(8, 10, 5);
        let v2 = Version::new(9, 5, 1);

        assert!(!v1.is_compatible(&v2));
    }

    #[test]
    fn is_compatible_with_minor_bump() {
        let v1 = Version::new(8, 10, 5);
        let v2 = Version::new(8, 9, 1);

        assert!(v1.is_compatible(&v2));
    }

    #[test]
    fn is_compatible_with_patch_bump() {
        let v1 = Version::new(8, 10, 5);
        let v2 = Version::new(8, 10, 4);

        assert!(v1.is_compatible(&v2));
    }

    #[test]
    fn is_compatible_with_patch_bump_on_beta() {
        let v1 = Version::new(0, 10, 6);
        let v2 = Version::new(0, 10, 5);

        assert!(v1.is_compatible(&v2));
    }

    #[test]
    fn is_not_compatible_with_minor_bump_on_beta() {
        let v1 = Version::new(0, 10, 5);
        let v2 = Version::new(0, 9, 20);

        assert!(!v1.is_compatible(&v2));
    }

    #[test]
    fn is_not_featureless_with_major_bump() {
        let v1 = Version::new(31, 9, 5);
        let v2 = Version::new(30, 10, 20);

        assert!(!v1.is_featureless(&v2));
    }

    #[test]
    fn is_not_featureless_with_minor_bump() {
        let v1 = Version::new(30, 11, 5);
        let v2 = Version::new(30, 9, 20);

        assert!(!v1.is_featureless(&v2));
    }

    #[test]
    fn is_featureless_with_patch_bump() {
        let v1 = Version::new(30, 11, 21);
        let v2 = Version::new(30, 11, 20);

        assert!(v1.is_featureless(&v2));
    }
}
