# boardbuilder

This is a tool I created for making bingo boards for my Old School RuneScape clan.

The code is in a **very** rough state and I'm aware it's not very good.
This was built in a couple of days in between, and sometimes _during_ gaming sessions.
I plan to come back and atone for all of the sins I committed while writing this.

## Building

You need Rust. Get it at https://www.rust-lang.org/tools/install.

Clone the project and run `cargo build`.
This should generate a runnable executable for your OS under the `target/` directory.

## Usage

```
boardbuilder <input YAML> <output PNG>
```

Will take a board input (struct `BoardBuilder`) specified as YAML and output a PNG image.

Here's an example of what that YAML looks like:

```yaml
rows: 1
cols: 2
content_rect:
  x1: 20
  y1: 20
  x2: 1900
  y2: 1900
tile_size: 216
tile_render_options:
  padding: 6
  border_size: 4
  inset_size: 4
  text_size: 20
  locked_theme:
    border_color: "#2F2B22FF"
    inset_color: "#75634EFF"
    background_color: "#4A3E32FF"
    text_color: "#FF9000FF"
  unlocked_theme:
    border_color: "#2F2B22FF"
    inset_color: "#75634EFF"
    background_color: "#574C40FF"
    text_color: "#FF9000FF"
image: .cache/test_board.png
tiles:
  - number: 1
    name: Serpentine helm
    image: https://oldschool.runescape.wiki/images/thumb/Serpentine_helm_detail.png/425px-Serpentine_helm_detail.png
    unlocked: false
  - number: 2
    name: 1M Agility XP
    image: https://oldschool.runescape.wiki/images/thumb/Mark_of_grace_detail.png/487px-Mark_of_grace_detail.png
    unlocked: true
```

## Licensing and Legal Info

My code is MIT licensed. See the full license text in `LICENSE` at the root of this repository.

Third-party assets are licensed separately, each subdirectory of the `assets/` directory will have the license text included with the relevant asset.

This code provides the technical capabilities to load in images from _any_ origin.
The examples show this with the OldSchool RuneScape Wiki - these images are intellectual property of Jagex Ltd.
Anything created with this tool should be used in a way compliant with Jagex's fan content policy.

As such I'm including the disclaimer from the fan content policy as well as a link to it here:

> Created using intellectual property belonging to Jagex Limited under the terms of Jagex's Fan Content Policy. This content is not endorsed by or affiliated with Jagex.

https://www.jagex.com/en-GB/terms/fancontentpolicy

Finally, the terms "Old School RuneScape" and "RuneScape" are trademarks of Jagex Ltd.
