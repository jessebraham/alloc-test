#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

#[macro_use]
extern crate alloc;

use core::fmt::Write;

use esp32_hal::{pac::Peripherals, prelude::*, RtcCntl, Serial, Timer};
use panic_halt as _;
use xtensa_lx_rt::entry;

#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

#[alloc_error_handler]
fn oom(_: core::alloc::Layout) -> ! {
    loop {}
}

fn init_heap() {
    use core::mem::MaybeUninit;

    const HEAP_SIZE: usize = 4 * 1024; // FIXME: this should probably be configurable
    static mut HEAP: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];

    unsafe {
        ALLOCATOR.init(HEAP.as_ptr() as usize, HEAP_SIZE);
    }
}

#[entry]
fn main() -> ! {
    init_heap();

    let peripherals = Peripherals::take().unwrap();

    let mut rtc_cntl = RtcCntl::new(peripherals.RTC_CNTL);
    let mut serial0 = Serial::new(peripherals.UART0).unwrap();
    let mut timer0 = Timer::new(peripherals.TIMG0);

    rtc_cntl.set_wdt_global_enable(false);
    timer0.disable();

    let mut v = vec![0, 1, 2];
    writeln!(serial0, "Vector elements: {:?}", v).ok();

    v.push(4);
    v.push(5);
    writeln!(serial0, "Vector elements: {:?}", v).ok();

    loop {}
}
