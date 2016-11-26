use rand;
use termion::terminal_size;
use termion::event::{Key, Event, MouseEvent, MouseButton};
use quickcheck::{Arbitrary, Gen, QuickCheck, StdGen};

use voidmap::*;

#[derive(Debug, Clone)]
struct Op {
    event: Event,
}

impl Arbitrary for Op {
    fn arbitrary<G: Gen>(g: &mut G) -> Op {
        let (c, u, x, y) =
            (g.gen_ascii_chars().nth(0).unwrap(), g.gen::<char>(), g.gen::<u16>(), g.gen::<u16>());
        let events = vec![
                Event::Key(Key::Char('\n')),
                Event::Key(Key::Char('\t')),
                Event::Key(Key::Char(c)),
                Event::Key(Key::Char(u)),
                Event::Key(Key::Ctrl('n')),
                Event::Key(Key::Ctrl(c)),
                Event::Key(Key::Ctrl(u)),
                Event::Key(Key::PageUp),
                Event::Key(Key::PageDown),
                Event::Key(Key::Esc),
                Event::Key(Key::Up),
                Event::Key(Key::Left),
                Event::Key(Key::Right),
                Event::Key(Key::Down),
                Event::Key(Key::Delete),
                Event::Key(Key::Backspace),
                Event::Mouse(MouseEvent::Press(MouseButton::Left, x, y)),
                Event::Mouse(MouseEvent::Release(x, y)),
            ];
        Op { event: *g.choose(&*events).unwrap() }
    }
}


#[derive(Debug, Clone)]
struct OpVec {
    ops: Vec<Op>,
}

impl Arbitrary for OpVec {
    fn arbitrary<G: Gen>(g: &mut G) -> OpVec {
        let mut ops = vec![];
        for _ in 0..g.gen_range(1, 100) {
            ops.push(Op::arbitrary(g));
        }
        OpVec { ops: ops }
    }

    fn shrink(&self) -> Box<Iterator<Item = OpVec>> {
        let mut smaller = vec![];
        for i in 0..self.ops.len() {
            let mut clone = self.clone();
            clone.ops.remove(i);
            smaller.push(clone);
        }

        Box::new(smaller.into_iter())
    }
}

fn prop_handle_events(ops: OpVec) -> bool {
    let mut screen = Screen::default();
    screen.is_test = true;
    screen.start_raw_mode();
    screen.draw();
    for op in &ops.ops {
        screen.dims = terminal_size().unwrap();


        let should_break = !screen.handle_event(op.event);

        if screen.should_auto_arrange() {
            screen.arrange();
        }

        screen.draw();
        screen.assert_node_consistency();

        if should_break {
            screen.cleanup();
            screen.save();
            break;
        }
    }
    true
}

#[test]
// #[ignore]
fn qc_merge_converges() {
    QuickCheck::new()
        .gen(StdGen::new(rand::thread_rng(), 1))
        .tests(1_000)
        .max_tests(10_000)
        .quickcheck(prop_handle_events as fn(OpVec) -> bool);
}

// TODO Arguments: (OpVec { ops: [Op { event: Key(Up) }, Op { event: Key(Backspace) }, Op { event: Key(Backspace) }, Op { event: Key(Backspace) }, Op { event: Key(Backspace) }, Op { event: Key(Down) }] })

// TODO make a ton of one char nodes with no children and autosort

// TODO backspace on unicode

// TODO drag & select & sort & resize & arrow specific qc
