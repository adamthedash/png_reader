pub fn read_be_u32_mut(input: &mut &[u8]) -> u32 {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u32>());
    *input = rest;
    u32::from_be_bytes(int_bytes.try_into().unwrap())
}

pub fn read_str_mut(input: &mut &[u8], num_bytes: usize) -> String {
    let (int_bytes, rest) = input.split_at(num_bytes);
    *input = rest;
    String::from_utf8(int_bytes.to_vec()).unwrap()
}


pub fn read_be_u32(input: &[u8]) -> u32 {
    let (int_bytes, _) = input.split_at(std::mem::size_of::<u32>());
    u32::from_be_bytes(int_bytes.try_into().unwrap())
}

pub fn read_be_u16(input: &[u8]) -> u16 {
    let (int_bytes, _) = input.split_at(std::mem::size_of::<u16>());
    u16::from_be_bytes(int_bytes.try_into().unwrap())
}

pub fn read_until_null(input: &[u8]) -> Vec<u8> {
    input.iter()
        .take_while(|&&x| x != 0)
        .copied()
        .collect()
}