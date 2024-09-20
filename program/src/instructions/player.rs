use solana_program::{
  account_info::AccountInfo,
  entrypoint::ProgramResult,
  pubkey::Pubkey
};


/*
  register_on_table:
    create a pda for the player that is linked to a selected table.
    once registered, the player join and leave different rounds as long as the table is not retired.
    
    player pda <- table pubkey
*/
pub fn register_on_table(program_id: &Pubkey, accounts: &[AccountInfo], __data: &[u8]) -> ProgramResult {
  Ok(())
}

// deposit_funds: player deposits funds into the pda to be used to wager bets
pub fn deposit_funds(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
  Ok(())
}

// withdraw_funds: player withdraws funds from pda
pub fn withdraw_funds(accounts: &[AccountInfo]) -> ProgramResult {
  Ok(())
}

// join_round: if there no ongoing round, a player can register for an upcoming round
pub fn join_round(accounts: &[AccountInfo]) -> ProgramResult {
  Ok(())
}

// increase_bet: initialize bet for round, or increase further for next round
pub fn increase_bet(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
  Ok(())
}

// decrease_bet: lower bet size once a round has completed (if funds were not collected for loss)
pub fn decrease_bet(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
  Ok(())
}

// hit: request a new card on turn
pub fn hit(accounts: &[AccountInfo]) -> ProgramResult {
  Ok(())
}

// stand: refuse a new card on turn
pub fn stand(accounts: &[AccountInfo]) -> ProgramResult { // we want both contributor to be removed and owner to be able to remove contributor
  Ok(())
}

// double: 2x the current bet on player hand during round on turn
pub fn double(accounts: &[AccountInfo]) -> ProgramResult { // we want both contributor to be removed and owner to be able to remove contributor
  Ok(())
}