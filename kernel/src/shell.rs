use crate::fat::FAT;
use crate::print::PRINTER;
use core::arch::asm;

const APP_TARGET: u32 = 0x0035_0000;

//SHELL
//Warning! Mutable static here
//TODO: Implement a mutex to get safe access to this
pub static mut SHELL: Shell = Shell {
    buffer: [0 as char; 256],
    arg: [0 as char; 11],
    cursor: 0,
};

const PROMPT: &str = "felix> ";

pub struct Shell {
    buffer: [char; 256],
    arg: [char; 11],
    cursor: usize,
}

impl Shell {
    pub fn init(&mut self) {
        self.buffer = [0 as char; 256];
        self.cursor = 0;

        unsafe {
            PRINTER.set_colors(0xc, 0);
            print!("{}", PROMPT);

            PRINTER.reset_colors();
        }
    }

    pub fn add(&mut self, c: char) {
        self.buffer[self.cursor] = c;
        self.cursor += 1;

        print!("{}", c);
    }

    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            self.buffer[self.cursor] = 0 as char;
            self.cursor -= 1;

            unsafe {
                PRINTER.delete();
            }
        }
    }

    pub fn enter(&mut self) {
        unsafe {
            PRINTER.new_line();
        }

        self.interpret();
        self.init();
    }

    #[allow(unused_unsafe)]
    fn interpret(&mut self) {
        match self.buffer {
            //test command
            b if equals("ping", &b) => {
                println!("PONG!");
            }

            //list root directory
            b if equals("ls", &b) => unsafe {
                FAT.list_entries();
            },

            //display content of file
            b if equals("cat", &b) => unsafe {
                for i in 4..15 {
                    self.arg[i - 4] = b[i];
                }

                let entry = FAT.search_file(&self.arg);

                if entry.name[0] != 0 {
                    FAT.read_file(&entry);

                    for c in FAT.buffer {
                        if c != 0 {
                            print!("{}", c as char);
                        }
                    }
                    println!();
                } else {
                    println!("File not found!");
                }
            },

            //loads a program
            b if equals("run", &b) => unsafe {
                for i in 4..15 {
                    self.arg[i - 4] = b[i];
                }

                let entry = FAT.search_file(&self.arg);
                if entry.name[0] != 0 {
                    FAT.read_file_to_target(&entry, APP_TARGET as *mut u32);

                    unsafe {
                        asm!("jmp {}", in(reg) APP_TARGET);
                    }
                } else {
                    println!("Program not found!");
                }
            },

            //help command
            b if equals("help", &b) => {
                println!("Available commands:\nls - lists root directory entries\ncat <file> - displays content of a file");
            }

            //empty, do nothing
            b if b[0] == '\0' => {}

            //unknown command
            _ => {
                println!("Unknown command!");
            }
        }
    }
}

fn equals(short: &str, long: &[char]) -> bool {
    let mut i = 0;
    for c in short.chars() {
        if c != long[i as usize] {
            return false;
        }
        i += 1;
    }
    true
}
