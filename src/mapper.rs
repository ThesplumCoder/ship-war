use std::str::FromStr;

pub fn str_to_coordinates(input: &String) -> [u8; 2] {
    let clean_str = input.trim().to_string();
    let numbers: Vec<&str> = clean_str.split(",").collect();
    let mut res: [u8; 2] = [0, 0];

    res[0] = u8::from_str(numbers.get(0).unwrap()).unwrap();
    res[1] = u8::from_str(numbers.get(1).unwrap()).unwrap();
    return res;
}