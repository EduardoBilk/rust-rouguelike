# A Robber
## a rouguelike console game writen in rust.

if you just want to see it working, you can donwload the binaries from the [releases page](https://github.com/EduardoBilk/rust-rouguelike/releases/tag/0.1.0)


or you can clone the repo and build it yourself. 
if so make sure to have rust proper installed in your machine.

### Controls

I intentionally didn't explain the controls in the game, beacause it makes part of the discovery.
But if you just want to play around, quick and easy, here it goes:

| Action        | key     |
| :-----------_ | :-----: | 
| walk          | arrows  |
| attack        | arrows  |
| grab itens (!)|    g    |
| char info     |    c    |
| inventory     |    i    |
| drop itens    |    d    |
| next level (<)|    <    |


### Rust instalation
if you are a command liner, please fell free to:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

if you prefer the old and gold standalone installers pick your best suite at the [official site](https://forge.rust-lang.org/infra/other-installation-methods.html#standalone)


### How to build the game 

make sure you have all dependencies installed
```
$ sudo apt-get install gcc g++ make libsdl2-dev
```

```
$ git clone https://github.com/EduardoBilk/rust-rouguelike.git a-robber
$ cd a-robber
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