use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::{day, EyreResult};

day! {
    parser,
    part1 => "Path score is {}",
    part2 => "Path score is {}",
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Amphipod {
    None,
    A,
    B,
    C,
    D,
}

impl std::fmt::Display for Amphipod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let disp = match self {
            Amphipod::None => ".",
            Amphipod::A => "A",
            Amphipod::B => "B",
            Amphipod::C => "C",
            Amphipod::D => "D",
        };

        write!(f, "{}", disp)
    }
}

impl Default for Amphipod {
    fn default() -> Self {
        Self::None
    }
}

impl Amphipod {
    fn cost(&self) -> u64 {
        match self {
            Amphipod::None => 0,
            Amphipod::A => 1,
            Amphipod::B => 10,
            Amphipod::C => 100,
            Amphipod::D => 1000,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub(crate) struct Board<const N: usize> {
    hall: [Amphipod; 11],
    a_room: [Amphipod; N],
    b_room: [Amphipod; N],
    c_room: [Amphipod; N],
    d_room: [Amphipod; N],
}

impl<const N: usize> Default for Board<N> {
    fn default() -> Self {
        fn amphi_none<const X: usize>() -> [Amphipod; X] {
            [Amphipod::None; X]
        }

        Self {
            hall: amphi_none(),
            a_room: amphi_none(),
            b_room: amphi_none(),
            c_room: amphi_none(),
            d_room: amphi_none(),
        }
    }
}

fn a_star<const N: usize>(start: Board<N>) -> (u64, Vec<Board<N>>) {
    fn reconstruct_path<const N: usize>(
        come_from: &HashMap<Board<N>, Board<N>>,
        mut current: Board<N>,
    ) -> Vec<Board<N>> {
        let mut total_path = vec![current];
        loop {
            match come_from.get(&current) {
                None => {
                    total_path.reverse();
                    return total_path;
                }
                Some(v) => {
                    current = *v;
                    total_path.push(current);
                }
            }
        }
    }

    #[derive(PartialEq, Eq)]
    struct Path<const N: usize> {
        board: Board<N>,
        f_score: u64,
    }

    impl<const N: usize> Ord for Path<N> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.f_score.cmp(&self.f_score)
        }
    }

    impl<const N: usize> PartialOrd for Path<N> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let goal = Board {
        hall: [Amphipod::None; 11],
        a_room: [Amphipod::A; N],
        b_room: [Amphipod::B; N],
        c_room: [Amphipod::C; N],
        d_room: [Amphipod::D; N],
    };

    let mut paths = BinaryHeap::new();
    paths.push(Path {
        f_score: start.heuristic(),
        board: start,
    });

    let mut come_from = HashMap::new();

    let mut g_scores = HashMap::new();
    g_scores.insert(start, 0);

    let mut f_scores = HashMap::new();
    f_scores.insert(start, start.heuristic());

    while !paths.is_empty() {
        let best = paths.pop().unwrap();

        if best.board == goal {
            return (
                *g_scores.get(&best.board).unwrap(),
                reconstruct_path(&come_from, goal),
            );
        }

        if let Some(&f) = f_scores.get(&best.board) {
            if f < best.f_score {
                continue;
            }
        }

        for (cost, neigh) in best.board.possible_steps() {
            let tentative_g = g_scores.get(&best.board).unwrap() + cost;

            match g_scores.get_mut(&neigh) {
                Some(g) if tentative_g < *g => {
                    *g = tentative_g;
                }
                None => {
                    g_scores.insert(neigh, tentative_g);
                }
                _ => continue,
            }

            let f_score = tentative_g + neigh.heuristic();
            f_scores.insert(neigh, f_score);
            paths.push(Path {
                f_score,
                board: neigh,
            });
            come_from.insert(neigh, best.board);
        }
    }

    panic!("No path to goal")
}

impl<const N: usize> Board<N> {
    fn rooms(&self) -> [([Amphipod; N], Amphipod, usize); 4] {
        [
            (self.a_room, Amphipod::A, 2),
            (self.b_room, Amphipod::B, 4),
            (self.c_room, Amphipod::C, 6),
            (self.d_room, Amphipod::D, 8),
        ]
    }

    fn possible_steps(&self) -> HashSet<(u64, Self)> {
        fn can_go_in_room<const N: usize>(
            room: [Amphipod; N],
            room_type: Amphipod,
            amphi: Amphipod,
        ) -> bool {
            amphi == room_type
                && room[0] == Amphipod::None
                && room[1..]
                    .iter()
                    .all(|&r| r == room_type || r == Amphipod::None)
        }

        let mut positions = HashSet::new();

        for (p, &amphi) in self.hall.iter().enumerate() {
            let rooms = self.rooms();

            for (mut room, ty, pos) in rooms {
                let range = if p < pos {
                    (p + 1)..=pos
                } else {
                    pos..=(p - 1)
                };
                if can_go_in_room(room, ty, amphi)
                    && self.hall[range].iter().all(|&p| p == Amphipod::None)
                {
                    let mut new_board = *self;
                    new_board.hall[p] = Amphipod::None;

                    let mut free_idx = 0;
                    for i in 0..N {
                        if room[i] == Amphipod::None {
                            free_idx = i;
                        }
                    }

                    room[free_idx] = amphi;

                    match ty {
                        Amphipod::A => new_board.a_room = room,
                        Amphipod::B => new_board.b_room = room,
                        Amphipod::C => new_board.c_room = room,
                        Amphipod::D => new_board.d_room = room,
                        _ => unreachable!(),
                    }
                    positions.insert((
                        ((pos as isize - p as isize).abs() as u64 + free_idx as u64 + 1)
                            * amphi.cost(),
                        new_board,
                    ));
                }
            }
        }

        for (room, ty, pos) in self.rooms() {
            let mut possible_interval = (pos, pos);
            loop {
                let mut increased = false;

                if possible_interval.0 > 0 && self.hall[possible_interval.0 - 1] == Amphipod::None {
                    possible_interval.0 -= 1;
                    increased = true;
                }

                if possible_interval.1 < 11 - 1
                    && self.hall[possible_interval.1 + 1] == Amphipod::None
                {
                    possible_interval.1 += 1;
                    increased = true;
                }

                if !increased {
                    break;
                }
            }

            let mut target_room = room;

            let first_non_free = match room.iter().enumerate().find(|&(_, &a)| a != Amphipod::None)
            {
                None => continue,
                Some((i, _)) => i,
            };

            let (steps_out, amphi) = if first_non_free == N - 1 {
                if room[first_non_free] == ty {
                    continue;
                } else {
                    target_room[first_non_free] = Amphipod::None;
                    (first_non_free + 1, room[first_non_free])
                }
            } else {
                if room[first_non_free] != ty || room[first_non_free + 1..].iter().any(|&r| r != ty)
                {
                    target_room[first_non_free] = Amphipod::None;
                    (first_non_free + 1, room[first_non_free])
                } else {
                    continue;
                }
            };

            for hall_pos in possible_interval.0..=possible_interval.1 {
                match hall_pos {
                    2 | 4 | 6 | 8 => continue,
                    _ => (),
                }

                let mut new_board = *self;
                new_board.hall[hall_pos] = amphi;
                match ty {
                    Amphipod::A => new_board.a_room = target_room,
                    Amphipod::B => new_board.b_room = target_room,
                    Amphipod::C => new_board.c_room = target_room,
                    Amphipod::D => new_board.d_room = target_room,
                    _ => unreachable!(),
                }

                positions.insert((
                    ((pos as isize - hall_pos as isize).abs() as u64 + steps_out as u64)
                        * amphi.cost(),
                    new_board,
                ));
            }
        }

        positions
    }

    fn heuristic(&self) -> u64 {
        fn hall_steps(pos: isize, amphi: Amphipod) -> u64 {
            let room_idx = match amphi {
                Amphipod::None => return 0,
                Amphipod::A => 2,
                Amphipod::B => 4,
                Amphipod::C => 6,
                Amphipod::D => 8,
            };
            (pos - room_idx).abs() as u64 + 1
        }

        let mut total = 0;
        for (p, a) in self.hall.iter().enumerate() {
            total += hall_steps(p as isize, *a) * a.cost();
        }

        fn amphi_room<const N: usize>(
            room: [Amphipod; N],
            room_type: Amphipod,
            room_idx: isize,
        ) -> u64 {
            let mut total = 0;

            for i in 0..N {
                if room[i] != room_type {
                    total += (hall_steps(room_idx, room[i]) + i as u64 + 1) * room[i].cost()
                }
            }

            total
        }

        total += amphi_room(self.a_room, Amphipod::A, 2);
        total += amphi_room(self.b_room, Amphipod::B, 4);
        total += amphi_room(self.c_room, Amphipod::C, 6);
        total += amphi_room(self.d_room, Amphipod::D, 8);

        total
    }
}

impl<const N: usize> std::fmt::Display for Board<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..13 {
            write!(f, "#")?;
        }

        write!(f, "\n#")?;
        for amphi in &self.hall {
            write!(f, "{}", amphi)?;
        }
        write!(
            f,
            "#\n###{}#{}#{}#{}###\n",
            self.a_room[0], self.b_room[0], self.c_room[0], self.d_room[0]
        )?;

        for i in 1..N {
            write!(
                f,
                "  #{}#{}#{}#{}#  \n",
                self.a_room[i], self.b_room[i], self.c_room[i], self.d_room[i]
            )?;
        }

        write!(f, "  #########   \n")
    }
}

type Parsed = Board<2>;

pub(crate) fn parser(input: &str) -> EyreResult<Parsed> {
    let mut lines = input.lines().skip(2);
    let mut board = Board::default();
    let amphi_filter = |a| match a {
        b'A' => Some(Amphipod::A),
        b'B' => Some(Amphipod::B),
        b'C' => Some(Amphipod::C),
        b'D' => Some(Amphipod::D),
        _ => None,
    };

    let mut add_lines = |i| {
        for (idx, amphi) in lines
            .next()
            .unwrap()
            .bytes()
            .filter_map(amphi_filter)
            .enumerate()
        {
            match idx {
                0 => board.a_room[i] = amphi,
                1 => board.b_room[i] = amphi,
                2 => board.c_room[i] = amphi,
                3 => board.d_room[i] = amphi,
                _ => unreachable!(),
            }
        }
    };

    add_lines(0);
    add_lines(1);

    Ok(board)
}

pub(crate) fn part1(board: Parsed) -> EyreResult<u64> {
    Ok(a_star(board).0)
}

pub(crate) fn part2(board: Parsed) -> EyreResult<u64> {
    use Amphipod::*;

    let a_room = [board.a_room[0], D, D, board.a_room[1]];
    let b_room = [board.b_room[0], C, B, board.b_room[1]];
    let c_room = [board.c_room[0], B, A, board.c_room[1]];
    let d_room = [board.d_room[0], A, C, board.d_room[1]];

    let board = Board {
        hall: board.hall,
        a_room,
        b_room,
        c_room,
        d_room,
    };
    let (score, _) = a_star(board);
    /* for p in path {
        println!("STEP:\n{}", p);
    } */
    Ok(score)
}
