# A Robber
## a rouguelike console game writen in rust.

to see it working you can donwload the binaries from the release


or you can clone the repo and build it yourself. if so make sure to have rust proper installed in your machine.

if you are a command liner, please fell free to:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

if you prefer the old and gold standalone installers pick your best suite at the [official site](https://forge.rust-lang.org/infra/other-installation-methods.html#standalone)



```
$ mkdir a-robber
$ cd a-robber
$ git clone https://github.com/EduardoBilk/rust-rouguelike.git .
```
Then you can run it with:

```
$ cargo run --release
```

or build your binary with:

```
$ cargo build --release
```
powered by libtcod. thanks to [Tomas Sedovic](https://github.com/tomassedovic) for the bindings.