pub fn plot_sparkline<T>(nums_in: Vec<T>) -> String
    where T: Into<i64>
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
    where T: Into<i64>
{
    let nums: Vec<_> = nums_in.into_iter().map(|n| n.into()).collect();
    let start = start.into();
    let step = (end.into() - start.clone()) / bars as i64;
    let mut counts = vec![0; bars];
    for &n in &nums {
        counts[(((n - start) / step) - 1) as usize] += 1;
    }
    plot_sparkline(counts)
}
