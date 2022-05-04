# Asylum node Testing Guide

You have 3 options to interact with Asylum on-chain:
1. PolkadotJS app.
2. [Asylum UI/Connection Library](https://gitlab.com/asylum-space/asylum-ui/-/tree/main/packages/connection-library) npm package.
3. [Asylum UI/Game Developers Console](https://gitlab.com/asylum-space/asylum-ui/-/tree/main/packages/game-developers-console) React Web App.

The Testing Guide describes option 1.
To start working with the Testing guide, please install the Asulum node and use the PolkadotJS app for interaction with the node: [Click here for details](https://gitlab.com/asylum-space/asylum-item-nft/-/blob/main/README.md).

### Tags

To create new tags, you need to upload tag's metadata to IPFS and call `create_interpretation_tag`. In the example, we're creating a couple tags `default-view`,`jpeg`:


1. Upload to IPFS `default-view` tag metadata and get its CID:

```json
{
  "id": "default-view",
  "description": "The default visualization for the item. MUST be present in all NFTs.",
  "metadataExtensions": {}
}
```

2. Create a `default-view` tag:

```json
{
  "tag": "default-view",
  "metadata": "{METADATA_CID}"
}
```

3. Upload to IPFS `jpeg` tag metadata and get its CID:

```json
{
  "id": "jpeg",
  "description": "in .jpeg format",
  "metadataExtensions": {
      "fileds": [
        {
          "name": "format",
          "type": "string",
          "default": ".jpeg",
          "description": "The format of source is JPEG"
        }
      ]
  }
}
```

4. Create a `jpeg` tag:

```json
{
  "tag": "jpeg",
  "metadata": "{METADATA_CID}"
}
```

### Template

Now we can create a template with interpretations that support tags created in the previous step. To do this, we need to call `create_template`.

1. Upload template metadata to IPFS and get its CID:

```json
{
  "description": "The best weapon for the Helloween party 2022",
}
```

2. Upload interpretation metadata to IPFS and get its CID:

```json
{
  "description": "Default view interpretation in JPG format",
  "format": ".jpg"
}
```

3. Upload interpretation source to IPFS and get its CID.

4. Call `create_template` extrinsic with the following arguments:

```json
 {
      "template-name": "Old sword",
      "metadata": "{TEMPLATE_METADATA_CID}",
      "max": 100,
      "interpretations": [
         {
            "tags": ["default-view", "jpeg"],
            "interpretation": {
               "id": "default-view-jpg",
               "src": "{INTERPRETATION_SOURCE_CID}",
               "metadata": "{INTERPRETATION_METADATA_CID}",
            },
         },
      ],
   }
```

### Items

After having a template, the owner can mint items from it. Call `mint_item_from_template` with the following arguments:

```json
{
    "owner": "{OWNER_ACCOUNT_ID}",
    "template-id": 0,
    "metadata": "{METADATA_CID}"
}
```

### Game

To create a game, we need to call extrinsic `create_game` with arguments:

```json
{
    "game": 0,
    "admins": {
        "{ADMIN_1_ACCOUNT_ID}",
        "{ADMIN_1_ACCOUNT_ID}"
    },
    "price": 10000
}
```

To allow ticket unpriviledged mint call `set_allow_unpriviledged_mint`:

```json
{
    "game": 0,
    "allow": true
}
```

We suppose that our game supports the "Old sword" template. Call `add_template_support` to save this association on-chain.

### Update template

When the template already exists and items are minted we still have a possibility to edit it - extend with new interpretations or fix the old ones.

Let's assume that we want to add a 3d model representation for the "Old sword" template to make it supported in 3d games and also fix the link for 2d interpretation.

> Note: to continue the guide here you need to create all necessary tags for the 3d model (`3d-model`, `obj`) as described in the Tags section **before** moving forward.

1. **Submit proposal**

To do this, anybody could submit a template change proposal. Call `submit_template_change_proposal` with two changes -  `Add` and `Modify`:

```json
{
    "author": "{SOMEONE_WHO_FOUND_THE_ISSUE_WITH_INTERPRETATION_ACCOUNT_ID}",
    "template-id": 0,
    "change-set": {
        Add: {
          "interpretations": [
            {
               "tags": ["3d-model", "obj"],
               "interpretation": {
                  "id": "3d-model-obj",
                  "src": "{3D_INTERPRETATION_SOURCE_CID}",
                  "metadata": "{3D_INTERPRETATION_METADATA_CID}",
               },
            },
          ]
        },
        Modify: {
          "interpretations": [
              {
                "id": "default-view-jpg",
                "src": "{NEW_INTERPRETATION_SOURCE_CID}",
                "metadata": "{METADATA_CID}",
              }
          ]
        }
    }
}
```
- In `Modify` change we're describing the changes of source or metadata of already existing interpretation.
- With the `Add` change, we're adding a new interpretation to the template. The important thing here is to keep the new interpretation's tags set unique, as the set of tags is the identifier of the interpretation within the template.

There are also two options for change - `ModifyTags` and `RemoveInterpretation`, that can be used in a similar way.

2. **Wait for the proposal approved**

Let's assume DAO accepted that proposal (currently done automatically after submitting the proposal)

3. **Update template**

Now the template's owner can call `update_template` extrinsic with the id of template and proposal, and all proposed updates will be applied to the template.

### Accept item update

After the template was updated it will request all minted Items to apply this update.

Owners of the items can update their items according to the last template state. To do this owner must call `accept_item_update`. If the owner doesn't do this, then all updates will be stored in a pending state and the item will save its previous state.
