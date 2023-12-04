use std::cmp::max;

#[derive(Debug, Copy, Clone, Default)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl Game {
    fn max_draws(self: &Game) -> Draw {
        let mut draw_sum = Draw {
            ..Default::default()
        };
        for d in self.draws.iter() {
            draw_sum.blue = max(d.blue, draw_sum.blue);
            draw_sum.red = max(d.red, draw_sum.red);
            draw_sum.green = max(d.green, draw_sum.green);
        }
        draw_sum
    }
}

fn parse_draw(draw_line: &str) -> Draw {
    let mut draw = Draw {
        ..Default::default()
    };

    for color_line in draw_line.split(",") {
        let mut color_split = color_line.trim().split(" ");
        let counter = color_split
            .next()
            .expect("Second value should be a number")
            .parse::<u32>()
            .expect("Second value should be a number");
        let color = color_split.next().expect("First value should be a color");
        match color {
            "red" => draw.red += counter,
            "green" => draw.green += counter,
            "blue" => draw.blue += counter,
            _ => {
                println!("Something is fould in Denmark: {} {}", color, counter);
            }
        }
    }

    draw
}

fn parse_game(input_line: &str) -> Game {
    let mut game_split = input_line.split(":");
    let game_id = game_split
        .next()
        .expect("There was no : in the line")
        .split(" ")
        .nth(1)
        .expect("Format was not 'Game n")
        .parse::<u32>()
        .expect("Cannor parse game id");

    let mut game = Game {
        id: game_id,
        draws: vec![],
    };
    for draw_line in game_split
        .next()
        .expect("There was no : in the line")
        .split(";")
    {
        game.draws.push(parse_draw(draw_line))
    }
    game
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

pub fn solve(input_lines: Vec<&str>) -> (u32, u32) {
    let mut id_sum = 0;
    let mut power = 0;
    for game_line in input_lines {
        let game = parse_game(game_line);
        let max_draws = game.max_draws();
        if max_draws.red <= MAX_RED && max_draws.green <= MAX_GREEN && max_draws.blue <= MAX_BLUE {
            id_sum += game.id;
        }
        power += max_draws.blue * max_draws.green * max_draws.red;
    }

    (id_sum, power)
}
