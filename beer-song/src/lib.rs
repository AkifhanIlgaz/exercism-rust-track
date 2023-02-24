pub fn verse(n: u32) -> String {
    match n {
        0 => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        _ => generate_verse(n)
    }
}

pub fn generate_verse(n: u32) -> String {
    let keywords = vec![
        generate_bottle_part(n),
        generate_take_part(n),
        generate_bottle_part(n - 1),
    ];

    format!("{0} of beer on the wall, {0} of beer.\nTake {1} down and pass it around, {2} of beer on the wall.\n", keywords[0], keywords[1], keywords[2])
}

pub fn generate_bottle_part(n: u32) -> String {
    match n {
        0 => "no more bottles".to_string(),
        1 => format!("{n} bottle"),
        _ => format!("{n} bottles"),
    }
}

pub fn generate_take_part(n: u32) -> String {
    match n {
        1 => "it".to_string(),
        _ => "one".to_string(),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut entire_song = String::new();

    for n in ((end + 1)..=start).rev() {
        let verse = format!("{}\n", verse(n));
        entire_song.push_str(&verse);
    }

    let last_verse = format!("{}", verse(end));
    entire_song.push_str(&last_verse);

    entire_song
}
