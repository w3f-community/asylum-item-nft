# Asylum lifecycle example

### Tags

To create new tags, we need to call `create_interpretation_tag` several times - call it with the next set of arguments:

default-view tag:

```json
{
  "tag": "default-view",
  "metadata": "ipfs://ipfs/default-view-metadata-hash"
}
```

```json
{
  "description": "The default visualization for the item. MUST be present in all NFTs.",
  "metadataExtension": {}
}
```

2d-sprite tag:

```json
{
  "tag": "2d-sprite",
  "metadata": "ipfs://ipfs/2d-sprite-metadata-hash"
}
```

```json
{
  "description": "2d picture representation",
  "metadataExtension": {}
}
```

jpeg tag:

```json
{
  "tag": "jpeg",
  "metadata": "ipfs://ipfs/jpeg-metadata-hash"
}
```

```json
{
  "description": "Anything in .jpeg format",
  "metadataExtension": {
      "fileFormat": {
        "type": "string",
        "default": ".jpeg",
        "description": "Used to help client parse content correctly"
    }
  }
}
```

### Template

Now we can create a template with interpretations that support tags created in the previous step. To do this, we need to call `create_template`. Pass the following arguments to the extrinsic:

```json
 {
      "template-name": "M16: Helloween edition",
      "metadata": "ipfs://ipfs/template-metadata-hash",
      "max": 666,
      "interpretations": [
         {
            "tags": ["default-view", "jpg"],
            "interpretation": {
               "id": "m16-pumpkin-jpg",
               "src": "https://zilliongamer.com/uploads/codm/skins/assault/m16/m16-pumpkin-repeater-cod-mobile.jpg",
               "metadata": "ipfs://ipfs/interpretation-metadata-hash",
            },
         },
      ],
   }
```

template's metadata:

```json
{
  "description": "The best weapon for the Helloween party 2022",
  "externalUri": "someUri",
  "mediaUri": "someUri"
}
```

interpretation's metadata:

```json
{
  "description": "You may use this interpretation in the inventory context of pixel 2D game",
  "name": "m16-pumpkin-repeater-cod-mobile"
}
```
### Items

We have the template so that the template issuer can mint items from this template. Call `mint_item_from_template`:

```json
{
    "owner": "owner-accoundId",
    "template-id": 0,
    "metadata": "ipfs://ipfs/item-metadata-hash"
}
```

### Game

To create game, we need to call extrinsic `create_game` with arguments:

```json
{
    "game": 0,
    "admins": {
        "admin0-accoundId",
        "admin1-accoundId"
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

We suppose that our game supports the "M16: Helloween edition" template. Call `add_template_support` to save this association onchain.

### Update template

Ooops, src for our interpretation is unavailable now - resource is lost. We need to change it. To do this, we will submit a template change proposal. Call `submit_template_change_proposal":

```json
{
    "author": "someone-who-found-the-issue-with-interpretation-accoundId",
    "template-id": 0,
    "change-set": {
        Modify {
            [
               "interpretation": {
               "id": "m16-pumpkin-jpg",
               "src": "ipfs://ipfs/interpretation-src-hash",
               "metadata": "ipfs://ipfs/interpretation-metadata-hash",
            }
            ]
        }
    }
}
```

Let's assume DAO accepted that proposal, the template's owner can call `update_template`, and the issue with interpretation will be fixed.

### Accept item update

Owners of the items which were minted from the corrupted template can update items according to the last template state. To do this call `accept_item_update`. If the owner doesn't do this, then all updates will be stored in a pending state.