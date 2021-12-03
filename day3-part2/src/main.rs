use report::ReportLine;

mod report;

const INPUT: &str = include_str!("./input.txt");

pub enum BitUsage {
    MoreZero,
    MoreOne,
    Equal,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let report_lines = INPUT
        .lines()
        .map(|line| line.parse::<report::ReportLine>())
        .collect::<Result<Vec<report::ReportLine>, _>>()?;

    let oxygen_generator_rating =
        find_oxygen_generator_rating(&report_lines.iter().collect::<Vec<_>>());
    let co2_scrubber_rating = find_co2_scrubber_rating(&report_lines.iter().collect::<Vec<_>>());

    println!("{}", oxygen_generator_rating * co2_scrubber_rating);

    Ok(())
}

pub fn compute_bit_usage(report_columns: &[report::ReportColumn]) -> Vec<BitUsage> {
    report_columns
        .iter()
        .map(|column| {
            let nb_ones = column.0.iter().filter(|&bit| *bit).count();
            let nb_zeros = column.0.len() - nb_ones;

            match nb_ones.cmp(&nb_zeros) {
                std::cmp::Ordering::Less => BitUsage::MoreZero,
                std::cmp::Ordering::Greater => BitUsage::MoreOne,
                std::cmp::Ordering::Equal => BitUsage::Equal,
            }
        })
        .collect::<Vec<_>>()
}

pub fn report_lines_to_columns(report_lines: &[&ReportLine]) -> Vec<report::ReportColumn> {
    (0..report_lines[0].0.len())
        .map(|column_index| {
            report::ReportColumn(
                report_lines
                    .iter()
                    .map(|line| line.0[column_index])
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>()
}

pub fn find_line_by_bit_criteria<'a>(
    report_lines: &[&'a ReportLine],
    bit_criteria: impl Fn(&ReportLine, &[BitUsage], usize) -> bool,
) -> &'a ReportLine {
    let mut filtered_report_lines = report_lines.to_vec();

    for i in 0..report_lines[0].0.len() {
        let bits_usage = compute_bit_usage(&report_lines_to_columns(&filtered_report_lines));

        filtered_report_lines = filtered_report_lines
            .into_iter()
            .filter(|&line| bit_criteria(line, &bits_usage, i))
            .collect();

        if filtered_report_lines.len() == 1 {
            return filtered_report_lines[0];
        }
    }

    unreachable!()
}

pub fn find_oxygen_generator_rating(report_lines: &[&ReportLine]) -> usize {
    line_to_number(find_line_by_bit_criteria(
        report_lines,
        |line, bits_usage, i| match bits_usage[i] {
            BitUsage::MoreZero => !line.0[i],
            BitUsage::MoreOne => line.0[i],
            BitUsage::Equal => line.0[i],
        },
    ))
}

pub fn find_co2_scrubber_rating(report_lines: &[&ReportLine]) -> usize {
    line_to_number(find_line_by_bit_criteria(
        report_lines,
        |line, bits_usage, i| match bits_usage[i] {
            BitUsage::MoreZero => line.0[i],
            BitUsage::MoreOne => !line.0[i],
            BitUsage::Equal => !line.0[i],
        },
    ))
}

pub fn line_to_number(line: &ReportLine) -> usize {
    line.0
        .iter()
        .fold(0, |acc, &bit| (acc << 1) | (bit as usize))
}
