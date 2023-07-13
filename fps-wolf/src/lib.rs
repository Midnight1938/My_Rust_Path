#![no_std]

use core::{arch::wasm32, panic::PanicInfo};


/* 
A pointer to the current state of the first gamepad 
 runtime will update this section of memory 
 with the state of our gamepad (keyboard) on each frame.
*/
const GAMEPAD1: *const u8 = 0x16 as *const u8;

// describe the bits in the gamepad which describe each button.
const BUTTON_LEFT: u8 = 16;     // 0b00010000
const BUTTON_RIGHT: u8 = 32;    // 0b00100000
const BUTTON_UP: u8 = 64;       // 0b01000000
const BUTTON_DOWN: u8 = 128;    // 0b10000000

extern "C" { // Import the `env` module from Wasm.

    fn vline(x: i32, y: i32, len: u32);
    /* 
    vline draws a vertical line on the window at x, y 
    and extends it down len pixels. 
    */
}

#[panic_handler]
fn phandler(_: &PanicInfo<'_>) -> ! {
    wasm32::unreachable!();
}
