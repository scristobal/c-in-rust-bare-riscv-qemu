#![no_std]
#![no_main]

use core::arch::naked_asm;

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn gcd(n: u32, m: u32) -> u32 {
    let pair = &mut bindings::Pair { n, m };
    unsafe { bindings::gcd(pair) }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
#[unsafe(naked)]
pub extern "C" fn _start() -> ! {
    naked_asm!("la sp, __stack_top", "call rust_main",)
}

#[unsafe(no_mangle)]
fn rust_main() -> ! {
    print("Starting\n");

    assert!(gcd(48, 18) == 6);
    assert!(gcd(100, 10) == 10);
    assert!(gcd(7, 13) == 1);
    // assert!(gcd(0, 5) == 5); // panics
    assert!(gcd(12, 12) == 12);

    print("Done\n");
    exit(0);
}

// QEMU virt UART
const UART_ADDR: usize = 0x10000000;
const VIRT_TEST: usize = 0x100000;

struct Uart;

impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        print(s);
        Ok(())
    }
}

fn print(s: &str) {
    for byte in s.bytes() {
        unsafe { (UART_ADDR as *mut u8).write_volatile(byte) };
    }
}

fn exit(code: u32) -> ! {
    let exit_code = if code == 0 {
        0x5555
    } else {
        (code << 16) | 0x3333
    };
    unsafe { (VIRT_TEST as *mut u32).write_volatile(exit_code) };
    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use core::fmt::Write;
    let mut uart = Uart;
    let _ = writeln!(uart, "Panic: {}", info);
    exit(1);
}
