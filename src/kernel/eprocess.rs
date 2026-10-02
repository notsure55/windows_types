use super::*;

extern crate alloc;
use super::result::Result;
use crate::check_status;

use alloc::{vec, vec::Vec};
use core::convert::From;
use core::ffi::CStr;
use wdk_sys::ntddk::{
    IoGetCurrentProcess, KeStackAttachProcess, KeUnstackDetachProcess, MmCopyMemory,
};

#[repr(C)]
pub struct Eprocess {
    pub raw: *mut EPROCESS,
    pub kapc_state: Option<_KAPC_STATE>,
}

impl core::ops::Deref for Eprocess {
    type Target = EPROCESS;

    fn deref(&self) -> &Self::Target {
        if let Some(raw) = unsafe { self.raw.as_ref() } {
            raw
        } else {
            panic!(
                "Pointer was null when trying to auto deref {:?}",
                self.image_name()
            );
        }
    }
}

impl core::ops::DerefMut for Eprocess {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if let Some(raw) = unsafe { self.raw.as_mut() } {
            raw
        } else {
            panic!(
                "Pointer was null when trying to auto derefmut {:?}",
                self.image_name()
            );
        }
    }
}

impl Eprocess {
    pub fn from_current() -> Self {
        let raw = unsafe { IoGetCurrentProcess() }.cast::<EPROCESS>();
        Self {
            raw,
            kapc_state: None,
        }
    }
    pub fn from_raw(raw: *mut EPROCESS) -> Self {
        Self {
            raw,
            kapc_state: None,
        }
    }
    pub fn get_peb(&self) -> Option<&peb::PEB> {
        if !self.is_attached() {
            return None;
        }

        unsafe { self.peb.as_ref() }
    }
    pub fn is_attached(&self) -> bool {
        self.raw == unsafe { IoGetCurrentProcess().cast::<EPROCESS>() }
    }
    pub fn attach(&mut self) {
        let mut kapc_state = _KAPC_STATE::default();

        unsafe { KeStackAttachProcess(self.raw.cast::<_KPROCESS>(), &mut kapc_state) };

        self.kapc_state = Some(kapc_state);
    }
    pub fn detach(&mut self) {
        if let Some(mut kapc_state) = unsafe { self.kapc_state.take() } {
            unsafe { KeUnstackDetachProcess(&mut kapc_state) };
        }
    }
    pub fn image_name<'a>(&'a self) -> Option<&'a str> {
        Some(
            CStr::from_bytes_until_nul(&self.image_file_name)
                .ok()?
                .to_str()
                .ok()?,
        )
    }
    pub fn next_process(&self) -> Self {
        Self::from_raw(unsafe {
            self.pcb
                .process_list_entry
                .Flink
                .cast::<*mut LIST_ENTRY>()
                .read()
                .byte_offset(-0x110)
                .cast::<EPROCESS>()
        })
    }
    pub fn read_virtual_memory(&mut self, va: impl Into<PVOID>, size: usize) -> Result<Vec<u8>> {
        if !self.is_attached() {
            self.attach();
        };

        let address = va.into();
        let mut buffer = vec![0u8; size];
        let mut number_of_bytes: u64 = Default::default();

        check_status!(unsafe {
            MmCopyMemory(
                buffer.as_ptr() as _,
                core::mem::transmute::<_, MM_COPY_ADDRESS>(address),
                size as _,
                MM_COPY_MEMORY_VIRTUAL,
                &mut number_of_bytes,
            )
        });

        self.detach();

        Result::Ok(buffer)
    }
    pub fn write_virtual_memory(
        &mut self,
        va: impl Into<PVOID>,
        size: usize,
        buffer: *const u8,
    ) -> Result<u64> {
        if !self.is_attached() {
            self.attach();
        };

        let address = va.into();
        let mut number_of_bytes: u64 = Default::default();

        check_status!(unsafe {
            MmCopyMemory(
                address,
                core::mem::transmute::<_, MM_COPY_ADDRESS>(buffer),
                size as _,
                MM_COPY_MEMORY_VIRTUAL,
                &mut number_of_bytes,
            )
        });

        self.detach();

        Result::Ok(number_of_bytes)
    }
}

#[repr(C, align(8))]
pub struct KPROCESS {
    pub header: _DISPATCHER_HEADER,
    pub profile_list_head: _LIST_ENTRY,
    pub directory_table_base: Uint8B,
    pub thread_list_head: _LIST_ENTRY,
    pub process_lock: Uint4B,
    pub process_timer_delay: Uint4B,
    pub deep_freeze_start_time: Uint8B,
    pub affinity: *mut _KAFFINITY_EX,
    pub auto_boost_state: _KAB_UM_PROCESS_CONTEXT,
    pub ready_list_head: _LIST_ENTRY,
    pub swap_list_entry: _SINGLE_LIST_ENTRY,
    pub active_processors: *mut _KAFFINITY_EX,
    pub process_flags: Int4B,
    pub spare0c: Uint4B,
    pub base_priority: Char,
    pub quantum_reset: Char,
    pub visited: Char,
    pub flags: _KEXECUTE_OPTIONS,
    pub active_groups_mask: _KGROUP_MASK,
    pub active_group_padding: [Uint8B; 2],
    pub ideal_processor_assignment_block: *mut _KI_IDEAL_PROCESSOR_ASSIGNMENT_BLOCK,
    pub padding: [Uint8B; 6],
    pub padding2: Uint4B,
    pub scheduler_assist_yield_boost_count: Uint4B,
    pub scheduler_assist_yield_boost_allowed_time: Int8B,
    pub spare0d: Uint4B,
    pub ideal_global_node: Uint2B,
    pub spare1: Uint2B,
    pub stack_count: _KSTACK_COUNT,
    pub process_list_entry: _LIST_ENTRY,
    pub cycle_time: Uint8B,
    pub context_switches: Uint8B,
    pub scheduling_group: *mut _KSCHEDULING_GROUP,
    pub kernel_time: Uint8B,
    pub user_time: Uint8B,
    pub ready_time: Uint8B,
    pub freeze_count: Uint4B,
    pub spare4: Uint4B,
    pub user_directory_table_base: Uint8B,
    pub address_policy: UChar,
    pub spare2: [UChar; 7],
    pub instrumentation_callback: *mut Void,
    pub secure_state: Void,
    pub kernel_wait_time: Uint8B,
    pub user_wait_time: Uint8B,
    pub last_rebalance_qpc: Uint8B,
    pub per_processor_cycle_times: *mut Void,
    pub extended_feature_disable_mask: Uint8B,
    pub primary_group: Uint2B,
    pub spare3: [Uint2B; 3],
    pub user_cet_logging: *mut Void,
    pub cpu_partition_list: _LIST_ENTRY,
    pub available_cpu_state: *mut _KPROCESS_AVAILABLE_CPU_STATE,
}

#[repr(C, align(8))]
pub struct EPROCESS {
    pub pcb: KPROCESS,
    pub padding: [u8; 0x118],
    /*pub process_lock: _EX_PUSH_LOCK,
    pub unique_process_id: *mut Void,
    pub active_process_links: _LIST_ENTRY,
    pub rundown_protect: _EX_RUNDOWN_REF,
    pub flags2: Uint4B,
    pub flags: Uint4B,
    pub create_time: _LARGE_INTEGER,
    pub process_quota_usage: [Uint8B; 2],
    pub process_quota_peak: [Uint8B; 2],
    pub peak_virtual_size: Uint8B,
    pub virtual_size: Uint8B,
    pub session_process_links: _LIST_ENTRY,
    pub exception_port_data: *mut Void,
    pub exception_port_value: Uint8B,
    pub token: _EX_FAST_REF,
    pub mm_reserved: Uint8B,
    pub address_creation_lock: _EX_PUSH_LOCK,
    pub page_table_commitment_lock: _EX_PUSH_LOCK,
    pub rotate_in_progress: *mut _ETHREAD,
    pub fork_in_progress: *mut _ETHREAD,
    pub commit_charge_job: *mut _EJOB,
    pub clone_root: _RTL_AVL_TREE,
    pub number_of_private_pages: Uint8B,
    pub mm_reserved2: Uint8B,
    pub win32_process: *mut Void,
    pub job: *mut _EJOB,
    pub section_object: *mut Void,
    pub section_base_address: *mut Void,
    pub cookie: Uint4B,
    pub working_set_watch: *mut _PAGEFAULT_HISTORY,
    pub win32_window_station: *mut Void,
    pub inherited_from_unique_process_id: *mut Void,
    pub owner_process_id: Uint8B,*/
    pub peb: *mut peb::PEB,
    pub session: *mut _PSP_SESSION_SPACE,
    pub spare1: *mut Void,
    pub quota_block: *mut _EPROCESS_QUOTA_BLOCK,
    pub object_table: *mut _HANDLE_TABLE,
    pub debug_port: *mut Void,
    pub wow64_process: *mut _EWOW64PROCESS,
    pub device_map: _EX_FAST_REF,
    pub etw_data_source: *mut Void,
    pub page_directory_pte: Uint8B,
    pub image_file_pointer: *mut _FILE_OBJECT,
    pub image_file_name: [UChar; 15],
    pub priority_class: UChar,
    pub security_port: *mut Void,
    pub se_audit_process_creation_info: _SE_AUDIT_PROCESS_CREATION_INFO,
    pub job_links: _LIST_ENTRY,
    pub highest_user_address: *mut Void,
    pub thread_list_head: _LIST_ENTRY,
    pub active_threads: Uint4B,
    pub image_path_hash: Uint4B,
    pub default_hard_error_processing: Uint4B,
    pub last_thread_exit_status: Int4B,
    pub prefetch_trace: _EX_FAST_REF,
    pub locked_pages_list: *mut Void,
    pub read_operation_count: _LARGE_INTEGER,
    pub write_operation_count: _LARGE_INTEGER,
    pub other_operation_count: _LARGE_INTEGER,
    pub read_transfer_count: _LARGE_INTEGER,
    pub write_transfer_count: _LARGE_INTEGER,
    pub other_transfer_count: _LARGE_INTEGER,
    pub commit_charge_limit: Uint8B,
    pub commit_charge: Uint8B,
    pub commit_charge_peak: Uint8B,
    pub vm: _MMSUPPORT_FULL,
    pub mm_process_links: _LIST_ENTRY,
    pub modified_page_count: Uint4B,
    pub exit_status: Int4B,
    pub vad_root: _RTL_AVL_TREE,
    pub vad_hint: *mut Void,
    pub vad_count: Uint8B,
    pub vad_physical_pages: Uint8B,
    pub vad_physical_pages_limit: Uint8B,
    pub alpc_context: _ALPC_PROCESS_CONTEXT,
    pub timer_resolution_link: _LIST_ENTRY,
    pub timer_resolution_stack_record: *mut _PO_DIAG_STACK_RECORD,
    pub requested_timer_resolution: Uint4B,
    pub smallest_timer_resolution: Uint4B,
    pub exit_time: _LARGE_INTEGER,
    pub inverted_function_table: *mut _INVERTED_FUNCTION_TABLE_KERNEL_MODE,
    pub inverted_function_table_lock: _EX_PUSH_LOCK,
    pub active_threads_high_watermark: Uint4B,
    pub large_private_vad_count: Uint4B,
    pub thread_list_lock: _EX_PUSH_LOCK,
    pub wnf_context: *mut Void,
    pub server_silo: *mut _EJOB,
    pub signature_level: UChar,
    pub section_signature_level: UChar,
    pub protection: _PS_PROTECTION,
    pub syscall_usage: Uint4B,
    pub syscall_usage_values: Uint4B,
    pub supervisor_device_asid: Int4B,
    pub supervisor_svm_data: *mut Void,
    pub network_counters: *mut _PROCESS_NETWORK_COUNTERS,
    pub execution: _PROCESS_EXECUTION,
    pub thread_index_table: *mut Void,
    pub freeze_work_links: _LIST_ENTRY,
    pub ppm_policy_lock: _EX_PUSH_LOCK,
    pub bam_ppm_policy: Uint4B,
}
