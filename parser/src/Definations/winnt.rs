
pub type BYTE = u8;
pub type WORD = u16;
pub type DWORD = u32;
pub type QDWORD = u64;
pub type LONG = u32;
pub type LONGLONG = i64;
pub type ULONGLONG = u64;

pub const ___IMAGE_NT_OPTIONAL_HDR32_MAGIC :u16=       0x10b;
pub const ___IMAGE_NT_OPTIONAL_HDR64_MAGIC   :u16=    0x20b;
pub const ___IMAGE_NUMBEROF_DIRECTORY_ENTRIES :u16=   16;
pub const ___IMAGE_DOS_SIGNATURE              :u16=   0x5A4D;

pub const ___IMAGE_DIRECTORY_ENTRY_EXPORT     :usize=     0;
pub const ___IMAGE_DIRECTORY_ENTRY_IMPORT     :usize=     1;
pub const ___IMAGE_DIRECTORY_ENTRY_RESOURCE    :usize=    2;
pub const ___IMAGE_DIRECTORY_ENTRY_EXCEPTION    :usize=   3;
pub const ___IMAGE_DIRECTORY_ENTRY_SECURITY     :usize=   4;
pub const ___IMAGE_DIRECTORY_ENTRY_BASERELOC    :usize=   5;
pub const ___IMAGE_DIRECTORY_ENTRY_DEBUG         :usize=  6;
pub const ___IMAGE_DIRECTORY_ENTRY_ARCHITECTURE  :usize=  7;
pub const ___IMAGE_DIRECTORY_ENTRY_GLOBALPTR     :usize=  8;
pub const ___IMAGE_DIRECTORY_ENTRY_TLS           :usize=  9;
pub const ___IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG   :usize= 10;
pub const ___IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT  :usize= 11;
pub const ___IMAGE_DIRECTORY_ENTRY_IAT           :usize= 12;
pub const ___IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT   :usize=13;
pub const ___IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR :usize=14;

pub const ___IMAGE_SIZEOF_SHORT_NAME             :usize= 8;
pub const ___IMAGE_SIZEOF_SECTION_HEADER         :usize= 40;

pub struct __IMAGE_DOS_HEADER{
   pub e_magic:WORD,
   pub e_cblp:WORD,
   pub e_cp:WORD,
   pub e_crlc:WORD,
   pub e_cparhdr: WORD,
   pub e_minalloc:WORD,
   pub e_maxalloc:WORD,
   pub e_ss:  WORD,
   pub e_sp:  WORD,
   pub e_csum:WORD,
   pub e_ip:  WORD,
   pub e_cs:  WORD,
   pub e_lfarlc:  WORD,
   pub e_ovno:WORD,
   pub e_res:  [WORD;4],
   pub e_oemid:   WORD,
   pub e_oeminfo: WORD,
   pub e_res2:[WORD;10],
   pub e_lfanew:LONG,

}
pub type __PIMAGE_DOS_HEADER = *mut IMAGE_DOS_HEADER;
pub struct __IMAGE_DATA_DIRECTORY{
   pub VirtualAddress: DWORD,
   pub Size: DWORD,
}
pub type __PIMAGE_DATA_DIRECTORY = *mut __PIMAGE_DATA_DIRECTORY;
pub struct __IMAGE_OPTIONAL_HEADER32{
   pub Magic: WORD,//The unsigned integer that identifies the state of the image file. The most common number is 0x10B, which identifies it as a normal executable file. 0x107 identifies it as a ROM image, and 0x20B identifies it as a PE32+ executable.
   pub MajorLinkerVersion:BYTE,//The linker major version number.
   pub MinorLinkerVersion:BYTE,//The linker minor version number.
   pub SizeOfCode: DWORD,//The size of the code (text) section, or the sum of all code sections if there are multiple sections.
   pub SizeOfInitializedData: DWORD,//The size of the initialized data section, or the sum of all such sections if there are multiple data sections.
   pub SizeOfUninitializedData: DWORD,//The size of the uninitialized data section (BSS), or the sum of all such sections if there are multiple BSS sections.
   pub AddressOfEntryPoint: DWORD,//The address of the entry point relative to the image base when the executable file is loaded into memory.
   pub BaseOfCode: DWORD,//The address that is relative to the image base of the beginning-of-code section
   pub BaseOfData: DWORD,//The address that is relative to the image base of the beginning-of-data section
   pub ImageBase: DWORD,//The preferred address of the first byte of image when loaded into memory; must be a multiple of 64 K. The default for DLLs is 0x10000000.
   pub SectionAlignment: DWORD,//The alignment (in bytes) of sections when they are loaded into memory. It must be greater than or equal to FileAlignment
   pub FileAlignment: DWORD,//The alignment factor (in bytes) that is used to align the raw data of sections in the image file. The value should be a power of 2 between 512 and 64 K, inclusive.
   pub MajorOperatingSystemVersion: WORD,//The major version number of the required operating system.
   pub MinorOperatingSystemVersion: WORD,//The minor version number of the required operating system.
   pub MajorImageVersion: WORD,//The major version number of the image.
   pub MinorImageVersion: WORD,
   pub MajorSubsystemVersion: WORD,
   pub MinorSubsystemVersion: WORD,
   pub Win32VersionValue: DWORD,//Reserved, must be zero.
   pub SizeOfImage: DWORD,//The size (in bytes) of the image, including all headers, as the image is loaded in memory.
   pub SizeOfHeaders: DWORD,//The combined size of an MS-DOS stub, PE header, and section headers rounded up to a multiple of FileAlignment.
   pub CheckSum:DWORD,//The image file checksum. The algorithm for computing the checksum is incorporated into IMAGHELP.DLL.
   pub Subsystem:WORD,//The subsystem that is required to run this image
   pub DllCharacteristics: WORD,
   pub SizeOfStackReserve: DWORD,//The size of the stack to reserve. Only SizeOfStackCommit is committed
   pub SizeOfStackCommit: DWORD,//The size of the stack to commit.
   pub SizeOfHeapReserve: DWORD,//The size of the local heap space to reserve
   pub SizeOfHeapCommit: DWORD,//The size of the local heap space to commit.
   pub LoaderFlags:DWORD,//Reserved, must be zero.
   pub NumberOfRvaAndSizes: DWORD,//The number of data-directory entries in the remainder of the optional header
   pub DataDirectory: [__IMAGE_DATA_DIRECTORY; __IMAGE_NUMBEROF_DIRECTORY_ENTRIES],
}
pub type __PIMAGE_OPTIONAL_HEADER32 = *mut __IMAGE_OPTIONAL_HEADER32;
pub struct __IMAGE_OPTIONAL_HEADER64{
   pub Magic: WORD,
   pub MajorLinkerVersion:BYTE,
   pub MinorLinkerVersion:BYTE,
   pub SizeOfCode: DWORD,
   pub SizeOfInitializedData: DWORD,
   pub SizeOfUninitializedData: DWORD,
   pub AddressOfEntryPoint: DWORD,
   pub BaseOfCode: DWORD,
   pub BaseOfData: DWORD,
   pub ImageBase: ULONGLONG,
   pub SectionAlignment: DWORD,
   pub FileAlignment: DWORD,
   pub MajorOperatingSystemVersion: WORD,
   pub MinorOperatingSystemVersion: WORD,
   pub MajorImageVersion: WORD,
   pub MinorImageVersion: WORD,
   pub MajorSubsystemVersion: WORD,
   pub MinorSubsystemVersion: WORD,
   pub Win32VersionValue: DWORD,
   pub SizeOfImage: DWORD,
   pub SizeOfHeaders: DWORD,
   pub CheckSum:DWORD,
   pub Subsystem:WORD,
   pub DllCharacteristics: WORD,
   pub SizeOfStackReserve: ULONGLONG,
   pub SizeOfStackCommit: ULONGLONG,
   pub SizeOfHeapReserve: ULONGLONG,
   pub SizeOfHeapCommit: ULONGLONG,
   pub LoaderFlags:DWORD,
   pub NumberOfRvaAndSizes: DWORD,
   pub DataDirectory: [__IMAGE_DATA_DIRECTORY; __IMAGE_NUMBEROF_DIRECTORY_ENTRIES],
}
pub type __PIMAGE_OPTIONAL_HEADER64 = *mut __IMAGE_OPTIONAL_HEADER64;
pub struct __IMAGE_FILE_HEADER{
   pub Machine: WORD,
   pub NumberOfSections: WORD,
   pub TimeStamp: DWORD,
   pub PointerToSymbolTable: DWORD,
   pub NumberOfSymbols: DWORD,
   pub SizeOfOptionalHeader: WORD,
   pub Characteristics: WORD
}
pub type __PIMAGE_FILE_HEADER = *mut __IMAGE_FILE_HEADER;

pub struct __IMAGE_NT_HEADER64{
    pub Signature: DWORD,
    pub FileHeader: __IMAGE_FILE_HEADER,
    pub OptionalHeader: __IMAGE_OPTIONAL_HEADER64,
}
pub type __PIMAGE_NT_HEADER64 = *mut __IMAGE_NT_HEADER64;
pub struct __IMAGE_NT_HEADER32{
    pub Signature: DWORD,
    pub FileHeader: __IMAGE_FILE_HEADER,
    pub OptionalHeader: __IMAGE_OPTIONAL_HEADER32,
}
pub type __PIMAGE_NT_HEADER32 = *mut __IMAGE_NT_HEADER32;
pub union IMAGE_IMPORT_DESCRIPTOR_Union {
    pub Characteristics: DWORD,
    pub OriginalFirstThunk: DWORD,
}
pub struct __IMAGE_IMPORT_DESCRIPTOR {
    pub DUMMYUNIONNAME: IMAGE_IMPORT_DESCRIPTOR_Union,
    pub TimeDateStamp: DWORD,
    pub ForwarderChain: DWORD,
    pub Name: DWORD,
    pub FirstThunk: DWORD,
}
pub type __PIMAGE_IMPORT_DESCRIPTOR = *mut __IMAGE_IMPORT_DESCRIPTOR;
pub struct __IMAGE_IMPORT_BY_NAME {
    pub Hint: DWORD,
    pub Name: [char;100]
}
pub type __PIMAGE_IMPORT_BY_NAME = *mut __IMAGE_IMPORT_BY_NAME;
pub struct __IMAGE_BASE_RELOCATION {
    pub VirtualAddress: DWORD,
    pub SizeOfBlock: DWORD}
pub type __PIMAGE_BASE_RELOCATION = *mut __IMAGE_BASE_RELOCATION;

pub union IMAGE_SECTION_HEADER_Union{
    pub PhysicalAddress: DWORD,
    pub VirtualAddress: DWORD,
}
#[derive(Debug, Clone, Copy)]
pub struct IMAGE_SECTION_HEADER {
    pub Name: [u8;___IMAGE_SIZEOF_SHORT_NAME],
    pub Misc: IMAGE_SECTION_HEADER_Union,
    pub VirtualAddress: DWORD,
    pub SizeOfRawData: DWORD,
    pub PointerToRawData: DWORD,
    pub PointerToRelocations: DWORD,
    pub PointerToLinenumbers: DWORD,
    pub NumberOfRelocations: WORD,
    pub NumberOfLinenumbers: WORD,
    pub Characteristics: DWORD,
}
