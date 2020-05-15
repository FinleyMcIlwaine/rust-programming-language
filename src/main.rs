
use std::mem;

fn core_data_types() {
    println!("\n*** CORE DATA TYPES ***\n");

    // Immutable unsigned 8-bit integer
    let a:u8 = 123;

    println!("Hello, Rust!\n\na = {}", a);

    // Mutable
    let mut b:u8 = 111;
    println!("b = {}", b);
    b = 42;
    println!("b = {}\n", b);

    // Type inference
    let mut c = 123456789;
    println!("c = {}, size = {}", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {} after modification, size = {}", c, mem::size_of_val(&c));

    // We have u8, i8, u16, i16, u32, i32, u64, i64
    // isize is the int size of the system
    let z:isize = 123;
    let size_of_z = mem::size_of_val(&z);

    println!("z = {}, takes up {} bytes, {} bit OS", z, size_of_z, size_of_z * 8);

    let d:char = 'x';
    println!("d = {}, takes up {} bytes", d, mem::size_of_val(&d));

    let e = 2.5; // Double-precision by default! 8 bytes
    println!("e = {}, takes up {} bytes", e, mem::size_of_val(&e));
    let e:f32 = 2.5; // Explicitly single precision
    println!("e = {}, takes up {} bytes", e, mem::size_of_val(&e));

    let g = false;
    println!("g = {}, takes up {} bytes", g, mem::size_of_val(&g));
}

fn operators() {
    println!("\n\n*** OPERATORS ***\n");

    // arithmetic
    let a = 2 + 3 * 4;
    println!("a = {}",a);

    let a_cubed = i32::pow(a,3);
    println!("a_cubed = {}", a_cubed);

    let e = 2.3; // Defaults to f32
    let e_cubed = f32::powi(e, 3); // Raise to integer power
    println!("e_cubed = {}", e_cubed);

    let e_pow = f64::powf(2.3, std::f64::consts::PI);
    println!("e_to_pi = {}", e_pow);

    // Bitwise
    let c = 1 | 2; // 1 = 01, 2 = 10, 1 | 2 = 11 = 3_10
    println!("1 | 2 = {}", c);

    let two_to_ten = 1 << 10;
    println!("2^10 = {}", two_to_ten);

    let pi_less_four = std::f64::consts::PI < 4.0;
    println!("PI < 4 = {}", pi_less_four);
}

fn main() {
    core_data_types();
    operators();
}