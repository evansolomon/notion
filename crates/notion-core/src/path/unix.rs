//! Provides functions for determining the paths of files and directories
//! in a standard Notion layout in Unix-based operating systems.

use std::env;
use std::path::PathBuf;

use notion_fail::{Fallible, NotionFail};

#[derive(Fail, Debug)]
#[fail(display = "environment variable 'HOME' is not set")]
pub(crate) struct NoHomeEnvVar;

impl NotionFail for NoHomeEnvVar {
    fn is_user_friendly(&self) -> bool { true }
    fn exit_code(&self) -> i32 { 4 }
}

// These are taken from: https://nodejs.org/dist/index.json and are used
// by `path::archive_root_dir` to determine the root directory of the
// contents of a Node installer archive.

cfg_if! {
    if #[cfg(target_os = "macos")] {
        /// The OS component of a Node distribution tarball's name.
        pub const OS: &'static str = "darwin";
    } else if #[cfg(target_os = "linux")] {
        /// The OS component of a Node distribution tarball's name.
        pub const OS: &'static str = "linux";
    } else {
        compile_error!("Unsupported target_os variant of unix (expected 'macos' or 'linux').");
    }
}

cfg_if! {
    if #[cfg(target_arch = "x86")] {
        /// The system architecture component of a Node distribution tarball's name.
        pub const ARCH: &'static str = "x86";
    } else if #[cfg(target_arch = "x86_64")] {
        /// The system architecture component of a Node distribution tarball's name.
        pub const ARCH: &'static str = "x64";
    } else {
        compile_error!("Unsupported target_arch variant of unix (expected 'x86' or 'x64').");
    }
}

// ~/
//     .notion/
//         cache/                                          cache_dir
//             node/                                       node_cache_dir
//                 node-dist-v4.8.4-linux-x64.tar.gz       archive_file("4.8.4")
//                 node-dist-v6.11.3-linux-x64.tar.gz
//                 node-dist-v8.6.0-linux-x64.tar.gz
//                 ...
//         versions/                                       versions_dir
//             node/                                       node_versions_dir
//                 4.8.4/                                  node_version_dir("4.8.4")
//                   bin/                                  node_version_bin_dir("4.8.4")
//                 6.11.3/
//                 8.6.0/
//                 ...
//         bin/                                            bin_dir
//             notion                                      notion_file
//         shim/                                           shim_dir
//             node                                        shim_file("node")
//             npm
//             npx
//             ...
//         launchbin                                       launchbin_file
//         launchscript                                    launchscript_file
//         config.toml                                     user_config_file
//         catalog.toml                                    user_catalog_file

fn notion_home() -> Fallible<PathBuf> {
    let home = env::home_dir().ok_or(NoHomeEnvVar)?;
    Ok(home.join(".notion"))
}

pub fn cache_dir() -> Fallible<PathBuf> {
    Ok(notion_home()?.join("cache"))
}

pub fn node_cache_dir() -> Fallible<PathBuf> {
    Ok(cache_dir()?.join("node"))
}

pub fn node_index_file() -> Fallible<PathBuf> {
    Ok(node_cache_dir()?.join("index.json"))
}

pub fn node_index_expiry_file() -> Fallible<PathBuf> {
    Ok(node_cache_dir()?.join("index.json.expires"))
}

pub fn archive_extension() -> String {
    String::from("tar.gz")
}

pub fn versions_dir() -> Fallible<PathBuf> {
    Ok(notion_home()?.join("versions"))
}

pub fn node_versions_dir() -> Fallible<PathBuf> {
    Ok(versions_dir()?.join("node"))
}

pub fn node_version_dir(version: &str) -> Fallible<PathBuf> {
    Ok(node_versions_dir()?.join(version))
}

pub fn node_version_bin_dir(version: &str) -> Fallible<PathBuf> {
    Ok(node_version_dir(version)?.join("bin"))
}

pub fn bin_dir() -> Fallible<PathBuf> {
    Ok(notion_home()?.join("bin"))
}

pub fn notion_file() -> Fallible<PathBuf> {
    Ok(bin_dir()?.join("notion"))
}

pub fn shim_dir() -> Fallible<PathBuf> {
    Ok(notion_home()?.join("shim"))
}

pub fn shim_file(toolname: &str) -> Fallible<PathBuf> {
    Ok(shim_dir()?.join(toolname))
}

pub fn launchbin_file() -> Fallible<PathBuf> {
    Ok(notion_home()?.join("launchbin"))
}

pub fn launchscript_file() -> Fallible<PathBuf> {
    Ok(notion_home()?.join("launchscript"))
}

pub fn user_config_file() -> Fallible<PathBuf> {
    Ok(notion_home()?.join("config.toml"))
}

pub fn user_catalog_file() -> Fallible<PathBuf> {
    Ok(notion_home()?.join("catalog.toml"))
}
