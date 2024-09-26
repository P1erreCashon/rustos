
const PA_WIDTH_SV39: usize = 56;
const VA_WIDTH_SV39: usize = 39;
const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - 12;
const VPN_WIDTH_SV39: usize = VA_WIDTH_SV39 - 12;

#[derive(Clone, Copy)]
pub struct PhysAddr(pub usize);

#[derive(Clone,Copy)]
pub struct PhysPageNum(pub usize);

#[derive(Clone,Copy)]
pub struct VirtAddr(pub usize);

#[derive(Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
pub struct VirtPageNum(pub usize);


//From all for usize
impl From<PhysAddr> for usize{
    fn from(value: PhysAddr) -> Self {
        value.0
    }
}
impl From<PhysPageNum> for usize{
    fn from(value: PhysPageNum) -> Self {
        value.0
    }
}
impl From<VirtAddr> for usize{
    fn from(value: VirtAddr) -> Self {
        value.0
    }
}
impl From<VirtPageNum> for usize{
    fn from(value: VirtPageNum) -> Self {
        value.0
    }
}

//From usize for all
impl From<usize> for PhysAddr {
    fn from(v: usize) -> Self {
        Self(v & ((1 << PA_WIDTH_SV39) - 1))
    }
}
impl From<usize> for PhysPageNum {
    fn from(v: usize) -> Self {
        Self(v & ((1 << PPN_WIDTH_SV39) - 1))
    }
}
impl From<usize> for VirtAddr {
    fn from(v: usize) -> Self {
        Self(v & ((1 << VA_WIDTH_SV39) - 1))
    }
}
impl From<usize> for VirtPageNum {
    fn from(v: usize) -> Self {
        Self(v & ((1 << VPN_WIDTH_SV39) - 1))
    }
}


//VPN <-> VA
impl From<VirtAddr> for VirtPageNum{
    fn from(value: VirtAddr) -> Self {
        VirtPageNum(value.0>>12)
    }
}

//PPN <-> PA
impl From<PhysAddr> for PhysPageNum{
    fn from(value: PhysAddr) -> Self {
        PhysPageNum(value.0>>12)
    }
}
impl From<PhysPageNum> for PhysAddr{
    fn from(value: PhysPageNum) -> Self {
        PhysAddr(value.0<<12)
    }
}

pub fn phys_page_round_up(pa:usize)->usize{
    (pa>>12)+1
}

pub fn phys_page_round_down(pa:usize)->usize{
    pa>>12
}

impl PhysPageNum{
    pub fn get_bytes_array(&self) -> &'static mut [u8] {
        let pa: PhysAddr = (*self).into();
        unsafe { core::slice::from_raw_parts_mut(pa.0 as *mut u8, 4096) }
    }
}