# Rust Monkey

[![License](https://img.shields.io/github/license/FollowTheProcess/rust-monkey)](https://github.com/FollowTheProcess/rust-monkey)
[![GitHub](https://img.shields.io/github/v/release/FollowTheProcess/rust-monkey?logo=github&sort=semver)](https://github.com/FollowTheProcess/rust-monkey)
[![CI](https://github.com/FollowTheProcess/rust-monkey/workflows/CI/badge.svg)](https://github.com/FollowTheProcess/rust-monkey/actions?query=workflow%3ACI)
[![Unsafe Forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance)

An implementation of the monkey programming language from [Writing an Interpreter in Go] in Rust 🦀

## Project Description

[monkey] is a dynamically typed, interpreted programming language by [Thorsten Ball] from the [Writing an Interpreter in Go] and [Writing a Compiler in Go] books.

I've done both in Go in the past and I want to try doing it in Rust now

## Installation

You probably don't want to, but if you must:

```bash
cargo install https://github.com/FollowTheProcess/rust-monkey
```

## Quickstart

```bash
monkey --version
```

```bash
monkey run script.monkey

# Or launch a repl
monkey
```

### Credits

This package was created with [copier] and the [FollowTheProcess/rust-template] project template.

[copier]: https://copier.readthedocs.io/en/stable/
[FollowTheProcess/rust-template]: https://github.com/FollowTheProcess/rust-template
[Writing an Interpreter in Go]: https://interpreterbook.com
[Writing a Compiler in Go]: https://compilerbook.com
[monkey]: https://monkeylang.org
[Thorsten Ball]: https://thorstenball.com
