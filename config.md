Todo_r Config Documentation
======

Note that while Todo_r is pre-1.0, these settings are subject to change.

The default configuration that `todor` loads can be found in `src/default_config.json`.

### Tags
```json
"tags": [
  "todo",
  "fix",
  "fixme"
]
```

This config lists the keywords that are tracked by `todor`. The list items are case-insensitive.

### Styles
```json
"styles": {
  "filepath": "u_",
  "tag": "green",
  "tags": {},
  "content": "cyan",
  "line_number": 8,
  "user": 8
}
```

ANSI printing styles for raw output of `todor`. Each item except `"tags"` takes either a string that is an ANSI color or a number 0-255 corresponding to the desired ANSI color. ANSI modifiers like bold, italic, and underline can also be added by prepending `b_`, `i_`, or `u_`.

`"tags"` lets you define specific ANSI styles on a tag by tag basis. So if you want `FIXME` comments to be red and `MAYB` comments to be magenta, you can set the config to
```json
"styles": {
  "tags": {
      "fixme": "red",
      "mayb": "magenta"
  }
}
```

### Default Extension
```json
"default_ext": "sh"
```

The default extension fallback that `todor` uses if the file extension is not supported. This extension has to be defined by `"comments"` below or by `"default_comments"`.

### Comment Types
```json
"comments": [
    {
      "exts": [
        "c",
        "h",
        "cpp",
        "rust",
      ],
      "types": [
        {
          "single": "//"
        },
        {
          "prefix": "/*",
          "suffix": "*/"
        }
      ]
  },
  {
    "ext": "py",
    "types": [
      {
        "single": "#"
      },
      {
        "prefix": "\"\"\"",
        "suffix": "\"\"\""
      }
    ]
  }
]
```
Each item in the list `"comments"` has two parts:
1. `"ext"` or `"exts"` defines the extensions to which to apply the defined comment type
2. `"types"` is a list of the types of comments that occur in these extensions

Comments of two types are supported: single-line and block.

#### Single-Line Comments
Single-line comments have only a prefix, so
```json
{
  "single": "//"
}
```
would match comments like
```rust
// TODO: item
```

#### Block Comments
Block comments have both a prefix and a suffix, so
```json
{
  "prefix": "/*",
  "suffix": "*/"
}
```
would match items like
```rust
/* TODO: item */
```

---
Note that `src/default_config.json` uses the config `"default_comments"` so that adding new comment types only overrides the comment types you want to override.
