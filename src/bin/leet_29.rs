// 29 Divide Two Integer
//
// Given two integers dividend and divisor, divide two integers without using multiplication,
// division, and mod operator.
//
// The integer division should truncate toward zero, which means losing its fractional part. For
// example, 8.345 would be truncated to 8, and -2.7335 would be truncated to -2.
//
// Return the quotient after dividing dividend by divisor.

struct Solution {}

impl Solution {
    pub fn divide(dividend:i32, divisor: i32) -> i32 {

        if dividend == i32::MIN && divisor == 1 {
            return dividend;
        }

        let negate_result = (dividend < 0) ^ (divisor < 0);
        let mut dividend = if dividend > 0 {
            -dividend
        } else {
            dividend
        };

        let divisor= if divisor > 0 {
            -divisor
        } else {
            divisor
        };

        let mut quotient = 0;

        let mut pow2: i32 = 1;
        let mut mul = divisor;

        // The basic idea is to factor the quotient into power-of-two multiples of the divisor, i.e., dividend = divisor * quotient and quotient is a power-of-two sum.
        // First, build a table where t[i] = (2^i, 2^i*divisor) for 0 <= i <= 31.
        //
        // For example, 127 / 5
        // t[0] = ( 1,   5)
        // t[1] = ( 2,  10)
        // t[2] = ( 4,  20)
        // t[3] = ( 8,  40)
        // t[4] = (16,  80)
        //
        //      dividend,  c, t[i]
        //           127,  0, (16, 80)
        //  47  (127-80), 16, ( 8, 40)
        //   7 (47 - 40), 24, ( 4, 20)
        //   7          , 24, ( 2, 10)
        //   7          , 24, ( 1,  5)
        //   2   (7 - 5), 25
        //
        // Final result is 25.
        let mut t: Vec<(i32, i32)> = vec![(pow2, mul)];
        for _ in 1..=30 {
            pow2 += pow2;
            mul = match mul.checked_add(mul) {
                Some(v) => v,
                None => break,
            };
            if mul < dividend {
                break;
            }
            t.push((pow2, mul));
        }

        for (pow2, mul) in t.iter().rev() {
            if dividend <= *mul {
                quotient += pow2;
                dividend -= mul;
            }
        }

        if negate_result {
            -quotient
        } else {
            quotient
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let testcases = vec![
            (10, 3, 3),
            (7, -3, -2),
            (i32::MAX, 1, i32::MAX),
            (i32::MAX, i32::MAX, 1),
            (i32::MIN, 1, i32::MIN),
            (i32::MIN, i32::MIN, 1),
            (i32::MIN, -1, i32::MAX),
            (i32::MIN+1, -1, i32::MAX),
            (i32::MAX, -1, i32::MIN+1),
        ];
        for (dividend, divisor, result) in testcases {
            assert_eq!(
                Solution::divide(dividend, divisor),
                result
            );
        }
    }
}
