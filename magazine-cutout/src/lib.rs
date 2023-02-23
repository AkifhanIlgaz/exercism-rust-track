use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_words_count =
        magazine
            .iter()
            .fold(HashMap::new(), |mut words_count, &word| {
                *words_count.entry(word).or_insert(0) += 1;
                words_count
            });

    for &word in note {
        let count = magazine_words_count.entry(word).or_insert(0);
        if *count == 0 {
            return false;
        }
        *count -= 1
    }

    true
}
