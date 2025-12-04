# Test Document

This is a test document for barkdocs. It contains various markdown elements.

## Headings Test

### Level 3 Heading

#### Level 4 Heading

##### Level 5 Heading

###### Level 6 Heading

## Text Formatting

This is **bold text** and this is *italic text*. You can also have ***bold italic***.

Here's some `inline code` in a paragraph.

This text has ~~strikethrough~~ formatting.

## Links

Here's a [link to GitHub](https://github.com).

## Lists

### Unordered List

- First item
- Second item
- Third item
- Fourth item

### Ordered List

1. First step
2. Second step
3. Third step
4. Fourth step

## Code Blocks

Here's a Rust code block:

```rust
fn main() {
    println!("Hello, barkdocs!");

    let numbers = vec![1, 2, 3, 4, 5];
    for n in numbers {
        println!("{}", n);
    }
}
```

And a Python example:

```python
def greet(name):
    return f"Hello, {name}!"

if __name__ == "__main__":
    print(greet("World"))
```

Plain code block:

```
Just some plain text
in a code block
without syntax highlighting
```

## Blockquotes

> This is a blockquote.
> It can span multiple lines.
> And contain **formatted** text.

## Horizontal Rule

---

## Long Paragraph for Scrolling

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.

Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo.

## Search Test Section

This section contains the word "barkdocs" multiple times to test search highlighting.
You can search for "barkdocs" and see it highlighted.
The barkdocs application should highlight all matches.

## Final Section

This is the end of the test document. You should be able to:

1. Scroll through all content
2. Search for text with `/`
3. Navigate via outline panel
4. Toggle line numbers with `#`
5. Toggle line wrap with `w`
6. Split the view with `Ctrl+W,v`

Press `?` for help or `q` to quit.
