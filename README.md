# mkdirt

A tiny joke, for when you misstype `mkdir`. It will still create the directory·ies, alongside with a pretty procedurally-generated dirt hill.

## Installation

You'll need to have `rust` and its friends installed, if you don't, check out [The Book](https://doc.rust-lang.org/book/second-edition/ch01-01-installation.html).

```sh
git clone https://github.com/adri326/mkdirt.git
cd mkdirt
cargo install --path .
```

`Cargo` may also prompt you to add the `~/.cargo/bin` path to your `$PATH`. If this hasn't yet been done, you can do it now by running:

```sh
echo "export PATH=\"$PATH:~/.cargo/bin/\";" >> ~/.bashrc
```

and restart bash or run `export PATH="$PATH:~/.cargo/bin/"`.

## Usage

Intentionally or unintentionally misspell `mkdir` in the `mkdirt` form.
