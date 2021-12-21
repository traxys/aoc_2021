use crate::{day, utils::split2, EyreResult};

day! {
    parser,
    part1 => "Score with deterministic dice {}",
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

pub(crate) fn part2(_: Parsed) -> EyreResult<()> {
    todo!()
}
