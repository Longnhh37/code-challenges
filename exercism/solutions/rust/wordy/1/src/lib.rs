pub fn answer(command: &str) -> Option<i32> {
    let mut tokens = command.split_ascii_whitespace().collect::<Vec<&str>>();
    let len = tokens.len();

    //strip '?'
    if let Some(cur) = tokens.last() {
        let tmp = &cur[..cur.len() - 1];
        tokens[len - 1] = tmp;
    }

    let mut operations: Vec<&str> = Vec::new();
    let mut i = 0;

    while i < len {
        let cur = tokens[i];

        match cur {
            "plus" => {
                operations.push("+");
                i += 1;
            }
            "minus" => {
                operations.push("-");
                i += 1;
            }
            "multiplied" => {
                operations.push("*");
                i += 2;
            }
            "divided" => {
                operations.push("/");
                i += 2;
            }
            "raised" => {
                operations.push("pow");
                let mut pow = tokens[i + 3];
                pow = &pow[..pow.len() - 2];
                operations.push(pow);
                i += 5;
            }
            "cubed" => return None,
            other if other.parse::<i32>().is_ok() => {
                operations.push(other);
                i += 1;
            }
            _ => i += 1,
        }
    }

    if operations.is_empty() {
        return None;
    }

    let mut res = match operations[0].parse::<i32>() {
        Ok(v) => v,
        _ => return None,
    };

    let mut i = 1;

    while i < operations.len() {
        let cmd = operations[i];
        let rhs = operations.get(i + 1)?.parse::<i32>().ok()?;

        match cmd {
            "+" => {
                res += rhs;
                i += 2;
            }
            "-" => {
                res -= rhs;
                i += 2;
            }
            "*" => {
                res *= rhs;
                i += 2;
            }
            "/" => {
                res /= rhs;
                i += 2;
            }
            "pow" => {
                res = res.pow(rhs as u32);
                i += 2;
            }
            _ => unreachable!(),
        }
    }

    Some(res)
}
