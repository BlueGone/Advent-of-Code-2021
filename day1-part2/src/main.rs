const INPUT: &str = include_str!("./input.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let numbers = INPUT
        .lines()
        .map(|line| line.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;

    let sums: Vec<i32> = numbers
        .windows(3)
        .map(|window| window.iter().sum())
        .collect();
    let nb_increasing = sums.windows(2).filter(|pair| pair[0] < pair[1]).count();

    println!("{}", nb_increasing);

    Ok(())
}
