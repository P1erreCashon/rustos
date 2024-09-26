#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::panic::PanicInfo;
use core::arch::global_asm;
use core::ptr::write_volatile;

#[macro_use]
mod console;
mod lang_items;
mod sync;
mod sbi;
mod config;
mod mm;
global_asm!(include_str!("entry.asm"));

use mm::alloc_frame;
use mm::vm_init;

#[inline(never)]
#[no_mangle]
fn rust_main()->! {
    println!("[kernel]Hello world");
    extern "C"{
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn boot_stack();
        fn boot_stack_top();
        fn sbss();
        fn ebss();
        fn skernel();
        fn ekernel();
    }
    println!("[kernel]skernel = {:x}",skernel as usize);
    println!("[kernel]stext = {:x}",stext as usize);
    println!("[kernel]etext= {:x}",etext as usize);
    println!("[kernel]srodata = {:x}",srodata as usize);
    println!("[kernel]erodata = {:x}",erodata as usize);
    println!("[kernel]sdata = {:x}",sdata as usize);
    println!("[kernel]edata = {:x}",edata as usize);
    println!("[kernel]boot_stack_bottom = {:x}",boot_stack as usize);
    println!("[kernel]boot_stack_top = {:x}",boot_stack_top as usize);
    println!("[kernel]sbss = {:x}",sbss as usize);
    println!("[kernel]ebss = {:x}",ebss as usize);
    println!("[kernel]ekernel = {:x}",ekernel as usize);
    (sbss as usize..ebss as usize).for_each(|x|{
        unsafe {
            write_volatile(x as *mut u8, 0);
        }
        
    });
    vm_init();
    println!("111");
    println!("222");
/*     loop{
        let p1 = alloc_frame();
        println!("p1:0x{:x}",p1.ppn.0);
        if p1.ppn.0 == 0x88000-1{
            break;
        }
    }
    let p1 = alloc_frame();
    println!("p1:0x{:x}",p1.ppn.0);
    let p2 = alloc_frame();
    println!("p2:0x{:x}",p2.ppn.0); */
    sbi::shutdown(false);
}
