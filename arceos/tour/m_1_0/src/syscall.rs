#![allow(dead_code)]

use axhal::arch::TrapFrame;
use axhal::trap::{register_trap_handler, SYSCALL,PAGE_FAULT};
use axerrno::LinuxError;
use axhal::mem::VirtAddr;
use axhal::paging::MappingFlags;
use axtask::TaskExtRef;

const SYS_EXIT: usize = 93;

#[register_trap_handler(SYSCALL)]
fn handle_syscall(tf: &TrapFrame, syscall_num: usize) -> isize {
    ax_println!("handle_syscall ...");
    let ret = match syscall_num {
        SYS_EXIT => {
            ax_println!("[SYS_EXIT]: system is exiting ..");
            axtask::exit(tf.arg0() as _)
        },
        _ => {
            ax_println!("Unimplemented syscall: {}", syscall_num);
            -LinuxError::ENOSYS.code() as _
        }
    };
    ret
}

#[register_trap_handler(PAGE_FAULT)]
fn handle_page_fault(vr:VirtAddr,mf:MappingFlags,user:bool)->bool{
    ax_println!("handle_page_fault");
    if user{
        if !axtask::current().task_ext().aspace.lock().handle_page_fault(vr,mf){
            axtask::exit(-1);
        }else {
            ax_println!("handle_page_fault ok");
        }
        true
    }else {
        false
    }
}
