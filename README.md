# Taha Lang

Welcome to Taha Lang, a lightweight programming language designed for fun and experimentation! Whether you're a seasoned developer or just starting out, Taha Lang is the perfect way to explore coding concepts and unleash your creativity.

## Installation

Requires the rust version 1.81 to be installed on your system.

```bash
git clone --depth 1 https://github.com/tahadostifam/Taha-Lang.git
cd Taha-Lang
cargo build --release
cp ./target/release/taha /usr/bin/taha

taha version
```

## Example Code

```
fn fibonacci(n) {
    if (n <= 0) {
        ret 0;
    } else if (n == 1) {
        ret 1;
    } else {
        ret fibonacci(n - 1) + fibonacci(n - 2);
    }
}

println("Welcome to Taha Lang!");

for #i = 0; i < 100000; i++ {
    println(fibonacci(i));
}
```

## How to rust? (Dev Mode)
```bash
cargo run -- ./examples/example.code
```

## Documentation

- [Wiki](https://github.com/tahadostifam/taha-lang/wiki)

## Community

**Geek Engineers** community shaped the Taha Lang and it welcomes people all around the world with passion and kindness.

Telegram: [@geek_engineers](https://t.me/geek_engineers)

## Open to Contribution

Taha Lang is an open-source project, and we value your contributions! If you're interested in helping to shape the future of Taha Lang, feel free to fork the repository, propose improvements, or report any issues you encounter.
