//! # Handling Files Relative to File Descriptor
//!
//! Main concept here is a `Dir` which holds `O_PATH` file descriptor, you
//! can create it with:
//!
//! * `Dir::open("/some/path")` -- open this directory as a file descriptor
//! * `Dir::from_raw_fd(fd)` -- uses a file descriptor provided elsewhere
//!
//! *Note after opening file descriptors refer to same directory regardless of
//! where it's moved or mounted (with `pivot_root` or `mount --move`). It may
//! also be unmounted or be out of chroot and you will still be able to
//! access files relative to it.*
//!
//! *Note3: Some OS's (e.g., macOS) do not provide `O_PATH`, in which case the
//! file descriptor is of regular type.*
//!
//! Most other operations are done on `Dir` object and are executed relative
//! to it:
//!
//! * `Dir::list_dir()`
//! * `Dir::sub_dir()`
//! * `Dir::read_link()`
//! * `Dir::open_file()`
//! * `Dir::create_file()`
//! * `Dir::update_file()`
//! * `Dir::create_dir()`
//! * `Dir::symlink()`
//! * `Dir::local_rename()`
//!
//! Functions that expect path relative to the directory accept both the
//! traditional path-like objects, such as Path, PathBuf and &str, and
//! `Entry` type returned from `list_dir()`. The latter is faster as underlying
//! system call wants `CString` and we keep that in entry.
//!
//! Note that if path supplied to any method of dir is absolute the Dir file
//! descriptor is ignored.
//!
//! Also while all methods of dir accept any path if you want to prevent
//! certain symlink attacks and race condition you should only use
//! a single-component path. I.e. open one part of a chain at a time.
#![warn(missing_docs)]

extern crate libc;

mod builder;
mod dir;
mod filetype;
mod list;
mod metadata;
mod name;

pub use crate::builder::{DirFlags, DirMethodFlags};
pub use crate::dir::{hardlink, rename, Dir, O_DIRECTORY, O_PATH, O_SEARCH};
pub use crate::filetype::SimpleType;
pub use crate::list::{DirIter, Entry};
pub use crate::metadata::{metadata_types, Metadata};
pub use crate::name::AsPath;

#[cfg(test)]
mod test {
    use std::mem;

    use super::Dir;

    fn assert_sync<T: Sync>(x: T) -> T {
        x
    }
    fn assert_send<T: Send>(x: T) -> T {
        x
    }

    #[test]
    fn test() {
        let d = Dir::new(3);
        let d = assert_sync(d);
        let d = assert_send(d);
        // don't execute close for our fake RawFd
        mem::forget(d);
    }
}
