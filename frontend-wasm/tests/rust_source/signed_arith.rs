#![no_std]
#![no_main]

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[inline(never)]
#[no_mangle]
fn div_s(a: i32, b: i32) -> i32 {
    a / b
}

#[inline(never)]
#[no_mangle]
fn div_u(a: u32, b: u32) -> u32 {
    a / b
}

#[inline(never)]
#[no_mangle]
fn rem_s(a: i32, b: i32) -> i32 {
    a % b
}

#[inline(never)]
#[no_mangle]
fn rem_u(a: u32, b: u32) -> u32 {
    a % b
}

#[inline(never)]
#[no_mangle]
fn shr_s(a: i32, b: i32) -> i32 {
    a >> b
}

#[inline(never)]
#[no_mangle]
fn shr_u(a: u32, b: u32) -> u32 {
    a >> b
}

#[no_mangle]
pub extern "C" fn __main() -> i32 {
    div_s(-8, -4) + rem_s(-8, -3) + shr_s(-16, 2) 
        + (div_u(8, 4) +  rem_u(8, 3) + shr_u(16, 2)) as i32
}
