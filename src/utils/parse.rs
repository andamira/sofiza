/// Receive a string, try to parse it as f32
///
pub(crate) fn check_f32(value: &str) -> f32 {
    let num: f32 = value
        .parse::<f32>()
        .unwrap_or_else(|_| panic!("ERROR: `{}` is not a valid f32 number", value));
    num
}

/// Receive a string, try to parse it as f32 between a given range
///
// NOTE: floating-point types cannot be used in patterns
// https://github.com/rust-lang/rust/issues/41620
// solution from: https://stackoverflow.com/a/58434531/940200
///
pub(crate) fn check_f32_between(value: &str, min: f32, max: f32) -> Option<f32> {
    let num = check_f32(value);
    match num {
        num if (min..=max).contains(&num) => Some(num),
        _ => None,
    }
}

/// Receive a string, try to parse it as u8
///
pub(crate) fn check_u8(value: &str) -> u8 {
    let num: u8 = value
        .parse::<u8>()
        .unwrap_or_else(|_| panic!("ERROR: `{}` is not a valid i8 number", value));
    num
}

/// Receive a string, try to parse it as u8 between a given range
///
pub(crate) fn check_u8_between(value: &str, min: u8, max: u8) -> Option<u8> {
    let num = check_u8(value);
    if num >= min && num <= max {
        Some(num)
    } else {
        None
    }
}

/// Receive a string, try to parse it as i8
///
pub(crate) fn check_i8(value: &str) -> i8 {
    let num: i8 = value
        .parse::<i8>()
        .unwrap_or_else(|_| panic!("ERROR: `{}` is not a valid i8 number", value));
    num
}

/// Receive a string, try to parse it as i8 between a given range
///
pub(crate) fn check_i8_between(value: &str, min: i8, max: i8) -> Option<i8> {
    let num = check_i8(value);
    if num >= min && num <= max {
        Some(num)
    } else {
        None
    }
}

/// Receive a string, try to parse it as i16
///
pub(crate) fn check_i16(value: &str) -> i16 {
    let num: i16 = value
        .parse::<i16>()
        .unwrap_or_else(|_| panic!("ERROR: `{}` is not a valid i16 number", value));
    num
}

/// Receive a string, try to parse it as i16 between a given range
///
pub(crate) fn check_i16_between(value: &str, min: i16, max: i16) -> Option<i16> {
    let num = check_i16(value);
    if num >= min && num <= max {
        Some(num)
    } else {
        None
    }
}

/// Receive a string, try to parse it as u16
///
pub(crate) fn check_u16(value: &str) -> u16 {
    let num: u16 = value
        .parse::<u16>()
        .unwrap_or_else(|_| panic!("ERROR: `{}` is not a valid u16 number", value));
    num
}

/// Receive a string, try to parse it as u16 between a given range
///
pub(crate) fn check_u16_between(value: &str, min: u16, max: u16) -> Option<u16> {
    let num = check_u16(value);
    if num >= min && num <= max {
        Some(num)
    } else {
        None
    }
}

/// Receive a string, try to parse it as u32
///
pub(crate) fn check_u32(value: &str) -> u32 {
    let num: u32 = value
        .parse::<u32>()
        .unwrap_or_else(|_| panic!("ERROR: `{}` is not a valid u32 number", value));
    num
}

/// Receive a string, try to parse it as u32 between a given range
///
pub(crate) fn check_u32_between(value: &str, min: u32, max: u32) -> Option<u32> {
    let num = check_u32(value);
    if num >= min && num <= max {
        Some(num)
    } else {
        None
    }
}

/*
// IDEA:WIP create wrapper macro
// check!(value, f32, 0.1, 100.)
//
// https://stackoverflow.com/questions/34214136/how-do-i-match-the-type-of-an-expression-in-a-rust-macro
// problem: know what to return
//
#[macro_export]
macro_rules! check {
    ( $value:expr, f32, $min:expr, $max:expr  ) => {
        let num = utils::check_f32_between($value, $min, $max);
        num.and(Some(Opcode::pan(num.unwrap_or_default())))
    };
}
*/

// IDEA:WIP make helper (generic?) function for parsing numbers
//
// pub(crate) fn test_helper<T: FromStr + Debug>(value: &str) -> T {
//     print_type(value);
//     let num  = value.parse::<T>()
//         .expect(&format!("ERROR: `{}` is not a valid {:?} number", value,
//                 "T"));
//                 //core::any::type_name::<f32>));
//     num
// }

// impl<T> Debug for <T as FromStr>::Err {
// }
