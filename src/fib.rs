pub fn get_fib1(n: u8) {
    let (mut a, mut b) = (1, 1);
    for _i in 2..n {
        next(&mut a, &mut b);
    }
    println!("{}", b);
}

pub fn get_fib2(n: u8) {
    let (mut a, mut b) = (1, 1);
    for _i in 2..n {
        a = a + b;
        swap(&mut a, &mut b);
    }
    println!("{}", b);
}

pub fn get_fib3(n: u8) {
    let mut fib = (1, 1);
    for _i in 2..n {
        fib = (fib.1, fib.0 + fib.1);
    }
    println!("{}", fib.1);
}

fn swap(a: &mut i32, b: &mut i32) {
    *a ^= *b;
    *b ^= *a;
    *a ^= *b;
}

fn next(a: &mut i32, b: &mut i32) {
    let c = *a + *b;
    *a = *b;
    *b = c;
}