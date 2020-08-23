#![no_std]
#![forbid(unsafe_code)]

extern crate core;

use bitflags::bitflags;
use core::fmt::{self, Write};

#[cfg(feature = "num")]
use num_derive as nd;

#[repr(u16)]
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "num", derive(nd::FromPrimitive, nd::ToPrimitive))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum FileType {
    /// directory
    IFDIR  = 0o040000,
    /// character device
    IFCHR  = 0o020000,
    /// block device
    IFBLK  = 0o060000,
    /// regular file
    IFREG  = 0o100000,
    // FIFO
    IFIFO  = 0o010000,
    // symbolic link
    IFLNK  = 0o120000,
    // socket
    IFSOCK = 0o140000,
}

impl FileType {
    /// file type bitmask
    pub const IFMT: u16 = 0o170000;

    /// Helper function for compatibility with bitflags structs
    #[inline(always)]
    #[cfg(feature = "num")]
    pub fn from_bits(x: u16) -> Option<Self> {
        <Self as num_traits::FromPrimitive>::from_u16(x)
    }

    /// Helper function for compatibility with bitflags structs
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        *self as _
    }
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(match self {
            Self::IFSOCK => 's',
            Self::IFLNK => 'l',
            Self::IFREG => '-',
            Self::IFBLK => 'b',
            Self::IFDIR => 'd',
            Self::IFCHR => 'c',
            Self::IFIFO => 'p',
        })
    }
}

bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "serde", serde(transparent))]
    pub struct Mode: u16 {
        // POSIX protection bits
        /// set user ID on exec
        const ISUID  = 0o004000;
        /// set group ID on exec
        const ISGID  = 0o002000;
        /// sticky
        const ISVTX  = 0o001000;

        // portable owner protection bits (both windows and unix)
        /// owner: read
        const IREAD  = 0o000400;
        /// owner: write
        const IWRITE = 0o000200;
        /// owner: exec
        const IEXEC  = 0o000100;

        // POSIX owner protection bits
        /// owner: read
        const IRUSR  = Self::IREAD.bits;
        /// owner: write
        const IWUSR  = Self::IWRITE.bits;
        /// owner: exec
        const IXUSR  = Self::IEXEC.bits;
        const IRWXU  = Self::IRUSR.bits | Self::IWUSR.bits | Self::IXUSR.bits;

        // POSIX group protection bits
        /// group: read
        const IRGRP  = Self::IRUSR.bits >> 3;
        /// group: write
        const IWGRP  = Self::IWUSR.bits >> 3;
        /// group: exec
        const IXGRP  = Self::IXUSR.bits >> 3;
        const IRWXG  = Self::IRGRP.bits | Self::IWGRP.bits | Self::IXGRP.bits;

        // POSIX other protection bits
        /// other: read
        const IROTH  = Self::IRGRP.bits >> 3;
        /// other: write
        const IWOTH  = Self::IWGRP.bits >> 3;
        /// other: exec
        const IXOTH  = Self::IXGRP.bits >> 3;
        const IRWXO  = Self::IROTH.bits | Self::IWOTH.bits | Self::IXOTH.bits;
    }
}

impl Mode {
    fn fmt_rwx_bits(&self, shift: u8, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sxbit = match shift {
            2 /* owner */ => self.contains(Self::ISUID),
            1 /* group */ => self.contains(Self::ISGID),
            0 /* other */ => self.contains(Self::ISVTX),
            _ => panic!("fmt_rwx_bits: illegal shift value"),
        };
        let protbits = (self.bits >> (3 * shift)) & 0o7;
        f.write_char(if (protbits & 0o4) > 0 { 'r' } else { '-' })?;
        f.write_char(if (protbits & 0o2) > 0 { 'w' } else { '-' })?;
        f.write_char(match (shift, (protbits & 0o1) > 0, sxbit) {
            (0, true, true) => 't',
            (0, false, true) => 'T',
            (_, true, true) => 's',
            (_, false, true) => 'S',
            (_, true, false) => 'x',
            (_, false, false) => '-',
        })?;
        Ok(())
    }
}

impl fmt::Display for Mode {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_rwx_bits(2, f)?;
        self.fmt_rwx_bits(1, f)?;
        self.fmt_rwx_bits(0, f)?;
        Ok(())
    }
}

#[cfg(feature = "num-traits")]
impl num_traits::FromPrimitive for Mode {
    #[inline]
    fn from_i64(n: i64) -> Option<Self> {
        Self::from_u16(u16::from_i64(n)?)
    }

    #[inline]
    fn from_u64(n: u64) -> Option<Self> {
        Self::from_u16(u16::from_u64(n)?)
    }

    #[inline(always)]
    fn from_u16(n: u16) -> Option<Self> {
        Self::from_bits(n)
    }
}

#[cfg(feature = "num-traits")]
impl num_traits::ToPrimitive for Mode {
    #[inline(always)]
    fn to_u64(&self) -> Option<u64> {
        Some(self.bits.into())
    }
    #[inline(always)]
    fn to_u16(&self) -> Option<u16> {
        Some(self.bits)
    }
    #[inline(always)]
    fn to_i64(&self) -> Option<i64> {
        Some(self.bits.into())
    }
}

/// Split a file mode into file type and protection bits
#[cfg(feature = "num")]
pub fn split(fmode: u16) -> Option<(FileType, Mode)> {
    let ft = FileType::from_bits(fmode & FileType::IFMT)?;
    let md = Mode::from_bits(fmode & !FileType::IFMT)?;
    Some((ft, md))
}

#[cfg(all(unix, any(test, feature = "nix")))]
mod nix_;

// we can't provide a TryFrom<umask::Mode> because the version of umask with
// an Into<u32> impl is currently not published on crates.io

/*
pub struct InvalidBits<T>(pub T);

#[cfg(feature = "umask")]
impl core::convert::TryFrom<umask::Mode> for Mode {
    type Error = InvalidBits<u32>;

    #[inline]
    fn try_from(x: umask::Mode) -> Result<Mode, InvalidBits<u32>> {
        let x: u32 = x.into();
        let db: u32 = Mode::all().bits.into();
        let ib = x & !db;
        if ib > 0 {
            Err(InvalidBits(ib))
        } else {
            Ok(Mode::from_bits(x as u16).unwrap())
        }
    }
}
*/

#[cfg(feature = "umask")]
impl From<Mode> for umask::Mode {
    #[inline]
    fn from(x: Mode) -> umask::Mode {
        umask::Mode::from(u32::from(x.bits))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn display() {
        macro_rules! bytes_and_str {
            ($($a:expr => $b:expr),+ $(,)?) => {
                $( assert_eq!(std::format!("{}", crate::Mode::from_bits($a).expect("unknown bitmask")), $b); )+
            }
        };
        extern crate std;
        bytes_and_str! {
            0o0200 => "-w-------",
            0o0706 => "rwx---rw-",
            0o0074 => "---rwxr--",
            0o0777 => "rwxrwxrwx",
            0o1777 => "rwxrwxrwt",
            0o2777 => "rwxrwsrwx",
            0o4777 => "rwsrwxrwx",
            0o7777 => "rwsrwsrwt",
            0o7747 => "rwsr-Srwt",
            0o7000 => "--S--S--T",
        }
    }
}
