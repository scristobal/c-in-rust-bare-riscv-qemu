const UART_ADDR: usize = 0x10000000;
const UART_LSR: usize = UART_ADDR + 5;
const VIRT_TEST: usize = 0x100000;

use core::fmt::{self, Write};

pub struct Uart;

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            putchar(byte);
        }
        Ok(())
    }
}

pub fn putchar(c: u8) {
    unsafe {
        (UART_ADDR as *mut u8).write_volatile(c);
    }
}

pub fn getchar() -> u8 {
    unsafe {
        while (UART_LSR as *const u8).read_volatile() & 1 == 0 {}
        (UART_ADDR as *const u8).read_volatile()
    }
}

pub fn print(s: &str) {
    for byte in s.bytes() {
        putchar(byte);
    }
}

pub fn print_u32(mut n: u32) {
    if n == 0 {
        putchar(b'0');
        return;
    }
    let mut buf = [0u8; 10];
    let mut i = 0;
    while n > 0 {
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
        i += 1;
    }
    while i > 0 {
        i -= 1;
        putchar(buf[i]);
    }
}

pub fn read_u32() -> u32 {
    let mut n: u32 = 0;
    loop {
        let c = getchar();
        putchar(c);
        match c {
            b'0'..=b'9' => n = n * 10 + (c - b'0') as u32,
            b' ' | b'\r' | b'\n' => {
                if c == b'\r' || c == b'\n' {
                    putchar(b'\n');
                }
                break;
            }
            _ => {}
        }
    }
    n
}

pub fn exit(code: u32) -> ! {
    let exit_code = if code == 0 {
        0x5555
    } else {
        (code << 16) | 0x3333
    };
    unsafe {
        (VIRT_TEST as *mut u32).write_volatile(exit_code);
    }
    loop {}
}
