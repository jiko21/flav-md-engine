pub mod table {
    use crate::lexer::lexer::lexer::{Align, Table, TableHead};
    use once_cell::sync::Lazy;
    use regex::Regex;

    static TABLE_HEAD_PATTERN: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(?:\s?(.+?)\s?\|)+?").unwrap());

    static LEFT_COLUMN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^:-+$").unwrap());

    static CENTER_COLUMN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^:-+:$").unwrap());

    static RIGHT_COLUMN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^-+:$").unwrap());

    pub fn is_table_block_start(input: &String) -> bool {
        if input.chars().nth(0).unwrap() == '|' && input.chars().last().unwrap() == '|' {
            TABLE_HEAD_PATTERN.is_match(&input[1..])
        } else {
            false
        }
    }

    fn get_table_column_name(input: &String) -> Vec<String> {
        let mut rslt: Vec<String> = vec![];
        for mat in TABLE_HEAD_PATTERN.captures_iter(input) {
            rslt.push(mat.get(1).map_or("", |m| m.as_str()).trim().to_string())
        }
        rslt
    }

    fn get_column_align(input: &String) -> Vec<Align> {
        let mut rslt: Vec<Align> = vec![];
        for mat in TABLE_HEAD_PATTERN.captures_iter(input) {
            let cell = mat.get(1).map_or("", |m| m.as_str()).trim().to_string();
            if LEFT_COLUMN.is_match(&cell) {
                rslt.push(Align::Left);
            } else if RIGHT_COLUMN.is_match(&cell) {
                rslt.push(Align::Right);
            } else if CENTER_COLUMN.is_match(&cell) {
                rslt.push(Align::Center);
            }
        }
        rslt
    }

    fn get_table_head_info(input: Vec<String>) -> Vec<TableHead> {
        let head = get_table_column_name(&input.get(0).unwrap()[1..].to_string());
        let align = get_column_align(&input.get(1).unwrap()[1..].to_string());
        println!("head: {:?}", head);
        println!("align: {:?}", align);
        let mut rslt: Vec<TableHead> = vec![];
        for i in 0..head.len() {
            rslt.push(TableHead::new(
                head.get(i).unwrap().to_string(),
                *align.get(i).unwrap(),
            ))
        }
        rslt
    }

    fn parse_table_body(input: Vec<String>) -> (Vec<Vec<String>>, usize) {
        let mut now_at: usize = 0;
        let mut rows: Vec<Vec<String>> = vec![];
        for item in input.into_iter() {
            if !is_table_block_start(&item) {
                break;
            }
            rows.push(get_table_column_name(&item[1..].to_string()));
            now_at += 1;
        }
        (rows, now_at)
    }

    pub fn parse_table(input: Vec<String>) -> (Table, usize) {
        let table_head = get_table_head_info(input[0..2].to_vec());
        let (rows, skip) = parse_table_body(input[2..].to_vec());
        (Table::new(table_head, rows), skip + 2)
    }

    #[cfg(test)]
    mod table_test {
        use super::*;
        use crate::{table, vec_string};

        #[test]
        fn test_is_table_block_start() {
            #[derive(Debug)]
            struct TestCase {
                it: String,
                input: String,
                expected: bool,
            }

            let test_cases = [
                TestCase {
                    it: String::from("should return true when input is table start"),
                    input: String::from("|  head1  | head2 | head3|"),
                    expected: true,
                },
                TestCase {
                    it: String::from("should return false when input is not table start1"),
                    input: String::from("  head1  | head2 | head3|"),
                    expected: false,
                },
                TestCase {
                    it: String::from("should return false when input is not table start2"),
                    input: String::from("|  head1  | head2 | head3"),
                    expected: false,
                },
            ];

            for test_case in test_cases.iter() {
                let output = is_table_block_start(&test_case.input);
                assert_eq!(output, test_case.expected, "Failed: {}\n", test_case.it);
            }
        }

        #[test]
        fn test_parse_table() {
            let input = vec_string![
                "|  head1  | head2 | head3|",
                "|:----:|-----:|:-----|",
                "|  aaa1  | bbb1 | ccc1|",
                "|  aaa2 | bbb2 | ccc2|",
            ];

            let expected = (
                table! {
                    head: vec![
                        TableHead::new("head1".to_string(), Align::Center),
                        TableHead::new("head2".to_string(), Align::Right),
                        TableHead::new("head3".to_string(), Align::Left),
                    ],
                    body: vec![
                        vec_string!["aaa1", "bbb1", "ccc1"],
                        vec_string!["aaa2", "bbb2", "ccc2"],
                    ],
                },
                4,
            );

            let output = parse_table(input);
            assert_eq!(output.0, expected.0);
        }
    }
}
