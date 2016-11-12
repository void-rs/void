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
