pub fn get_key_primitive_type(i: usize) -> u32 {
    return (2 * i + 1) as u32;
}

pub fn get_value_primitive_type(i: usize) -> u128 {
    return (2 * i + 1) as u128;
}

pub fn get_key_custom_type(i: usize) -> String {
    return format!("key: {}", get_key_primitive_type(i));
}

pub fn get_value_custom_type(i: usize) -> String {
    return format!("value: {}", get_value_primitive_type(i));
}
