mod report;

const INPUT: &str = include_str!("./input.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let report_lines = INPUT
        .lines()
        .map(|line| line.parse::<report::ReportLine>())
        .collect::<Result<Vec<report::ReportLine>, _>>()?;

    let report_columns = (0..report_lines[0].0.len())
        .map(|column_index| {
            report::ReportColumn(
                report_lines
                    .iter()
                    .map(|line| line.0[column_index])
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let most_used_bits = report_columns
        .iter()
        .map(|column| (column.0.iter().filter(|&c| *c).count()) > (column.0.len() / 2))
        .collect::<Vec<_>>();

    let gamma = most_used_bits
        .iter()
        .fold(0, |acc, &bit| (acc << 1) | (bit as usize));
    let epsilon = most_used_bits
        .iter()
        .map(|bit| !bit)
        .fold(0, |acc, bit| (acc << 1) | (bit as usize));

    println!("{}", gamma * epsilon);

    Ok(())
}
