mod big_integer;

use big_integer::BigInteger;

fn main() {
    let a = BigInteger::from_vec(vec![16_000_000_000_000_000_000]);
    let b = BigInteger::from_vec(vec![16_000_000_000_000_000_000, std::u64::MAX, 1]);
    println!("{:?}", a + b);

    let c = BigInteger::from_vec(vec![2, 2]);
    let d = BigInteger::from_vec(vec![3, 2, 5]);
    println!("{:?}", c * d);
    let e = BigInteger::from_vec(vec![std::u64::MAX, 2]);
    let f = BigInteger::from_vec(vec![2]);
    println!("{:?}", e * f);

    let g = BigInteger::from_vec(vec![2]);
    let h = BigInteger::from_vec(vec![5]);
    println!("{:?}", g.pow(h));
    let i = BigInteger::from_vec(vec![std::u64::MAX]);
    let j = BigInteger::from_vec(vec![2]);
    println!("{:?}", i.pow(j));
    let k = BigInteger::new();
    let l = BigInteger::new();
    println!("{:?}", k.pow(l));
}
