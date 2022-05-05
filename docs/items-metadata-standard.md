TODO:
- add Flow diagram right part
- add descriptions
- add info about template and item update

# Items metadata standard

Every Item in the Asylum ecosystem can have custom metadata, but will also inherit metadata from its Template (including Interpretations).

# Template metadata standard

Template metadata currently contains only the `description` field and has the following structure:
```json
{
  "description": "string",
}
```
Example of this metadata can be found in the [Asylum Node testing guide](https://gitlab.com/asylum-space/asylum-item-nft/-/blob/main/docs/testing-guide.md) in the Template section.

Template metadata does not have any additional fields as all other data is stored in the Interpretations and its metadata.

# Tag metadata standard

Every Tag stored in the Asylum ecosystem will have attached metadata.

Besides the common fields like `id` and `description` that are used to provide the understanding of the Tag's semantics, Tag metadata also has a complex field - `metadataExtensions`.

**`metadataExtensions` is the object, that describes the way how the concrete Tag will affect the metadata of Interpretation.** That means that every tag applied to the interpretations can bring additional metadata to these interpretations.

For example `png` tag can bring the required `fileFormat` field to the metadata with the default value `png`.

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
Example of this metadata can be found in the [Asylum Node testing guide](https://gitlab.com/asylum-space/asylum-item-nft/-/blob/main/docs/testing-guide.md) in the Tags section.

# Interpretation metadata standard

Interpretation metadata contains only the `description` field, but can be extended but using extensions from the tags (see next section).

The structure of interpretation metadata:
```json
{
  "description": "string",
  "added-field": "some-value"
}
```

# Examples of metadata extensions usage

Let's consider a few examples of `metadataExtensions` usage.

1. Fist one - is the `pixel-styled` tag, that used to define pictures in pixeled style and can bring fields like `pixel-size` or `smoothed`.

Metadata of the `pixel-styled` tag:
```json
{
  "id": "pixel-styled",
  "description": "picture in pixeled style",
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

Metadata of interpretations with `pixel-styled` tag:
```json
{
  "description": "....",
  "pixel-size": "128",
  "smoothed": false,
}
```

2. Another example - is the `animation-sprite-atlas` tag, which is used to define an atlas for 2d animation and will bring required fields like `tile-size-h`, `tile-size-w`, `tiles-count`, and `padding`.

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
  "description": "....",
  "tile-size-h": "64",
  "tile-size-w": "64",
  "tiles-count": "21",
  "padding": "2",
}
```
