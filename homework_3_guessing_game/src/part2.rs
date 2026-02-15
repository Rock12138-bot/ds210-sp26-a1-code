use std::cmp::Ordering;

use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, mut min: u32, mut max: u32) -> u32 {
        
        //for guess in min..max {
        let mut mid = min + ((max-min)/2);
        let remember = player.ask_to_compare(mid);
            if remember == 0 {
                return mid
            } else if remember == -1 {
                return Part2::guess_the_number(player, min, mid);
            } else if remember == 1 {
                return Part2::guess_the_number(player, mid, max);
            } else {
                return 0;
            }
        //}

    //return 100
    }
        
        
}

