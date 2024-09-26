use bitflags::*;
use alloc::vec::Vec;
use super::{address::{PhysAddr, PhysPageNum, VirtPageNum}, alloc_frame, frame_allocator::PhysFrame};


bitflags! {
    pub struct PTEFlags: u8 {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
    }
}

#[derive(Clone, Copy)]
struct PageTableEntry{
    pub bits:usize,
}

impl PageTableEntry{
    fn new(ppn:PhysPageNum,flags:PTEFlags)->Self{
        Self { bits: ppn.0<<10 | flags.bits as usize }
    }
    pub fn flags(&self)->PTEFlags{
        PTEFlags::from_bits(self.bits as u8).unwrap()
    }
    fn get_ppn(&self)->PhysPageNum{
        PhysPageNum(self.bits>>10)
    }
    pub fn is_valid(&self)->bool{
        (self.flags()&PTEFlags::V)!=PTEFlags::empty()
    }
}

pub struct PageTable{
    page_directory:PhysPageNum,
    frames:Vec<PhysFrame>
}

impl PageTable{
    pub fn new()->Self{
        Self { page_directory: PhysPageNum::from(0), frames: Vec::new() }
    }
    pub fn token(&self) -> usize {
        8usize << 60 | self.page_directory.0
    }
    pub fn init(&mut self){
        let frame = alloc_frame();
        self.page_directory = frame.ppn;
        self.frames.push(frame);
    }
    pub fn map(&mut self,vpn:VirtPageNum,ppn:PhysPageNum,flags:PTEFlags){
        let mut page_directory_pa:PhysAddr = PhysAddr::from(self.page_directory);
        let mut index = 0;
        for j in (0..=2){
            let i = 2-j;
            index = (vpn.0>>(i*9))&((1<<9)-1);
            let pte:&mut PageTableEntry;
            unsafe {
                pte =&mut core::slice::from_raw_parts_mut(page_directory_pa.0 as *mut PageTableEntry,512)[index];
            }
            if i == 0 {
                if pte.is_valid(){
                    println!("panic: vpn:0x{:x} ppn:0x{:x}",vpn.0,ppn.0);
                    panic!("pagetable::PageTable::map remap err");
                }
                *pte = PageTableEntry::new(ppn, PTEFlags::V|flags);
                return;
            }
             if !pte.is_valid(){
                let frame = alloc_frame();
                *pte = PageTableEntry::new(frame.ppn, PTEFlags::V);
                self.frames.push(frame);
            }
            page_directory_pa = PhysAddr::from(pte.get_ppn());
        }
    }
/*     pub fn showppn(&mut self,vpn:VirtPageNum){
        let mut page_directory_pa:PhysAddr = PhysAddr::from(self.page_directory);
        let mut index = 0;
        for j in (0..=2){
            let i = 2-j;
            index = (vpn.0>>(i*9))&((1<<9)-1);
            let pte:&mut PageTableEntry;
            unsafe {
                pte =&mut core::slice::from_raw_parts_mut(page_directory_pa.0 as *mut PageTableEntry,512)[index];
            }
            if i == 0 {
                if !pte.is_valid(){
                    println!("panic: vpn:0x{:x}",vpn.0);
                    panic!("pagetable::PageTable::illegal va err");
                }
                println!("panic: vpn:0x{:x} ppn:0x{:x} PTEFLAGS:{}",vpn.0,pte.get_ppn().0,pte.flags().bits);
                return;
            }
             if !pte.is_valid(){
                panic!("pagetable::PageTable::already va not mapped");
            }
            page_directory_pa = PhysAddr::from(pte.get_ppn());
        }
    } */
    pub fn unmap(&mut self,vpn:VirtPageNum){
        let mut page_directory_pa:PhysAddr = PhysAddr::from(self.page_directory);
        let mut index = 0;
        for i in 2..=0{
            index = vpn.0>>((i*9)&(1<<9-1));
            let pte:&mut PageTableEntry;
            unsafe {
                pte =&mut core::slice::from_raw_parts_mut(page_directory_pa.0 as *mut PageTableEntry,512)[index];
             }
            if i == 0 {
                if !pte.is_valid(){
                    panic!("pagetable::PageTable::map remap err");
                }
                *pte = PageTableEntry::new(pte.get_ppn(), PTEFlags::empty());
                return;
            }
            if !pte.is_valid(){
                panic!("pagetable::PageTable::already unmaped");
            }
            page_directory_pa = PhysAddr::from(pte.get_ppn());
        }
    }
}