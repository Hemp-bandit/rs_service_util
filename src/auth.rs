pub fn gen_access_value(bit: u64) -> u64 {
    let mod_val = bit % 31;
    let last_number = 1 << (mod_val.min(31) - 1);
    last_number
}

pub fn marge_access(arr: Vec<u64>) -> u64 {
    let mut res = 0;
    arr.into_iter().for_each(|val| {
        res += val;
    });
    res
}

pub fn has_access(auth: u64, access: Vec<u64>) -> bool {
    let mut res = false;
    access.into_iter().for_each(|val| {
        res = val & auth > 0;
    });
    res
}
