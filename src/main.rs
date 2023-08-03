use std::time::Instant;
use std::{fs, io};

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();
    let sentence = get_sentence(&mut rng);
    let scrambled_sentence = scramble(&sentence, &mut rng);

    let start = Instant::now();
    println!("{:?}", scrambled_sentence);

    while guess(&sentence) {
        println!("Wrong");
        println!("{sentence}")
    }
    let duration = start.elapsed();
    println!("Correct!");
    println!("It took you {:?}", duration);

    let (pos, mut entries, time_entries) = get_leaderboard(duration.as_millis());
    println!(
        "This means that you are in position {} on the leaderboard",
        pos + 1
    );
    println!("Enter your username to continue");
    let mut player = String::new();
    io::stdin()
        .read_line(&mut player)
        .expect("failed to readline");

    player.insert(0, ' ');
    entries.insert(pos, player.trim().to_string());
    let mut new_file = String::new();
    for (name, time) in entries.into_iter().zip(time_entries.into_iter()) {
        let line = format!("{time} {name}\n");
        print!("{line}");
        new_file.push_str(&line);
    }
    fs::write("leaderboard", new_file).expect("error writing leaderboard");
}

fn get_leaderboard(time: u128) -> (usize, Vec<String>, Vec<u128>) {
    let contents = fs::read_to_string("leaderboard").expect("oop");
    let entries: Vec<String> = contents
        .lines()
        .map(|s| s.split_whitespace().nth(1).unwrap().to_owned())
        .collect();
    let mut entries_time: Vec<u128> = contents
        .lines()
        .map(|s| s.split_whitespace().next().unwrap().parse().unwrap())
        .collect();
    match entries_time.binary_search(&time) {
        Ok(pos) => (pos, entries, entries_time),
        Err(pos) => {
            entries_time.insert(pos, time);
            (pos, entries, entries_time)
        }
    }
}

fn guess(sentence: &str) -> bool {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to readline");
    if guess.to_lowercase().trim() == sentence.to_lowercase() {
        false
    } else {
        true
    }
}

fn scramble(original: &str, rng: &mut ThreadRng) -> String {
    let mut words = original
        .split_whitespace()
        .map(|w| w.chars().collect::<Vec<char>>());

    let mut new_sentence = String::new();

    for mut w in &mut words {
        println!("{:?}", w);
        w.shuffle(rng);
        for c in w {
            new_sentence.push(c)
        }
        new_sentence.push(' ')
    }

    return new_sentence;
}

fn get_sentence(rng: &mut ThreadRng) -> String {
    let contents = fs::read_to_string("sentences").expect("oop");
    let lines: Vec<String> = contents.lines().map(|s| s.to_owned()).collect();
    lines.choose(rng).unwrap().to_owned()
}
