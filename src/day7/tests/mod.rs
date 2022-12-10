use super::*;

#[test]
fn test_directory_size() {
    let test_directory = Directory {
        files: vec![
            File {
                name: "a".to_string(),
                size: 123,
            },
            File {
                name: "b".to_string(),
                size: 234,
            },
        ],
        directories: vec![Directory {
            files: vec![File {
                name: "c".to_string(),
                size: 456,
            }],
            directories: vec![
                Directory {
                    files: vec![
                        File {
                            name: "d".to_string(),
                            size: 567,
                        },
                        File {
                            name: "e".to_string(),
                            size: 678,
                        },
                    ],
                    directories: vec![],
                },
                Directory {
                    files: vec![File {
                        name: "f".to_string(),
                        size: 89,
                    }],
                    directories: vec![],
                },
            ],
        }],
    };

    assert_eq!(2147, test_directory.size());
}
