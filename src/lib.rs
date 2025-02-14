#![no_std]

//! Embed git information in your code at compile-time.
//!
//! ```
//! use comfy_git_version::git_version;
//! const GIT_VERSION: &str = git_version!();
//! ```
//!
//! The version number will have a `-modified` suffix if your git worktree had
//! untracked or changed files.
//!
//! These macros do not depend on libgit, but simply uses the `git` binary directly.
//! So you must have `git` installed somewhere in your `PATH`.

pub use comfy_git_version_macro::git_version;

/// Run `git describe` at compile time with custom flags.
///
/// This is just a short-hand for `git_version!(args = [...])`,
/// to be backwards compatible with earlier versions of this crate.
#[macro_export]
macro_rules! git_describe {
	($($args:tt)*) => {
		$crate::comfy_git_version!(args = [$($args)*])
	};
}
