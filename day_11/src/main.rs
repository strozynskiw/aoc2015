fn verify_increasing_letters(input: &str) -> bool {
    input
        .as_bytes()
        .windows(3)
        .any(|window| window[1] == window[0] + 1 && window[2] == window[1] + 1)
}

fn verify_forbidden_letters(input: &str) -> bool {
    let forbidden = ['i', 'u', 'l'];
    !input.chars().any(|c| forbidden.contains(&c))
}

fn verify_double(input: &str) -> bool {
    input
        .as_bytes()
        .windows(2)
        .enumerate()
        .any(|(index, first)| {
            input.as_bytes().windows(2).skip(index + 2).any(|second| {
                first[0] == first[1] && second[0] == second[1] && first[0] != second[0]
            })
        })
}

fn verify(input: &[u8]) -> bool {
    let text = String::from_utf8(input.to_vec()).unwrap();
    verify_increasing_letters(&text) && verify_forbidden_letters(&text) && verify_double(&text)
}

fn get_next_password(input: &str) -> String {
    let mut passwd: Vec<u8> = input.as_bytes().to_vec();
    assert_eq!(8, passwd.len());
    let forbidden = ['i', 'u', 'l'];
    loop {
        passwd[7] += 1;
        for i in (0..8).rev() {
            if forbidden.contains(&(passwd[i] as char)) {
                passwd[i] += 1;
            }
            if passwd[i] > b'z' {
                passwd[i] = b'a';
                passwd[i - 1] += 1;
            }

            if verify(&passwd) {
                return String::from(std::str::from_utf8(&passwd).unwrap());
            }
        }
    }
}

fn main() {
    let r1 = get_next_password("hepxcrrq");
    let r2 = get_next_password("hepxxyzz");

    println!("Result1: {}", r1);
    println!("Result2: {}", r2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_increasing_letters() {
        assert_eq!(true, verify_increasing_letters("hijklmmn"));
        assert_eq!(false, verify_increasing_letters("abbceffg"));
        assert_eq!(false, verify_increasing_letters("abbcegjk"));
    }

    #[test]
    fn test_verify_forbidden_letters() {
        assert_eq!(false, verify_forbidden_letters("hijklmmn"));
        assert_eq!(true, verify_forbidden_letters("abbceffg"));
        assert_eq!(true, verify_forbidden_letters("abbcegjk"));
    }

    #[test]
    fn test_verify_double() {
        assert_eq!(false, verify_double("hijklmmn"));
        assert_eq!(true, verify_double("abbceffg"));
        assert_eq!(false, verify_double("abbcegjk"));
    }
}
