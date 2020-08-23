use crate::{FileType, Mode};
use core::convert::TryInto;
use nix::sys::stat::{Mode as UnixMode, SFlag as UnixFileType};

impl From<UnixMode> for Mode {
    #[inline]
    fn from(x: UnixMode) -> Mode {
        Mode::from_bits(x.bits().try_into().unwrap()).unwrap()
    }
}

impl From<Mode> for UnixMode {
    #[inline]
    fn from(x: Mode) -> UnixMode {
        UnixMode::from_bits(x.bits().into()).unwrap()
    }
}

#[cfg(feature = "num")]
impl From<UnixFileType> for FileType {
    #[inline]
    // x.bits() might be u16 on some platforms
    #[allow(clippy::useless_conversion)]
    fn from(x: UnixFileType) -> FileType {
        use num_traits::FromPrimitive;
        FileType::from_u32(x.bits().into()).unwrap()
    }
}

impl From<FileType> for UnixFileType {
    #[inline]
    // x.bits() might be u32 on some platforms
    #[allow(clippy::useless_conversion)]
    fn from(x: FileType) -> UnixFileType {
        UnixFileType::from_bits(x.bits().into()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn nixeq() {
        macro_rules! nixaeq {
            ($($name:ident),+) => {
                $(
                    let um = paste::paste! { nix::sys::stat::Mode::[<S_ $name>] };
                    assert_eq!(nix::libc::mode_t::from(crate::Mode::$name.bits()), um.bits());
                )+
            }
        };
        nixaeq!(IRWXU, IRUSR, IWUSR, IXUSR, IRWXG, IRGRP, IWGRP, IXGRP, IRWXO, IROTH, IWOTH, IXOTH);
    }
}
