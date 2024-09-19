#![no_std]

use core::cmp::Ordering;
use core::fmt;
use core::{
    option::Option::{self, Some},
    write,
    prelude::rust_2021::derive,
    cmp::{Ord, PartialOrd, PartialEq, Eq}
};

#[derive(Debug, PartialEq, Eq)]
pub enum VersionType {
    Release,
    Beta(u16),
    Alpha(u16),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Version {
    major: u16,
    minor: u16,
    patch: u16,
    version_type: VersionType,
}

impl Version {
    /// Creates a new `Version` manually.
    pub fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
            version_type: VersionType::Release,
        }
    }

    /// Creates a `Version` from the package version in `Cargo.toml`.
    /// Please use parameter `env!("CARGO_PKG_VERSION")`
    pub fn from_pkg(cargo_pkg_version: &str) -> Self {
        let version_str = cargo_pkg_version;
        let mut version_parts = version_str.split('.').map(|s| s.parse::<u16>().unwrap());

        Self {
            major: version_parts.next().unwrap(),
            minor: version_parts.next().unwrap(),
            patch: version_parts.next().unwrap(),
            version_type: VersionType::Release,
        }
    }

    /// Sets the version as a release version.
    pub fn release(mut self) -> Self {
        self.version_type = VersionType::Release;
        self
    }

    /// Sets the version as a beta version with an optional beta number.
    pub fn beta(mut self, beta_number: u16) -> Self {
        self.version_type = VersionType::Beta(beta_number);
        self
    }

    /// Sets the version as an alpha version with an optional alpha number.
    pub fn alpha(mut self, alpha_number: u16) -> Self {
        self.version_type = VersionType::Alpha(alpha_number);
        self
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.version_type {
            VersionType::Release => write!(f, "v{}.{}.{}-release", self.major, self.minor, self.patch),
            VersionType::Beta(0) => write!(f, "v{}.{}.{}-beta", self.major, self.minor, self.patch),
            VersionType::Beta(number) => write!(f, "v{}.{}.{}-beta{}", self.major, self.minor, self.patch, number),
            VersionType::Alpha(0) => write!(f, "v{}.{}.{}-alpha", self.major, self.minor, self.patch),
            VersionType::Alpha(number) => write!(f, "v{}.{}.{}-alpha{}", self.major, self.minor, self.patch, number),
        }
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp = self.major.cmp(&other.major)
            .then_with(|| self.minor.cmp(&other.minor))
            .then_with(|| self.patch.cmp(&other.patch));

        if cmp != Ordering::Equal {
            return cmp;
        }

        match (&self.version_type, &other.version_type) {
            (VersionType::Release, VersionType::Release) => Ordering::Equal,
            (VersionType::Release, _) => Ordering::Greater,
            (_, VersionType::Release) => Ordering::Less,
            (VersionType::Beta(a), VersionType::Beta(b)) => a.cmp(b),
            (VersionType::Beta(_), VersionType::Alpha(_)) => Ordering::Greater,
            (VersionType::Alpha(_), VersionType::Beta(_)) => Ordering::Less,
            (VersionType::Alpha(a), VersionType::Alpha(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_display() {
//         assert_eq!(format!("{}", Version::new(1, 0, 0).release()), "v1.0.0-release");
//         assert_eq!(format!("{}", Version::new(1, 0, 0).beta(0)), "v1.0.0-beta");
//         assert_eq!(format!("{}", Version::new(1, 0, 0).beta(1)), "v1.0.0-beta1");
//         assert_eq!(format!("{}", Version::new(1, 0, 0).alpha(0)), "v1.0.0-alpha");
//         assert_eq!(format!("{}", Version::new(1, 0, 0).alpha(1)), "v1.0.0-alpha1");
//     }

//     #[test]
//     fn test_ordering() {
//         let release = Version::new(1, 0, 0).release();
//         let beta = Version::new(1, 0, 0).beta(1);
//         let alpha = Version::new(1, 0, 0).alpha(1);

//         assert!(release > beta);
//         assert!(beta > alpha);
//         assert!(alpha < release);
//     }

//     #[test]
//     fn test_from_pkg() {
//         let release = Version::from_pkg().release();
//         assert_eq!(format!("{}", release), format!("v{}-release", env!("CARGO_PKG_VERSION")));
//     }
// }