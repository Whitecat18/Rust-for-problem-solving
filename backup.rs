use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use winapi::shared::ntdef::HANDLE;
use winapi::shared::ntdef::OBJECT_ATTRIBUTES;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::minwinbase::SECURITY_ATTRIBUTES;
use ntapi::ntioapi::IO_STATUS_BLOCK;
use winapi::shared::ntdef::NTSTATUS;
use winapi::shared::ntdef::PVOID;
use winapi::shared::ntdef::UNICODE_STRING;

#[link(name = "ntdll")]
extern "system" {
    fn NtOpenFile(
        FileHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut SECURITY_ATTRIBUTES,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        ShareAccess: u32,
        OpenOptions: u32,
    ) -> NTSTATUS;
    fn NtMapViewOfSection(
        SectionHandle: HANDLE,
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        ZeroBits: *mut usize,
        CommitSize: usize,
        SectionOffset: *mut winapi::um::winnt::LARGE_INTEGER,
        ViewSize: *mut usize,
        InheritDisposition: u32,
        AllocationType: u32,
        Protect: u32,
    ) -> NTSTATUS;
    fn NtCreateSection(
        SectionHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut SECURITY_ATTRIBUTES,
        MaximumSize: *mut winapi::um::winnt::LARGE_INTEGER,
        SectionPageProtection: u32,
        AllocationAttributes: u32,
        FileHandle: HANDLE,
    ) -> NTSTATUS;
    fn RtlInitUnicodeString(
        DestinationString: *mut UNICODE_STRING,
        SourceString: *const u16,
    );
}

fn spoof_image_loading(){
    let mut h_file:HANDLE = INVALID_HANDLE_VALUE;
    let mut section_size: usize = 0;
    let module_name_path = OsStr::new("\\??\\C:\\windows\\system32\\advapi32.dll")
        .encode_wide().chain(Some(0).into_iter())
        .collect::<Vec<u16>>();

    let module_name = UNICODE_STRING{
        Length: (module_name_path.len() * 2) as u16,
        MaximumLength: (module_name_path.len() * 2) as u16,
        Buffer: module_name_path.as_ptr() as *mut u16,
    };

    let mut object_attributes: OBJECT_ATTRIBUTES = unsafe{ std::mem::zeroed()};
    object_attributes.Length = std::mem::size_of::<OBJECT_ATTRIBUTES>() as u32;
    let mut io_status_block = IO_STATUS_BLOCK{
        Information: 0,
        u: ntapi::ntioapi::IO_STATUS_BLOCK_u { Status: 0 },
    };

    unsafe{
        RtlInitUnicodeString(&mut object_attributes.lpSecurityDescriptor, module_name.Buffer);
    }

}
fn main(){

}
