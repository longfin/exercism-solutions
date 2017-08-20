pub fn raindrops(number: i32) -> String {
    let factors = vec![(3, "Pling"), (5, "Plang"), (7, "Plong")];
    let mut ret = String::from("");

    for (f, s) in factors {
        if number % f == 0 {
            ret.push_str(s)
        }
    }

    if ret.is_empty() {
        ret = number.to_string()
    }
    return ret;
}
