use crate::drivers::pic::PICS;
use crate::task::CPUState;
use crate::task::TASK_MANAGER;
use core::arch::asm;

pub const TIMER_INT: u8 = 32;

//timer handler
#[naked]
pub extern "C" fn timer() {
    unsafe {
        asm!(
            //save registers
            "push ebp",
            "push edi",
            "push esi",
            "push edx",
            "push ecx",
            "push ebx",
            "push eax",
            //give esp and int num to c func
            "push esp",
            "call timer_handler",
            //set esp to return value of c func
            "mov esp, eax",
            //restore registers
            "pop eax",
            "pop ebx",
            "pop ecx",
            "pop edx",
            "pop esi",
            "pop edi",
            "pop ebp",
            //return interrupt
            "iretd",
            options(noreturn)
        );
    }
}

#[no_mangle]
pub extern "C" fn timer_handler(esp: u32) -> u32 {
    unsafe {
        let new_esp: u32 = TASK_MANAGER.schedule(esp as *mut CPUState) as u32;

        PICS.end_interrupt(TIMER_INT);

        return new_esp;
    }
}
