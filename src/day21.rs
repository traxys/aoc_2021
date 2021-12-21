use crate::{day, utils::split2, EyreResult};
use std::collections::HashMap;

day! {
    parser,
    part1 => "Score with deterministic dice: {}",
    part2 => "Wins with dirac dice: {}",
}

#[derive(Debug)]
pub(crate) struct State {
    die: u64,
    total_rolls: u64,
    player1: u8,
    score1: u64,
    player2: u8,
    score2: u64,
}

impl State {
    fn roll(&mut self) -> u64 {
        self.roll1() + self.roll1() + self.roll1()
    }

    fn roll1(&mut self) -> u64 {
        let res = self.die + 1;
        self.die = (self.die + 1) % 100;
        self.total_rolls += 1;
        res
    }

    fn play1(&mut self) {
        let roll = self.roll();
        let next = (self.player1 as u64 + roll) % 10;
        self.player1 = next as u8;
        self.score1 += next + 1;
    }

    fn play2(&mut self) {
        let roll = self.roll();
        let next = (self.player2 as u64 + roll) % 10;
        self.player2 = next as u8;
        self.score2 += next + 1;
    }

    fn win1(&self) -> bool {
        self.score1 >= 1000
    }

    fn win2(&self) -> bool {
        self.score2 >= 1000
    }

    fn play(&mut self) -> u8 {
        loop {
            self.play1();
            if self.win1() {
                return 1;
            }

            self.play2();
            if self.win2() {
                return 2;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::State;

    fn null_state() -> State {
        State {
            die: 0,
            player1: 0,
            player2: 0,
            score1: 0,
            score2: 0,
            total_rolls: 0,
        }
    }

    #[test]
    fn roll() {
        let mut state = null_state();
        assert_eq!(state.roll(), 1 + 2 + 3);
        assert_eq!(state.roll(), 4 + 5 + 6);
    }

    #[test]
    fn play() {
        macro_rules! assert_play {
            ($s:expr, 1, $pos:expr, $score:expr) => {
                $s.play1();
                assert_eq!($s.player1, $pos - 1, "player1 in invalid position");
                assert_eq!($s.score1, $score, "player1 with invalid score");
            };

            ($s:expr, 2, $pos:expr, $score:expr) => {
                $s.play2();
                assert_eq!($s.player2, $pos - 1, "player2 in invalid position");
                assert_eq!($s.score2, $score, "player2 with invalid score");
            };
        }

        let mut state = null_state();
        state.player1 = 3;
        state.player2 = 7;
        assert_play!(state, 1, 10, 10);
        assert_play!(state, 2, 3, 3);
        assert_play!(state, 1, 4, 14);
        assert_play!(state, 2, 6, 9);
        assert_play!(state, 1, 6, 20);
        assert_play!(state, 2, 7, 16);
        assert_play!(state, 1, 6, 26);
        assert_play!(state, 2, 6, 22);
    }
}

type Parsed = State;

fn parse_player(p: &str) -> EyreResult<u8> {
    let (_, p) = split2(p, ":").ok_or(color_eyre::eyre::eyre!("Malformed player"))?;
    Ok(p.trim().parse()?)
}

pub(crate) fn parser(input: &str) -> EyreResult<Parsed> {
    let (p1, p2) = split2(input, "\n").ok_or(color_eyre::eyre::eyre!("no newline"))?;
    Ok(State {
        total_rolls: 0,
        die: 0,
        player1: parse_player(p1)? - 1,
        player2: parse_player(p2)? - 1,
        score1: 0,
        score2: 0,
    })
}

pub(crate) fn part1(mut state: Parsed) -> EyreResult<u64> {
    let winner = state.play();
    Ok(if winner == 1 {
        state.score2 * state.total_rolls
    } else {
        state.score1 * state.total_rolls
    })
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Situation {
    score1: u8,
    score2: u8,
    pos1: u8,
    pos2: u8,
}

fn score_next(die: u8, pos: u8) -> (u8, u8) {
    let next = (pos + die) % 10;
    let score = next + 1;
    (next, score)
}

impl Situation {
    fn is_win(&self) -> bool {
        self.win1() || self.win2()
    }

    fn win1(&self) -> bool {
        self.score1 >= 21
    }

    fn win2(&self) -> bool {
        self.score2 >= 21
    }

    fn play1(self, roll: u8) -> Self {
        let (next, score) = score_next(roll, self.pos1);
        let mut new = self;
        new.pos1 = next;
        new.score1 += score;
        new
    }

    fn dirac1(self) -> [(Situation, usize); 7] {
        [
            (self.play1(3), 1),
            (self.play1(4), 3),
            (self.play1(5), 6),
            (self.play1(6), 7),
            (self.play1(7), 6),
            (self.play1(8), 3),
            (self.play1(9), 1),
        ]
    }

    fn play2(self, roll: u8) -> Self {
        let (next, score) = score_next(roll, self.pos2);
        let mut new = self;
        new.pos2 = next;
        new.score2 += score;
        new
    }

    fn dirac2(self) -> [(Situation, usize); 7] {
        [
            (self.play2(3), 1),
            (self.play2(4), 3),
            (self.play2(5), 6),
            (self.play2(6), 7),
            (self.play2(7), 6),
            (self.play2(8), 3),
            (self.play2(9), 1),
        ]
    }
}

struct DiracState {
    situations: HashMap<Situation, usize>,
}

impl DiracState {
    fn new(initial: State) -> Self {
        let mut situations = HashMap::new();

        situations.insert(
            Situation {
                score1: 0,
                score2: 0,
                pos1: initial.player1,
                pos2: initial.player2,
            },
            1,
        );

        Self { situations }
    }

    fn step(&mut self, player1: bool) -> bool {
        let mut new_situations = HashMap::with_capacity(self.situations.len() * 3);

        let mut all_win = true;

        for (&situation, &count) in self.situations.iter() {
            if situation.is_win() {
                *new_situations.entry(situation).or_insert(0) += count;
            } else {
                let situations = if player1 {
                    situation.dirac1()
                } else {
                    situation.dirac2()
                };

                for (situation, repeat) in situations {
                    *new_situations.entry(situation).or_insert(0) += repeat * count;
                }

                all_win = false;
            }
        }

        self.situations = new_situations;
        all_win
    }

    fn run(&mut self) {
        let mut player1 = true;
        loop {
            if self.step(player1) {
                return;
            }

            player1 = !player1;
        }
    }

    fn count_wins(&self) -> (usize, usize) {
        self.situations
            .iter()
            .fold((0, 0), |(win1, win2), (situation, count)| {
                assert!(situation.is_win());
                if situation.win1() {
                    assert!(!situation.win2());
                    (win1 + count, win2)
                } else {
                    (win1, win2 + count)
                }
            })
    }
}

pub(crate) fn part2(state: Parsed) -> EyreResult<usize> {
    let mut dirac_state = DiracState::new(state);
    dirac_state.run();
    let (win1, win2) = dirac_state.count_wins();
    Ok(std::cmp::max(win1, win2))
}
