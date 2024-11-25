# Clinku

[*lipu Linku*](https://linku.la) is an online dictionary for the constructed language toki
pona. It is supported by the public dataset *sona Linku* and the annual surveys *wile Linku*. It contains information about the definition,
historical data, usage, creation, writing, pronunciation, etymology and
etc. of each word in the language.

Clinku (CLI + Linku) is an unofficial frontend of sona Linku for the
command line interface.

## Install

Latest stable release

```sh
cargo install clinku
```

Latest git commit

```sh
cargo install --git https://github.com/ivaaane/clinku
```

## Usage

Syntax

```
clinku [OPTIONS] <WORD>
```

Print help

```sh
clinku
clinku -h
clinku --help
```

View information of a word

```sh
clinku toki
```

Only output a specific field

```sh
clinku pona --usage
clinku pona --definition
clinku pona --etymology
clinku pona --data
clinku pona --also
clinku pona --commentary
clinku pona --creator
```

Search in the sandbox

```sh
clinku Pingo --sandbox
clinku Pingo --definition --sandbox
```

## Contribute

All contributions are welcome!! You probably know better than me how
to do Rust, so come here and do your part. If you don't, leave an
issue if anything unwanted happens.

o pona!
