#![allow(dead_code)]

use windows_type::windows_type;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IMAGE_DOS_HEADER {
    pub e_magic: u16,
    pub e_cblp: u16,
    pub e_cp: u16,
    pub e_crlc: u16,
    pub e_cparhdr: u16,
    pub e_minalloc: u16,
    pub e_maxalloc: u16,
    pub e_ss: u16,
    pub e_sp: u16,
    pub e_csum: u16,
    pub e_ip: u16,
    pub e_cs: u16,
    pub e_lfarlc: u16,
    pub e_ovno: u16,
    pub e_res: [u16; 4],
    pub e_oemid: u16,
    pub e_oeminfo: u16,
    pub e_res2: [u16; 10],
    pub e_lfanew: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IMAGE_FILE_HEADER {
    pub machine: u16,
    pub number_of_sections: u16,
    pub time_date_stamp: u32,
    pub pointer_to_symbol_table: u32,
    pub number_of_symbols: u32,
    pub size_of_optional_header: u16,
    pub characteristics: u16,
}

#[windows_type(x32, x64)]
pub struct IMAGE_OPTIONAL_HEADER {
    magic: u16,
    major_linker_version: u8,
    minor_linker_version: u8,
    size_of_code: u32,
    size_of_initialized_data: u32,
    size_of_uninitialized_data: u32,
    address_of_entry_point: u32,
    base_of_code: u32,
    #[x32]
    base_of_data: u32,
    image_base: u64,
    section_alignment: u32,
    file_alignment: u32,
    major_operating_system_version: u16,
    minor_operating_system_version: u16,
    major_image_version: u16,
    minor_image_version: u16,
    major_subsystem_version: u16,
    minor_subsystem_version: u16,
    win32_version_value: u32,
    size_of_image: u32,
    size_of_headers: u32,
    check_sum: u32,
    subsystem: u16,
    dll_characteristics: u16,
    size_of_stack_reserve: u64,
    size_of_stack_commit: u64,
    size_of_heap_reserve: u64,
    size_of_heap_commit: u64,
    loader_flags: u32,
    number_of_rva_and_sizes: u32,
    data_directory: [IMAGE_DATA_DIRECTORY; 16],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IMAGE_DATA_DIRECTORY {
    pub virtual_address: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IMAGE_NT_HEADERS {
    pub signature: u32,
    pub file_header: IMAGE_FILE_HEADER,
    //#[obj]
    pub optional_header: IMAGE_OPTIONAL_HEADER32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct IMAGE_SECTION_HEADER {
    pub name: [u8; 8],
    pub virtual_size: u32,
    pub virtual_address: u32,  // 0x1000 aligned
    pub size_of_raw_data: u32, // 0x200 aligned
    pub pointer_to_raw_data: u32,
    pub pointer_to_relocations: u32,
    pub pointer_to_line_numbers: u32,
    pub number_of_relocations: u16,
    pub number_of_line_numbers: u16,
    pub characteristics: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SectionCharaceristics {
    type_no_pad: bool,
    cnt_code: bool,
    cnt_initialized_data: bool,
    cnt_uninitialized_data: bool,
    lnk_other: bool,
    lnk_info: bool,
    lnk_remove: bool,
    lnk_comdat: bool,
    gprel: bool,
    mem_16bit: bool,
    mem_locked: bool,
    mem_preload: bool,
    align_1bytes: bool,
    align_2bytes: bool,
    align_8bytes: bool,
    align_128bytes: bool,
    lnk_nreloc_ovfl: bool,
    mem_discardable: bool,
    mem_not_cached: bool,
    mem_not_paged: bool,
    mem_shared: bool,
    mem_execute: bool,
    mem_read: bool,
    mem_write: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMAGE_IMPORT_DESCRIPTOR {
    pub original_first_thunk: u32,
    pub time_date_stamp: u32,
    pub forwarder_chain: u32,
    pub name: u32,
    pub first_thunk: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMAGE_FUNCTION_ENTRY64 {
    pub starting_address: u32,
    pub ending_address: u32,
    pub unwind_info_address: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UNWIND_INFO {
    pub version_and_flag: u8,
    pub size_of_prolog: u8,
    pub count_of_unwind_codes: u8,
    pub frame_register_and_frame_register_offset: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMAGE_EXPORT_DIRECTORY {
    pub blank: u32,
    pub time_data_stamp: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub name: u32,
    pub base: u32,
    pub number_of_functions: u32,
    pub number_of_names: u32,
    pub address_of_functions: u32,
    pub address_of_names: u32,
    pub address_of_name_ordinals: u32,
}
