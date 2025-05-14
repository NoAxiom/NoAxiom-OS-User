use alloc::string::{String, ToString};

use bitflags::bitflags;

bitflags! {
    // pub struct OpenFlags:u32{
    //     const O_RDONLY = 0x0;
    //     const O_WRONLY = 0x1;
    //     const O_RDWR = 0x2;
    //     const O_CREATE = 0x40;
    //     const O_EXCL = 0x200;
    //     const O_NOCTTY = 0x400;
    //     const O_TRUNC = 0x1000;
    //     const O_APPEND = 0x2000;
    //     const O_NONBLOCK = 0x4000;
    //     const O_NOFOLLOW = 0x400000;
    //     const O_DIRECTORY = 0x200000;
    // }
    pub struct OpenFlags: u32 {
        // TODO do not use 0
        // NOTE: bitflags do not encourage zero bit flag, we should not directly check `O_RDONLY`
        const O_RDONLY    = 0;
        const O_WRONLY    = 1 << 0;
        const O_RDWR      = 1 << 1;
        const O_CREATE    = 1 << 6;
        const O_EXCL      = 1 << 7;
        const O_TRUNC     = 1 << 9;
        const O_APPEND    = 1 << 10;
        const O_NONBLOCK  = 1 << 11;
        const O_LARGEFILE = 1 << 15;
        const O_DIRECTORY = 1 << 16;
        const O_NOFOLLOW  = 1 << 17;
        const O_CLOEXEC   = 1 << 19;
    }
}
bitflags! {
    pub struct FileMode:u32{
        const FMODE_READ = 0x0;
        const FMODE_WRITE = 0x1;
        const FMODE_RDWR = 0x2;
        const FMODE_EXEC = 0x5; //read and execute
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct Stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_mode: u32,
    pub st_nlink: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_rdev: u64,
    __pad: u64,
    pub st_size: u64,
    pub st_blksize: u32,
    __pad2: u32,
    pub st_blocks: u64,
    pub st_atime_sec: u64,
    pub st_atime_nsec: u64,
    pub st_mtime_sec: u64,
    pub st_mtime_nsec: u64,
    pub st_ctime_sec: u64,
    pub st_ctime_nsec: u64,
    unused: u64,
} // 128

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct StatTime {
    pub year: u32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}
bitflags! {
    #[derive(Default)]
     pub struct InodeMode:u32{
        const S_SYMLINK = 0120000;
        const S_DIR = 0040000;
        const S_FILE = 0100000;
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Dirent64 {
    /// ino is an inode number
    pub ino: u64,
    /// off is an offset to next linux_dirent
    pub off: i64,
    /// reclen is the length of this linux_dirent
    pub reclen: u16,
    /// type is the file type
    pub type_: DirentType,
    /// name is the filename (null-terminated)
    pub name: [u8; 0],
}

impl Dirent64 {
    pub fn get_name(&self) -> &str {
        unsafe {
            let name = self.name.as_ptr();
            let name = core::ffi::CStr::from_ptr(name as *const i8);
            name.to_str().unwrap()
        }
    }
    pub fn len(&self) -> usize {
        self.reclen as usize
    }
}

bitflags! {
    pub struct DirentType:u8{
        const DT_UNKNOWN = 0;
        const DT_FIFO = 1;
        const DT_CHR = 2;
        const DT_DIR = 4;
        const DT_BLK = 6;
        const DT_REG = 8;
        const DT_LNK = 10;
        const DT_SOCK = 12;
        const DT_WHT = 14;
    }
}

impl ToString for DirentType {
    fn to_string(&self) -> String {
        match *self {
            DirentType::DT_UNKNOWN => "unknown".to_string(),
            DirentType::DT_FIFO => "fifo".to_string(),
            DirentType::DT_CHR => "char".to_string(),
            DirentType::DT_DIR => "dir".to_string(),
            DirentType::DT_BLK => "block".to_string(),
            DirentType::DT_REG => "regular".to_string(),
            DirentType::DT_LNK => "link".to_string(),
            DirentType::DT_SOCK => "sock".to_string(),
            DirentType::DT_WHT => "whiteout".to_string(),
            _ => "unknown".to_string(),
        }
    }
}
