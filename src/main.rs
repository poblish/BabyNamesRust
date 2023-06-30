fn main() {
    let mut fore_names: Vec<&str> = vec![
        "Rohan", "Nathaniel", "Anthony", "Chris", "Jonathan", "Lemur", "Harry", "Percy", "Peregrine",
        "James", "Jamie", "Sidney", "Gabriel", "Leyton", "Curtley", "Jarvis",
    ];
    let mut middle_names: Vec<&str> = vec![
        "Rohan", "Nathaniel", "Tony", "Chris", "Jonathan", "Lemur", "Harry", "Percy", "Peregrine",
        "James", "Jamie", "Sidney", "Gabriel", "Leyton", "Curtley", "Jarvis",
    ];
    let mut count = 0;

    fore_names.sort_unstable();
    middle_names.sort_unstable();

    for (_i, name1) in fore_names.iter().enumerate() {
        count += 1;
        println!("{} ... {} Jordan-Regan", count, name1);

        let syllables1 = syllable_count(name1);

        for name2 in middle_names.iter() {
            if name1 != name2 {
                let syllables2 = syllable_count(name2);

                if name1.chars().next() == name2.chars().next()
                    || (syllables1 == 1 && syllables2 == 1)
                    || (syllables1 == 1 && syllables2 >= 3)
                    || (syllables1 >= 3 && syllables2 >= 3)
                    || (syllables1 >= 3 && syllables2 == 1)
                {
                    continue;
                }

                count += 1;
                println!("{} ... {} {} Jordan-Regan", count, name1, name2);
            }
        }
    }
}

fn syllable_count(in_str: &str) -> usize {
    if in_str == "Anthony" || in_str == "Gabriel" {
        return 3;
    }

    if in_str == "James" {
        return 1;
    }

    let runes_1: Vec<char> = in_str.chars().collect();
    let mut syllables = 0;
    let mut last_char: char = '.';
    let mut was_vowel = false;

    for c in runes_1.iter().map(|&ch| ch.to_ascii_lowercase()) {
        if was_vowel && ((last_char == 'u' && c == 'a') || (last_char == 'i' && c == 'a')) {
            syllables += 1;
        } else if is_vowel(c) || c == 'y' {
            if !was_vowel && (!(c == 'e') || !runes_1.is_empty()) {
                syllables += 1;
                was_vowel = true;
            }
        } else {
            was_vowel = false;
        }

        last_char = c;
    }

    if syllables == 0 {
        return 1;
    }

    syllables
}

fn is_vowel(ch: char) -> bool {
    match ch {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}