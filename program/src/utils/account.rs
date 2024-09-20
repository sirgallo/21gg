use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  program_error:: ProgramError,
  pubkey::Pubkey,
  program::invoke,
  system_instruction::create_account
};


// get_account_data_as_buffer gets the account metadata as well as the account info as a byte slice
pub fn get_account_data_as_vec<'a>(account: &'a AccountInfo) -> Result<Vec<u8>, ProgramError> {
  match account.data_is_empty() {
    true => return Err(ProgramError::AccountDataTooSmall),
    false => {
      let account_ref = account.data.borrow();
      Ok(account_ref[..].to_vec())
    }
  }
}

// create_pda_account: generic program derived account generation function
pub fn create_pda_account(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
  let account_info_iter = &mut accounts.iter();
  let funding_account = next_account_info(account_info_iter)?;
  let pda_account = next_account_info(account_info_iter)?;

  // parse seed, space, and lamports from data
  let seed = &data[1..9]; // adjust based on your seed length
  let space = u64::from_le_bytes(data[9..17].try_into().unwrap()) as usize;
  let lamports = u64::from_le_bytes(data[17..25].try_into().unwrap());

  // derive the pda
  let (pda, bump_seed) = Pubkey::find_program_address(&[seed], program_id);
  if pda != *pda_account.key { return Err(ProgramError::InvalidSeeds); }

  // create the account instruction
  let create_account_instruction = create_account(
    funding_account.key,
    pda_account.key,
    lamports,
    space as u64,
    program_id,
  );

  // invoke the instruction to create the pda
  invoke(&create_account_instruction, &[funding_account.clone(), pda_account.clone()])?;
  Ok(())
}