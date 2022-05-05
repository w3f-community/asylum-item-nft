# Items metadata standard

All NFT items in Asylum ecosystem extends `pallet_uniques` and `RMRK` standards and inherits tranferability and extensibility. But we want also to bring gaming semantics and interoperability to items, that's why we introduced such abstractions as Template and Interpretation.

## Asylum entities flow diagram

![](/docs/img/asylum-entities-flow-diagram.png)

* **Template:** The extension of the classic NFT Collection. The `Template` has a set of supported `Interpretations`, and all items minted from this `Template` support these `Interpretations` as well.
* **Interpretation:** The description of the media resource, which is used to interpret the `Template` in different contexts. To describe such context, `Interpretation` must be associated with the unique set of `Tags`. This set of `Tags` defines the format of `Interpretation`'s metadata.
* **Tag:** The `Tag` is used to give an `Interpretation` a special semantic, allowing `Game Client` to query specific `Interpretation` according to the context of usage. `Tag` can describe a list of fields, which forms `Interpretaion`'s metadata.
* **Item:** The NFT minted from a particular `Template`. `Item` has the same `Interpretation` list, specified by `Template` at the time of its minting, but can differ in the future with upgrading the `Template`. The owner of `Item` might reject upgrading this `Item` according to the latest updates of `Template`.

## Template metadata standard

Template metadata currently contains only the `description` field and has the following structure:
```json
{
  "description": "string",
}
```
You can find an example of configuring the metadata in the [Asylum Node testing guide](https://gitlab.com/asylum-space/asylum-item-nft/-/blob/main/docs/testing-guide.md) in the Template section.

Template metadata does not have any additional fields as all the info is stored in the Interpretations and its metadata.

## Tag metadata standard

Every Interpretation in the Asylum ecosystem is associated with a specific set of Tags. **Consider Tags as a verbal description of Interpretation**. Tags can carry interpretation metadata fields with predefined or configurable values, or even without any metadata, bringing specific semantics to the Interpretation (like the `pixeled` tag). 

Besides the common Tag fields like `id` and `description` that are used to provide the understanding of the Tag semantics, Tag metadata also has a complex field - `metadataExtensions`.

**`metadataExtensions` is the object that describes the way how the concrete Tag affects the metadata of Interpretation.**

For example `png` tag can bring the required `format` field to the metadata with the default value `png`.

The whole structure of tag metadata:
```json
{
  "id": "string",
  "description": "string",
  "metadataExtensions": {
      "fileds": [
        {
          "name": "string",
          "type": "string",
          "default": "string",
          "description": "string"
        }
      ]
  }
}
```
You can find an example of configuring the metadata in the [Asylum Node testing guide](https://gitlab.com/asylum-space/asylum-item-nft/-/blob/main/docs/testing-guide.md) in the Tags section.

## Interpretation metadata standard

Interpretation metadata contains only the `description` field but can be extended by Tags (see examples in the next section).

The structure of interpretation metadata:
```json
{
  "description": "string",
  "tag-metadata-field-name": "tag-metadata-field-name"
}
```

## Examples of `metadataExtensions` usage

Let's consider a few examples of `metadataExtensions` usage.

1. Fist one is the `pixeled` tag, which is used to define pictures in pixeled style and can bring fields like `pixel-size` or `smoothed`.

Metadata of the `pixeled` tag:
```json
{
  "id": "pixeled",
  "description": "in pixeled style",
  "metadataExtensions": {
      "fileds": [
        {
          "name": "pixel-size",
          "type": "number",
          "default": "",
          "description": "Size of pixeles"
        },
        {
          "name": "smoothed",
          "type": "boolean",
          "default": false,
          "description": "Is image smoothed"
        }
      ]
  }
}
```

Metadata of interpretations with `pixeled` tag:
```json
{
  "description": "interpretation description",
  "pixel-size": 128,
  "smoothed": false,
}
```

2. Another example is the `animation-sprite-atlas` tag, which is used to define an atlas for 2d animation and will bring required fields like `tile-size-h`, `tile-size-w`, `tiles-count`, and `padding`.

Metadata of the `animation-sprite-atlas` tag:
```json
{
  "id": "animation-sprite-atlas",
  "description": "sprite atlas used for the 2d animation",
  "metadataExtensions": {
      "fileds": [
        {
          "name": "tile-size-h",
          "type": "number",
          "default": "",
          "description": "Height of every tile in pixels"
        },
        {
          "name": "tile-size-w",
          "type": "number",
          "default": "",
          "description": "Width of every tile in pixels"
        },,
        {
          "name": "padding",
          "type": "number",
          "default": "",
          "description": "Padding between tiles in pixels"
        }
        {
          "name": "tiles-count",
          "type": "number",
          "default": "",
          "description": "Count of all tiles in the atlas"
        }
      ]
  }
}
```

Metadata of interpretations with `animation-sprite-atlas` tag:
```json
{
  "description": "interpretation description",
  "tile-size-h": 64,
  "tile-size-w": 64,
  "tiles-count": 21,
  "padding": 2,
}
```
