use defer_lite::defer;
use widestring::U16CString;
use windows_sys::Win32::{
    Foundation::{CloseHandle, GetLastError, LUID},
    Security::{
        AdjustTokenPrivileges, LookupPrivilegeValueW, SE_PRIVILEGE_ENABLED,
        TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES, TOKEN_QUERY,
    },
    System::Threading::{GetCurrentProcess, OpenProcessToken},
};

#[inline(always)]
pub fn get_last_error() -> u32 {
    unsafe { GetLastError() }
}
pub fn set_privilege(name: String) -> Result<(), String> {
    unsafe {
        let mut token_handle = 0;

        if OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
            &mut token_handle,
        ) == 0
        {
            return Err("OpenProcessToken failed".to_string());
        }

        defer! {CloseHandle(token_handle);}

        let mut lookup_id = std::mem::MaybeUninit::<LUID>::zeroed();

        let name = U16CString::from_str(name).unwrap();

        if LookupPrivilegeValueW(std::ptr::null(), name.as_ptr(), lookup_id.as_mut_ptr()) == 0 {
            return Err("LookupPrivilegeValueA failed".to_string());
        }

        let mut token_priv = std::mem::MaybeUninit::<TOKEN_PRIVILEGES>::zeroed();
        let token_priv_ptr = token_priv.as_mut_ptr();

        (*token_priv_ptr).PrivilegeCount = 1;
        (*token_priv_ptr).Privileges[0].Luid = lookup_id.assume_init();
        (*token_priv_ptr).Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;

        if AdjustTokenPrivileges(
            token_handle,
            0,
            token_priv.as_ptr(),
            std::mem::size_of::<TOKEN_PRIVILEGES>() as u32,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        ) == 0
        {
            return Err("AdjustTokenPrivileges failed".to_string());
        }
    }

    Ok(())
}
