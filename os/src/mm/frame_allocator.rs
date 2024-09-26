use super::address::PhysPageNum;
use super::address::PhysAddr;
use alloc::vec::Vec;
use crate::{config::PHYSTOP, sync::spinlock::Mutex};
use super::{phys_page_round_down,phys_page_round_up};
use lazy_static::*;
pub struct PhysFrame{pub ppn:PhysPageNum}

impl PhysFrame{
    pub fn new(ppn:PhysPageNum)->Self{
        let pa = PhysAddr::from(ppn);
        let bytes_array:&mut[u8];//将当前页清零
        unsafe {
            bytes_array = core::slice::from_raw_parts_mut(pa.0 as *mut u8,4096);
        }
        for i in bytes_array {
            *i = 0;
        }
        Self{
            ppn:ppn
        }
    }
    pub fn new_nonzero(ppn:PhysPageNum)->Self{
        Self{
            ppn:ppn
        }
    }
}

impl Drop for PhysFrame{
    fn drop(&mut self) {
        dealloc_frame(self.ppn);
    }
}

pub struct LinkedFrameAllocator{
    current:usize,
    end:usize,
    recycle:Vec<PhysPageNum>
}

lazy_static!{
    pub static ref FRAME_ALLOCATOR:Mutex<LinkedFrameAllocator> = Mutex::new(LinkedFrameAllocator::new());
}

impl LinkedFrameAllocator{
    pub const fn new()->Self{
        Self{
            current:0,
            end:0,
            recycle:Vec::new()
        }
    }
    fn init(&mut self){
        extern "C"{
            fn ekernel();
        }
        self.current = phys_page_round_up(ekernel as usize);
        self.end = phys_page_round_down(PHYSTOP);
        println!("[kernel]frame allocator current:{:x}",self.current);
        println!("[kernel]frame allocator end:{:x}",self.end);
    }
    fn alloc(&mut self)->PhysPageNum{//todo:优化alloc
          if self.current < self.end {
            self.current+=1;
            return PhysPageNum((self.current-1).into());
          }
          else{
            if let Some(ppn)= self.recycle.pop(){
                println!("alloc 0x{:x}",ppn.0);
                return ppn
            }
            else{
                panic!("Out of PhysMem");
            }
          }
    }
    fn dealloc(&mut self,ppn:PhysPageNum){
        let phys_page_num:usize = ppn.into();
        if phys_page_num >= self.current{
            panic!("FrameAllocator::dealloc Illegel PPN");
        }
        self.recycle.push(ppn);
    }
}

pub fn frame_allocator_init(){
    let mut allocator = FRAME_ALLOCATOR.lock();
    allocator.init();
}

pub fn alloc_frame()->PhysFrame{
    let mut allocator = FRAME_ALLOCATOR.lock();
    let ret = allocator.alloc();
    PhysFrame::new(ret)
}

pub fn dealloc_frame(ppn:PhysPageNum){
    let mut allocator = FRAME_ALLOCATOR.lock();
    allocator.dealloc(ppn);
}