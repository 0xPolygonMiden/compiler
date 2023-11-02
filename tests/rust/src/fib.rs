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

    if n < 1000 {
        1
    } else {
        0
    }
}
