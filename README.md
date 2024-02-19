# Bro
Runner

## Build
```bash
cargo fmt
cargo build
```

## Run
```bash
./bro --help
```
or
```bash
cargo run -- --help
```

## demo help
```bash
Usage: bro [COMMAND]

Commands:
  config  Configure the sandbox, bro!.
  run     Bro, run this repo for me
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

[ilia@fedora bro-run]$ ./bro config --help
Configure the sandbox, bro!.

Usage: bro config [sandbox_name]

Arguments:
  [sandbox_name]  Name of the sandbox [possible values: docker, kvm, skipper, aws, k8s, container, ssh]

Options:
  -h, --help  Print help
[ilia@fedora bro-run]$ 
```
## More
later