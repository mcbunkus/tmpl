# Requirements Document: tmpl

## Overview

`tmpl` is a convenient tool for generating files and projects from templates. It
exists to automate recurring project and file creation. It can be used for
generating notes, creating CMake files with boilerplate, license files, etc.

## Goals

- Primary goal: automate boilerplate project and/or file creation
- Secondary goal: dust off Go knowledge

## User Stories

- As a developer, I want to automate project creation
- As a developer, I want to generate and append changelogs
- As a user, I want to generate markdown notes with boilerplate

## Functional Requirements

### Core Features

1. **Template Generation**
   - Variable substitution with defaults specified in templates
   - Output to specified directory, defaults to current directory

2. **Template Management**
   - List available templates
   - Edit existing templates, if `$EDITOR` environment variable is set
   - Create new templates with blank scaffold in `$EDITOR`

## Non-Functional Requirements

### Usability

- Simple, intuitive command syntax
- Helpful error messages
- Built-in help/documentation

### Performance

- Fast template generation (aim for <1 second for all typical files in template combined)
- Minimal dependencies

### Compatibility

- Target platforms: Linux, macOS, Windows

## Technical Constraints

- Will only support TOML format for easy human editing
- Templates will be stored in the user's config directory in a directory called
  `tmpl/templates`, (e.g. `~/.config/tmpl/templates/*.toml` on Linux)

## Future Considerations

- None yet, starting with KISS mindset

## Out of Scope

- I honestly don't know yet, I'll come back here when something comes up

