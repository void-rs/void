use rand::{self, seq::SliceRandom};

pub fn random_fg_color() -> String {
    use termion::color::*;
    let colors: Vec<String> = vec![
        format!("{}", Fg(LightGreen)),
        format!("{}", Fg(LightBlack)),
        format!("{}", Fg(LightRed)),
        format!("{}", Fg(LightGreen)),
        //format!("{}", Fg(LightYellow)),
        format!("{}", Fg(LightBlue)),
        format!("{}", Fg(LightMagenta)),
        format!("{}", Fg(LightCyan)),
        //format!("{}", Fg(LightWhite)),
        format!("{}", Fg(Green)),
        format!("{}", Fg(Black)),
        format!("{}", Fg(Red)),
        format!("{}", Fg(Green)),
        format!("{}", Fg(Yellow)),
        format!("{}", Fg(Blue)),
        format!("{}", Fg(Magenta)),
        format!("{}", Fg(Cyan)),
        //format!("{}", Fg(White)),
    ];
    colors.choose(&mut rand::thread_rng()).unwrap().clone()
}
