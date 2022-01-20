use crate::std_iter;

fn decode_string(s: &[u8]) -> Option<Vec<u8>> {
    match s {
        [b'"', inner @ .., b'"'] => decode_inner_string(inner, vec![]),
        _ => None,
    }
}

fn decode_inner_string(s: &[u8], mut so_far: Vec<u8>) -> Option<Vec<u8>> {
    match s {
        [] => Some(so_far),
        [b'\\', c @ b'"' | c @ b'\\', tail @ ..] => {
            so_far.push(*c);
            decode_inner_string(tail, so_far)
        }
        [b'\\', b'x', a @ b'0'..=b'9' | a @ b'a'..=b'f', b @ b'0'..=b'9' | b @ b'a'..=b'f', tail @ ..] =>
        {
            let num = u8::from_str_radix(std::str::from_utf8(&[*a, *b]).unwrap(), 16).unwrap();
            so_far.push(num);
            decode_inner_string(tail, so_far)
        }
        [c, tail @ ..] => {
            so_far.push(*c);
            decode_inner_string(tail, so_far)
        }
    }
}

fn encode_string(s: &[u8]) -> Vec<u8> {
    let mut builder = vec![b'"'];
    for c in s {
        match *c {
            b'"' => builder.extend([b'\\', b'"']),
            b'\\' => builder.extend([b'\\', b'\\']),
            c => builder.push(c),
        };
    }
    builder.push(b'"');
    builder
}

pub fn part1() {
    let result: usize = std_iter!(Lines)
        .map(|l| l.len() - decode_string(l.as_bytes()).unwrap().len())
        .sum();
    println!("{}", result);
}

pub fn part2() {
    let result: usize = std_iter!(Lines)
        .map(|l| encode_string(l.as_bytes()).len() - l.len())
        .sum();
    println!("{}", result);
}
