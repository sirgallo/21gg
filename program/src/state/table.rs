use borsh::{BorshSchema, BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::utils::map::DataMap;


#[derive(BorshSerialize, BorshDeserialize, BorshSchema, Clone, Debug)]
pub struct Card {
  pub suit: Suit,
  pub rank: Rank,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct Deck {
  pub cards: Vec<Card>, // 3 decks [Card; 52 * 3]
}

/*
Table Account:

The table account is a program derived account that is associated with newly created blackjack tables.

Each table consists of:
	- the name of the table
	- the creator
	- the current deck
	- the current dealer cards
	- the players, stored in a b-tree, where each node is links to another account
*/
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Table {
	// pub owner: Pubkey, // pub key generated as program derived address --> this has to be owner, weird enforcement for pdas, just make implicit 
	pub creator: Pubkey, // pub key associated with the wallet/user that owns the repository
	pub name: String,
	pub deck: Deck,
	pub round: u8, // the current round after each shuffle
  pub status: Status, // the status of the current round
	pub players: DataMap<Pubkey, Pubkey> // enforces order of players with B-tree
}

pub fn try_from_slice(mut data: &[u8]) -> std::io::Result<Table> {
  Table::deserialize(&mut data)
}



#[derive(BorshSerialize, BorshDeserialize, BorshSchema, Clone, Copy, Debug)]
pub enum Rank { 
  Two, 
  Three, 
  Four, 
  Five, 
  Six, 
  Seven, 
  Eight, 
  Nine, 
  Ten, 
  Jack, 
  Queen, 
  King, 
  Ace
}

#[derive(BorshSerialize, BorshDeserialize, BorshSchema, Clone, Copy, Debug)]
pub enum Suit { 
  Hearts, 
  Diamonds, 
  Clubs, 
  Spades
}

#[derive(BorshSerialize, BorshDeserialize, BorshSchema, Clone, Debug, PartialEq)]
pub enum Status {
  Ready, // awaiting new round
  Ongoing, // round in progress
  Completed // round completed
}


pub const DECK_SIZE: usize = 52;
pub const TOTAL_DECKS: usize = 3;
pub const SUITS: [Suit; 4] = [ Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades ];
pub const RANKS: [Rank; 13]= [
  Rank::Two, // 2
  Rank::Three, // 3
  Rank::Four, // 4
  Rank::Five, // 5
  Rank::Six, // 6
  Rank::Seven, // 7
  Rank::Eight, // 8
  Rank::Nine, // 9
  Rank::Ten, // 10
  Rank::Jack, // 10
  Rank::Queen, // 10
  Rank::King, // 10
  Rank::Ace, // 1 or 11
];