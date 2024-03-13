// pub mod time_map;
// pub mod check_inclusion;
// pub mod longest_repeating_string;
// pub mod min_window_substring;
// pub mod lru_cache;
pub mod word_machine;

fn main() { 
    let result = word_machine::word_machine("4 5 6 - 7 +".to_string());
    println!("{:?}", result);
}