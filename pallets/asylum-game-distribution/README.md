# Asylum Game Distribution System Module

A simple, secure module for dealing with asylum games and tickets

## Overview

The Asylum GDS module is based on pallet-uniques. This module provides functionality for games and tickets management, including:

* Game creation
* Game destroying
* Ticket issuance
* Ticket transfer
* Ticket burning

### Terminology

* **Game:** The collection of tickets.
* **Ticket:** The NFT, which is used as proof of ownership of the game.

### Goals

The Asylum GDS pallet is designed to make the following possible:

* Allow accounts to create and destroy games (collections of tickets).
* Allow the account to mint, burn, and transfer tickets.
* Move tickets between accounts.
* Allow an account to freeze and unfreeze tickets within a
  game or the entire game.
* Allow the owner of a ticket instance to delegate the ability to transfer the ticket to some
  named third party.

## Interface

### Game dispatchables
* `create_game`: Create a new game.
* `destroy_game`: Destroy a game.
* `freeze_game`: Prevent all tickets within a game from being transferred.
* `thaw_game`: Revert the effects of a previous `freeze_game`.
* `transfer_game_ownership`: Alter the owner of a game.
* `set_team`: Alter the permissioned accounts of a game.

### Ticket dispatchables
* `mint_ticket`: Mint a new ticket within an asset class. Any account can mint a ticket if it can transfer the game `price` to the game owner account.
* `transfer_ticket`: Transfer a ticket to a new owner.
* `burn_ticket`: Burn a ticket within a game.
* `freeze_ticket`: Prevent a ticket from being transferred.
* `thaw_ticket`: Revert the effects of a previous `freeze_ticket`.
* `approve_transfer`: Name a delegate who may authorize a transfer.
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
* [`Uniques`](https://docs.rs/pallet-assets/latest/pallet_uniques/)

License: Apache-2.0
