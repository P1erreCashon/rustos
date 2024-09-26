

mod address;
mod frame_allocator;
mod pagetable;
mod heap_allocator;
mod mem_space;
pub use address::{phys_page_round_down,phys_page_round_up};
pub use frame_allocator::{alloc_frame,dealloc_frame,frame_allocator_init};
pub use heap_allocator::init_heap;
use mem_space::{kernel_memspace_init,activate_vm};
pub use address::{PhysAddr,VirtAddr,PhysPageNum,VirtPageNum};


pub fn vm_init(){
    init_heap();
    frame_allocator_init();
    kernel_memspace_init();
    activate_vm();
}