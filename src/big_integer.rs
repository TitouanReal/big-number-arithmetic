use std::ops::{Add, Mul};

#[derive(Debug, Clone)]
pub struct BigInteger {
    values: Vec<u64>,
}

impl BigInteger {
    pub fn new() -> Self {
        BigInteger { values: Vec::new() }
    }

    pub fn from_vec(mut values: Vec<u64>) -> Self {
        while !values.is_empty() && values[values.len() - 1] == 0 {
            values.pop();
        }
        BigInteger { values: values }
    }

    pub fn pow(self, other: Self) -> Self {
        let mut result = BigInteger::from_vec(vec![1]);

        for (i, digit) in other.values.iter().enumerate() {
            let mut fragment_values = vec![0; i];
            fragment_values.push(1);
            let mut fragment = BigInteger::from_vec(fragment_values);
            for _ in 0..*digit {
                fragment = fragment * self.clone();
            }

            result = result * fragment;
        }

        result
    }
}

impl Add for BigInteger {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = Self::new();
        let mut carry = 0;

        let mut a_iter = self.values.iter().peekable();
        let mut b_iter = other.values.iter().peekable();

        while let (Some(a), Some(b)) = (a_iter.peek(), b_iter.peek()) {
            result.values.push(a.wrapping_add(**b));
            if **b == std::u64::MAX {
                carry = 1;
            } else {
                if **a > std::u64::MAX - *b - carry {
                    carry = 1;
                } else {
                    carry = 0;
                }
            }
            a_iter.next();
            b_iter.next();
        }

        while let Some(a) = a_iter.next() {
            result.values.push(a.wrapping_add(carry));
            if *a == std::u64::MAX && carry == 1 {
                carry = 1;
            } else {
                carry = 0;
            }
        }

        while let Some(b) = b_iter.next() {
            result.values.push(b.wrapping_add(carry));
            if *b == std::u64::MAX && carry == 1 {
                carry = 1;
            } else {
                carry = 0;
            }
        }

        if carry != 0 {
            result.values.push(carry);
        }

        result
    }
}

impl Mul for BigInteger {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = Self::new();

        for (i, a) in self.values.iter().enumerate() {
            for (j, b) in other.values.iter().enumerate() {
                let product: u128 = *a as u128 * *b as u128;
                let high: u64 = (product >> 64) as u64;
                let low: u64 = product as u64;

                let mut result_fragment = vec![0; i + j];
                result_fragment.push(low);
                if high != 0 {
                    result_fragment.push(high);
                }
                result = result + BigInteger::from_vec(result_fragment);
            }
        }

        result
    }
}
