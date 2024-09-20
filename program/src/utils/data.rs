use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{account_info::AccountInfo, program_error::ProgramError};


// deserialize_data: generic deserialization helper using borsh deserialization
pub fn deserialize_data<T>(account: &AccountInfo) -> Result<T, ProgramError>
where 
  T: BorshDeserialize
{
  let data_array = &account.data.borrow();
  let data = T::try_from_slice(&data_array)?;
  Ok(data)
}

// serialize_data: generic serialization helper using borsh serialization
pub fn serialize_data<T>(account: &AccountInfo, data: &T) -> Result<(), ProgramError>
where 
  T: BorshSerialize
{
  let mut data_array = &mut account.data.borrow_mut()[..];
  data.serialize(&mut data_array)?;
  Ok(())
}