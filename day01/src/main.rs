// all based upon the excellent work of fasterthanli.me
// https://fasterthanli.me/series/advent-of-code-2022/part-1
use itertools::Itertools;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let max = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(v)) = it.next() {
                sum = Some(sum.unwrap_or(0) + v);
            }
            sum
        })
        .max();

    println!("{max:?}");
   
    Ok(())
}
