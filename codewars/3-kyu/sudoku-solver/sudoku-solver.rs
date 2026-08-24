    let mut box_mask = [0u16; 9];
    let mut empties = Vec::with_capacity(81);
    
    for i in 0..9 {
        for j in 0..9 {
            let v = puzzle[i][j];
            if v == 0 {
                empties.push((i, j));
            } else {
                let bit = 1u16 << (v - 1);
                row_mask[i] |= bit;
                col_mask[j] |= bit;
                box_mask[box_idx(i, j)] |= bit;
            }
        }
    }
    
    let mut progress = true;
    
    while progress && !empties.is_empty() {
        progress = false;
        let mut idx = 0;
        while idx < empties.len() {
            let (r, c) = empties[idx];
            let b = box_idx(r, c);
            let used = row_mask[r] | col_mask[c] | box_mask[b];
            let candidates = !used & 0x1FF;
            
            if candidates.count_ones() == 1 {
                let v = candidates.trailing_zeros() as u8 + 1;
                puzzle[r][c] = v;
                
                let bit = 1u16 << (v - 1);
                row_mask[r] |= bit;
                col_mask[c] |= bit;
                box_mask[b] |= bit;
                
                empties.swap_remove(idx);
                progress = true;
            } else {
                idx += 1;
            }
        }
    }
}