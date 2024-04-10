use std::iter::zip;

fn vecs_eq<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    a.len() == b.len() && zip(a, b).all(|(x, y)| x == y)
}

pub fn test() {
    let test_cases = [
        (10, vec![10, 5, 16, 8, 4, 2, 1]),
        (32, vec![32, 16, 8, 4, 2, 1]),
        (85, vec![85, 256, 128, 64, 32, 16, 8, 4, 2, 1]),
    ];

    for (test_case, expected_result) in test_cases {
        let mut result = Vec::<i32>::new();
        collatz(test_case, &mut result);

        if vecs_eq(&result, &expected_result) {
            println!("{} -> {:?} (correct)", test_case, result);
        } else {
            println!(
                "{} -> {:?} (incorrect, expected {:?})",
                test_case, result, expected_result
            );
        }
    }
}

fn step(n: i32) -> i32 {
    match n % 2 {
        0 => ((n as f32) / 2.0).round() as i32,
        _ => (3 * n) + 1,
    }
}

fn collatz(n: i32, out: &mut Vec<i32>) {
    if n == 1 {
        out.push(1);
    } else {
        let c = step(n);
        out.push(n);
        collatz(c, out);
    }
}
