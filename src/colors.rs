use rand::{self, Rng};

pub fn random_fg_color() -> String {
    use termion::color::*;
    let colors: Vec<String> = vec![
        format!("{}", Fg(LightGreen)),
        // format!("{}", Fg(LightBlack)),
        format!("{}", Fg(LightRed)),
        format!("{}", Fg(LightGreen)),
        format!("{}", Fg(LightYellow)),
        // format!("{}", Fg(LightBlue)),
        format!("{}", Fg(LightMagenta)),
        format!("{}", Fg(LightCyan)),
        format!("{}", Fg(LightWhite)),
    ];
    let c = &*rand::thread_rng().choose(&*colors).unwrap();
    c.clone()
}
