# Local development

There're no limitations and you can run this application on your computer as any other application.

```bash
cargo run
```

It just fails to read temperature sensors values and you'll see _N/A_ in the UI.

## Temperature simulation

Enable `simulate-temperature` feature if you'd like to see some temperature values
on your computer.

```bash
cargo run --features simulate-temperature
```

_N/A_ will disappear and some fake values will be displayed.
