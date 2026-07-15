use std::collections::HashMap;

pub fn tally(match_results: &str) -> String {
    let title = "Team                           | MP |  W |  D |  L |  P";

    if match_results.is_empty() {
        return title.to_string();
    }

    let map = build_tally(match_results);
    display_tally(title, map)
}

fn build_tally(match_results: &str) -> HashMap<String, [u32; 3]> {
    let mut map: HashMap<String, [u32; 3]> = HashMap::new();

    for result in match_results.split("\n") {
        let mut iter = result.split(';');

        let team1 = iter.next().unwrap();
        let team2 = iter.next().unwrap();
        let outcome = iter.next().unwrap();

        fn update(map: &mut HashMap<String, [u32; 3]>, team: &str, idx: usize) {
            map.entry(team.to_string()).or_insert([0; 3])[idx] += 1;
        }

        match outcome {
            "win" => {
                update(&mut map, team1, 0);
                update(&mut map, team2, 2);
            }
            "loss" => {
                update(&mut map, team1, 2);
                update(&mut map, team2, 0);
            }
            "draw" => {
                update(&mut map, team1, 1);
                update(&mut map, team2, 1);
            }
            _ => unreachable!(),
        }
    }

    map
}

fn display_tally(title: &str, map: HashMap<String, [u32; 3]>) -> String {
    let mut teams: Vec<_> = map.into_iter().collect();

    teams.sort_by(|a, b| {
        let points_a = a.1[0] * 3 + a.1[1];
        let points_b = b.1[0] * 3 + b.1[1];

        points_b.cmp(&points_a).then_with(|| a.0.cmp(&b.0))
    });

    let out_building = teams
        .iter()
        .map(|(name, [w, d, l])| {
            let mp = w + d + l;
            let p = w * 3 + d;

            format!(
                "{:<31}|{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
                name, mp, w, d, l, p
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let mut out = String::new();
    out.push_str(title);
    out.push('\n');
    out.push_str(&out_building);

    out
}
