use crate::consts::KERNEL_OFFSET;
use bitmap_allocator::BitAlloc;
// Depends on kernel
use super::{BootInfo, MemoryRegionType};
use crate::memory::{active_table, init_heap, FRAME_ALLOCATOR};
use log::*;
use rcore_memory::paging::*;
use rcore_memory::PAGE_SIZE;

pub fn init(boot_info: &BootInfo) {
    init_frame_allocator(boot_info);
    init_device_vm_map();
    init_heap();
    info!("memory: init end");
}

/// Init FrameAllocator and insert all 'Usable' regions from BootInfo.
fn init_frame_allocator(boot_info: &BootInfo) {
    let mut ba = FRAME_ALLOCATOR.lock();
    for region in boot_info.memory_map.iter() {
        if region.region_type == MemoryRegionType::Usable {
            ba.insert(
                region.range.start_frame_number as usize..region.range.end_frame_number as usize,
            );
        }
    }
}

fn init_device_vm_map() {
    let mut page_table = active_table();
    // IOAPIC
    page_table
        .map(KERNEL_OFFSET + 0xfec00000, 0xfec00000)
        .update();
    // LocalAPIC
    page_table
        .map(KERNEL_OFFSET + 0xfee00000, 0xfee00000)
        .update();
}
