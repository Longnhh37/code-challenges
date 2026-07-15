#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, Clone)]
enum Frame {
    Strike,
    Spare(u16, u16),
    Open(u16, u16),
    Tenth(Vec<u16>),
}

pub struct BowlingGame {
    frames: Vec<Frame>,
    current: Vec<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: vec![],
            current: vec![],
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.frames.len() == 10 {
            return Err(Error::GameComplete);
        }

        // Frame 1 -> 9
        if self.frames.len() < 9 {
            match self.current.len() {
                0 => {
                    if pins == 10 {
                        self.frames.push(Frame::Strike);
                    } else {
                        self.current.push(pins);
                    }
                }
                1 => {
                    let first = self.current[0];
                    if first + pins > 10 {
                        return Err(Error::NotEnoughPinsLeft);
                    }

                    if first + pins == 10 {
                        self.frames.push(Frame::Spare(first, pins));
                    } else {
                        self.frames.push(Frame::Open(first, pins));
                    }

                    self.current.clear();
                }
                _ => unreachable!(),
            }

            return Ok(());
        }

        // Frame 10
        self.current.push(pins);

        let r = &self.current;

        match r.len() {
            1 => Ok(()),

            2 => {
                if r[0] != 10 && r[0] + r[1] > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }

                if r[0] + r[1] < 10 {
                    self.frames.push(Frame::Tenth(r.clone()));
                    self.current.clear();
                }

                Ok(())
            }

            3 => {
                let r0 = r[0];
                let r1 = r[1];

                if !(r0 == 10 || r0 + r1 == 10) {
                    return Err(Error::GameComplete);
                }

                if r0 == 10 && r1 < 10 && r1 + r[2] > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }

                self.frames.push(Frame::Tenth(r.clone()));
                self.current.clear();

                Ok(())
            }
            _ => Err(Error::GameComplete),
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.frames.len() < 10 {
            return None;
        }

        let mut total = 0;

        for i in 0..10 {
            match &self.frames[i] {
                Frame::Strike => {
                    total += 10 + self.next_rolls(i, 2)?;
                }

                Frame::Spare(_, _) => {
                    total += 10 + self.next_rolls(i, 1)?;
                }

                Frame::Open(a, b) => {
                    total += a + b;
                }

                Frame::Tenth(rolls) => {
                    total += rolls.iter().sum::<u16>();
                }
            }
        }

        Some(total)
    }

    fn next_rolls(&self, frame_idx: usize, count: usize) -> Option<u16> {
        let mut rolls = vec![];

        for frame in &self.frames[frame_idx + 1..] {
            match frame {
                Frame::Strike => rolls.push(10),

                Frame::Spare(a, b) | Frame::Open(a, b) => {
                    rolls.push(*a);
                    rolls.push(*b);
                }

                Frame::Tenth(rs) => {
                    rolls.extend(rs);
                }
            }
            
            if rolls.len() >= count {
                return Some(rolls.iter().take(count).sum());
            }
        }

        None
    }
}
