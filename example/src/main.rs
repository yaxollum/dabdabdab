use sha2::{Digest, Sha256};
use std::io;
use std::io::Write;

fn get_buff(n: i64) -> Vec<u8> {
    const LEN: i64 = 37;
    let mut numbers: Vec<i64> = (0..LEN).collect();
    let mut buff = io::Cursor::new(Vec::<u8>::new());
    for i in 0..LEN {
        let j = (n % (LEN - i)) as usize;
        write!(buff, "{}", numbers[j]).unwrap();
        if i != LEN - 1 {
            write!(buff, " ").unwrap();
        }
        numbers.remove(j);
    }
    buff.into_inner()
}

fn is_dab(buff: &[u8]) -> bool {
    let digest = Sha256::digest(buff);
    digest[0] == 0xda
        && digest[1] == 0xbd
        && digest[2] == 0xab
        && digest[3] == 0xda
        && digest[4] / 16 == 0xb
}

fn main() {
    for n in 420000000000000.. {
        if n % 1000000 == 0 {
            println!("{}", n)
        }
        let str = get_buff(n);
        if is_dab(&str) {
            println!("{}\n{:?}", n, String::from_utf8(str).unwrap());
            return;
        }
    }
}
