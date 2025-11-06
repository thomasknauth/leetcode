// 546 Number of Provinces

// There are n cities. Some of them are connected, while some are not. If city a is connected
// directly with city b, and city b is connected directly with city c, then city a is connected
// indirectly with city c.
//
// A province is a group of directly or indirectly connected cities and no other cities outside of
// the group.
//
// You are given an n x n matrix isConnected where isConnected[i][j] = 1 if the ith city and the jth
// city are directly connected, and isConnected[i][j] = 0 otherwise.
//
// Return the total number of provinces.

use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut cities: HashSet<_> = (0..is_connected.len() as i32).collect();
        let mut provinces: Vec<HashSet<_>> = Vec::new();
        
        while !cities.is_empty() {
            let mut province = HashSet::new();
            let mut q = Vec::new();

            let city = *cities.iter().next().unwrap();
            cities.remove(&city);
            q.push(city);

            while !q.is_empty() {
                let city = q.pop().unwrap();
                province.insert(city);

                for i in 0..is_connected.len() as i32 {
                    if i == city {
                        continue;
                    }
                    if is_connected[city as usize][i as usize] == 1 {
                        if !province.contains(&i) {
                            q.push(i);
                        }
                    }
                }
            }
            for city in province.iter() {
                cities.remove(&city);
            }
            provinces.push(province);
        }
        return provinces.len() as i32
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob() {
        let testcases = vec![
            (vec![vec![1,1,0], vec![1,1,0], vec![0,0,1]], 2),
            (vec![vec![1,0,0], vec![0,1,0], vec![0,0,1]], 3),            
            (vec![vec![1,1,0,0], vec![1,1,0,0], vec![0,0,1,1], vec![0,0,1,1]], 2),            
        ];
        for (nums, result) in testcases {
            if Solution::find_circle_num(nums.clone()) != result {
                println!("{:?}", nums);
                panic!();
            }
        }
    }
}
