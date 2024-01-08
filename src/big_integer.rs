use std::ops::{Add, Mul, Sub};

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

impl Sub for BigInteger {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let self_len = self.values.len();
        let other_len = other.values.len();
        if self_len < other_len {
            panic!("Overflow occurred during substraction");
        }

        let mut signed_result: Vec<i128> = vec![];

        for (i, value) in other.values.iter().enumerate() {
            signed_result.push(i128::from(self.values[i].clone()) - i128::from(value.clone()));
        }
        for i in other_len..self_len {
            signed_result.push(i128::from(self.values[i]));
        }

        let mut stop = true;
        loop {
            for value in signed_result.iter() {
                if value < &0 {
                    stop = false;
                }
            }
            if stop {
                break;
            }
            for i in 0..signed_result.len() {
                if signed_result[i] < 0 {
                    if i + 1 == signed_result.len() {
                        panic!("Overflow occurred during substraction");
                    }
                    signed_result[i] = signed_result[i] + i128::from(std::u64::MAX) + 1;
                    signed_result[i + 1] = signed_result[i + 1] - 1;
                }
            }
            stop = true;
        }

        let mut result: Vec<u64> = vec![];
        for value in signed_result.iter() {
            result.push(u64::try_from(value.clone()).unwrap());
        }

        return Self::from_vec(result);
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
