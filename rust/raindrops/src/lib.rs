pub fn raindrops(n: u32) -> String {
    if n % 3 ==0 {
        if n % 5==0 {
            if n%7==0 {
                return "PlingPlangPlong".to_owned();
            }
            else {
                return "PlingPlang".to_owned();
            }
        } 
        else if n%7==0 {
            return "PlingPlong".to_owned();
        }
        else {
            return "Pling".to_owned();
        }
    } else if n%5==0 {
        if n%7==0 {
            return "PlangPlong".to_owned();
        }
        else {
            return "Plang".to_owned();
        }
    } else if n%7==0 {
        return "Plong".to_owned();
    } else {
        return n.to_string();
    }
}