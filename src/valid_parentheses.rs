// https://leetcode.com/problems/valid-parentheses/

use std::collections::HashMap;

const TEST_CASES: [(&str, bool); 7] = [
    ("()[]{}", true),
    ("(([])){[()][]}", true),
    ("())[]{}", false),
    ("[(])", false),
    ("(", false),
    ("((", false),
    ("))", false),
];

pub fn test() {
    for (test_case, expected_result) in TEST_CASES {
        let result = validate_parentheses(test_case);
        println!("\"{}\" -> {} ({})", test_case, result, if result == expected_result { "correct" } else {"incorrect"});
    }
}

fn validate_parentheses(parentheses: &str) -> bool {
    let parentheses_map = HashMap::from([
        ('}', '{'),
        (']', '['),
        (')', '('),
    ]);
    let open_parentheses = parentheses_map.values().collect::<Vec<&char>>();

    let mut stack = Vec::<char>::new();

    for p in parentheses.chars() {
        if open_parentheses.contains(&&p) {
            stack.push(p);
        } else {
            let popped = stack.pop();

            if popped.is_none() || popped.unwrap() != parentheses_map[&p] {
                return false;
            }
        }
    }

    return stack.len() == 0;
}
