use std::env;
use std::fs::File;
use std::collections::HashMap;
use std::io::{self, Error, ErrorKind, Read};

use regex::Regex;
use termion::event::{Event, Key, MouseEvent};


#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum Action {
    LeftClick(u16, u16),
    Release(u16, u16),
    Char(char),
    UnselectRet,
    ScrollUp,
    ScrollDown,
    DeleteSelected,
    SelectUp,
    SelectDown,
    SelectLeft,
    SelectRight,
    EraseChar,
    CreateSibling,
    CreateChild,
    CreateFreeNode,
    ExecSelected,
    DrillDown,
    PopUp,
    PrefixJump,
    ToggleCompleted,
    ToggleHideCompleted,
    Arrow,
    Arrange,
    AutoArrange,
    ToggleCollapsed,
    Quit,
    Save,
    ToggleShowLogs,
    EnterCmd,
    FindTask,
    YankPasteNode,
}

fn str_to_action(input: String) -> Option<Action> {
    match &*input {
        "unselect" => Some(Action::UnselectRet),
        "scroll_up" => Some(Action::ScrollUp),
        "scroll_down" => Some(Action::ScrollDown),
        "delete" => Some(Action::DeleteSelected),
        "select_up" => Some(Action::SelectUp),
        "select_down" => Some(Action::SelectDown),
        "select_left" => Some(Action::SelectLeft),
        "select_right" => Some(Action::SelectRight),
        "erase" => Some(Action::EraseChar),
        "create_sibling" => Some(Action::CreateSibling),
        "create_child" => Some(Action::CreateChild),
        "create_free_node" => Some(Action::CreateFreeNode),
        "execute" => Some(Action::ExecSelected),
        "drill_down" => Some(Action::DrillDown),
        "pop_up" => Some(Action::PopUp),
        "jump" => Some(Action::PrefixJump),
        "toggle_completed" => Some(Action::ToggleCompleted),
        "toggle_hide_completed" => Some(Action::ToggleHideCompleted),
        "arrow" => Some(Action::Arrow),
        "arrange" => Some(Action::Arrange),
        "auto_arrange_view" => Some(Action::AutoArrange),
        "toggle_collapsed" => Some(Action::ToggleCollapsed),
        "quit" => Some(Action::Quit),
        "save" => Some(Action::Save),
        "toggle_show_logs" => Some(Action::ToggleShowLogs),
        "enter_command" => Some(Action::EnterCmd),
        "find_task" => Some(Action::FindTask),
        "yank_paste_node" => Some(Action::YankPasteNode),
        _ => None,
    }
}

fn str_to_key(input: String) -> Option<Key> {
    use termion::event::Key::*;

    lazy_static! {
        static ref RE: Regex = Regex::new(r"C-(.)").unwrap();
    }

    match &*input {
        "esc" => Some(Esc),
        "pgup" => Some(PageUp),
        "pgdn" => Some(PageDown),
        "del" => Some(Delete),
        "up" => Some(Up),
        "down" => Some(Down),
        "left" => Some(Left),
        "right" => Some(Right),
        "backspace" => Some(Backspace),
        "enter" => Some(Char('\n')),
        "tab" => Some(Char('\t')),
        other => {
            RE.captures_iter(other)
                .nth(0)
                .and_then(|n| n.at(1))
                .and_then(|r| r.chars().nth(0))
                .map(|c| Ctrl(c))
        }
    }
}

pub struct Config {
    config: HashMap<Key, Action>,
}

impl Default for Config {
    fn default() -> Config {
        use termion::event::Key::*;
        Config {
            config: vec![
                (Esc, Action::UnselectRet),
                (PageUp, Action::ScrollUp),
                (PageDown, Action::ScrollDown),
                (Delete, Action::DeleteSelected),
                (Up, Action::SelectUp),
                (Down, Action::SelectDown),
                (Left, Action::SelectLeft),
                (Right, Action::SelectRight),
                (Backspace, Action::EraseChar),
                (F(1), Action::PrefixJump),
                (Char('\n'), Action::CreateSibling),
                (Char('\t'), Action::CreateChild),
                (Ctrl('n'), Action::CreateFreeNode),
                (Ctrl('k'), Action::ExecSelected),
                (Ctrl('w'), Action::DrillDown),
                (Ctrl('q'), Action::PopUp),
                (Ctrl('f'), Action::PrefixJump),
                (Ctrl('a'), Action::ToggleCompleted),
                (Ctrl('h'), Action::ToggleHideCompleted),
                (Ctrl('r'), Action::Arrow),
                (Ctrl('p'), Action::Arrange),
                (Ctrl('z'), Action::AutoArrange),
                (Ctrl('t'), Action::ToggleCollapsed),
                (Ctrl('c'), Action::Quit),
                (Ctrl('x'), Action::Save),
                (Ctrl('l'), Action::ToggleShowLogs),
                (Ctrl('e'), Action::EnterCmd),
                (Ctrl('v'), Action::FindTask),
                (Ctrl('y'), Action::YankPasteNode),
            ]
                .into_iter()
                .collect(),
        }
    }
}

impl Config {
    pub fn maybe_parsed_from_env() -> io::Result<Config> {
        if let Ok(p) = env::var("KEYFILE") {
            Config::parse_file(p)
        } else {
            Ok(Config::default())
        }
    }

    pub fn parse_file(p: String) -> io::Result<Config> {
        let mut buf = String::new();
        let mut f = File::open(p)?;
        f.read_to_string(&mut buf)?;
        let mut config = Config::default();
        for (line_number, line) in buf.lines().enumerate() {
            let e = format!("invalid config at line {}: {}", line_number, line);

            let parts: Vec<_> = line.split(":").map(|p| p.trim()).collect();
            if parts.len() != 2 {
                error!("{}", e);
                return Err(Error::new(ErrorKind::Other, e));
            }

            let (raw_action, raw_key) = (parts[0], parts[1]);

            let key_opt = str_to_key(raw_key.to_owned());
            let action_opt = str_to_action(raw_action.to_owned());

            if key_opt.is_none() || action_opt.is_none() {
                error!("{}", e);
                return Err(Error::new(ErrorKind::Other, e));
            }

            let key = key_opt.unwrap();
            let action = action_opt.unwrap();

            config.config.insert(key, action);
        }

        Ok(config)
    }

    pub fn map(&self, e: Event) -> Option<Action> {
        use termion::event::Key::*;
        info!("matching event {:?}", e);
        match e {
            Event::Key(Char(c)) => {
                if let Some(action) = self.config.get(&Char(c)).cloned() {
                    Some(action)
                } else {
                    Some(Action::Char(c))
                }
            }
            Event::Mouse(MouseEvent::Press(_, x, y)) => Some(Action::LeftClick(x, y)),
            Event::Mouse(MouseEvent::Release(x, y)) => Some(Action::Release(x, y)),
            Event::Mouse(MouseEvent::Hold(_, _)) => None,
            Event::Key(other) => {
                let lookup = self.config.get(&other).cloned();
                if lookup.is_none() {
                    warn!("Weird event {:?}", other);
                }
                lookup
            }
            Event::Unsupported => {
                warn!("Unsupported input event received");
                None
            }
        }
    }
}
