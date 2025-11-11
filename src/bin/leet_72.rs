// 72 Edit Distance
//
// Given two strings word1 and word2, return the minimum number of operations required to convert
// word1 to word2.
//
// You have the following three operations permitted on a word:
//
// - Insert a character
// - Delete a character
// - Replace a character

struct Solution {}

use std::cmp::min;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (b1, b2) = if word1.len() > word2.len() {
            (word1.as_bytes(), word2.as_bytes())
        } else {
            (word2.as_bytes(), word1.as_bytes())
        };
        Solution::min_distance_inner(b1, b2)
    }

    pub fn min_distance_inner(w1: &[u8], w2: &[u8]) -> i32 {
        assert!(w1.len() >= w2.len());

        if w2.is_empty() {
            return w1.len().try_into().unwrap();
        }

        let mut dp: Vec<Vec<i32>> = (0..=w1.len()).map(|_| vec![0; w2.len()+1]).collect();

        for i in 0..=w1.len() {
            dp[i][0] = i.try_into().unwrap();
        }
        for j in 0..=w2.len() {
            dp[0][j] = j.try_into().unwrap();
        }

        for i in 1..=w1.len() {
            for j in 1..=w2.len() {
                if w1[i-1] == w2[j-1] {
                    dp[i][j] = dp[i-1][j-1];
                } else {
                    dp[i][j] = 1 + min(min(dp[i-1][j], dp[i][j-1]), dp[i-1][j-1]);
                }
            }
        }
        dp[w1.len()][w2.len()]
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    fn random_string_with_rng<R: Rng + ?Sized>(rng: &mut R, n: usize) -> String {
        (0..n)
            .map(|_| {
                let c = rng.random_range(b'a'..=b'z'); // generate random byte in 'a'..'z'
                c as char
            })
            .collect()
    }

    #[test]
    fn test_min_distance() {
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        let seed = 42;
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        let s1 = random_string_with_rng(&mut rng, 500);
        let s2 = random_string_with_rng(&mut rng, 500);
        let s3 = String::from_utf8(s1.as_bytes()[1..].to_vec()).unwrap();

        let testcases = vec![
            ("horse", "ros", 3),
            ("intention", "execution", 5),
            ("aba", "aa", 1),
            ("abab", "aa", 2),
            ("aaa", "", 3),
            ("", "", 0),
            ("abc", "abc", 0),
            (&s1, &s2, 442),
            (&s1, &s1, 0),
            (&s1, &s3, 1),
            ("distance", "springbok", 9)
        ];
        for (word1, word2, count) in testcases {
            assert_eq!(
                Solution::min_distance(word1.to_string(), word2.to_string()),
                count,
                "{} {}", word1, word2
            );
        }
    }
}