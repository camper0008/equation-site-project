# esdoc

**E**quation **S**ite **Doc**ument

Simple markup syntax,

## Example

```ts
import { EsParser, EsDocument } from "esdoc";

const esdocText = `

title""My article""

Sentence one is
consistent and on
the same line.

Sentence two is consistent and on the same line.

math""x = \frac{-b \pm \sqrt{b^2 - 4c}}{2}""

The equation is turned into SVG and text be Katex.

image  ""https://images.unsplash.com/photo-1529778873920-4da4926a72c2""

Space between command and first double-double quote is ignores.

Everything between the double-double quotes are passed as-is to the directive function.

code""
const CHARS = 'plmonkij9buvhyc2gtxfr5zde3sw1aqZX4CASDQ0WEVBN7FGHRTYM6JKLU8IOP';
const randchar = (chars: string = CHARS): string => 
    chars.charAt(Math.floor(Math.random() * chars.length));
const randstr = (length: number, chars: string = CHARS): string => 
    length > 0 ? randchar(chars) + randstr(length - 1, chars) : '';
""
`;

const doc = new EsParser(esdocText).parse();

const html = doc.toHtml();

const md = doc.toMd();

const portableJson = doc.toJson();

const doc2 = EsDocument.from(portableJson);
```

## Syntax

### Plaintext

```
<...text>
``` 

```
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.

Duis aute irure dolor in reprehenderit in voluptate 
velit 
esse 
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
```

### Title

```
title""<text>""
```

```
title""Lorem ipsum dolor sit amet.""
````

### Math

```
math""<latex>""
```

```
math""x = \frac{a - 2}{b}""
```

This uses LaTeX for equations.

### Image

```
image""<src>""
```

```
image""https://www.advantagepetcare.com.au/sites/g/files/adhwdz311/files/styles/paragraph_image/public/2020-07/istock-539027929_unrestricted_1110x800.jpg?itok=jiLQgT-c""
```

#### Future

```
image""<src>;<alt>""
```

### Code

```
code""<code>""
```

```
code""
#include <stdio.h>

int main(int argc, char** argv)
{
    printf("Esdoc sucks!\n");
}
""
```

#### Future

```
code""<lang>;<code>""
```


[List of langauges](https://highlightjs.org/static/demo/)

```
code""c++;
#include <iosteam>

int main(String args[])
{
    std::cout << "Esdoc sucks!\n";
}
""
```

## Installation

### Math on Web

If using math in the browser, be sure to add the [Katex](https://katex.org/) font:

```html
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.15.3/dist/katex.css" integrity="sha384-A3N+UgNMKg9+LRsC2HUE0ECxFY7qhztVFORxHQZEPm7lnog2poqmm7CQ91wSEnBc" crossorigin="anonymous"><link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.15.3/dist/katex.css" integrity="sha384-A3N+UgNMKg9+LRsC2HUE0ECxFY7qhztVFORxHQZEPm7lnog2poqmm7CQ91wSEnBc" crossorigin="anonymous">
```
