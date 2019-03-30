use std::cmp::{Ord, Ordering};

pub fn qsort<T: Ord>(set: &mut Vec<T>) {
    if set.len() == 0 {
        return;
    }
    let j = set.len() - 1;
    let i = 0;
    qsort_part(set, i, j);
}

fn qsort_part<T: Ord>(set: &mut Vec<T>, i: usize, j: usize) {
    // if length is zero or 1
    if j <= i {
        return;
    }
    // if only 2 elements in vec slice
    if i + 1 == j {
        if set[i].cmp(&set[j]) == Ordering::Greater {
            set.swap(i, j);
        }
        return;
    }
    let p = i;
    let less = &mut (i + 1);
    let great = &mut j.clone();
    move_from_less(set, p, less, great);
    if p != *great {
        set.swap(p, *great);
        qsort_part(set, p, *great - 1);
    }
    if *great + 1 < j {
        qsort_part(set, *great + 1, j);
    }
}

fn move_from_less<T: Ord>(set: &mut Vec<T>, p: usize, less: &mut usize, great: &mut usize) {
    while less <= great {
        match set[p].cmp(&set[*less]) {
            Ordering::Less => {
                return move_from_greater(set, p, less, great);
            }
            Ordering::Equal | Ordering::Greater => {
                *less += 1;
                continue;
            }
        }
    }
}

fn move_from_greater<T: Ord>(set: &mut Vec<T>, p: usize, less: &mut usize, great: &mut usize) {
    while less <= great {
        match set[p].cmp(&set[*great]) {
            Ordering::Equal | Ordering::Greater => {
                set.swap(*less, *great);
                *less += 1;
                *great -= 1;
                return move_from_less(set, p, less, great);
            }
            Ordering::Less => {
                *great -= 1;
                continue;
            }
        }
    }
}

#[cfg(test)]
mod test;
