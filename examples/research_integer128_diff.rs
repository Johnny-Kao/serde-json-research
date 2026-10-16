use std::io::Cursor;

fn inspect<T: serde::de::DeserializeOwned + std::fmt::Debug>(text: &str, mode: usize) -> String {
    let value = match mode {
        0 => serde_json::from_str::<T>(text),
        1 => serde_json::from_slice::<T>(text.as_bytes()),
        _ => serde_json::from_reader::<_, T>(Cursor::new(text.as_bytes())),
    };
    match value {
        Ok(v) => format!("ok:{v:?}"),
        Err(e) => format!("err:{:?}:{}:{}:{}", e.classify(), e.line(), e.column(), e),
    }
}

fn main() {
    let mut inputs: Vec<String> = vec![
        "".into(), " ".into(), "0".into(), "-0".into(), "1".into(),
        "-1".into(), "01".into(), "-01".into(), "-".into(), "+1".into(),
        "1e2".into(), "1.0".into(), "1e".into(), "-1e2".into(),
        "170141183460469231731687303715884105727".into(),
        "170141183460469231731687303715884105728".into(),
        "-170141183460469231731687303715884105728".into(),
        "-170141183460469231731687303715884105729".into(),
        "340282366920938463463374607431768211455".into(),
        "340282366920938463463374607431768211456".into(),
        "1x".into(), "1 2".into(), "1\n2".into(),
        "9999999999999999999999999999999999999999999999999999999999x".into(),
    ];
    for n in 1..=90 {
        for digit in ['0', '1', '8', '9'] {
            let s = digit.to_string().repeat(n);
            inputs.push(s.clone());
            inputs.push(format!("-{s}"));
            inputs.push(format!("{s}x"));
        }
    }
    let mut state: u64 = 0x8f6a_9b1d_72c3_45ef;
    for _ in 0..12000 {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let n = (state as usize % 100) + 1;
        let mut s = String::new();
        if state & 1 != 0 { s.push('-'); }
        let mut value = state;
        for _ in 0..n {
            value = value.wrapping_mul(6364136223846793005).wrapping_add(1);
            s.push(char::from(b'0' + (value % 10) as u8));
        }
        if state & 4 != 0 { s.push('x'); }
        if state & 8 != 0 { s.push_str(".0"); }
        inputs.push(s);
    }
    for (id, input) in inputs.iter().enumerate() {
        for mode in 0..3 {
            println!("{id}|{mode}|i|{}", inspect::<i128>(input, mode));
            println!("{id}|{mode}|u|{}", inspect::<u128>(input, mode));
        }
    }
}
