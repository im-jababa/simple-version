use crate::*;


#[test]
fn test_new() {
    let version = Version::<u32>::new(1, 2, 3);

    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 3);
    assert!(version.build.is_none());
}


#[test]
fn test_mutatbility() {
    let mut version = Version::<u32>::new(1, 2, 3);

    version.major = 4;
    version.minor = 5;
    version.patch = 6;

    assert_eq!(version.major, 4);
    assert_eq!(version.minor, 5);
    assert_eq!(version.patch, 6);
    assert!(version.build.is_none());
}


#[test]
fn test_build_number() {
    let version = Version::<u32>::new(1, 2, 3).build(4);

    assert!(version.build.is_some_and(|build| build == 4));
}


#[test]
fn test_basic_compare() {
    let version1 = Version::<u32>::new(1, 999, 0);
    let version2 = Version::<u32>::new(2, 0, 0);
    
    assert!(version1 < version2);
}


#[test]
fn test_build_num_compare() {
    let version1 = Version::<u32>::new(1, 0, 0).build(1);
    let version2 = Version::<u32>::new(1, 0, 0).build(2);

    assert!(version1 < version2);
}


#[test]
fn test_mixed_compare() {
    let version1 = Version::<u32>::new(1, 0, 0);
    let version2 = Version::<u32>::new(1, 0, 0).build(1);

    assert!(version1 < version2);
}


#[test]
fn test_pkg_version() {
    let major: u32 = env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap();
    let minor: u32 = env!("CARGO_PKG_VERSION_MINOR").parse().unwrap();
    let patch: u32 = env!("CARGO_PKG_VERSION_PATCH").parse().unwrap();
    let version = Version::<u32>::new(major, minor, patch);

    let version_from_pkg = version_from_pkg!(u32);

    assert_eq!(version, version_from_pkg);
}