//! Misc

#[macro_export]
/// Helper to create version string or build time for your crate.
///
/// Package version with git info is in format `{CARGO_PKG_VERSION}-{git
/// btanch}-{git commit}-{debug or release}`, like `0.4.0-main-c298bc2-DEBUG`.
///
/// Should have `git` installed.
///
/// # Example
///
/// - Create a `build.rs` in the root dir if not persists, and use this macro in
///   the main func. The main func should return `Result`.
///
///   - For a version string, use `crate_version!(VERSION)`
///   - For a build time in RFC3339 format, use `crate_version!(BUILD_TIME)`.
///
///     Don't forget to add `chrono` to your build dependencies.
/// - In your crate, place `crate_version!(VERSION => pub VERSION)`,
///   `crate_version!(VERSION => pub VERSION)` where you like.
///
///   You can even do this: `crate_version!(VERSION => pub VERSION,
/// env!("CARGO_PKG_NAME", "/"))`
macro_rules! crate_version {
    (VERSION) => {{
        use std::{env, fs::File, io::Write, path::Path, process::Command};

        let main_version = env!("CARGO_PKG_VERSION");

        let branch = Command::new("git")
            .args(["branch", "--show-current"])
            .output()
            .map(|o| String::from_utf8(o.stdout).unwrap())
            .unwrap();

        let commit = Command::new("git")
            .args(["describe", "--always"])
            .output()
            .map(|o| String::from_utf8(o.stdout).unwrap())
            .unwrap();

        let release_mode = if cfg!(debug_assertions) || cfg!(test) {
            "DEBUG"
        } else {
            "RELEASE"
        };

        let version =
            format!("{}-{}-{}-{}", main_version, branch, commit, release_mode).replace('\n', "");
        File::create(Path::new(&env::var("OUT_DIR")?).join("VERSION"))?
            .write_all(version.trim().as_bytes())?;
    }};
    (BUILD_TIME) => {{
        use std::{env, fs::File, io::Write, path::Path};

        let now = chrono::Local::now().to_rfc3339();
        File::create(Path::new(&env::var("OUT_DIR")?).join("BUILD_TIME"))?
            .write_all(now.trim().as_bytes())?;
    }};
    (VERSION => $vis:vis $name:ident) => {
        /// The git version.
        $vis static $name: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));
    };
    (VERSION => $vis:vis $name:ident, $($c:tt)*) => {
        /// The git version.
        $vis static $name: &'static str = concat!($($c)*, include_str!(concat!(env!("OUT_DIR"), "/VERSION")));
    };
    (BUILD_TIME => $vis:vis $name:ident) => {
        /// The git version.
        $vis static $name: &'static str = include_str!(concat!(env!("OUT_DIR"), "/BUILD_TIME"));
    };
}
