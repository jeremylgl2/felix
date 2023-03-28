use core::arch::asm;
use core::mem::size_of;

const IDT_ENTRIES: usize = 256;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct IdtEntry {
    offset_low: u16,       //lower 16 bits of handler func address
    segment_selector: u16, //segment selector of gdt entry
    reserved: u8,          //always zero
    flags: u8,             //entry flags
    offset_high: u16,      //higher 16 bits of handler func address
}

#[repr(C, packed)]
pub struct InterruptDescriptorTable {
    pub entries: [IdtEntry; IDT_ENTRIES],
}

#[repr(C, packed)]
pub struct IdtDescriptor {
    size: u16,                               //idt size
    offset: *const InterruptDescriptorTable, //pointer to idt
}

impl InterruptDescriptorTable {
    pub fn new() -> Self {
        //fill table with entries with a generic handler
        Self {
            entries: [IdtEntry::new(generic_handler as u32); IDT_ENTRIES],
        }
    }

    pub fn add(&mut self, int: usize, handler: u32) {
        let entry = IdtEntry::new(handler);
        self.entries[int] = entry;
    }

    //load idt using lidt instruction
    pub fn load(&self) {
        let descriptor = IdtDescriptor {
            size: (IDT_ENTRIES * size_of::<IdtEntry>() - 1) as u16, //calculate size of idt
            offset: self,                                           //pointer to idt
        };

        unsafe {
            asm!("lidt [{0:e}]", in(reg) &descriptor);
        }
    }

    //add exception handlers for various cpu exceptions
    pub fn add_exceptions(&mut self) {
        self.add(0x0, div_error as u32);
        self.add(0x6, invalid_opcode as u32);
        self.add(0x8, double_fault as u32);
        self.add(0xd, general_protection_fault as u32);
        self.add(0xe, page_fault as u32);
    }
}

impl IdtEntry {
    pub fn new(offset: u32) -> Self {
        let offset_low: u16 = ((offset << 16) >> 16) as u16; //calculate lower 16 bits of offset

        let offset_high: u16 = (offset >> 16) as u16; //calculate higher 16 bits of offset

        let segment_selector: u16 = {
            //segment selector of gdt entry
            let rpl = 0b00 << 0; //ring privilege level (0 for ring 0)
            let ti = 0b0 << 2; //0 to use gdt, 1 to use ldt
            let index = 0b1 << 3; //entry index of gdt code segment, in my case code entry is the second (0b1)

            rpl | ti | index
        };

        let reserved: u8 = 0; //always zero

        let flags: u8 = {
            //entry flags
            let gate_type = 0xe << 0; //gate type, 0xe for 32bit interrupt gate, 0xf for 32bit trap gate
            let zero = 0 << 3; //always zero
            let dpl = 0 << 5; //ring allowed to use this interrupt
            let p = 1 << 7; //presence bit, 1 to enable

            gate_type | zero | dpl | p
        };

        Self {
            offset_low: offset_low,
            segment_selector: segment_selector,
            reserved: reserved,
            flags: flags,
            offset_high: offset_high,
        }
    }
}

//handle excpetion based on interrupt number
#[no_mangle]
pub extern "C" fn exception_handler(int: u32) -> ! {
    match int {
        0x00 => {
            println!("DIVISION ERROR!");
        }
        0x06 => {
            println!("INVALID OPCODE!");
        }
        0x08 => {
            println!("DOUBLE FAULT!");
        }
        0x0D => {
            println!("GENERAL PROTECTION FAULT!");
        }
        0x0E => {
            println!("PAGE FAULT!");
        }
        0xFF => {
            println!("EXCEPTION!");
        }
        _ => {
            println!("EXCEPTION!");
        }
    }

    loop {}
}

#[naked]
pub extern "C" fn div_error() {
    unsafe {
        asm!("push 0x00", "call exception_handler", options(noreturn));
    }
}

#[naked]
pub extern "C" fn invalid_opcode() {
    unsafe {
        asm!("push 0x06", "call exception_handler", options(noreturn));
    }
}

#[naked]
pub extern "C" fn double_fault() {
    unsafe {
        asm!("push 0x08", "call exception_handler", options(noreturn));
    }
}

#[naked]
pub extern "C" fn general_protection_fault() {
    unsafe {
        asm!("push 0x0d", "call exception_handler", options(noreturn));
    }
}

#[naked]
pub extern "C" fn page_fault() {
    unsafe {
        asm!("push 0x0e", "call exception_handler", options(noreturn));
    }
}

#[naked]
pub extern "C" fn generic_handler() {
    unsafe {
        asm!("push 0xff", "call exception_handler", options(noreturn));
    }
}