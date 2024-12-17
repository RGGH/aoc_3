#![allow(unused)]
use regex::Regex;
use std::io::{BufReader, Read};

struct Tup(i32, i32);

fn main() {
    let data = import_data();
    let res = parse_it(data);
    let answer = sum_result(res);
    println!("{:?}", answer);
}

fn import_data() -> String {
    let mut f = std::fs::File::open("data.txt").expect("check file exists");
    let mut buf = String::new();
    let res = f.read_to_string(&mut buf).unwrap();
    buf
}

fn parse_it(input: String) -> Vec<String> {
    // regex the good stuff : mul(x,y)
    let re = Regex::new(r"mul\(\d*,\d*\)").unwrap();
    let hay = input;
    let mul_tuples = re
        .find_iter(&hay)
        .map(|m| m.as_str()[4..m.as_str().len() - 1].to_string())
        .collect();
    mul_tuples
}

fn sum_result(input: Vec<String>) -> i32 {
    let mut total = 0;
    for v in input {
        let numbers: Vec<i32> = v
            .split(',')
            .filter_map(|x| x.trim().parse::<i32>().ok())
            .collect();
                if numbers.len() == 2 {
            total += numbers[0] * numbers[1];
        } else {
            eprintln!("Malformed input: {}", v);
        }
    }
    total 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_result() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let expected = 161;
        let actual = sum_result();
        assert_eq!(expected, actual);
    }
}
