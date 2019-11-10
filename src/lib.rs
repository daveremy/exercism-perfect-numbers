#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    match num {
        0 => None,
        1 => Some(Classification::Deficient),
        _ => match (2..=sqrt(num)).fold(1, |sum, i| sum_factor(num, sum, i)) {
            f if f == num => Some(Classification::Perfect),
            f if f > num => Some(Classification::Abundant),
            f if f < num => Some(Classification::Deficient),
            _ => None,
        },
    }
}

fn sqrt(n: u64) -> u64 {
    (n as f64).sqrt() as u64
}

fn sum_factor(num: u64, sum: u64, i: u64) -> u64 {
    if num % i == 0 {
        if i == num / i {
            sum + i
        } else {
            sum + i + num / i
        }
    } else {
        sum
    }
}
