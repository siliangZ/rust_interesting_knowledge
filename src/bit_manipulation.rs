fn get_bit_at(num: i32, bit_loc: u8) -> bool {
    let result = num & (1 << bit_loc);
    println!("{result:b}");
    result != 0
}

fn set_bit_at(num: i32, bit_loc: u8) -> i32 {
    num | (1 << bit_loc)
}

fn clear_bit_at(num: i32, bit_loc: u8) -> i32 {
    num & (!(1 << bit_loc))
}

// clear all significant bits upto loc(including)
fn clear_bit_to(num: i32, bit_loc: u8) -> i32 {
    num & ((1 << bit_loc) - 1)
}

// clear all significant bits from loc(including)
fn clear_bit_from(num: i32, bit_loc: u8) -> i32 {
    num & (-1 << (bit_loc + 1))
}

fn update_bit_at(num: i32, bit_loc: u8, value: bool) -> i32 {
    let num = clear_bit_at(num, bit_loc);
    num | ((value as i32) << bit_loc)
}
