#![no_std]


#[cfg(test)]
mod tests;


/// A struct that represents a version consisting of major, minor, patch, and an optional build number.
/// 
/// The generic parameter `T` specifies the numerical type to use for each version component.
/// 
/// ***
/// # Examples
///
/// Creating a version without a build number:
///
/// ```rust
/// use simple_version::Version;
///
/// let version: Version<u32> = Version::new(1, 2, 3);
/// ```
/// 
/// ***
///
/// Creating a version with a build number:
///
/// ```rust
/// use simple_version::Version;
///
/// let version: Version<u32> = Version::new(1, 2, 3).build(4);
/// ```
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version<T: Ord> {
    /// The major version component.
    /// 
    /// ***
    /// # Examples
    /// 
    /// ```rust
    /// use simple_version::Version;
    ///
    /// let mut version = Version::<u32>::new(1, 2, 3);
    /// 
    /// // read major version
    /// println!("{}", version.major);
    /// 
    /// // set major version
    /// version.major = 4;
    /// ```
    pub major: T,

    /// The minor version component.
    /// 
    /// ***
    /// # Examples
    /// 
    /// ```rust
    /// use simple_version::Version;
    /// 
    /// let mut version = Version::<u32>::new(1, 2, 3);
    /// 
    /// // read minor version
    /// println!("{}", version.minor);
    /// 
    /// // set minor version
    /// version.minor = 4;
    /// ```
    pub minor: T,

    /// The patch version component.
    /// 
    /// ***
    /// # Examples
    /// 
    /// ```rust
    /// use simple_version::Version;
    /// 
    /// let mut version = Version::<u32>::new(1, 2, 3);
    /// 
    /// // read patch version
    /// println!("{}", version.patch);
    /// 
    /// // set patch version
    /// version.patch = 4;
    /// ```
    pub patch: T,

    /// The optional build number.
    /// 
    /// ***
    /// # Examples
    /// 
    /// ```rust
    /// use simple_version::Version;
    /// 
    /// // version with a build number
    /// let mut version = Version::<u32>::new(1, 2, 3).build(4);
    /// 
    /// // read build number
    /// if let Some(build) = version.build {
    ///     println!("{}", build);
    /// }
    /// 
    /// // set build number
    /// version.build = Some(5);
    /// ```
    pub build: Option<T>,
}


impl<T: Ord> Version<T> {
    /// Creates a new `Version<T>` without a build number.
    /// 
    /// ***
    /// # Arguments
    /// 
    /// - `major`: the major version component
    /// - `minor`: the minor version component
    /// - `patch`: the patch version component
    /// 
    /// ***
    /// # Examples
    /// 
    /// ```rust
    /// use simple_version::Version;
    /// 
    /// let version: Version<u32> = Version::new(1, 2, 3);
    /// ```
    pub fn new(major: T, minor: T, patch: T) -> Version<T> {
        Version { major, minor, patch, build: None }
    }

    /// Adds a build number to the existing version object and returns it.
    /// 
    /// ***
    /// # Arguments
    /// 
    /// - `build`: the build number
    /// 
    /// ***
    /// # Examples
    /// 
    /// ```rust
    /// use simple_version::Version;
    /// 
    /// let version: Version<u32> = Version::new(1, 2, 3).build(4);
    /// ```
    pub fn build(mut self, build: T) -> Version<T> {
        self.build = Some(build);
        self
    }
}


impl<T: Ord + core::fmt::Display> core::fmt::Display for Version<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(build) = &self.build {
            write!(f, "{}.{}.{}+{}", self.major, self.minor, self.patch, build)
        } else {
            write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
        }
    }
}


/// Creates a new `Version<T>` from the `CARGO_PKG_VERSION` environment variable
/// at compile time (i.e., from your crate's `Cargo.toml`).
/// 
/// ***
/// # Examples
/// 
/// ```rust
/// use simple_version::{Version, version_from_pkg};
///
/// let version: Version<u32> = version_from_pkg!(u32);
/// // Now `version` might be e.g. 1.2.3 if your crate's version is "1.2.3"
/// ```
#[macro_export]
macro_rules! version_from_pkg {
    ($t:ty) => {{
        let _major: $t = env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap();
        let _minor: $t = env!("CARGO_PKG_VERSION_MINOR").parse().unwrap();
        let _patch: $t = env!("CARGO_PKG_VERSION_PATCH").parse().unwrap();
        $crate::Version::new(_major, _minor, _patch)
    }};
}