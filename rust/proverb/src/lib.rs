pub fn build_proverb(things: Vec<&str>) -> String {
    if things.len() == 0 {
        return String::new();
    }

    let mut ret:Vec<String> = Vec::new();
    let mut iter = things.iter();
    let mut first = iter.next();
    loop {
        match first {
            Some(f) => {
                let second = iter.next();
                match second {
                    Some(s) => ret.push(format!("For want of a {} the {} was lost.", f, s)),
                    _ => break
                }
                first = second;
            },
            _ => break
        }
    }
    let root;
    if things.len() < 3 {
        root = String::from(things[0]);
    } else {
        root = format!("{}{} {}", things[2], things[1], things[0]);
    }
    ret.push(format!("And all for the want of a {}.", root));
    return ret.join("\n");
}
