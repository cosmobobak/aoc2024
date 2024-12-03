use nom::{
    character::complete::{char, digit1},
    IResult,
};

fn bytes_to_int(bytes: &[u8]) -> i32 {
    bytes
        .iter()
        .fold(0, |acc, b| acc * 10 + i32::from(*b - b'0'))
}

fn parse_pair(input: &[u8]) -> IResult<&[u8], (i32, i32)> {
    let (input, _) = char('(')(input)?;
    let (input, num1) = digit1(input)?;
    let (input, _) = char(',')(input)?;
    let (input, num2) = digit1(input)?;
    let (input, _) = char(')')(input)?;
    let num1 = bytes_to_int(num1);
    let num2 = bytes_to_int(num2);
    Ok((input, (num1, num2)))
}

pub fn task03() {
    let start = std::time::Instant::now();
    let task = include_bytes!("../tasks/task03.txt");

    let mut sum1 = 0;
    let mut sum2 = 0;
    let mut enabled = true;
    for (i, byte) in task.iter().enumerate() {
        match *byte {
            b'm' if task.get(i + 1..i + 3) == Some(b"ul") => {
                let text = &task[i + 3..];
                if let Ok((_, (a, b))) = parse_pair(text) {
                    sum1 += a * b;
                    if enabled {
                        sum2 += a * b;
                    }
                }
            }
            b'd' if task.get(i + 1..i + 4) == Some(b"o()") => {
                enabled = true;
            }
            b'd' if task.get(i + 1..i + 7) == Some(b"on't()") => {
                enabled = false;
            }
            _ => {}
        }
    }

    println!("Part 1: {sum1}");
    println!("Part 2: {sum2}");

    let elapsed = start.elapsed();
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
}
