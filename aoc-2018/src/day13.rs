use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Copy)]
struct Position {
    y: i32,
    x: i32,
}

impl Position {
    fn up(self) -> Position {
        Position {
            y: self.y - 1,
            x: self.x,
        }
    }
    fn down(self) -> Position {
        Position {
            y: self.y + 1,
            x: self.x,
        }
    }
    fn left(self) -> Position {
        Position {
            y: self.y,
            x: self.x - 1,
        }
    }
    fn right(self) -> Position {
        Position {
            y: self.y,
            x: self.x + 1,
        }
    }
    fn move_pos(self, direction: &Direction) -> Position {
        match direction {
            Direction::Up => self.up(),
            Direction::Down => self.down(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum TurnDirection {
    Left,
    Straight,
    Right,
}

impl TurnDirection {
    fn next(self) -> TurnDirection {
        match self {
            TurnDirection::Left => TurnDirection::Straight,
            TurnDirection::Straight => TurnDirection::Right,
            TurnDirection::Right => TurnDirection::Left,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
struct Cart {
    id: usize,
    pos: Position,
    direction: Direction,
    next_intersection: TurnDirection,
}

impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> Ordering {
        self.pos.cmp(&other.pos)
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Cart {
    fn rotate(self, track: Track) -> Cart {
        match track {
            Track::LeftCurve => self.leftcurve(),
            Track::RightCurve => self.rightcurve(),
            Track::Intersection => self.intersection(),
            _ => self.clone(),
        }
    }

    fn move_cart(self) -> Cart {
        Cart {
            id: self.id,
            pos: self.pos.move_pos(&self.direction),
            direction: self.direction,
            next_intersection: self.next_intersection,
        }
    }

    fn leftcurve(self) -> Cart {
        let next_direction = match self.direction {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
        };

        Cart {
            id: self.id,
            pos: self.pos,
            direction: next_direction,
            next_intersection: self.next_intersection,
        }
    }

    fn rightcurve(self) -> Cart {
        let next_direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Down,
        };
        Cart {
            id: self.id,
            pos: self.pos,
            direction: next_direction,
            next_intersection: self.next_intersection,
        }
    }

    fn intersection(self) -> Cart {
        let rotated_cart = match self.next_intersection {
            TurnDirection::Left => self.leftcurve(),
            TurnDirection::Right => self.rightcurve(),
            TurnDirection::Straight => self.clone(),
        };

        Cart {
            id: self.id,
            pos: rotated_cart.pos,
            direction: rotated_cart.direction,
            next_intersection: rotated_cart.next_intersection.next(),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Track {
    Intersection,
    Vertical,
    Horizontal,
    RightCurve,
    LeftCurve,
}

pub fn solve() {
    let (carts, tracks) = parse();
    solve1(&carts, tracks.clone());
    solve2(carts.to_vec(), tracks.clone());
}

fn parse() -> (Vec<Cart>, HashMap<Position, Track>) {
    let filename = "input/day13input";
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut tracks = HashMap::new();
    let mut carts = Vec::new();

    for (y, l) in contents.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let pos = Position {
                y: y as i32,
                x: x as i32,
            };

            if c == '/' {
                tracks.insert(pos, Track::RightCurve);
            } else if c == '\\' {
                tracks.insert(pos, Track::LeftCurve);
            } else if c == '-' {
                tracks.insert(pos, Track::Horizontal);
            } else if c == '|' {
                tracks.insert(pos, Track::Vertical);
            } else if c == '+' {
                tracks.insert(pos, Track::Intersection);
            } else if c == '>' {
                tracks.insert(pos.clone(), Track::Horizontal);

                let cart = Cart {
                    id: carts.len(),
                    pos: pos.clone(),
                    direction: Direction::Right,
                    next_intersection: TurnDirection::Left,
                };
                carts.push(cart);
            } else if c == '<' {
                tracks.insert(pos.clone(), Track::Horizontal);

                let cart = Cart {
                    id: carts.len(),
                    pos: pos.clone(),
                    direction: Direction::Left,
                    next_intersection: TurnDirection::Left,
                };
                carts.push(cart);
            } else if c == '^' {
                tracks.insert(pos.clone(), Track::Vertical);

                let cart = Cart {
                    id: carts.len(),
                    pos: pos.clone(),
                    direction: Direction::Up,
                    next_intersection: TurnDirection::Left,
                };
                carts.push(cart);
            } else if c == 'v' {
                tracks.insert(pos.clone(), Track::Vertical);

                let cart = Cart {
                    id: carts.len(),
                    pos: pos.clone(),
                    direction: Direction::Down,
                    next_intersection: TurnDirection::Left,
                };
                carts.push(cart);
            }
        }
    }

    (carts, tracks)
}

fn solve1(carts: &Vec<Cart>, tracks: HashMap<Position, Track>) {
    let mut moving_carts: Vec<Cart> = carts.clone();
    moving_carts.sort_unstable();

    let mut collisions: Vec<Position> = Vec::new();

    let mut crash_pos = Position { y: -1, x: -1 };
    loop {
        let collisions = collides(&moving_carts, &moving_carts);
        if collisions.len() > 0 {
            break;
        }

        let mut next_carts = Vec::new();
        let mut collided = false;
        let mut updated_carts: Vec<Cart> = moving_carts.clone();

        for cart in moving_carts.iter() {
            let new_cart = cart.move_cart();
            updated_carts.remove(0);
            updated_carts.push(new_cart.clone());
            next_carts.push(new_cart);

            let collisions = collides(&next_carts, &next_carts);

            if collisions.len() > 0 {
                println!("{:?}", collisions);
                if (collisions[0].direction == Direction::Up
                    && collisions[1].direction == Direction::Down)
                    || (collisions[0].direction == Direction::Down
                        && collisions[1].direction == Direction::Up)
                    || (collisions[0].direction == Direction::Left
                        && collisions[1].direction == Direction::Right)
                    || (collisions[0].direction == Direction::Right
                        && collisions[1].direction == Direction::Left)
                {
                    crash_pos = collisions[0].pos.clone();
                    collided = true;
                    break;
                }
            }
        }

        if collided {
            println!("Part one: {},{}", crash_pos.x, crash_pos.y);
            return;
        }

        moving_carts = next_carts.clone();

        moving_carts = moving_carts
            .iter()
            .map(|cart| {
                let track = tracks.get(&cart.pos).unwrap();
                cart.rotate(*track)
            })
            .collect();

        moving_carts.sort_unstable();
        // print_carts(&moving_carts, &tracks);
    }

    let collisions = collides(&moving_carts, &moving_carts);
    let crash_pos = collisions[0].pos.clone();

    println!("Part one: {},{}", crash_pos.x, crash_pos.y);
}

fn collides(from_carts: &Vec<Cart>, to_carts: &Vec<Cart>) -> Vec<Cart> {
    let mut collisions = Vec::new();
    for cart in from_carts.iter() {
        for other in to_carts.iter() {
            if cart.id != other.id && cart.pos == other.pos {
                collisions.push(cart.clone());
                collisions.push(other.clone());
            }
        }
    }
    return collisions;
}

fn base_map(tracks: &HashMap<Position, Track>) -> Vec<Vec<String>> {
    let mut map: Vec<Vec<String>> = vec![vec![String::from(" "); 150]; 150];
    for (pos, track) in tracks.iter() {
        let s = match track {
            Track::Intersection => String::from("+"),
            Track::Horizontal => String::from("-"),
            Track::Vertical => String::from("|"),
            Track::LeftCurve => String::from("\\"),
            Track::RightCurve => String::from("/"),
        };
        map[pos.y as usize][pos.x as usize] = s;
    }
    return map;
}

fn print_carts(carts: &Vec<Cart>, tracks: &HashMap<Position, Track>) {
    let mut map = base_map(tracks);
    for cart in carts.iter() {
        let s = match cart.direction {
            Direction::Left => String::from("<"),
            Direction::Right => String::from(">"),
            Direction::Up => String::from("^"),
            Direction::Down => String::from("v"),
        };
        map[cart.pos.y as usize][cart.pos.x as usize] = s;
    }

    for y in 0..150 {
        for x in 0..150 {
            print!("{}", map[y][x]);
        }
        print!("\n");
    }
    println!("-------------------------")
}

fn solve2(carts: Vec<Cart>, tracks: HashMap<Position, Track>) {
    println!("{:?},{:?},{:?}", 0, 0, 0);
}
