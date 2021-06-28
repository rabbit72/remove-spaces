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
    const SPACE: char = ' ';
    let mut from_index = 0; // copy from this index
    let mut to_index = 0;  // copy to this index
    let mut is_previous_space: bool = false;  // set true when a single space is written

    let original_length = target.len();

    // step 1: skipping spaces at the begining
    while from_index < original_length && target.chars().nth(from_index).unwrap() == SPACE {
        from_index += 1;
    }
 
    // step 2: processing all chars after possible spaces
    while from_index < original_length {
        if target.chars().nth(from_index).unwrap() != SPACE {  // copies 'from_index' to 'to_index'
            let ch = target.chars().nth(from_index).unwrap().to_string();
            target.replace_range(to_index..to_index + 1, &ch);
            to_index += 1;
            is_previous_space = false;
        } else {
            if !is_previous_space {  //puts one space if the previous is not a space
                target.replace_range(to_index..to_index + 1, &SPACE.to_string());
                to_index += 1;
                is_previous_space = true;  //stops appearing more than 1 space
            }
        }
        from_index += 1;
    }
    // step 3: cutting the string inplace
    if to_index > 1 && is_previous_space {
        target.truncate(to_index - 1);
    } else {
        target.truncate(to_index);
    }
}
