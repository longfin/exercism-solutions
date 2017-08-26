fn is_prime(n: u64) -> bool{
    return match n {
        1 => false,
        2 => true,
        _ => {
            let mut ret = true;
            for i in 2..((n as f64).sqrt() as u64 + 1) {
                if n % i == 0 {
                    ret = false
                }
            }
            ret
        }
    }
}

pub fn nth(n: u64) -> Result<u64, String> {
    let mut pc = 0;
    let mut i = 0;
    let mut ret = 0;

    if n == 0 {
        return Err(String::from(""));
    }

    while pc < n {
        i += 1;
        if is_prime(i) {
            ret = i;
            pc += 1;
        }
    }
    Ok(ret)
}
