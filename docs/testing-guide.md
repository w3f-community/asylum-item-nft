# Asylum node Testing Guide

You have 3 options to interact with Asylum on-chain:
1. PolkadotJS app.
2. [Asylum UI/Connection Library](https://gitlab.com/asylum-space/asylum-ui/-/tree/main/packages/connection-library) npm package.
3. [Asylum UI/Game Developers Console](https://gitlab.com/asylum-space/asylum-ui/-/tree/main/packages/game-developers-console) React Web App.

The Testing Guide describes the option 1.
To start working with the Testing guide, please install Asulum node and use PolkadotJS app for interaction with the node: [Click here for details](https://gitlab.com/asylum-space/asylum-item-nft/-/blob/main/README.md).

### Tags

To create new tags, you need to upload tag's metadata to IPFS and call `create_interpretation_tag`. In the example we're creating couple tags `default-view`,`jpeg`:


1. Upload to IPFS `default-view` tag metadata and get its CID:

```json
{
  "id": "default-view",
  "description": "The default visualization for the item. MUST be present in all NFTs.",
  "metadataExtensions": {}
}
```

2. Create `default-view` tag:

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

4. Create `jpeg` tag:

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
  "externalUri": "someUri",
  "mediaUri": "someUri"
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
               "id": "old-sword-default-view-jpg",
               "src": "{INTERPRETATION_SOURCE_CID}",
               "metadata": "{INTERPRETATION_METADATA_CID}",
            },
         },
      ],
   }
```

### Items

After having a template, owner can mint items from it. Call `mint_item_from_template` with the following arguments:

```json
{
    "owner": "{OWNER_ACCOUNT_ID}",
    "template-id": 0,
    "metadata": "{METADATA_CID}"
}
```

### Game

To create game, we need to call extrinsic `create_game` with arguments:

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

We suppose that our game supports the "Old sword" template. Call `add_template_support` to save this association onchain.

### Update template

Ooops, src for our interpretation is unavailable now - resource is lost. We need to change it. To do this, we will submit a template change proposal. Call `submit_template_change_proposal`:

```json
{
    "author": "someone-who-found-the-issue-with-interpretation-accoundId",
    "template-id": 0,
    "change-set": {
        Modify {
            [
               "interpretation": {
               "id": "m16-pumpkin-jpg",
               "src": "{INTERPRETATION_SOURCE_CID}",
               "metadata": "{METADATA_CID}",
            }
            ]
        }
    }
}
```

Let's assume DAO accepted that proposal, the template's owner can call `update_template`, and the issue with interpretation will be fixed.

### Accept item update

Owners of the items which were minted from the corrupted template can update items according to the last template state. To do this call `accept_item_update`. If the owner doesn't do this, then all updates will be stored in a pending state.
