Welcome to the future of documentation—1998.

`mdbook-chm` drags your sleek, modern Rust docs kicking and screaming into the warm, musty arms of Microsoft Compiled HTML Help. Because nothing says “cutting-edge” like a format that died with Internet Explorer 6 and still thinks `<font>` tags are the height of style.

You didn’t ask for this. Nobody asked for this. Microsoft *really* didn’t ask for this—they’ve buried the CHM compiler so deep you'd think it held classified documents. Finding it was an adventure in digital archaeology.

But against all odds, it works. And now your docs can ship in glorious `.chm` format, just like Grandma used to make.

> **“I don’t know where this file came from. I didn’t install anything. Why is this on the shared drive? What is Rust?”**  
> — *Gary, IT Manager, 2003, Satisfied Customer*

Run `cargo install --path .`, and add `[output.chm]` to your `book.toml`. By default, find it in book/chm/ after running `mdbook build`.