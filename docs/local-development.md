# Local development

There're no limitations and you can run this application on your computer as any other application.
It just fails to read temperature sensors values and you'll see _N/A_ in the UI.

## Temperature simulation

If you'd like to see some temperature values on your computer, you have to
enable `simulate-temperature` feature.

```bash
cargo run --features simulate-temperature
```

Then you'll see some fake temperature values instead of the _N/A_.
