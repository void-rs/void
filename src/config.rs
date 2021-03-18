use std::{
    collections::HashMap,
    env, fmt,
    fs::File,
    io::{self, Error, ErrorKind, Read},
};

use termion::event::{Event, Key, MouseEvent};

#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum Action {
    LeftClick(u16, u16),
    RightClick(u16, u16),
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
    AutoArrange,
    ToggleCollapsed,
    Quit,
    Save,
    ToggleShowLogs,
    EnterCmd,
    FindTask,
    YankPasteNode,
    RaiseSelected,
    LowerSelected,
    Search,
    UndoDelete,
    Help,
    SelectParent,
    SelectNextSibling,
    SelectPrevSibling,
    Insert,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::LeftClick(..) | Action::RightClick(..) | Action::Release(..) => {
                write!(f, "Other action")
            }
            Action::Arrow => write!(f, "Start or end arrow"),
            Action::AutoArrange => write!(f, "Toggle automatic arrangement"),
            Action::Char(c) => write!(f, "Input character {}", c),
            Action::CreateChild => write!(f, "Create new child node"),
            Action::CreateFreeNode => write!(f, "Create new free node"),
            Action::CreateSibling => write!(f, "Create new sibling node"),
            Action::DeleteSelected => write!(f, "Delete selected node"),
            Action::DrillDown => write!(f, "Move down in hierarchy"),
            Action::EnterCmd => write!(f, "Enter command"),
            Action::EraseChar => write!(f, "Erase character"),
            Action::ExecSelected => write!(f, "Execute node content"),
            Action::FindTask => write!(f, "Autoassign task"),
            Action::Help => write!(f, "Display help"),
            Action::Insert => write!(f, "Enter insert mode"),
            Action::LowerSelected => write!(f, "Move selected node down"),
            Action::PopUp => write!(f, "Move up in hierarchy"),
            Action::PrefixJump => write!(f, "Select by prefix"),
            Action::Quit => write!(f, "Quit void"),
            Action::RaiseSelected => write!(f, "Move selected node up"),
            Action::Save => write!(f, "Save"),
            Action::ScrollDown => write!(f, "Scroll view down"),
            Action::ScrollUp => write!(f, "Scroll view up"),
            Action::Search => write!(f, "Search for node"),
            Action::SelectDown => write!(f, "Select next node down"),
            Action::SelectLeft => write!(f, "Select next node left"),
            Action::SelectNextSibling => write!(f, "Select next sibling"),
            Action::SelectParent => write!(f, "Select parent node"),
            Action::SelectPrevSibling => write!(f, "Select previous sibling"),
            Action::SelectRight => write!(f, "Select next node right"),
            Action::SelectUp => write!(f, "Select next node up"),
            Action::ToggleCollapsed => write!(f, "Toggle collapsing of children"),
            Action::ToggleCompleted => write!(f, "Toggle completed"),
            Action::ToggleHideCompleted => write!(f, "Toggle hiding of completed tasks"),
            Action::ToggleShowLogs => write!(f, "Toggle log"),
            Action::UndoDelete => write!(f, "Undo deletion"),
            Action::UnselectRet => write!(f, "Unselect node / leave insert mode"),
            Action::YankPasteNode => write!(f, "Yank node"),
        }
    }
}

fn to_action(input: String) -> Option<Action> {
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
        "auto_arrange" => Some(Action::AutoArrange),
        "toggle_collapsed" => Some(Action::ToggleCollapsed),
        "quit" => Some(Action::Quit),
        "save" => Some(Action::Save),
        "toggle_show_logs" => Some(Action::ToggleShowLogs),
        "enter_command" => Some(Action::EnterCmd),
        "find_task" => Some(Action::FindTask),
        "yank_paste_node" => Some(Action::YankPasteNode),
        "raise_selected" => Some(Action::RaiseSelected),
        "lower_selected" => Some(Action::LowerSelected),
        "search" => Some(Action::Search),
        "undo_delete" => Some(Action::UndoDelete),
        "help" => Some(Action::Help),
        "select_parent" => Some(Action::SelectParent),
        "select_next_sibling" => Some(Action::SelectNextSibling),
        "select_prev_sibling" => Some(Action::SelectPrevSibling),
        "insert" => Some(Action::Insert),
        _ => None,
    }
}

// Alt and Control must be specified with capital letters C- and A-
fn to_key(raw_key: String) -> Option<Key> {
    use termion::event::Key::{Alt, Char, Ctrl};

    fn extract_key(raw_key: &str, idx: usize) -> Option<char> {
        raw_key.chars().nth(idx)
    }

    match &*raw_key {
        "esc" => Some(Key::Esc),
        "pgup" => Some(Key::PageUp),
        "pgdn" => Some(Key::PageDown),
        "del" => Some(Key::Delete),
        "backspace" => Some(Key::Backspace),
        "up" => Some(Key::Up),
        "down" => Some(Key::Down),
        "left" => Some(Key::Left),
        "right" => Some(Key::Right),

        "space" => Some(Char(' ')),
        "enter" => Some(Char('\n')),
        "tab" => Some(Char('\t')),

        key if key.len() == 1 => extract_key(key, 0).map(Char),

        key if key.starts_with("A-") => extract_key(key, 2).map(Alt),
        key if key.starts_with("C-") => extract_key(key, 2).map(Ctrl),

        _ => None,
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    config: HashMap<Key, Action>,
    pub modal: bool,
    pub stricken: String,
    pub collapsed: String,
    pub hide_stricken: String,
    pub free_text: String,
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
                (Ctrl('p'), Action::AutoArrange),
                (Ctrl('t'), Action::ToggleCollapsed),
                (Ctrl('c'), Action::Quit),
                (Ctrl('x'), Action::Save),
                (Ctrl('l'), Action::ToggleShowLogs),
                (Ctrl('e'), Action::EnterCmd),
                (Ctrl('v'), Action::FindTask),
                (Ctrl('y'), Action::YankPasteNode),
                (Ctrl('g'), Action::RaiseSelected),
                (Ctrl('d'), Action::LowerSelected),
                (Ctrl('u'), Action::Search),
                (Ctrl('z'), Action::UndoDelete),
                (Ctrl('?'), Action::Help),
                (Alt('P'), Action::SelectParent),
                (Alt('n'), Action::SelectNextSibling),
                (Alt('p'), Action::SelectPrevSibling),
            ]
            .into_iter()
            .collect(),
            modal: false,
            stricken: "☠".to_owned(),
            collapsed: "⊞".to_owned(),
            hide_stricken: "⚔".to_owned(),
            free_text: "✏".to_owned(),
        }
    }
}

struct FmtKey(Key);

impl fmt::Display for FmtKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            Key::PageDown => write!(f, "{}", "PgDn"),
            Key::PageUp => write!(f, "{}", "PgUp"),
            Key::Delete => write!(f, "{}", "Del"),
            Key::Alt(c) => write!(f, "A-{}", FmtKey(Key::Char(*c))),
            Key::Ctrl(c) => write!(f, "C-{}", FmtKey(Key::Char(*c))),
            Key::Char(' ') => write!(f, "{}", "Space"),
            Key::Char('\n') => write!(f, "{}", "Enter"),
            Key::Char('\t') => write!(f, "{}", "Tab"),
            Key::Char(c) => write!(f, "{}", c),
            other => fmt::Debug::fmt(other, f),
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.modal {
            writeln!(f, "Modal mode enabled.").unwrap();
        }
        writeln!(f, "Configured Hotkeys:").unwrap();
        let mut hotkeys = self
            .config
            .iter()
            .map(|(key, action)| format!("    {}: {}", action, FmtKey(*key)))
            .collect::<Vec<_>>();
        hotkeys.sort();
        hotkeys
            .into_iter()
            .map(|string| writeln!(f, "{}", string))
            .collect::<Result<Vec<()>, _>>()?;
        Ok(())
    }
}

impl Config {
    pub fn maybe_parsed_from_env() -> io::Result<Config> {
        if let Ok(p) = env::var("KEYFILE") {
            Config::parse_keyfile(p)
        } else {
            Ok(Config::default())
        }
    }

    pub fn parse_keyfile(p: String) -> io::Result<Config> {
        let mut buf = String::new();
        let mut f = File::open(p)?;
        f.read_to_string(&mut buf)?;
        let mut config = Config::default();
        for (mut line_num, line) in buf.lines().enumerate() {
            if line == "" || line.starts_with('#') {
                continue;
            }
            if line == "modal" {
                config.modal = true;
                continue;
            }
            if line == "no_defaults" {
                config.config = HashMap::new();
                continue;
            }

            // Zero based indexing inappropriate here.
            line_num += 1;

            let parts: Vec<_> = line.splitn(2, ':').map(|p| p.trim()).collect();
            if parts.len() != 2 {
                let e = format!("No colon found on line {}", line_num);
                error!("{}", e);
                return Err(Error::new(ErrorKind::Other, e));
            }

            let (option, param) = (parts[0], parts[1]);
            match (option, param) {
                ("stricken", p) => {
                    config.stricken = p.to_owned();
                }
                ("collapsed", p) => {
                    config.collapsed = p.to_owned();
                }
                ("hide_stricken", p) => {
                    config.hide_stricken = p.to_owned();
                }
                ("free_text", p) => {
                    config.free_text = p.to_owned();
                }
                (raw_action, raw_key) => {
                    let key_opt = to_key(raw_key.to_owned());
                    let action_opt = to_action(raw_action.to_owned());

                    if key_opt.is_none() || action_opt.is_none() {
                        let e = format!("invalid config at line {}: {}", line_num, line);
                        error!("{}", e);
                        return Err(Error::new(ErrorKind::Other, e));
                    }

                    let key = key_opt.unwrap();
                    let action = action_opt.unwrap();

                    config.config.insert(key, action);
                }
            }
        }

        Ok(config)
    }

    pub fn map(&self, e: Event) -> Option<Action> {
        use termion::event::{Key::*, MouseButton};
        match e {
            Event::Key(Char(c)) => {
                if let Some(action) = self.config.get(&Char(c)).cloned() {
                    Some(action)
                } else {
                    Some(Action::Char(c))
                }
            }
            Event::Mouse(MouseEvent::Press(MouseButton::Right, x, y)) => {
                Some(Action::RightClick(x, y))
            }
            Event::Mouse(MouseEvent::Press(_, x, y)) => Some(Action::LeftClick(x, y)),
            Event::Mouse(MouseEvent::Release(x, y)) => Some(Action::Release(x, y)),
            Event::Mouse(MouseEvent::Hold(..)) => None,
            Event::Key(other) => {
                let lookup = self.config.get(&other).cloned();
                if lookup.is_none() {
                    warn!("Weird event {:?}", other);
                }
                lookup
            }
            other => {
                warn!("Unknown event received: {:?}", other);
                None
            }
        }
    }
}
