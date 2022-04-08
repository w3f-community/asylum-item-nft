# Asylum Core Module

A simple, secure module for dealing with asylum templates and items

## Overview

The Asylum module provides functionality for templates and items management, including:

* Interpretation type creation
* Template creation
* Template destroying
* Template update
* Item issuance
* Item transfer
* Item burning
* Item update
* Compatability with pallet-uniques and RMRK

### Terminology

* **Interpretation:** The abstraction of how we can interpret the item in different contexts, such as pixel art or anime style.
* **Interpretation type:** The common interpretation context for several interpretations includes 2D, 3D, sound, etc.
* **Template:** The extension of the classic NFT's Collection. The template has a set of supported interpretations, and all items minted from this template should support these interpretations too.
* **item:** The extension of the classic NFT. Item has a set of supported interpretations.
* **Interpretation type creation:** The creation of a new interpretation type.
* **Template creation:** The creation of a new template.
* **Template destruction:** The destruction of a template.
* **Template update:** The action of updating template's supported interpretations: add/modify/remove types/interpretations
* **Item issuance:** Creating a new item from the template.
* **Item transfer:** The action of transferring an item from one account to another.
* **Item burning:** The destruction of an item.
* **Item update:** The action of updating the item's supported interpretations to the last version of the item's template. Only item's owner can update item.
* **Compatability with pallet-uniques and RMRK:** The interpretations are RMRK resources. Asylum NFTs can be used in pallet-uniques and RMRK contexts but with cut functionality.

### Goals

The Asylum core pallet is designed to make the following possible:

* Allow accounts to create, destroy and update templates (collections of items).
* Allow the account to mint, burn, and update items within a template.
* Allow submitting template change proposals.
* Move items between accounts.

## Interface

### Interpretation dispatchables
* `create_interpretation_type`: Create new interpretation type.

### Template dispatchables
* `create_template`: Create new template.
* `destroy_template`: Destroy template
* `update_template`: Update template according to proposal

### Item dispatchables
* `mint_item_from_template`: Mint new item from the template, i.e., mint item with the same set of supported interpretations as the template has.
* `transfer_item`: Move an item from the sender account to another.
* `burn_item`: Destroy an item.
* `update_item`: Update item according to the newest version of the template.

### DAO dispatchables
* `submit_template_change_proposal`: Submit proposal with template changes.

## Related Modules

* [`System`](https://docs.rs/frame-system/latest/frame_system/)
* [`Support`](https://docs.rs/frame-support/latest/frame_support/)
* [`Uniques`](https://docs.rs/pallet-assets/latest/pallet_uniques/)
* [`RMRK`](https://rmrk-team.github.io/rmrk-substrate/#/pallets/rmrk-core)

License: Apache-2.0
