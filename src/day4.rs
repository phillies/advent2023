use std::collections::{HashMap, HashSet};

struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
}

impl Card {
    fn count_matching_numbers(self: &Card) -> u32 {
        self.winning_numbers.intersection(&self.my_numbers).count() as u32
    }
}

/// Generic function which parses the input of format
/// xxx <id>: payload
/// into id and payload.
fn parse_id_payload(input_line: &String) -> (u32, String) {
    let mut game_split = input_line.split(":");
    let id = game_split
        .next()
        .expect("There was no : in the line")
        .split(" ")
        .filter_map(|x| x.parse::<u32>().ok())
        .next()
        .expect("Cannot parse id");

    let payload = game_split
        .next()
        .expect("There was nothing behind the :")
        .to_string();

    (id, payload)
}

fn parse_card(id: u32, payload: &String) -> Card {
    let mut payload_split = payload.split("|");

    let winning_numbers = payload_split
        .next()
        .expect("No winning numbers")
        .split(" ")
        .filter_map(|x| x.parse::<u32>().ok())
        .collect::<HashSet<u32>>();

    let my_numbers = payload_split
        .next()
        .expect("No drawn numbers")
        .split(" ")
        .filter_map(|x| x.parse::<u32>().ok())
        .collect::<HashSet<u32>>();

    Card {
        id: id,
        winning_numbers,
        my_numbers,
    }
}

pub fn solve(input: &Vec<String>) -> (u32, u32) {
    let mut points = 0;
    let mut total_cards = 0;
    let mut copies = HashMap::<u32, u32>::new();

    for input_line in input.into_iter() {
        let (id, payload) = parse_id_payload(input_line);
        let card = parse_card(id, &payload);
        let matching_numbers = card.count_matching_numbers();
        let this_card_instances = copies.get(&card.id).unwrap_or(&0) + 1;
        total_cards += this_card_instances;

        if matching_numbers > 0 {
            // wins -> points: 1 -> 1, 2 -> 2, 3 -> 4, 4 -> 8, 5 -> 16 ... wins -> 2^(wins-1)
            let this_card_points = 2u32.pow(matching_numbers - 1);
            points += this_card_points;

            // We don't add cards after the maximum card id
            for ii in 1..matching_numbers + 1 {
                copies.insert(
                    card.id + ii,
                    copies.get(&(card.id + ii)).unwrap_or(&0) + this_card_instances,
                );
            }
        }
    }
    (points, total_cards)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::my_io::read_input_to_vector;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_day4() {
        let result_1 = 13;
        let result_2 = 30;
        let input = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
        ];
        let (output_1, output_2) = solve(&input);
        assert_eq!(result_1, output_1);
        assert_eq!(result_2, output_2);
    }

    #[bench]
    fn bench_day4_both(b: &mut Bencher) {
        let input = read_input_to_vector("data/day4.txt");
        b.iter(|| {
            solve(&input);
        });
    }
}
