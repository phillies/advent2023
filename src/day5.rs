use std::collections::BTreeSet;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Copy)]
struct Range {
    start: i64,
    end: i64,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Copy)]
struct Mapping {
    start: i64,
    end: i64,
    shift: i64,
}

impl Mapping {
    fn contains(self: &Mapping, value: i64) -> bool {
        value >= self.start && value <= self.end
    }

    fn map(self: &Mapping, value: i64) -> i64 {
        if self.contains(value) {
            value + self.shift
        } else {
            value
        }
    }

    fn map_range(self: &Mapping, range: &Range) -> (Option<Range>, Vec<Range>) {
        let mut unmaped_result = vec![];
        let mut mapped_result = None;

        if self.contains(range.start) && self.contains(range.end) {
            mapped_result = Some(Range {
                start: range.start + self.shift,
                end: range.end + self.shift,
            });
        } else if range.start < self.start && range.end > self.end {
            unmaped_result.push(Range {
                start: range.start,
                end: self.start - 1,
            });
            unmaped_result.push(Range {
                start: self.end + 1,
                end: range.end,
            });
            mapped_result = Some(Range {
                start: self.start + self.shift,
                end: self.end + self.shift,
            });
        } else if self.contains(range.start) {
            mapped_result = Some(Range {
                start: range.start + self.shift,
                end: self.end + self.shift,
            });
            unmaped_result.push(Range {
                start: self.end + 1,
                end: range.end,
            });
        } else if self.contains(range.end) {
            unmaped_result.push(Range {
                start: range.start,
                end: self.start - 1,
            });
            mapped_result = Some(Range {
                start: self.start + self.shift,
                end: range.end + self.shift,
            });
        } else {
            unmaped_result.push(range.clone());
        }

        (mapped_result, unmaped_result)
    }
}

#[derive(Debug, Clone)]
struct AdventMap {
    mappings: BTreeSet<Mapping>,
}

impl AdventMap {
    fn new() -> AdventMap {
        AdventMap {
            mappings: BTreeSet::new(),
        }
    }

    fn parse_and_insert(self: &mut AdventMap, line: String) {
        let mut parts = line.split(" ");
        let destination = parts.next().unwrap().parse::<i64>().unwrap();
        let source = parts.next().unwrap().parse::<i64>().unwrap();
        let length = parts.next().unwrap().parse::<i64>().unwrap();
        self.mappings.insert(Mapping {
            start: source,
            end: source + length - 1,
            shift: destination - source,
        });
    }

    fn apply(self: &AdventMap, value: i64) -> i64 {
        for mapping in self.mappings.iter() {
            if mapping.contains(value) {
                return mapping.map(value);
            }
        }
        value
    }

    fn apply_range(self: &AdventMap, ranges: &Vec<Range>) -> Vec<Range> {
        let mut mapped_results = vec![];
        let mut unmaped_ranges = ranges.clone();

        self.mappings.iter().for_each(|x| {
            let mut all_unmapped = vec![];
            for r in unmaped_ranges.iter() {
                let (new_mapped, new_unmapped) = x.map_range(r);

                if let Some(mapped) = new_mapped {
                    mapped_results.push(mapped);
                }
                all_unmapped.extend(new_unmapped);
            }
            unmaped_ranges = all_unmapped;
        });

        mapped_results.extend(unmaped_ranges);
        mapped_results
    }
}

fn create_map(iter: &mut dyn Iterator<Item = String>) -> AdventMap {
    let mut map = AdventMap::new();
    while let Some(line) = iter.next() {
        if line.contains(":") {
            break;
        }
        map.parse_and_insert(line);
    }
    map
}

fn get_seeds(seed_line: &String) -> Vec<i64> {
    seed_line
        .split(":")
        .nth(1)
        .expect("Cannot parse seeds")
        .split(" ")
        .filter_map(|x| x.parse::<i64>().ok())
        .collect()
}

fn parse_input_to_maps(input: &Vec<String>) -> Vec<AdventMap> {
    // create_map advances the iterator to the next block, so we can sequentially construct the maps
    let mut filtered_lines = input.iter().filter(|x| !x.is_empty()).map(|x| x.clone());
    let seed_to_soil_map = create_map(&mut filtered_lines);
    let soil_to_fertilizer_map = create_map(&mut filtered_lines);
    let fertilizer_to_water_map = create_map(&mut filtered_lines);
    let water_to_light_map = create_map(&mut filtered_lines);
    let light_to_temperature_map = create_map(&mut filtered_lines);
    let temperature_to_humidity_map = create_map(&mut filtered_lines);
    let humidity_to_location_map = create_map(&mut filtered_lines);

    vec![
        seed_to_soil_map,
        soil_to_fertilizer_map,
        fertilizer_to_water_map,
        water_to_light_map,
        light_to_temperature_map,
        temperature_to_humidity_map,
        humidity_to_location_map,
    ]
}

fn calculate_single_locations(seeds: &Vec<i64>, maps: &Vec<AdventMap>) -> Vec<i64> {
    seeds
        .iter()
        .map(|x| {
            let mut value = *x;
            for map in maps.iter() {
                value = map.apply(value);
            }
            value
        })
        .collect()
}

fn seeds_to_ranges(seeds: &Vec<i64>) -> Vec<Range> {
    seeds
        .array_chunks::<2>()
        .map(|x| Range {
            start: x[0],
            end: x[0] + x[1] - 1,
        })
        .collect()
}

pub fn solve(input: &Vec<String>) -> (u64, u64) {
    let seeds = get_seeds(&input[0]);
    let maps = parse_input_to_maps(&input[3..].to_vec());

    let locations = calculate_single_locations(&seeds, &maps);

    let seed_ranges = seeds_to_ranges(&seeds);

    let location_ranges = maps.iter().fold(seed_ranges, |x, acc| acc.apply_range(&x));
    let min_value = location_ranges
        .iter()
        .fold(1000000000000, |x, acc| x.min(acc.start));
    (*locations.iter().min().unwrap() as u64, min_value as u64)
}
#[cfg(test)]
mod tests {

    use super::*;
    use crate::my_io::read_input_to_vector;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_day5() {
        let result_1 = 35;
        let result_2 = 46;
        let input = vec![
            "seeds: 79 14 55 13".to_string(),
            "".to_string(),
            "seed-to-soil map:".to_string(),
            "50 98 2".to_string(),
            "52 50 48".to_string(),
            "".to_string(),
            "soil-to-fertilizer map:".to_string(),
            "0 15 37".to_string(),
            "37 52 2".to_string(),
            "39 0 15".to_string(),
            "".to_string(),
            "fertilizer-to-water map:".to_string(),
            "49 53 8".to_string(),
            "0 11 42".to_string(),
            "42 0 7".to_string(),
            "57 7 4".to_string(),
            "".to_string(),
            "water-to-light map:".to_string(),
            "88 18 7".to_string(),
            "18 25 70".to_string(),
            "".to_string(),
            "light-to-temperature map:".to_string(),
            "45 77 23".to_string(),
            "81 45 19".to_string(),
            "68 64 13".to_string(),
            "".to_string(),
            "temperature-to-humidity map:".to_string(),
            "0 69 1".to_string(),
            "1 0 69".to_string(),
            "".to_string(),
            "humidity-to-location map:".to_string(),
            "60 56 37".to_string(),
            "56 93 4".to_string(),
        ];
        let (output_1, output_2) = solve(&input);
        assert_eq!(result_1, output_1);
        assert_eq!(result_2, output_2);
    }

    #[bench]
    fn bench_day5_part_1(b: &mut Bencher) {
        let input = read_input_to_vector("data/day5.txt");
        b.iter(|| {
            let seeds = get_seeds(&input[0]);
            let maps = parse_input_to_maps(&input[2..].to_vec());

            let locations = calculate_single_locations(&seeds, &maps);
            let _result = locations.iter().min().unwrap();
        });
    }

    #[bench]
    fn bench_day5_part_2(b: &mut Bencher) {
        let input = read_input_to_vector("data/day5.txt");
        b.iter(|| {
            let seeds = get_seeds(&input[0]);
            let maps = parse_input_to_maps(&input[3..].to_vec());

            let seed_ranges = seeds_to_ranges(&seeds);

            let location_ranges = maps.iter().fold(seed_ranges, |x, acc| acc.apply_range(&x));
            let _min_value = location_ranges
                .iter()
                .fold(1000000000000, |x, acc| x.min(acc.start));
        });
    }
}
