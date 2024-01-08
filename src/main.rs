mod big_integer;

use big_integer::BigInteger;

fn main() {
    let a = BigInteger::from_vec(vec![16_000_000_000_000_000_000]);
    let b = BigInteger::from_vec(vec![16_000_000_000_000_000_000, std::u64::MAX, 1]);
    println!("add {:?}", a + b);

    let c = BigInteger::from_vec(vec![2, 2]);
    let d = BigInteger::from_vec(vec![3, 2, 5]);
    println!("mul a {:?}", c * d);
    let e = BigInteger::from_vec(vec![std::u64::MAX, 2]);
    let f = BigInteger::from_vec(vec![2]);
    println!("mul b {:?}", e * f);

    let g = BigInteger::from_vec(vec![2, 7, 9]);
    let h = BigInteger::from_vec(vec![5]);
    println!("pow a {:?}", g.pow(h));
    let i = BigInteger::from_vec(vec![2]);
    let j = BigInteger::from_vec(vec![0, 1]);
    println!("pow b {:?}", i.pow(j));
    let k = BigInteger::new();
    let l = BigInteger::new();
    println!("pow c {:?}", k.pow(l));

    let m = BigInteger::from_vec(vec![0, 1, 1]);
    let n = BigInteger::from_vec(vec![0, 5]);
    println!("sub {:?}", m - n);

    let o = BigInteger::from_vec(vec![0, 1]);
    let p = BigInteger::from_vec(vec![16_000_000_000_000_000_000]);
    println!("mod {:?}", o % p);

    let q = BigInteger::from_vec(vec![5, 6, 15]);
    let r = BigInteger::from_vec(vec![3, 1]);
    let s = BigInteger::from_vec(vec![1_591_181_525_541]);
    println!("mod mul {:?}", q.mod_mul(r, s));

    let t = BigInteger::from_vec(vec![5, 6, 15]);
    let u = BigInteger::from_vec(vec![3, 1]);
    let v = BigInteger::from_vec(vec![1_591_181_525_541]);
    println!("mod pow {:?}", t.mod_pow(u, v));
}
