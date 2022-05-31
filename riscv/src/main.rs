#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

#[macro_use]
extern crate alloc;

use core::fmt::Write;

use esp32c3_hal::{pac::Peripherals, prelude::*, RtcCntl, Timer, UsbSerialJtag};
use panic_halt as _;
use riscv_rt::entry;

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
    let mut timer0 = Timer::new(peripherals.TIMG0);
    let mut timer1 = Timer::new(peripherals.TIMG1);

    rtc_cntl.set_super_wdt_enable(false);
    rtc_cntl.set_wdt_enable(false);
    timer0.disable();
    timer1.disable();

    let mut v = vec![0, 1, 2];
    writeln!(UsbSerialJtag, "Vector elements: {:?}", v).ok();

    v.push(4);
    v.push(5);
    writeln!(UsbSerialJtag, "Vector elements: {:?}", v).ok();

    loop {}
}
