const SPEED: f64 = 1.0;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    /// The brute force solution, works reasonably well since rust is quite fast :-D
    // fn wins(self: &Race, hold_time: u64) -> bool {
    //     (hold_time * (SPEED as u64)) * (self.time - hold_time) > self.distance
    // }

    // fn number_of_beatings_brute_force(self: &Race) -> u64 {
    //     (0..self.time).filter(|t| self.wins(*t)).count() as u64
    // }

    /// Using quradratic formula:
    /// d = distance, t = time to beat, s = speed per second holding, h = hold time
    /// The time our boat races is time to beat minus hold time
    /// The distance it travels is speed per second holding times the hold time
    /// d = (t - h) * (h * s) = t * h * s - h^2 * s
    /// this can be rearranged to a quadratic equation
    /// 0 = h^2 - h * t + d / s
    /// which can be solved using the quadratic formula
    /// h = (t +- sqrt(t^2 - 4 * d / s)) / 2
    /// and as we need to be faster we take the rounded up number of the lower solution and the rounded down number of the upper solution
    /// the number of beatings is then the difference between the two solutions (including the solutions themselves, hence the +1)
    fn number_of_beatings(self: &Race) -> u64 {
        let t = self.time as f64;
        // We want to beat the other boat by at least one unit of distance
        let d = self.distance as f64 + 1.0;

        let root_value = (t * t / 4.0 - d / SPEED).sqrt();
        let lower_solution = (t / 2.0 - root_value).ceil() as u64;
        let upper_solution = (t / 2.0 + root_value).floor() as u64;
        return upper_solution - lower_solution + 1;
    }
}

fn parse_races(times: &str, distances: &str) -> Vec<Race> {
    let times_split = times.split_whitespace();
    let distances_split = distances.split_whitespace();

    times_split
        .zip(distances_split)
        .map(|(t, d)| Race {
            time: t.parse().unwrap(),
            distance: d.parse().unwrap(),
        })
        .collect()
}

pub fn solve(input: &Vec<String>) -> (u64, u64) {
    let times = input[0].split(":").nth(1).unwrap().trim();
    let distances = input[1].split(":").nth(1).unwrap().trim();
    let races = parse_races(times, distances);

    let winnings_product = races.iter().map(|r| r.number_of_beatings()).product();

    let long_race = parse_races(
        times.replace(" ", "").as_str(),
        distances.replace(" ", "").as_str(),
    );
    (
        winnings_product,
        long_race.first().unwrap().number_of_beatings(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::my_io::read_input_to_vector;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_day6() {
        let result_1 = 288;
        let result_2 = 71503;
        let input = vec![
            "Time:      7  15   30".to_string(),
            "Distance:  9  40  200".to_string(),
        ];
        let (output_1, output_2) = solve(&input);
        assert_eq!(result_1, output_1);
        assert_eq!(result_2, output_2);
    }

    #[bench]
    fn bench_day6_part_1(b: &mut Bencher) {
        let input = read_input_to_vector("data/day6.txt");
        b.iter(|| {
            let times = input[0].split(":").nth(1).unwrap().trim();
            let distances = input[1].split(":").nth(1).unwrap().trim();
            let races = parse_races(times, distances);

            races
                .iter()
                .map(|r| r.number_of_beatings())
                .product::<u64>()
        });
    }

    #[bench]
    fn bench_day6_part_2(b: &mut Bencher) {
        let input = read_input_to_vector("data/day6.txt");
        b.iter(|| {
            let times = input[0].split(":").nth(1).unwrap().trim();
            let distances = input[1].split(":").nth(1).unwrap().trim();

            parse_races(
                times.replace(" ", "").as_str(),
                distances.replace(" ", "").as_str(),
            )
        });
    }
}
