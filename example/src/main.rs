use sha2::{Digest, Sha256};
use std::env;
use std::io;
use std::io::Write;
use std::process::exit;

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

fn post_result(worker_id: &str, result: &str) {
    let form = [("worker_id", worker_id), ("result", result)];
    ureq::post("http://localhost:6969/results")
        .send_form(form)
        .expect("Error POSTing result");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: {} <worker id> <initial N>", args[0]);
        exit(1);
    }
    let worker_id = &args[1];
    let starting_n = args[2].parse::<i64>().expect("Failed to parse starting N.");
    for n in starting_n.. {
        if n % 1000000 == 0 {
            post_result(worker_id, &format!("{}", n))
        }
        let str = get_buff(n);
        if is_dab(&str) {
            post_result(
                worker_id,
                &format!("{}\n{:?}", n, String::from_utf8(str).unwrap()),
            );
            return;
        }
    }
}
