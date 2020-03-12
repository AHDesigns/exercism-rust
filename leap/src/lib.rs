pub fn is_leap_year(year: u64) -> bool {
    let div_by = c_div_by(year);
    match year {
        _ if div_by(400) => true,
        _ if div_by(100) => false,
        _ if div_by(4) => true,
        _ => false,
    }
}

fn c_div_by(y: u64) -> impl Fn(u64) -> bool {
    move |num| y % num == 0
}
