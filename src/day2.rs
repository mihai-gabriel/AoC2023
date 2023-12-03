pub fn part1() -> i32 {
    // BAG - given from the puzzle description
    const BAG: (i32, i32, i32) = (12, 13, 14);

    include_str!("inputs/day2.txt").lines().fold(0, |acc, l| {
        let (id, game) = l.split_once(":").unwrap();
        let (_, id) = id.split_once(" ").unwrap();
        let rounds: Vec<_> = game.split(";").collect();

        let valid = rounds.iter().all(|r| {
            let c: Vec<_> = r.split(",").collect();

            c.iter().all(|c| {
                let (count, color) = c.trim().split_once(" ").unwrap();
                let count = count.parse::<i32>().unwrap();

                match color {
                    "red" => count <= BAG.0,
                    "green" => count <= BAG.1,
                    "blue" => count <= BAG.2,
                    _ => false,
                }
            })
        });

        acc + if valid { id.parse::<i32>().unwrap() } else { 0 }
    })
}

pub fn part2() -> i32 {
    include_str!("inputs/day2.txt").lines().fold(0, |acc, l| {
        let (_, game) = l.split_once(":").unwrap();
        let rounds: Vec<_> = game.split(";").collect();
        let mut bag = (0, 0, 0);

        rounds.iter().for_each(|r| {
            let c: Vec<_> = r.split(",").collect();

            c.iter().for_each(|c| {
                let (count, color) = c.trim().split_once(" ").unwrap();
                let count = count.parse::<i32>().unwrap();

                match color {
                    "red" => bag.0 = i32::max(bag.0, count),
                    "green" => bag.1 = i32::max(bag.1, count),
                    "blue" => bag.2 = i32::max(bag.2, count),
                    _ => unreachable!(),
                }
            });
        });

        acc + (bag.0 * bag.1 * bag.2)
    })
}
