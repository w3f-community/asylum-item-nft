# Asylum Game Distribution System Pallet

A simple, secure module for dealing with Asylum games and tickets

## Overview

The Asylum GDS module is based on `pallet_uniques`. This module provides functionality for games and tickets management, including:

* Game creation
* Game destroying
* Ticket issuance
* Ticket transfer
* Ticket burning

### Flow diagram

![](/docs/img/asylum-flow-diagram.png)

### Terminology

* **Game:** The `Game` consists of: 
  - description metadata,
  - runnable `Game Client`,
  - set of admins (or DAO) and owner who can modify the game,
  - _[planed]_ on-chain state and game back-end (probably TEE), which modifies the state.
* **Ticket:** The NFT, which is used as a pass to the `Game`.
* **Game Client:** The binary (e. g. WASM), which the Player uses to run and play the `Game`. Right now, it will be a link to the server where the game is spun up.


## Interface

### Game dispatchables
* `create_game`: Create a new game.
* `destroy_game`: Destroy a game.
* `freeze_game`: Prevent all tickets within a game from being transferred.
* `thaw_game`: Revert the effects of a previous `freeze_game`.
* `transfer_game_ownership`: Alter the owner of a game.
* `set_team`: Alter the permissioned accounts of a game.

### Ticket dispatchables
* `mint_ticket`: Mint a new ticket within an asset class.
* `transfer_ticket`: Transfer a ticket to a new owner.
* `burn_ticket`: Burn a ticket within a game.
* `freeze_ticket`: Prevent a ticket from being transferred.
* `thaw_ticket`: Revert the effects of a previous `freeze_ticket`.
* `approve_transfer`: Assign a delegator who can authorize a transfer.
* `cancel_approval`: Revert the effects of a previous `approve_transfer`.

### Metadata (permissioned) dispatchables
* `set_attribute`: Set a metadata attribute of a ticket or game.
* `clear_attribute`: Remove a metadata attribute of a ticket or game.
* `set_ticket_metadata`: Set general metadata of a tieckt.
* `clear_ticket_metadata`: Remove general metadata of a ticket.
* `set_game_metadata`: Set general metadata of a game.
* `clear_game_metadata`: Remove general metadata of agame.

## Related Modules

* [`System`](https://docs.rs/frame-system/latest/frame_system/)
* [`Support`](https://docs.rs/frame-support/latest/frame_support/)
* [`Uniques`](https://paritytech.github.io/substrate/master/pallet_uniques/index.html)

License: MIT
