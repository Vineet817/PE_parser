use crate::Definations::winnt::{
    ___IMAGE_NT_OPTIONAL_HDR32_MAGIC, ___IMAGE_NT_OPTIONAL_HDR64_MAGIC,
    __IMAGE_DOS_HEADER, __IMAGE_FILE_HEADER,
};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::ptr::read_unaligned;

pub fn initiate(mut file: File) -> u32 {
    let mut dos_buf = [0u8; size_of::<__IMAGE_DOS_HEADER>()];
    file.read_exact(&mut dos_buf)
        .expect("Error reading dos header in file.read_exact");
    let dos_header: __IMAGE_DOS_HEADER =
        unsafe { read_unaligned(dos_buf.as_ptr() as *const __IMAGE_DOS_HEADER) };
    let mut PEFILE_type = [0u8; 2];
    // Now access e_lfanew safely by copying its value
    let e_lfanew = dos_header.e_lfanew;
    let seek_ptr_for_bit_magic =
        dos_header.e_lfanew as u64 + 4 + size_of::<__IMAGE_FILE_HEADER>() as u64;
    file.seek(SeekFrom::Start(seek_ptr_for_bit_magic))
        .expect("Error seeking to file header");
    file.read_exact(&mut PEFILE_type)
        .expect("Error reading dos header in file.read_exact");
    let magic = u16::from_le_bytes(PEFILE_type);
    if magic == ___IMAGE_NT_OPTIONAL_HDR32_MAGIC {
        println!("file is 32-bit format");
        return 32;
    } else if magic == ___IMAGE_NT_OPTIONAL_HDR64_MAGIC {
        println!("file is 64-bit format");
        return 64;
    } else {
        println!("file is neither 32-bit format, nor 64-bit format");
        return 1;
    }
    1
}
