# parse_segoe_mdl2

This little helper tool has a single purpose:

* find the unicode points and their descriptions
  for Microsoft® Segoe MDL2 icon fonts
* create a rust source file listing all glyphs as const

Because a parsable CSS description is missing, the information
is extracted from the Microsoft online documentation page (html source).

## Overview

The documentation is published via the the following URL.

[Segoe MDL2 icons](https://docs.microsoft.com/en-us/windows/uwp/design/style/segoe-ui-symbol-font#icon-list)

Save this page as `segoe_codepoints.html`

## Examples

The Servo project provides the basics.
The compunity worked to extracted the developed parser and selectors from
this project and created the standlone libary crate `kuchiki`.
Documentation is quite rare. Have a look at the source :)

I had problems to compile the example `find_matches`.
It seems quite important to use the correct OpenSSL library version.
If you encounter this problem, downgrade to an oldder Openssl libary (v1.0)

``` bash
OPENSSL_INCLUDE_DIR=/usr/include/openssl-1.0 OPENSSL_LIB_DIR=/usr/lib/openssl-1.0 cargo run --example find_matches
```

## Installation

### clone the repostiory

```sh
$ git clone https://github.com/rzerres/parse_segoe_mdl2.git
$ cd parse_segoe_mdl2
```
### build/run the binary

```sh
cd <project_root>
$ cargo run --release
```
Two examples are included to test `kuchiki` libary functionality we rely on.

* find_matches
* test_segoe_mdl2

### install the binary

To install the compiled binary you can use rust package manager `cargo` as well.

```sh
cd <project_root>
$ cargo install
```

---

[Logo-CC_BY]: https://i.creativecommons.org/l/by/4.0/88x31.png "Creative Common Logo"
[License-CC_BY]: https://creativecommons.org/licenses/by/4.0/legalcode "Creative Common License"
This work is licensed under a [Creative Common License 4.0][License-CC_BY]

![Creative Common Logo][Logo-CC_BY]

<A9> 2020 Ralf Zerres, Networkx GmbH
