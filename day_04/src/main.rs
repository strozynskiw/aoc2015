use md5;

fn get_hash(key: &str, index: u32) -> u32 {
    let hash = md5::compute(format!("{}{}", key, index));
    ((hash[0] as u32) << 24) + ((hash[1] as u32) << 16) + ((hash[2] as u32) << 8) + (hash[3] as u32)
}

fn part_1(input: &str) -> u32 {
    for i in 0.. {
        let hash = get_hash(input, i);
        if i % 5000 == 0 {
            println!("Index: {}", i);
        }
        if hash & 0xFFFF_F000 == 0 {
            println!("Index {}, hash {:08x}", i, hash);
            return i;
        }
    }
    0
}

fn part_2(input: &str) -> u32 {
    for i in 0.. {
        let hash = get_hash(input, i);
        if i % 50000 == 0 {
            println!("Index: {}", i);
        }
        if hash & 0xFFFF_FF00 == 0 {
            println!("Index {}, hash {:08x}", i, hash);
            return i;
        }
    }
    0
}

fn main() {
    let key = "yzbqklnj";
    println!("Result1: {}", part_1(&key));
    println!("Result2: {}", part_2(&key));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(609043, part_1("abcdef"));
        assert_eq!(282749, part_1("yzbqklnj"));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(9962624, part_2("yzbqklnj"));
    }
}
