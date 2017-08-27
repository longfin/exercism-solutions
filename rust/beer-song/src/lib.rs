pub fn verse(n: i32) -> String {
    match n {
        0 => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        _ => format!("{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {m} bottle{p} of beer on the wall.\n",
                     n=n, m=n-1, p={if n-1 > 1 {"s"} else {""}}),
    }
}

pub fn sing(from: i32, to:i32) -> String {
    let mut ret = String::new();
    for i in (to..from+1).rev() {
        ret.push_str(verse(i).as_str());
        ret.push('\n');
    }
    ret.pop();
    ret
}
