# 21gg

### just for fun


## welcome

`21gg` is meant to be a fast paced, simplified blackjack game with mechanics similar to popular `crash` games. Each game is a round, where a round has a window of time to enter a bet before closing and distributing cards. The table has an automatically sorted deck of cards (using three 52 card decks) that is then dealt to each player, where each player gets 2 cards and the table gets 2 cards. If a player receives 2 cards where the sum of both is higher than the hand of the dealer, they will double their initial bet. Otherwise, the wager is deposited to the table, which transfers the loss to a global payout pool.

The global payout pool is the core of the platform. Application users have the option to deposit money as collateral into the payout pool. Losses incurred by players of `21gg` will be payed back to the pool, while wins will be paid from the pool. The payout pool will also have a max payout of 5% at any given time, with 5% always held as native Solana. The other 95% of the pool will be a "basket" of `liquid staked tokens (LSTs)`, rotating through the tokens with the highest validator emissions (~17% at the time of writing). Liquidity providers will also get kickback from the interest earned through the LSTs. The goal of the above is incentivize participation on the backend by building a dynamic and transparent liquidity pool for a truly decentralized gambling experience.

Each transaction will also have a small platform fee, of `0.5%`, which will be paid directly into a developer multisig wallet.