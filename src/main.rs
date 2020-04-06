fn mod_pow(base: u32, exponent: u32, modulus: u32) -> u32 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    let mut base = base % modulus;
    let mut exponent = exponent;
    while exponent != 0 {
        if (exponent & 1) != 0 {
            result = result*base % modulus;
        }
        exponent = exponent >> 1;
        base = base*base % modulus;
    }
    result
}

fn main() {
    println!("{}", mod_pow(2, 23, 10));
}
