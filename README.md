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
```
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


# demo run
After configuring sandbox
```bash
[ilia@fedora] ./bro run https://github.com/IliaFeldgun/bro-run
```
or
```
[ilia@fedora bro-run]$ cargo build && ./bro run .
   Compiling bro-run v0.1.0 (/home/ilia/work/bro-run)
    Finished dev [unoptimized + debuginfo] target(s) in 0.54s
Running git repo /home/ilia/work/bro-run, bro!
Bro, I'm loading detectors from: rustlang.yaml
Bro, I'm loading detectors from: make.yaml
Bro, I'm loading detectors from: python.yaml
Bro, I can make this, detected rustlang!
cargo build-ing, bro!
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s

[ilia@fedora bro-run]$ 
```



## More
later