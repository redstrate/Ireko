# Ireko

Library for reading save files from [Dolls Nest](https://store.steampowered.com/app/1839430/Dolls_Nest/). Only the demo
is supported because that's what is available at the time of writing.

**The parser is not yet complete, and can only dump parts of the structured data. The code is also generally terrible because I'm still exploring the format.**

## Usage

Pass the `.sav` file you want to read as an argument. Not all `.sav` files are known to work, such as `Level.sav` and `Player.sav`.

```shell
$ ireko DollsNestDemo/Saved/SaveGames/AR0XJGFWA6HNIQ1AAUJ9UR828/Persistent.sav
Ok(
    TaggedSerialization {
        size_in_bytes: 135908,
        objs: [
            TaggedObject {
                size_in_bytes: 135904,
                entries: [
                    Entry {
                        name: "SavedDataVersion",
                        type: Some(
                            Int(
                                IntProperty {
                                    integer_size: 4,
                                    value: 0,
                                },
                            ),
                        ),
                    },
...
```

## Technical Details

Dolls Nest uses [the EasyMultiSave plugin](https://www.fab.com/listings/49f745a1-cbdd-4b18-8278-22ae1075d91d), which
saves using the `.sav` file extension. The save data is zlib-compressed, and decompressing them reveals a very similar
tagged structure to GVAS. See more
information [in this blog post](https://redstrate.com/blog/2025/02/reverse-engineering-dolls-nest-demo-saves/).

## Credits & Thank You

* [uesave-rs](https://github.com/trumank/uesave-rs) for figuring out the basics of the EasyMultiSave format.

## License

![GPLv3](https://www.gnu.org/graphics/gplv3-127x51.png)

This project is licensed under the GNU General Public License 3.
