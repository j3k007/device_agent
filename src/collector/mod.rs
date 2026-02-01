pub mod common;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "macos")]
pub mod macos;

use crate::models::SystemInfo;

pub fn collect_all_info() -> SystemInfo {
    let mut info = common::collect_basic_info();
    #[cfg(target_os = "windows")]
    {
        info.services = windows::get_windows_services();
        info.installed_software = windows::get_installed_software();
        info.windows_updates = Some(windows::get_windows_updates());
    }

    #[cfg(target_os = "linux")]
    {
        info.services = linux::get_services();
        info.installed_software = linux::get_installed_software();
    }

    #[cfg(target_os = "macos")]
    {
        info.services = macos::get_services();
        info.installed_software = macos::get_installed_application();
    }
    
    info
}