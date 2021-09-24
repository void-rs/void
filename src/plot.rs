use std::cmp;

pub fn plot_sparkline<T>(nums_in: Vec<T>) -> String
where
    T: Into<i64>,
{
    const BARS: [char; 9] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

    let nums: Vec<_> = nums_in.into_iter().map(|n| n.into()).collect();
    let max = nums.iter().max().unwrap();

    let mut ret = String::new();
    for n in &nums {
        if *max as i64 == 0 {
            ret.push(BARS[0]);
        } else {
            let idx = (BARS.len() - 1) as i64 * n / max;
            ret.push(BARS[idx as usize]);
        }
    }
    ret
}

pub fn bounded_count_sparkline<T>(nums_in: Vec<T>, start: T, end: T, bars: usize) -> String
where
    T: Into<i64> + PartialOrd<T>,
{
    if bars == 0 {
        return String::new();
    }

    let (start, end, rev) = if start <= end {
        (start, end, false)
    } else {
        (end, start, true)
    };

    let start = start.into();
    let end = end.into();
    let nums: Vec<_> = nums_in.into_iter().map(|n| n.into()).collect();
    let step = (end - start) / bars as i64;
    let mut counts = vec![0; bars];

    if step == 0 || nums.is_empty() {
        return String::from_utf8(vec![b' '; bars]).unwrap();
    }

    let start = start as usize;
    let step = step as usize;

    for &n in &nums {
        let n = cmp::max(n as usize, start) as usize;
        let idx = (n - start) / step;
        counts[cmp::min(idx, bars - 1)] += 1;
    }

    let plot = plot_sparkline(counts);
    if rev {
        plot.chars().rev().collect()
    } else {
        plot
    }
}
