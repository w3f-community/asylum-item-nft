# Asylum Core Pallet

A simple, secure module for dealing with Asylum `Templates` and `Items`.

## Overview

The Asylum module provides functionality for `Templates` and `Items` management, including:

* Interpretation `Tag` creation
* `Template` creation
* `Template` destroying
* `Template` update
* `Item` issuance
* `Item` transfer
* `Item` burning
* `Item` update
* Compatability with [pallet_uniques](https://docs.rs/pallet-assets/latest/pallet_uniques/) and [RMRK pallets](https://rmrk-team.github.io/rmrk-substrate/#/pallets/rmrk-core)

### Flow diagram

![](/docs/img/asylum-flow-diagram.png)

### Terminology

* **Template:** The extension of the classic NFT's Collection. The template has a set of supported `Interpretations`, and all items minted from this template should support these `Interpretations` too.
* **Interpretation:** The description of the media resource used to interpretate the `Template` in different contexts. To describe such context `Interpretation` must be associated with the unique set of `Tags`. This set of `Tags` defined the format of `Interpretation`'s metadata.
* **Tag:** The `Tag` is used to give an `Interpretation` a special semanthic alowing `Game Client` to query specific `Interpretation` according to the context of usage. `Tag` can describe a list of fields, which forms `Interpretaion` metadata.
* **Item:** The NFT minted from particular `Template`. `Item` has the same `Interpretation` list, specified by `Template` at the time of its minting, but can differ in future with upgrading the `Template`. The owner of `Item` might not want to upgrade this `Item` according to latest updates of `Template`.
* **Interpretation Tag creation:** The creation of a new interpretation tag.
* **Template creation:** The creation of a new `Template`.
* **Template destruction:** The destruction of a `Template`.
* **Template update:** The action of updating `Interpretations` supported the template: add/modify/remove
* **Item issuance:** Creating a new `Item` from the `Template`.
* **Item transfer:** The action of transferring an `Item` from one account to another.
* **Item burning:** The destruction of an `Item`.
* **Item update:** The action of updating the `Item`'s supported interpretations to the last version of the `Item`'s template. Triggered automatically after `Template` update, but the `Item`'s owner should accept all changes.
* **Compatability with pallet-uniques and RMRK:** The interpretations are RMRK resources. Asylum NFTs can be used in pallet-uniques and RMRK contexts but with cut functionality.

### Goals

The Asylum core pallet is designed to make the following possible:

* Allow accounts to create, destroy and update templates (collections of items).
* Allow the account to mint, burn, and update items within a template.
* Allow submitting template change proposals.
* Move items between accounts.

## Interface

### Interpretation dispatchables
* `create_interpretation_tag`: Create new interpretation tag.

### Template dispatchables
* `create_template`: Create new template.
* `destroy_template`: Destroy template.
* `update_template`: Update template according to proposal and request all items update after this.

### Item dispatchables
* `mint_item_from_template`: Mint new item from the template, i.e., mint item with the same set of supported interpretations by template.
* `transfer_item`: Move an item from the sender account to receiver.
* `burn_item`: Destroy an item.
* `accept_item_update`: Accept all template's updates till the newest version of the template.

### DAO dispatchables
* `submit_template_change_proposal`: Submit proposal with template changes.

## Related Modules

* [`System`](https://docs.rs/frame-system/latest/frame_system/)
* [`Support`](https://docs.rs/frame-support/latest/frame_support/)
* [`Uniques`](https://docs.rs/pallet-assets/latest/pallet_uniques/)
* [`RMRK`](https://rmrk-team.github.io/rmrk-substrate/#/pallets/rmrk-core)

License: MIT
