use lazy_regex::regex;

pub fn part1() -> u32 {
    include_str!("inputs/day1.txt").lines().fold(0, |acc, l| {
        let d: Vec<_> = l.chars().filter_map(|c| c.to_digit(10)).collect();

        let f = d.first().unwrap();
        let lt = d.last().unwrap();

        acc + f * 10 + lt
    })
}

pub fn part2() -> i32 {
    include_str!("inputs/day1.txt").lines().fold(0, |acc, l| {
        let p = regex!(r"^(one|two|three|four|five|six|seven|eight|nine|[1-9])");
        let mut d = vec![];

        (0..l.len()).for_each(|i| {
            if let Some(n) = p.find(&l[i..]) {
                d.push(match n.as_str() {
                    "one" => 1,
                    "two" => 2,
                    "three" => 3,
                    "four" => 4,
                    "five" => 5,
                    "six" => 6,
                    "seven" => 7,
                    "eight" => 8,
                    "nine" => 9,
                    _ => n.as_str().parse::<i32>().unwrap_or(0),
                })
            }
        });

        let f = d.first().unwrap();
        let lt = d.last().unwrap();

        acc + f * 10 + lt
    })
}
