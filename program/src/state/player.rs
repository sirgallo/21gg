use borsh::{BorshDeserialize, BorshSerialize};

use super::deck::Card;


/*
  Player:

    The player is a program derived account that is linked 1 -> 1 to a table.
    This simplifies design and account management, where each table a user is registered on is treated as a unique user.
*/
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Player {
  // pub owner: Pubkey, // pub key of wallet for user --> this has to be owner, weird enforcement, just make implicit 
  pub name: String,
  pub rounds: u64,
  pub wins: u64,
  pub balance: u32,
  pub hand: Vec<Card>
}


pub fn try_from_slice(mut data: &[u8]) -> std::io::Result<Player> {
  Player::deserialize(&mut data)
}