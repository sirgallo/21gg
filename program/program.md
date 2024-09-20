# blkjck

## program

### accounts

There are three basic accounts that manage state within `blkjck`, which include:

  - deck
  - player
  - table

#### table

The table account is a program derived account which contains the following struct signature:

```rs
#[derive(BorshSerialize, BorshDeserialize, BorshSchema, Clone, Debug)]
pub struct Chair {
	pub balance: u32,
	pub hand: Vec<Card>
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Table {
	pub table_creator: Pubkey, // pub key associated with the wallet/user that owns the repository
	pub table_name: String,
	pub table_deck: Pubkey, // the deck also contains the current round and the status of the round
	pub table_dealer: Vec<Card>,
	pub table_players: DataMap<Pubkey, Chair>
}
```

The public key of pda for the repository account is implicitly linked to the account and does not need to be explicitly defined in the table account object.

available instructions are as follows:

```rs
pub fn open(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult
pub fn close(accounts: &[AccountInfo]) -> ProgramResult
pub fn start_round(accounts: &[AccountInfo]) -> ProgramResult
pub fn finish_round(accounts: &[AccountInfo]) -> ProgramResult
pub fn payout(accounts: &[AccountInfo]) -> ProgramResult
pub fn collect(accounts: &[AccountInfo]) -> ProgramResult
```

#### player

The user account contains the following struct signature:

```rs
#[derive(BorshSerialize, BorshDeserialize)]
pub struct User {
  pub display_name: String,
  pub repositories: DataMap<String, Pubkey>
}

pub fn try_from_slice(mut data: &[u8]) -> std::io::Result<User> {
  User::deserialize(&mut data)
}
```

The public key of the owner of the user account is implicitly linked to the account and does not need to be explicitly defined in the user account object.

available instructions are as follows:

```rs
pub fn create_user_account(_program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult
pub fn add_repository(accounts: &[AccountInfo<'_>]) -> ProgramResult
pub fn remove_repository(accounts: &[AccountInfo<'_>]) -> ProgramResult
```