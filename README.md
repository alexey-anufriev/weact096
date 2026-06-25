# WeAct 0.96 Inch Display Driver

Rust driver and test CLI for the WeAct 0.96-inch USB display.

The workspace has three parts:

- `crates/weact-display`: core display protocol, framebuffer, and driver.
- `crates/weact-display-serial`: serial-port transport for real hardware.
- `apps/weact-cli`: small CLI for testing the display.

## Build

```bash
cargo build
```

Run tests:

```bash
cargo test
```

## Find The Display

```bash
cargo run -p weact-cli -- find-port
```

This searches serial ports whose USB product name contains `Display FS 0.96 Inch`.

## Fill The Screen

With automatic port lookup:

```bash
cargo run -p weact-cli -- fill --color red
```

With an explicit port:

```bash
cargo run -p weact-cli -- fill --port /dev/ttyACM1 --color red
```

Optional flags:

```bash
--color red|green|blue|black|white
--orientation portrait|landscape|portrait-flipped|landscape-flipped
--brightness 0..100
```
