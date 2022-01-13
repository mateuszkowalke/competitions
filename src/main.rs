fn main() {}

use std::convert::TryFrom;

fn add(num1: u32, num2: u32) -> u64 {
    let num1: Vec<u32> = num1
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let num2: Vec<u32> = num2
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let front: Vec<u32>;
    let rest: Vec<u32>;
    let d_len: i32 = i32::try_from(num1.len()).unwrap() - i32::try_from(num2.len()).unwrap();

    if d_len < 0 {
        front = num2[0..num2.len() - num1.len()].to_vec();
        rest = num1
            .iter()
            .zip(&num2[num2.len() - num1.len()..])
            .map(|(num1, num2)| num1 + num2)
            .collect();
    } else if d_len > 0 {
        front = num1[0..num1.len() - num2.len()].to_vec();
        rest = num2
            .iter()
            .zip(&num1[num1.len() - num2.len()..])
            .map(|(num1, num2)| num1 + num2)
            .collect();
    } else {
        let res: Vec<u32> = num2
            .iter()
            .zip(&num1)
            .map(|(num1, num2)| num1 + num2)
            .collect();
        return res
            .into_iter()
            .map(|i| i.to_string())
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
    }

    [front, rest]
        .concat()
        .into_iter()
        .map(|i| i.to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

#[test]
fn test_real_add() {
    assert_eq!(add(2, 11), 13);
    assert_eq!(add(0, 1), 1);
    assert_eq!(add(0, 0), 0);
}

#[test]
fn test_silly_add() {
    assert_eq!(add(16, 18), 214);
    assert_eq!(add(26, 39), 515);
    assert_eq!(add(122, 81), 1103);
}
