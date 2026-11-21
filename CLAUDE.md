# Claude Code Instructions

This document contains instructions for Claude Code to use when performing tasks
or generating source code.

## About Naked Time

The name of the product is Naked Time. Naked Time is a time management product
that helps users to plan their day and track time spent working on important
activities. Naked Time is based around the
[Pomodoro Technique](https://en.wikipedia.org/wiki/Pomodoro_Technique).

## Source code file headers

All source files (e.g. program files that can have comments) should include the
standard comment in the file header:

```plain
Copyright 2026 Michael F. Collins, III
Licensed under the Naked Time license. See LICENSE.md for details.
```

This comment block should be at the top of source files. Source files such as
shell scripts that have shebang headers can place the shebang header above the
standard file header comment block.

## .gitattributes

Any files with new extensions should be added to the `.gitattributes` file to
indicate whether the file is text or binary, and if text, what line endings
should be used when the file is committed to the Git repository. Singular files
such as dotfiles (e.g. `.gitignore` or `Dockerfile`) should have their filename
included in the `.gitattributes` file.

## Architectural decision records (ADRs)

Any significant architectural decision records should be captured as an
Architectural Decision Record using the `software-architecture:adr` skill. ADRs
should be located in the `docs/adrs` subdirectory.
