pub fn prev_prime(nbr: u64) -> u64  {
    let mut n = nbr;
    while n > 1{
        n -= 1;
        if is_prime(n){
            return n;
        }
    }
    0
}

fn is_prime(nbr: u64) -> bool{
    match nbr{
        0 | 1 => false,
        2 => true,
        _ if nbr % 2 == 0 => false,
        _ => {
            let limit_super = (nbr as f64).sqrt() as u64;
            for i in (3..=limit_super).step_by(2){
                if nbr % i == 0{
                    return false;
                }
            }
            true
        }
    }
}