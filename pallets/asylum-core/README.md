# Asylum Core Pallet

A simple, secure module for dealing with Asylum `Templates` and `Items`.

## Overview

The Asylum module provides functionality for `Templates` and `Items` management, including:

* Interpretation `Tag` creation
* `Template` creation
* `Template` destroying
* `Template` update
* `Item` minting
* `Item` transfer
* `Item` burning
* `Item` update
* Compatability with [pallet_uniques](https://paritytech.github.io/substrate/master/pallet_uniques/index.html) and [RMRK pallets](https://rmrk-team.github.io/rmrk-substrate/#/pallets/rmrk-core)

### Flow diagram

![](/docs/img/asylum-flow-diagram.png)

### Terminology

Entities:
* **Template:** The extension of the classic NFT Collection. The `Teamlate` has a set of supported `Interpretations`, and all items minted from this `Template` supports these `Interpretations` as well.
* **Interpretation:** The description of the media resource, which is used to interpretate the `Template` in different contexts. To describe such context `Interpretation` must be associated with the unique set of `Tags`. This set of `Tags` defines the format of `Interpretation`'s metadata.
* **Tag:** The `Tag` is used to give an `Interpretation` a special semanthic alowing `Game Client` to query specific `Interpretation` according to the context of usage. `Tag` can describe a list of fields, which forms `Interpretaion`'s metadata.
* **Item:** The NFT minted from particular `Template`. `Item` has the same `Interpretation` list, specified by `Template` at the time of its minting, but can differ in future with upgrading the `Template`. The owner of `Item` might reject upgrading this `Item` according to latest updates of `Template`.

Actions:
* **Template update:** The action of updating `Interpretation` list of `Template`. The update divided in two steps: 
    1. Anyone creates proposal to update `Template` interpretations.
    2. DAO votes for proposal _(right now the step is skipped)_.
    3. `Template` owner applies proposal, after that `Template` will be updated.
* **Item update:** The action of updating the `Item`'s supported interpretations to the last version of the `Item`'s template. Triggered automatically after `Template` update, but the `Item`'s owner should accept all changes.
* **Compatability with pallet-uniques and RMRK:** Interpretations are RMRK resources. Asylum NFTs can be used in `pallet_uniques` and RMRK contexts but with cut functionality.

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
* [`Uniques`](https://paritytech.github.io/substrate/master/pallet_uniques/index.html)
* [`RMRK`](https://rmrk-team.github.io/rmrk-substrate/#/pallets/rmrk-core)

License: MIT
