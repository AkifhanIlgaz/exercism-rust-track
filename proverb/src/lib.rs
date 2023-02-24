use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    if let Some(first_word) = list.first() {
        return list
            .windows(2)
            .map(|words| format!("For want of a {} the {} was lost.", words[0], words[1]))
            .chain(once(format!("And all for the want of a {}.", first_word)))
            .collect::<Vec<String>>()
            .join("\n");
    }

    String::new()
}
