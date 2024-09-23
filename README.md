# Brainrusty

Brainrusty is a lightweight [brainfuck](https://en.wikipedia.org/wiki/Brainfuck) interpreter.

## Compiling

```bash
cargo build --release
```

## Testing
```bash
cargo test
```

## Running
```bash
cargo run --release -- --help
```

There are some brainfuck samples in `./samples/` directory.

```bash
cargo run --release -- ./samples/<program>
```

Certain brainfuck programs require EOF-terminated input to do computing internally. In Unix-like systems, pressing `Ctrl+D` normally sends `EOF`.

Those in `./samples/` that require `EOF`:
1. `./samples/qsort.b`
2. `./samples/bsort.b`
3. `./samples/isort.b`
4. `./samples/xmastree.b`

All programs in `./samples/` are working properly:

- [x] [hello-world.bf](./samples/hello-world.bf)
- [x] [xmastree.b](./samples/xmastree.b)
- [x] [dbf2c.b](./samples/dbf2c.b)
- [x] [fib.bf](./samples/fib.bf)
- [x] [squares.bf](./samples/squares.bf)
- [x] [squares2.bf](./samples/squares2.bf)
- [x] [sierpinski.bf](./samples/sierpinski.bf)
- [x] [thuemorse.b](./samples/thuemorse.b)
- [x] [factorial2.b](./samples/factorial2.b)
- [x] [impeccable.b](./samples/impeccable.b)
- [x] [bsort.b](./samples/bsort.b)
- [x] [isort.b](./samples/isort.b)
- [x] [qsort.b](./samples/qsort.b)

> https://www.brainfuck.org/

### Disassembly
```bash
cargo run --release -- ./samples/<program> -d
```

> [!IMPORTANT]
> It's essential to remember: this implementation is not fully-featured, but key-features are implemented. I do not certify that all brainfuck programs in the world will run perfectly here. Certain programs require specific features in interpreter.
