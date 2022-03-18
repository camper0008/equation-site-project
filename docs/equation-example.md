# equation example

Example post + syntax for how an equation post is written.

## syntax

Posts are described much like a raw string literal from rust or f-strings from python, though using two double quotes (`""`) for delimiting characters, as that is what we thought would be a good balance between being ergonomic, (such as not using too esoteric delimiting characters, such as `¤`, and having them be easily reachable, so not something like e.g. `^^`, which would get annoying quickly), and avoiding having to escape everything, (e.g. if we used parenthesis, you'd have to escape them frequently while using math or text, which would get annoying fast).

Currently, the syntax specification includes `image`, `math`, `text`.

The math object uses [LaTex](https://en.wikipedia.org/wiki/LaTeX) for equations.

## example post

```
text""Hello, this is an example post!""

text""Here's an image:""

image""https://image-host.com""

text""And here's an equation:""

math""a^2 + b^2 = c^2""

text""Here's another equation:""

math""\frac{a}{b} = \sqrt{c + d}""
```
