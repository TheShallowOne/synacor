use alloc::{String, Vec};

pub fn u16_to_string(val: u16) -> String {
    let mut digits = Vec::new();
    let mut val = val;
    loop {
        digits.push(digit_to_str(val % 10));

        if 0 < val {
            val /= 10;
        } else {
            break;
        }
    }

    digits.reverse();
    String::from_utf8_lossy(&digits).into_owned()
}

fn digit_to_str(val: u16) -> u8 {
    let c = match val {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        _ => '_',
    };
    c as u8
}
