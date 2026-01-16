use bytemuck::Pod;
use std::mem;
use std::sync::{Arc, Mutex};
use windows::Win32::Foundation::*;
use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;
use windows::Win32::System::Diagnostics::ToolHelp::*;
use windows::Win32::System::ProcessStatus::*;
use windows::Win32::System::Threading::*;

use crate::discord::client::DiscordRPC;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum StringEncoding {
    Utf8,
    Utf16,
}

pub struct Application {
    pub handle: HANDLE,
    pub base_address: u64,
    pub is_running: bool,
    pub process_name: String,
    pub rpc: DiscordRPC,
}

impl Application {
    pub fn new(process_name: &str) -> Option<Self> {
        let pid = match Self::find_process_id(process_name) {
            Some(pid) => {
                eprintln!("Found {} with PID: {}", process_name, pid);
                pid
            }
            None => {
                eprintln!("Process {} not found in process list", process_name);
                return None;
            }
        };

        let handle = match unsafe { OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, false, pid) } {
            Ok(h) => {
                eprintln!("Successfully opened process handle");
                h
            }
            Err(e) => {
                eprintln!("Failed to open process: {} (try running as admin)", e);
                return None;
            }
        };

        let base_address = match Self::get_module_base(handle) {
            Some(addr) => {
                eprintln!("Got base address: 0x{:X}", addr);
                addr
            }
            None => {
                eprintln!("Failed to get module base address");
                return None;
            }
        };

        let rpc = DiscordRPC::new();

        Some(Self {
            handle,
            base_address,
            is_running: true,
            process_name: process_name.to_string(),
            rpc,
        })
    }

    pub fn check_running(&mut self) -> bool {
        if !self.is_running {
            return false;
        }

        let mut exit_code: u32 = 0;
        let success = unsafe { GetExitCodeProcess(self.handle, &mut exit_code) }.is_ok();

        if success && exit_code == STILL_ACTIVE.0 as u32 {
            true
        } else {
            self.is_running = false;
            false
        }
    }

    fn find_process_id(process_name: &str) -> Option<u32> {
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).ok()?;
            let mut entry = PROCESSENTRY32 {
                dwSize: std::mem::size_of::<PROCESSENTRY32>() as u32,
                ..Default::default()
            };

            Process32First(snapshot, &mut entry).ok()?;

            let target = process_name.to_lowercase();

            loop {
                let exe_bytes: Vec<u8> = entry
                    .szExeFile
                    .iter()
                    .take_while(|&&c| c != 0)
                    .map(|&c| c as u8)
                    .collect();

                if let Ok(name) = String::from_utf8(exe_bytes) {
                    if name.to_lowercase() == target || name.to_lowercase().ends_with(&target) {
                        eprintln!("Found process: {} (PID: {})", name, entry.th32ProcessID);
                        return Some(entry.th32ProcessID);
                    }
                }

                if Process32Next(snapshot, &mut entry).is_err() {
                    break;
                }
            }
        }
        None
    }

    fn get_module_base(handle: HANDLE) -> Option<u64> {
        unsafe {
            let mut modules = [HMODULE(std::ptr::null_mut()); 1024];
            let mut needed = 0u32;

            EnumProcessModules(
                handle,
                modules.as_mut_ptr(),
                (modules.len() * mem::size_of::<HMODULE>()) as u32,
                &mut needed,
            )
            .ok()?;

            if needed == 0 {
                return None;
            }

            Some(modules[0].0 as u64)
        }
    }

    fn resolve(&mut self, base_offset: u64, offsets: &[u64]) -> Option<u64> {
        if !self.is_running {
            return None;
        }

        let mut address = self.base_address + base_offset;

        if offsets.is_empty() {
            return Some(address);
        }

        for &offset in &offsets[..offsets.len().saturating_sub(1)] {
            address = self.rpm::<u64>(address + offset)?;
        }

        address += offsets.last().copied().unwrap_or(0);
        Some(address)
    }

    fn rpm<T: Pod>(&mut self, address: u64) -> Option<T> {
        if !self.is_running {
            return None;
        }

        let mut value = T::zeroed();
        let size = mem::size_of::<T>();

        let success = unsafe {
            ReadProcessMemory(
                self.handle,
                address as *const _,
                &mut value as *mut _ as *mut _,
                size,
                None,
            )
            .is_ok()
        };

        if success {
            Some(value)
        } else {
            None
        }
    }

    pub fn read<T: Pod>(&mut self, base_offset: u64, offsets: &[u64]) -> Option<T> {
        let address = self.resolve(base_offset, offsets)?;
        self.rpm(address)
    }

    pub fn read_string(
        &mut self,
        base_offset: u64,
        offsets: &[u64],
        max_length: usize,
        encoding: StringEncoding,
    ) -> Option<String> {
        let address = self.resolve(base_offset, offsets)?;

        match encoding {
            StringEncoding::Utf16 => {
                let mut buffer = vec![0u16; max_length];

                let success = unsafe {
                    ReadProcessMemory(
                        self.handle,
                        address as *const _,
                        buffer.as_mut_ptr() as *mut _,
                        max_length * 2, // UTF-16 = 2 bytes per char
                        None,
                    )
                    .is_ok()
                };

                if !success {
                    return None;
                }

                let len = buffer.iter().position(|&c| c == 0).unwrap_or(max_length);
                String::from_utf16(&buffer[..len]).ok()
            }
            StringEncoding::Utf8 => {
                let mut buffer = vec![0u8; max_length];

                let success = unsafe {
                    ReadProcessMemory(
                        self.handle,
                        address as *const _,
                        buffer.as_mut_ptr() as *mut _,
                        max_length,
                        None,
                    )
                    .is_ok()
                };

                if !success {
                    return None;
                }

                let len = buffer.iter().position(|&b| b == 0).unwrap_or(max_length);
                String::from_utf8(buffer[..len].to_vec()).ok()
            }
        }
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        if !self.handle.is_invalid() {
            unsafe {
                let _ = CloseHandle(self.handle);
            }
        }
    }
}

unsafe impl Send for Application {}
unsafe impl Sync for Application {}

pub type AppState = Arc<Mutex<Option<Application>>>;
