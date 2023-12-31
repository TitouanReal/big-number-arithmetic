mod big_integer;

use big_integer::BigInteger;

fn main() {
    let a = BigInteger::from_vec(vec![16_000_000_000_000_000_000]);
    let b = BigInteger::from_vec(vec![16_000_000_000_000_000_000, std::u64::MAX, 1]);
    println!("{:?}", a + b);
}
