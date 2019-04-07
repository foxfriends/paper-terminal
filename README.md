[syncat]: https://github.com/oinkiguana/syncat
[oinkiguana/paper]: https://github.com/oinkiguana/paper
[ttscoff/mdless]: https://github.com/ttscoff/mdless
[lunaryorn/mdcat]: https://github.com/lunaryorn/mdcat

# Paper terminal

See [paper.png](./paper.png) to see what this looks like!

Writes a file to a paper in your terminal. *Especially* if that file is Markdown! Features supported
include:

1.  The usual text, and paragraphs with automatic line-wrapping. You can manually wrap with
    hard breaks as expected.

    Otherwise, paragraphs will be nicely spaced.
2.  Headings
3.  __Bold__ / *Italic* / *__Bold and Italic__* / ~~Strikethrough~~
4.  Lists
    *   Ordered
    *   Unordered
        *   Nested
5.  Rules
6.  `Inline code`
7.  Code blocks, with [syncat][] integration for syntax highlighting. Note that you must install
    syncat and make the syncat executable available on your path for this to work.
    ```rust
    fn main() {
        println!("Hello world");
    }
    ```
8.  Blockquotes

    >   Blockquotes
    >   >  And even nested block quotes

9.  And even images! Here's a photo of my cat

    ![My cat. His name is Cato](./cato.png)

10. Task lists:
    - [x] Easy
    - [ ] Hard
11. Footnotes[^ft]

    [^ft]: This is the footnote!

12. Tables

## Comparison with other command line Markdown renderers

Not a very good comparison... this is more of an example of a table!

| Tool                 | CommonMark | Paper | Paging | Wrapping | Syntax     | Images    | Tables | Looks good\* |
| :------------------- | :--------- | :---- | :----- | :------- | :--------- | :-------- | :----- | :----------- |
| [oinkiguana/paper][] | Yes        | Yes   | No     | Yes      | syncat     | Pixelated | Yes    | Yes          |
| [ttscoff/mdless][]   | Yes        | No    | Yes    | No       | pygmentize | Sometimes | Yes    | No           |
| [lunaryorn/mdcat][]  | Yes        | No    | No     | No       | syntect    | Sometimes | No     | No           |

\* subjective

## Future features

In future, I hope to leverage syncat stylesheets to allow customization of all the parts, but for now,
the style is fixed.

## Installation

This is not yet published to crates.io, so you will have to install from source:

```bash
git clone https://github.com/oinkiguana/paper-terminal
cd paper-terminal
cargo install --path .
```

## Usage

```bash
# Print the help
paper --help

# Render README.md
paper README.md

# Render README.md, with syntax highlighting
paper README.md -s
```

```
paper 0.1.0
Cameron Eldridge <cameldridge+git@gmail.com>
Prints papers in your terminal

USAGE:
    paper [FLAGS] [OPTIONS] [file]...

FLAGS:
        --dev         Print in debug mode
        --help        Prints help information
    -n, --no-paper    Don't bother with the whole paper part, just print the markdown nicely
    -s, --syncat      Use syncat to highlight code blocks. Requires you have syncat installed.
    -V, --version     Prints version information

OPTIONS:
    -h, --h-margin <h-margin>    Horizontal margin
    -m, --margin <margin>        Margin (shortcut for horizontal and vertical margin the same) [default: 6]
    -v, --v-margin <v-margin>    Vertical margin
    -w, --width <width>          The width of the paper (text and margin) [default: 92]

ARGS:
    <file>...    Files to print
```
