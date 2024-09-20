use rand::thread_rng;
use rand::seq::SliceRandom;
use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  program::invoke,
  program_error::ProgramError,
  pubkey::Pubkey,
  system_instruction::create_account
};

use crate::state::table::{
  Card, Status, Table,
  DECK_SIZE, TOTAL_DECKS, RANKS, SUITS
};

use crate::utils::account::create_pda_account;
use crate::utils::data::deserialize_data;

/*
  open_table:
    create a pda for the table, which links pdas for both players and the c
    once registered, the player join and leave different rounds as long as the table is not retired.
    
    player pda <- table pubkey
*/
pub fn open_table(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
  create_pda_account(program_id, accounts, data)
}

// make a table unusable
pub fn close(accounts: &[AccountInfo]) -> ProgramResult {
  Ok(())
}

// start a round of blackjack
pub fn start_round(accounts: &[AccountInfo]) -> ProgramResult {
  let account_info_iter = &mut accounts.iter();
  let funding_account = next_account_info(account_info_iter)?;

  Ok(())
}

// finish the round of blackjack
pub fn finish_round(accounts: &[AccountInfo]) -> ProgramResult {
  Ok(())
}

pub fn deal_card(accounts: &[AccountInfo], ) -> Option<Card> {
  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();
  if trigger_reshuffle() { // check if a reshuffle is required
    self.cards = shuffle_cards();
    self.status = Status::Ongoing;
  }

  return self.cards.pop(); // pop the last card off the deck
}

// payout winners from the liquidity pool according to the size of their bets
pub fn payout(__accounts: &[AccountInfo]) -> ProgramResult {
  Ok(())
}

// collect bets from losing players and deposit into the liquidity pool
pub fn collect(__accounts: &[AccountInfo]) -> ProgramResult {
  Ok(())
}

fn trigger_reshuffle(table: &Table) -> bool {
  let exceeded_threshold = table.deck.cards.len() <= (DECK_SIZE * TOTAL_DECKS) / 2;
  let round_ready = table.status == Status::Ready;
  return exceeded_threshold && round_ready;
}

/*
  shuffle_deck:
    1.) initialize the deck in order
    2.) shuffle the deck in place
*/
fn shuffle_cards() -> Vec<Card> {
  let mut cards: Vec<Card> = Vec::with_capacity(DECK_SIZE * TOTAL_DECKS);
  for _ in 0..TOTAL_DECKS {
    for &suit in &SUITS {
      for &rank in &RANKS { cards.push(Card{ suit, rank }); }
    }
  }

  let mut rand_num_gen = thread_rng();
  cards.shuffle(&mut rand_num_gen);
  return cards;
}

fn increment_round(table: &mut Table) {
  if table.status == Status::Completed { // if round has completed, update round
    table.status = Status::Ready;
    table.round += 1;
  }
}