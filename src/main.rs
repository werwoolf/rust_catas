mod codewars_catas;

struct BigInt {
    digits: Vec<u32>, // Store digits in reverse order, least significant digit first
}

impl BigInt {
    // Create a BigInt from a string
    fn from_str(value: &str) -> BigInt {
        let mut digits = Vec::new();
        for chunk in value.chars().rev().collect::<Vec<char>>().chunks(9) {
            let s: String = chunk.iter().rev().collect();
            digits.push(s.parse::<u32>().unwrap());
        }
        BigInt { digits }
    }

    // Multiply two BigInts
    fn mul(&self, other: &BigInt) -> BigInt {
        let mut result = vec![0u64; self.digits.len() + other.digits.len()];
        for (i, &a) in self.digits.iter().enumerate() {
            let mut carry = 0u64;
            for (j, &b) in other.digits.iter().enumerate() {
                let idx = i + j;
                let sum = result[idx] + (a as u64) * (b as u64) + carry;
                result[idx] = sum % 1_000_000_000;
                carry = sum / 1_000_000_000;
            }
            if carry > 0 {
                result[i + other.digits.len()] += carry;
            }
        }

        // Trim leading zeros
        while result.len() > 1 && *result.last().unwrap() == 0 {
            result.pop();
        }

        BigInt {
            digits: result.iter().map(|&x| x as u32).collect(),
        }
    }

    // Exponentiation by squaring
    fn pow(&self, exp: &BigInt) -> BigInt {
        let mut base = self.clone();
        let mut exponent = exp.clone();
        let mut result = BigInt::from_str("1");

        while !exponent.is_zero() {
            if exponent.is_odd() {
                result = result.mul(&base);
            }
            base = base.mul(&base);
            // exponent = &exponent.div_by_two();
        }

        result
    }

    // Check if BigInt is zero
    fn is_zero(&self) -> bool {
        self.digits.len() == 1 && self.digits[0] == 0
    }

    // Check if BigInt is odd
    fn is_odd(&self) -> bool {
        self.digits[0] % 2 != 0
    }

    // Divide BigInt by 2
    fn div_by_two(&self) -> BigInt {
        let mut result = Vec::new();
        let mut carry = 0u64;

        for &digit in self.digits.iter().rev() {
            let num = carry * 1_000_000_000 + digit as u64;
            result.push((num / 2) as u32);
            carry = num % 2;
        }

        result.reverse();
        while result.len() > 1 && *result.last().unwrap() == 0 {
            result.pop();
        }

        BigInt { digits: result }
    }
}

fn main() {
    println!("////////{}", last_digit_big_number("33", "5"));
}

fn last_digit_big_number(str1: &str, str2: &str) -> i128 {
    let parsed1 = str1.parse::<i128>().unwrap();
    let parsed2 = str2.parse::<i128>().unwrap();

    return parsed1.pow(parsed2 as u32);
}
