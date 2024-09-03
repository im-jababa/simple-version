# simple-version
You can create a new `major`.`minor`.`path` version manually or automatically generate from cargo package version which is in `Cargo.toml`.

### How to use
```
use simple_version::Version;
```

### Get a cargo package version as release version
```
// Create a new release version from cargo package version.
let release_version::Version = Version::from_pkg();

// or
let release_version::Version = Version::from_pkg().release();

println!("{}", release_version);
```
> `v?.?.?-release`

### Get a cargo package version as beta version
```
// Create a new beta version from cargo package version.
let beta_version_default::Version = Version::from_pkg().beta(0);
let beta_version_with_number::Version = Version::from_pkg().beta(1);

println!("{}", beta_version_default);
println!("{}", beta_version_with_number);
```
> `v?.?.?-beta`

> `v?.?.?-beta1`

### Get a cargo package version as alpha version
```
// Create a new alpha version from cargo package version.
let alpha_version_default::Version = Version::from_pkg().alpha(0);
let alpha_version_with_number::Version = Version::from_pkg().alpha(1);

println!("{}", alpha_version_default);
println!("{}", alpha_version_with_number);
```
> `v?.?.?-alpha`

> `v?.?.?-alpha1`

### Create a new version manually
```
// new release version
// you omit `.release()` at the end.
let release: Version = Version::new(major, minor, patch).release();

// new beta version
let beta: Version = Version::new(major, minor, patch).beta(beta_number);

// new alpha version
let alpha: Version = Version::new(major, minor, patch).alpha(alpha_number);
```

### Some exeptions
```
// You can do this but it will...
let version: Version = Version::new(1, 2, 3).beta(4).beta(5).beta(6);
println!("{}", version);
```
> `v1.3.4-beta6`

### Comparison
You can compair between versions.

`release` > `beta` > `alpha` in same major, minor, patch version