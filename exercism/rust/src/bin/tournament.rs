use std::collections::HashMap;

// ===== Domain model =====
#[derive(Default, Debug, Clone, Copy)]
struct Record {
    win: u32,
    draw: u32,
    loss: u32,
}

impl Record {
    fn points(&self) -> u32 {
        self.win * 3 + self.draw
    }

    fn matches(&self) -> u32 {
        self.win + self.draw + self.loss
    }
}

// ===== Outcome enum =====
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn parse(s: &str) -> Self {
        match s {
            "win" => Outcome::Win,
            "loss" => Outcome::Loss,
            "draw" => Outcome::Draw,
            _ => unreachable!(),
        }
    }
}

// ===== Parse 1 dòng =====
fn parse_line(line: &str) -> (&str, &str, Outcome) {
    let mut it = line.split(';');
    let t1 = it.next().unwrap();
    let t2 = it.next().unwrap();
    let outcome = Outcome::parse(it.next().unwrap());
    (t1, t2, outcome)
}

// ===== Main =====
pub fn tally(input: &str) -> String {
    let title = "Team                           | MP |  W |  D |  L |  P";

    if input.is_empty() {
        return title.to_string();
    }

    // =====  Build map  =====
    let map = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(parse_line)
        .fold(
            HashMap::<String, Record>::new(),
            |mut acc, (t1, t2, outcome)| {
                match outcome {
                    Outcome::Win => {
                        {
                            let r1 = acc.entry(t1.to_string()).or_default();
                            r1.win += 1;
                        }

                        {
                            let r2 = acc.entry(t2.to_string()).or_default();
                            r2.loss += 1;
                        }
                    }
                    Outcome::Loss => {
                        {
                            let r1 = acc.entry(t1.to_string()).or_default();
                            r1.loss += 1;
                        }

                        {
                            let r2 = acc.entry(t2.to_string()).or_default();
                            r2.win += 1;
                        }
                    }
                    Outcome::Draw => {
                        {
                            let r1 = acc.entry(t1.to_string()).or_default();
                            r1.draw += 1;
                        }

                        {
                            let r2 = acc.entry(t2.to_string()).or_default();
                            r2.draw += 1;
                        }
                    }
                }

                acc
            },
        );

    display(title, map)
}

// ===== Display =====
fn display(title: &str, map: HashMap<String, Record>) -> String {
    let mut teams: Vec<_> = map.into_iter().collect();

    teams.sort_by(|a, b| b.1.points().cmp(&a.1.points()).then_with(|| a.0.cmp(&b.0)));

    let body = teams
        .iter()
        .map(|(name, r)| {
            format!(
                "{:<31}|{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
                name,
                r.matches(),
                r.win,
                r.draw,
                r.loss,
                r.points()
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!("{}\n{}", title, body)
}
