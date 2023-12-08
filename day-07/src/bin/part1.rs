use std::collections::BTreeSet;

use day_07::{calculate_total_winnings, rank_hands, Hand};

fn main() {
    // automatically sorts the hands using Ord implementation in ascending order
    let hands: BTreeSet<Hand> = rank_hands("input.txt");
    let total_winnings = calculate_total_winnings(&hands);
    println!("Total winnings: {total_winnings}");
}
