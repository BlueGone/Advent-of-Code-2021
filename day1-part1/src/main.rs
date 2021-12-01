const INPUT: &str = include_str!("./input.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let numbers = INPUT
        .lines()
        .map(|line| line.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;

    let windows = numbers.windows(2);
    let nb_increasing = windows.filter(|pair| pair[0] < pair[1]).count();

    println!("{}", nb_increasing);

    Ok(())
}
