use crossterm::terminal::disable_raw_mode;

pub fn quit() {
    disable_raw_mode();
}

pub fn fmt_time(time: i64) -> (usize, String) {
    if time == 0 {
        return (0, String::new());
    }
    let mut time_new = time;
    let mut res = Vec::new();
    let mut num = 0;
    while time_new > 0 {
        num += 1;
        let val = time_new % 60;
        if val > 10 {
            res.push(val.to_string());
        } else {
            res.push(format!("0{}", val.to_string()));
        }
        time_new /= 60;
    }
    res.reverse();
    (num, res.join(":"))
}
