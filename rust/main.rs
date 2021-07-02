fn main() {
    let targets = vec![
        " On  my   home world",
        "On my home world",
        "  On  my        home world    ",
        "On  my        home world  ",
        "    ",
        " ",
        "",
        "  4",
        "4  ",
        "42",
    ];

    let mut targets: Vec<String> = targets.into_iter().map(|x| x.to_string()).collect();

    for (i, target) in targets.iter_mut().enumerate() {
        print!("Case {}:\n", i + 1);
        print!("Input: |{}|\n", target);
        remove_extra_spaces(target);
        print!("Output: |{}|\n\n", target);
    }
}


pub fn remove_extra_spaces(target: &mut String) {
    let mut is_previous_space: bool = true;  // set true when a single space is written

    // step 1: processing all chars
    target.retain(|ch| {
        let retain: bool = !ch.is_whitespace() || !is_previous_space;
        is_previous_space = ch.is_whitespace();
        retain
    });

    // step 2: cutting the possible last whitespace
    target.truncate(target.trim_end().len());
}
