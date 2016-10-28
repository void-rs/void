use std;

fn plot_graph<T>(nums_in: Vec<T>)
    where T: Into<i64>
{
    const BARS: [char; 9] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

    let nums: Vec<_> = nums_in.into_iter().map(|n| n.into()).collect();
    let max = nums.iter().max();

    for n in &nums {
        let idx = (BARS.len() - 1) as i64 * n / max.unwrap();
        print!("{}", BARS[idx as usize]);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, RustcEncodable, RustcDecodable)]
pub enum Content {
    Text {
        text: String,
    },
    Plot(Vec<i64>),
}

impl Content {
    pub fn draw(&self) {
        match *self {
            Content::Text { ref text } => print!("{}", text),
            Content::Plot(ref data) => plot_graph(data.clone()),
        }
    }

    pub fn len(&self) -> usize {
        match *self {
            Content::Text { ref text } => text.len(),
            Content::Plot(ref data) => data.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn backspace(&mut self) {
        match *self {
            Content::Text { ref mut text } => {
                let newlen = std::cmp::max(text.len(), 1) - 1;
                *text = text.clone()[..newlen].to_string();
            }
            Content::Plot(_) => unimplemented!(),
        }
    }

    pub fn append(&mut self, c: char) {
        match *self {
            Content::Text { ref mut text } => {
                text.push(c);
            }
            Content::Plot(_) => {
                unimplemented!();
            }
        }
    }
}
