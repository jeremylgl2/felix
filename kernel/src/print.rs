use core::arch::asm;

//bios interrupt to print to the screen
//TODO: Implement a printf function that is able to format strings
pub fn prints(message: &str) {
    unsafe {
        asm!("mov si, {0:x}", //move given string address to si
            "2:",
            "lodsb", //load a byte (next character) from si to al
            "or al, al", //bitwise or on al, if al is null set zf to true
            "jz 1f", //if zf is true (end of string) jump to end

            "mov ah, 0x0e",
            "mov bh, 0",
            "int 0x10", //tell the bios to write content of al to screen

            "jmp 2b", //start again
            "1:",
            in(reg) message.as_ptr());
    }
}

//print only a char
pub fn printc(c: char) {
    unsafe {
        asm!(
            "mov ah, 0x0e",
            "mov bh, 0",
            "int 0x10", //tell the bios to write content of al to screen
            in("ax") c as u16);
    }
}

//set bios video mode to clear the screen
#[allow(dead_code)]
pub fn clear() {
    unsafe {
        asm!("mov ah, 0x00", "mov al, 0x03", "int 0x10");
    }
}