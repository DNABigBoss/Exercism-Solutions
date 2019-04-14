#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None
    }
    else {
            let sum:u64 = (1..num/2+1).filter(|i| num%i==0).sum(); 
            match sum {
                sum if sum == num => {Some(Classification::Perfect)},
                sum if sum > num => {Some(Classification::Abundant)},
                _ => {Some(Classification::Deficient)}
            }
    }
}
