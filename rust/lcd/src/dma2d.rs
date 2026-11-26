const BASE: usize = 0x4002_B000;

const CR: *mut u32 = (BASE + 0x00) as *mut u32;
const ISR: *const u32 = (BASE + 0x04) as *const u32;
const IFCR: *mut u32 = (BASE + 0x08) as *mut u32;

const OPFCCR: *mut u32 = (BASE + 0x34) as *mut u32;
const OCOLR: *mut u32 = (BASE + 0x38) as *mut u32;
const OMAR: *mut u32 = (BASE + 0x3C) as *mut u32;
const OOR: *mut u32 = (BASE + 0x40) as *mut u32;
const NLR: *mut u32 = (BASE + 0x44) as *mut u32;

const MODE_R2M: u32 = 0b11 << 16;
const FORMAT_RGB565: u32 = 0b010;

const START: u32 = 1;
const TRANSFER_COMPLETE: u32 = 1 << 1;
const ERRORS: u32 = (1 << 0) | (1 << 5);

/// # Safety
/// The destination must be valid for the entire transfer.
/// DMA2D must not be used concurrently elsewhere.
pub unsafe fn fill(
    address: *mut u16,
    stride: usize,
    width: usize,
    height: usize,
    color: u16,
) -> bool {
    if width == 0 || height == 0 {
        return true;
    }

    if width > stride || width > 0x3FFF || height > 0xFFFF || stride - width > 0x3FFF {
        return false;
    }

    unsafe {
        while CR.read_volatile() & START != 0 {
            core::hint::spin_loop();
        }

        IFCR.write_volatile(0x3F);

        CR.write_volatile(MODE_R2M);
        OPFCCR.write_volatile(FORMAT_RGB565);
        OCOLR.write_volatile(color as u32);

        OMAR.write_volatile(address as u32);
        OOR.write_volatile((stride - width) as u32);

        NLR.write_volatile(((width as u32) << 16) | height as u32);

        cortex_m::asm::dsb();

        CR.write_volatile(MODE_R2M | START);

        loop {
            let status = ISR.read_volatile();

            if status & ERRORS != 0 {
                return false;
            }

            if status & TRANSFER_COMPLETE != 0 {
                break;
            }

            core::hint::spin_loop();
        }

        IFCR.write_volatile(0x3F);
        cortex_m::asm::dsb();
    }

    true
}
