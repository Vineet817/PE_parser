use crate::Definations::winnt::{DWORD, WORD};

pub struct RICH_HEADER_INFO {
    pub size: i32,
    pub ptr_to_buffer: *mut i8,
    pub entries: i32,
}
pub type PRICH_HEADER_INFO = *mut RICH_HEADER_INFO;
pub struct RICH_HEADER_ENTRY {
    pub prodID: WORD,
    pub buildID: WORD,
    pub useCount: DWORD,
}
pub type PRICH_HEADER_ENTRY = *mut RICH_HEADER_ENTRY;
pub struct RICH_HEADER {
    pub entries: PRICH_HEADER_ENTRY,
}
pub type PRICH_HEADER = *mut RICH_HEADER;
#[repr(C)]
pub struct ILT_ENTRY_32 {
    pub field_1: DWORD, // union -> represented as DWORD, interpret bits via helpers
}
impl ILT_ENTRY_32 {
    pub fn ordinal(&self) -> u16 {
        (self.field_1 & 0xFFFF) as u16
    }
    pub fn hint_name_table(&self) -> u32 {
        self.field_1 & 0xFFFFFFFF
    }
    pub fn ordinal_name_flag(&self) -> bool {
        (self.field_1 & (1 << 31)) != 0
    }
}

#[repr(C)]
pub struct ILT_ENTRY_64 {
    pub field_2: DWORD,
    pub ordinal_name_flag: DWORD, // this was a :1 bitfield in C
}

impl ILT_ENTRY_64 {
    pub fn ordinal(&self) -> u16 {
        (self.field_2 & 0xFFFF) as u16
    }
    pub fn hint_name_table(&self) -> u32 {
        self.field_2 & 0xFFFFFFFF
    }
    pub fn ordinal_name_flag(&self) -> bool {
        (self.ordinal_name_flag & 1) != 0
    }
}

#[repr(C)]
pub struct BASE_RELOC_ENTRY {
    raw: WORD, // 16 bits (OFFSET:12, TYPE:4)
}

impl BASE_RELOC_ENTRY {
    pub fn offset(&self) -> u16 {
        self.raw & 0x0FFF // lower 12 bits
    }
    pub fn r#type(&self) -> u8 {
        ((self.raw >> 12) & 0xF) as u8 // upper 4 bits
    }
}
