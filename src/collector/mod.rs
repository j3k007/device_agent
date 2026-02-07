pub mod common;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "macos")]
pub mod macos;

use crate::models::SystemInfo;
use crate::config::Config;

pub fn collect_all_info(config: &Config) -> SystemInfo {
    let mut info = common::collect_basic_info(config);

    // Collect services if enabled
    if config.collection.include_services {
        #[cfg(target_os = "windows")]
        {
            info.services = windows::get_services();  // ✅ Consistent naming
        }

        #[cfg(target_os = "linux")]
        {
            info.services = linux::get_services();  // ✅ Consistent naming
        }

        #[cfg(target_os = "macos")]
        {
            info.services = macos::get_services();  // ✅ Consistent naming
        }
    }

    // Collect software if enabled
    if config.collection.include_software {
        #[cfg(target_os = "windows")]
        {
            info.installed_software = windows::get_installed_software();
        }

        #[cfg(target_os = "linux")]
        {
            info.installed_software = linux::get_installed_software();
        }

        #[cfg(target_os = "macos")]
        {
            info.installed_software = macos::get_installed_software();
        }
    }
    
    info
}