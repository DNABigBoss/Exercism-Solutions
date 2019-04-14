use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut set = HashSet::new();

    if sum % 2 == 0 {
        for a in 1..=sum/3 {
            for b in a+1..=sum/2 {
                let c = sum - a - b;
                if a.pow(2) + b.pow(2) == c.pow(2) { 
                    set.insert([a,b,c]);
                }
            }
        }
        set
    }
    else {
        set
    }
}
