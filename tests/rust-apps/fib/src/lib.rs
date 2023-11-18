#![no_std]

#[no_mangle]
pub fn fib(n: u32) -> u32 {
    // TODO: switch to Fibonacci after i32 intrinsics PR is merged (missing addition)
    // let mut a = 0;
    // let mut b = 1;
    // for _ in 0..n {
    //     let c = a + b;
    //     a = b;
    //     b = c;
    // }
    // a

    let mut a = n;
    while a > 3 {
        a = a / 3;
    }
    a
}
