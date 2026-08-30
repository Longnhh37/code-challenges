fn calc_poly(pol_list: &[i32], x: i32) -> String {
    let n = pol_list.len();
    let mut res = 0i32;
    let mut msg = "For ".to_string();
    let mut first = true;
    
    for (i, &coef) in pol_list.iter().enumerate() {
        if coef == 0 {
            continue;
        }
        let pow = n - 1 - i;
        res += coef * x.pow(pow as u32);
        
        if coef > 0 {
            if !first {
                msg.push_str(" + ");
            }
        } else if first {
            msg.push('-');
        } else {
            msg.push_str(" - ");
        }
        first = false;
        
        let coef_abs = coef.abs();
        if coef_abs != 1 && pow != 0 {
            msg.push_str(&coef_abs.to_string());
            msg.push('*');
        }
        
        match pow {
            0 => msg.push_str(&coef_abs.to_string()),
            1 => msg.push('x'),
            _ => msg.push_str(&format!("x^{}", pow)),
        }
    }
    
    msg.push_str(&format!(" with x = {} the value is {}", x, res));
    msg
}
​