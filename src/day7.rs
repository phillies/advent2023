use std::collections::HashMap;

#[derive(Debug)]
struct Hand {
    cards: [char; 5],
    bet: u64,
    high_card: u64,
    card_map: HashMap<char, u64>,
    value: u64,
}

struct HandJoker {
    cards: [char; 5],
    bet: u64,
    high_card: u64,
    card_map: HashMap<char, u64>,
    joker_count: u64,
    value: u64,
}

trait Handy {
    fn new(cards: [char; 5], bet: u64) -> Self;

    fn get_bet(self: &Self) -> u64;
}

impl Hand {
    fn _calculate_high_card_value(self: &mut Hand) {
        self.high_card = 0;
        self.cards.iter().rev().enumerate().for_each(|(i, &c)| {
            let offset = 10u64.pow(2 * i as u32);
            match c {
                c if c.is_ascii_digit() => {
                    self.high_card += c.to_digit(10).unwrap() as u64 * offset;
                }
                'T' => {
                    self.high_card += 10 * offset;
                }
                'J' => {
                    self.high_card += 11 * offset;
                }
                'Q' => {
                    self.high_card += 12 * offset;
                }
                'K' => {
                    self.high_card += 13 * offset;
                }
                'A' => {
                    self.high_card += 14 * offset;
                }
                _ => panic!("Invalid card!"),
            }
        });
    }

    fn _calculate_value(self: &mut Hand) {
        if self.is_5_of_a_kind() {
            self.value = 60000000000 + self.high_card;
        } else if self.is_4_of_a_kind() {
            self.value = 50000000000 + self.high_card;
        } else if self.is_full_house() {
            self.value = 40000000000 + self.high_card;
        } else if self.is_3_of_a_kind() {
            self.value = 30000000000 + self.high_card;
        } else if self.is_2_pair() {
            self.value = 20000000000 + self.high_card;
        } else if self.is_1_pair() {
            self.value = 10000000000 + self.high_card;
        } else if self.is_high_card() {
            self.value = self.high_card;
        } else {
            panic!("Invalid hand!");
        }
    }

    fn _build_card_map(self: &mut Hand) {
        self.card_map.clear();
        self.cards.iter().for_each(|&c| {
            self.card_map.entry(c).and_modify(|e| *e += 1).or_insert(1);
        });
    }

    fn is_5_of_a_kind(self: &Hand) -> bool {
        self.cards.iter().all(|&c| c == self.cards[0])
    }
    fn is_4_of_a_kind(self: &Hand) -> bool {
        self.card_map.values().any(|&v| v == 4)
    }
    fn is_full_house(self: &Hand) -> bool {
        self.card_map.values().any(|&v| v == 3) && self.card_map.values().any(|&v| v == 2)
    }
    fn is_3_of_a_kind(self: &Hand) -> bool {
        self.card_map.values().any(|&v| v == 3) && self.card_map.values().any(|&v| v == 1)
    }
    fn is_2_pair(self: &Hand) -> bool {
        self.card_map.values().filter(|&&v| v == 2).count() == 2
    }
    fn is_1_pair(self: &Hand) -> bool {
        self.card_map.values().filter(|&&v| v == 2).count() == 1
            && self.card_map.values().all(|&v| v <= 2)
    }
    fn is_high_card(self: &Hand) -> bool {
        self.card_map.values().all(|&v| v == 1)
    }
}

impl Handy for Hand {
    fn new(cards: [char; 5], bet: u64) -> Hand {
        let mut hand = Hand {
            cards,
            bet,
            high_card: 0,
            card_map: HashMap::new(),
            value: 0,
        };
        hand._build_card_map();
        hand._calculate_high_card_value();
        hand._calculate_value();
        hand
    }

    fn get_bet(self: &Self) -> u64 {
        self.bet
    }
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value).reverse()
    }
}

impl HandJoker {
    fn _calculate_high_card_value(self: &mut HandJoker) {
        self.high_card = 0;
        self.cards.iter().rev().enumerate().for_each(|(i, &c)| {
            let offset = 10u64.pow(2 * i as u32);
            match c {
                c if c.is_ascii_digit() => {
                    self.high_card += c.to_digit(10).unwrap() as u64 * offset;
                }
                'T' => {
                    self.high_card += 10 * offset;
                }
                'J' => {
                    self.high_card += 1 * offset;
                }
                'Q' => {
                    self.high_card += 12 * offset;
                }
                'K' => {
                    self.high_card += 13 * offset;
                }
                'A' => {
                    self.high_card += 14 * offset;
                }
                _ => panic!("Invalid card!"),
            }
        });
    }

    fn _calculate_value(self: &mut HandJoker) {
        if self.is_5_of_a_kind() {
            self.value = 60000000000 + self.high_card;
        } else if self.is_4_of_a_kind() {
            self.value = 50000000000 + self.high_card;
        } else if self.is_full_house() {
            self.value = 40000000000 + self.high_card;
        } else if self.is_3_of_a_kind() {
            self.value = 30000000000 + self.high_card;
        } else if self.is_2_pair() {
            self.value = 20000000000 + self.high_card;
        } else if self.is_1_pair() {
            self.value = 10000000000 + self.high_card;
        } else if self.is_high_card() {
            self.value = self.high_card;
        } else {
            panic!("Invalid hand!");
        }
    }

    fn _build_card_map(self: &mut HandJoker) {
        self.card_map.clear();
        self.cards.iter().for_each(|&c| {
            if c != 'J' {
                self.card_map.entry(c).and_modify(|e| *e += 1).or_insert(1);
            }
        });
    }

    fn is_5_of_a_kind(self: &HandJoker) -> bool {
        self.cards.iter().all(|&c| c == self.cards[0])
            || (self.card_map.values().max().unwrap_or(&0) + self.joker_count) == 5
    }
    fn is_4_of_a_kind(self: &HandJoker) -> bool {
        self.card_map.values().any(|&v| v == 4)
            || (self.card_map.values().max().unwrap_or(&0) + self.joker_count) == 4
    }
    fn is_full_house(self: &HandJoker) -> bool {
        (self.card_map.values().any(|&v| v == 3) && self.card_map.values().any(|&v| v == 2))
            || ((self.card_map.values().max().unwrap_or(&0) + self.joker_count) == 3
                && (*self.card_map.values().min().unwrap_or(&0) == 2))
    }
    fn is_3_of_a_kind(self: &HandJoker) -> bool {
        (self.card_map.values().any(|&v| v == 3) && self.card_map.values().any(|&v| v == 1))
            || (self.card_map.values().max().unwrap_or(&0) + self.joker_count) == 3
    }
    fn is_2_pair(self: &HandJoker) -> bool {
        self.card_map.values().filter(|&&v| v == 2).count() == 2
    }
    fn is_1_pair(self: &HandJoker) -> bool {
        (self.card_map.values().filter(|&&v| v == 2).count() == 1
            && self.card_map.values().all(|&v| v <= 2))
            || (self.card_map.values().all(|&v| v == 1) && self.joker_count == 1)
    }
    fn is_high_card(self: &HandJoker) -> bool {
        self.card_map.values().all(|&v| v == 1)
    }
}

impl Handy for HandJoker {
    fn new(cards: [char; 5], bet: u64) -> HandJoker {
        let mut hand = HandJoker {
            cards,
            bet,
            high_card: 0,
            card_map: HashMap::new(),
            joker_count: cards.iter().filter(|&&c| c == 'J').count() as u64,
            value: 0,
        };
        hand._build_card_map();
        hand._calculate_high_card_value();
        hand._calculate_value();
        hand
    }
    fn get_bet(self: &Self) -> u64 {
        self.bet
    }
}

impl Eq for HandJoker {}

impl PartialEq for HandJoker {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl PartialOrd for HandJoker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for HandJoker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value).reverse()
    }
}

fn play_hands<T: Handy + std::cmp::Ord>(input: &Vec<String>) -> u64 {
    let mut hands = vec![];

    for line in input {
        let mut hand_data = line.split_whitespace();
        let cards = hand_data
            .next()
            .unwrap()
            .chars()
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();
        let bet = hand_data.next().unwrap().parse::<u64>().unwrap();
        hands.push(T::new(cards, bet));
    }

    hands.sort();

    let mut hand_value = 0;

    hands.iter().enumerate().for_each(|(i, h)| {
        hand_value += h.get_bet() * (i as u64 + 1);
    });

    hand_value
}

pub fn solve(input: &Vec<String>) -> (u64, u64) {
    let hand_value = play_hands::<Hand>(input);
    let hand2_value = play_hands::<HandJoker>(input);

    (hand_value, hand2_value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::my_io::read_input_to_vector;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_day7() {
        let result_1 = 6440;
        let result_2 = 5905;
        let input = vec![
            "32T3K 765".to_string(),
            "T55J5 684".to_string(),
            "KK677 28".to_string(),
            "KTJJT 220".to_string(),
            "QQQJA 483".to_string(),
        ];
        let (output_1, output_2) = solve(&input);
        assert_eq!(result_1, output_1);
        assert_eq!(result_2, output_2);
    }

    #[bench]
    fn bench_day7_part_1(b: &mut Bencher) {
        let input = read_input_to_vector("data/day7.txt");
        b.iter(|| {
            play_hands::<Hand>(&input);
        });
    }

    #[bench]
    fn bench_dat7_part_2(b: &mut Bencher) {
        let input = read_input_to_vector("data/day7.txt");
        b.iter(|| {
            play_hands::<HandJoker>(&input);
        });
    }
}
