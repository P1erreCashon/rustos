use super::pagetable::{self, PTEFlags, PageTable};
use super::{alloc_frame, PhysPageNum, VirtPageNum};
use alloc::collections::btree_map::BTreeMap;
use alloc::vec::Vec;
use super::frame_allocator::PhysFrame;
use bitflags::*;
use lazy_static::lazy_static;
use crate::sync::spinlock::Mutex;
use crate::config::PHYSTOP;
use riscv::register::satp;
use core::arch::asm;


lazy_static!{
    pub static ref KERNEL_MEMSPACE:Mutex<MemSpace> = Mutex::new(MemSpace::new());
}

enum MapType{
    Identical,
    Framed
}
bitflags! {
    /// map permission corresponding to that in pte: `R W X U`
    pub struct MapPermission: u8 {
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
    }
}
struct MapArea{
    vpn_range:(VirtPageNum,VirtPageNum), //[ )左闭右开 
    frames:BTreeMap<VirtPageNum,PhysFrame>,
    map_type:MapType,
    map_perm:MapPermission
}

pub struct MemSpace{
    pagetable:PageTable,
    map_areas:Vec<MapArea>,
}

impl MapArea{
    pub fn new(svpn:VirtPageNum,evpn:VirtPageNum,map_type:MapType,map_perm:MapPermission)->Self{  
        Self{
            vpn_range:(svpn,evpn),
            frames:BTreeMap::new(),
            map_type,
            map_perm
        }
    }
    fn map_one(&mut self,pagetable:&mut PageTable,vpn:VirtPageNum){//map one page
        if vpn.0< self.vpn_range.0.0 || vpn.0>=self.vpn_range.1.0{
            panic!("mam_space::MapArea::map_one vpn out of bound");
        }
        match self.map_type{
            MapType::Identical=>{
                let ppn = PhysPageNum::from(vpn.0);
                pagetable.map(vpn,ppn,PTEFlags::from_bits(self.map_perm.bits).unwrap() );
                self.frames.insert(vpn, PhysFrame::new_nonzero(ppn));
            }
            MapType::Framed=>{
                let frame = alloc_frame();
                pagetable.map(vpn,frame.ppn, PTEFlags::from_bits(self.map_perm.bits).unwrap());
                self.frames.insert(vpn, frame);
            }
        }
    }
    fn map(&mut self,pagetable:&mut PageTable){//map all vpn
 //       println!("mapping 0x{:x} 0x{:x}",self.vpn_range.0.0,self.vpn_range.1.0);
        for i in self.vpn_range.0.0..self.vpn_range.1.0{
 //           println!("mapping i 0x{:x}",i);
            self.map_one(pagetable, VirtPageNum::from(i));
        }
    }
}

impl MemSpace{
    pub fn new()->Self{
        Self{
            pagetable:PageTable::new(),
            map_areas:Vec::new()
        }
    }
    fn insert(&mut self,mut map_area:MapArea){//map and insert
        map_area.map(&mut self.pagetable);
        self.map_areas.push(map_area);
    }
    fn init_kernel_memspace(&mut self){
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
            fn ekernel();
        }
        self.pagetable.init();
        println!("[kernel]memspace init");
     //        println!("stext {:x} etext {:x}",VirtPageNum::from((stext as usize)>>12).0,VirtPageNum::from(((etext as usize)>>12)+1).0);
      //      println!("srodata {:x} erodata {:x}",phys_page_round_down(srodata as usize),phys_page_round_down(erodata as usize -1));
     //       println!("sdata {:x} edata {:x}",phys_page_round_down(sdata as usize),phys_page_round_down(edata as usize -1));
     //       println!("boot_stack {:x} boot_stack_top {:x}",phys_page_round_down(boot_stack as usize),phys_page_round_down(boot_stack_top as usize -1));
     //       println!("sbss {:x} ebss {:x}",phys_page_round_down(sbss as usize),phys_page_round_down(ebss as usize -1));
     //       println!("ekernel {:x} PHYSTOP {:x}",phys_page_round_down(ekernel as usize),phys_page_round_down(PHYSTOP as usize -1));
        self.insert(MapArea::new(
            VirtPageNum::from((stext as usize)>>12), 
            VirtPageNum::from(((etext as usize-1)>>12)+1), 
            MapType::Identical, 
            MapPermission::R|MapPermission::X)
        );
        self.insert(MapArea::new(
            VirtPageNum::from((srodata as usize)>>12), 
            VirtPageNum::from(((erodata as usize-1)>>12)+1), 
            MapType::Identical, 
            MapPermission::R)
        );
        self.insert(MapArea::new(
            VirtPageNum::from((sdata as usize)>>12), 
            VirtPageNum::from(((edata as usize-1)>>12)+1), 
            MapType::Identical, 
            MapPermission::R|MapPermission::W)
        );
        self.insert(MapArea::new(
            VirtPageNum::from((boot_stack as usize)>>12), 
            VirtPageNum::from(((boot_stack_top as usize-1)>>12)+1), 
            MapType::Identical, 
            MapPermission::R|MapPermission::W)
        );
        self.insert(MapArea::new(
            VirtPageNum::from((sbss as usize)>>12), 
            VirtPageNum::from(((ebss as usize-1)>>12)+1), 
            MapType::Identical, 
            MapPermission::R|MapPermission::W)
        );
        self.insert(MapArea::new(
            VirtPageNum::from((ekernel as usize)>>12), 
            VirtPageNum::from(((PHYSTOP as usize-1)>>12)+1), 
            MapType::Identical, 
            MapPermission::R|MapPermission::W)
        );
            
    }
}

impl Drop for MapArea{
    fn drop(&mut self) {
        while !self.frames.is_empty() {
            self.frames.pop_first();
        }
    }
}

impl Drop for MemSpace{
    fn drop(&mut self) {
        while !self.map_areas.is_empty(){
            self.map_areas.pop();
        }
    }
}

pub fn kernel_memspace_init(){
    let mut kernel_memspace = KERNEL_MEMSPACE.lock();
    kernel_memspace.init_kernel_memspace();
}
pub fn activate_vm(){
    let mut kernel_memspace = KERNEL_MEMSPACE.lock();
    let satp = kernel_memspace.pagetable.token();
    unsafe {
        satp::write(satp);
        asm!("sfence.vma");
    }
}