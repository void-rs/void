pub fn plot_sparkline<T>(nums_in: Vec<T>)
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
