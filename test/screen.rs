use rand;
use termion::event::{Key, Event, MouseEvent, MouseButton};
use quickcheck::{Arbitrary, Gen, QuickCheck, StdGen};

use climate::*;

#[derive(Debug, Clone)]
struct Op {
    event: Event,
}

impl Arbitrary for Op {
    fn arbitrary<G: Gen>(g: &mut G) -> Op {
        let (c, x, y) = (g.gen::<char>(), g.gen::<u16>(), g.gen::<u16>());
        let events = vec![
                Event::Key(Key::Char(c)),
                Event::Key(Key::Alt('\u{1b}')),
                Event::Key(Key::Ctrl(c)),
                Event::Key(Key::Up),
                Event::Key(Key::Down),
                Event::Key(Key::Backspace),
                Event::Mouse(MouseEvent::Press(MouseButton::Left, x, y)),
                Event::Mouse(MouseEvent::Release(x, y)),
                Event::Mouse(MouseEvent::Hold(x, y)),
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
    for op in &ops.ops {
        screen.handle_event(op.event);
        screen.draw();
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
