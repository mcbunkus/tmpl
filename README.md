# tmpl

`tmpl` is a command-line, template-based file generation tool using TOML files.
`tmpl` refers to these TOML files as "specs", and provides you the ability to
instruct `tmpl` what to generate and where.

I created this project because I didn't know of something quite like this. It's
admittedly very niche, and wouldn't be used much, but I wanted it for
bootstrapping projects just how I like them, and I wanted to brush up my Rust.

## Features

- Generate multiple files with template content from a single TOML file, using
  Jinja syntax.
- Generate files in arbitrarily nested directories.
- Manages spec file location, editing, copying, and creation.

## Installation

### From Source

`tmpl` can be installed by building from source. Pre-built binaries are not
provided because I'm probably the only person who will use this, and I don't
want to set up CI/CD on Codeberg. Installing is straightforward if `cargo` is
installed on your machine:

```bash
git clone https://codeberg.org/mcbunkus/tmpl.git
cd tmpl
cargo install .
```

This will install `tmpl` to `$HOME/.cargo/bin/`, which you probably already know
if you got this far.

## Quick Start

Let's run `--help` to make sure it's installed correctly:

```bash
$ tmpl --help
A command line, template-based file generation tool.

Usage: tmpl <COMMAND>

Commands:
  ls    List specs in the specs directory
  new   Create a new spec with some example content. The new spec will be opened in your $EDITOR, unless --no-edit is specified
  gen   Generate templates from a spec, with options if specified in your spec file
  edit  Open a spec in your editor of choice
  rm    Delete one or more specs
  cp    Copy a spec
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Of course, there aren't any specs yet. Let's make a new one:

```bash
tmpl new my.new.spec
```

This will create `my.new.spec` in the spec directory (`$HOME/.local/share/tmpl/`). If path to a text editor is defined in your `$EDITOR` environment variable, `tmpl` will open your new spec in your `$EDITOR`. It will look like this:

```toml
[variables]
project = "project-name"
user = "alebeau"  # this will generate with your username

[[templates]]
path = "README.md"
body = """

# {{ project }}

Created by {{ user }}.
"""
```

Save it and go to an empty directory. Let's try to generate a new spec:

```bash
tmpl gen my.new.spec
```

And voila! A new `README.md` should be generated in your current working
directory:

```markdown
# project-name

Created by alebeau.
```

`tmpl` can do a lot more than that though. Here's a boilerplate spec for a C++17
project:

```toml
[variables]
user = "alebeau"
cmake_version = "3.20"
project_name = "example-project"
cpp_version = 17

[[templates]]
path = "CMakeLists.txt"
body = """
cmake_minimum_required(VERSION {{ cmake_version }})
project({{ project_name }} VERSION 1.0.0 LANGUAGES CXX)

# Set C++ standard
set(CMAKE_CXX_STANDARD {{ cpp_version }})
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Add executable
add_executable(${PROJECT_NAME}
    src/main.cpp
)

# Include directories
target_include_directories(${PROJECT_NAME} PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)

# Compiler warnings
if(MSVC)
    target_compile_options(${PROJECT_NAME} PRIVATE /W4)
else()
    target_compile_options(${PROJECT_NAME} PRIVATE -Wall -Wextra -Wpedantic)
endif()

# Installation rules
install(TARGETS ${PROJECT_NAME} DESTINATION bin)
"""

[[templates]]
path = "README.md"
body = """
# {{ project_name }}

- CMake Version {{ cmake_version }}
- c++{{ cpp_version }}

Created by {{ user }}
"""

[[templates]]
path = "src/main.cpp"
body = """
#include <iostream>

int main()
{
    std::cout << "Hello, {{ user }}! From {{ project_name }}" << std::endl;
}
"""
```

Paths can be nested as deeply as you want. See [Spec File Format](#spec-file-format)
below for a more detailed description of how spec files work.

## Usage

### Commands

#### `ls` - For listing the specs in your spec directory

```
List specs in the specs directory

Usage: tmpl ls [OPTIONS]

Options:
  -l, --list-vars  List the default arguments for each spec
  -h, --help       Print help
```

#### `new` - For creating new specs

```
Create a new spec with some example content. The new spec will be opened in your $EDITOR, unless --no-edit is specified

Usage: tmpl new [OPTIONS] <NAME>

Arguments:
  <NAME>  The name of your new spec

Options:
      --no-edit  Don't open it in your $EDITOR after creation
  -h, --help     Print help
```

#### `gen` - For generating a spec

```
Generate templates from a spec, with options if specified in your spec file

Usage: tmpl gen [OPTIONS] <NAME>

Arguments:
  <NAME>  The spec's name

Options:
  -o <KEY> <VALUE>  Options as key-value pairs (can be specified multiple times)
  -c <WORKDIR>      The directory to generate the spec in
  -h, --help        Print help
```

#### `edit` - For editing a spec in your `$EDITOR`

```
Open a spec in your editor of choice

Usage: tmpl edit <NAME>

Arguments:
  <NAME>  The spec's name

Options:
  -h, --help  Print help
```

#### `rm`

For removing one or more specs from your spec directory. It will prompt you
before deleting each spec, to make sure you really wanted to delete it. `-y` to
skip prompts.

```
Delete one or more specs

Usage: tmpl rm [OPTIONS] [TO_DELETE]…

Arguments:
  [TO_DELETE]…

Options:
  -y, --yes   Confirm yes for all given specs
  -h, --help  Print help
```

#### `cp` - Copy a spec to another new name

```
Copy a spec

Usage: tmpl cp [OPTIONS] <SOURCE> <DEST>

Arguments:
  <SOURCE>  The spec you want to copy
  <DEST>    The name of the new spec

Options:
  -y, --yes   Skip are you sure prompt
  -h, --help  Print help
```

## Spec File Format

A spec file is a TOML file with two sections: `variables` and `templates`.

### Variables

`variables` are the default template variables that will be expanded in
templates, and is a table of key value pairs. All TOML types are accepted. The
keys will your variables in the templates you define below it. An example
`variables` section:

```toml
[variables]
authors = ["John Doe", "Jane Smith"]
rust_edition = "2024"
project_name = "example-project"
copyright = 2025
# ...
```

### Templates

`templates` is an array of tables with 2 fields each: `path` and `body`. `path`
is the full path to the resulting file, and `body` is its contents. Here's an
example:

```toml
# year and name are keys in your [variables] table.

[[templates]]
path = "LICENSE"
body = """
MIT License

Copyright (c) [{{ year }}] [{{ name }}]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"""
```

Currently, it isn't possible to use template variables in the `path` field.
Solely because I didn't think about it at first, but it will be implemented.
However, the `body` field is a multi-line string that can use template
variables, using Jinja syntax. See
[minijinja's docs](https://docs.rs/minijinja/latest/minijinja/) for a more
detailed explanation on how templates works. This project is just a wrapper
around [minijinja](https://github.com/mitsuhiko/minijinja), big thanks to its
developers.

## Configuration

Adding user configuration is planned in the future, to do things like making the
spec directory configurable, or specifying special instructions for your editor.
