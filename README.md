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
ilia@Ilia-uh:~/work/bro-run$ ./bro help
Usage: bro [COMMAND]

Commands:
  config  Configure the sandbox, bro!.
  run     Bro, run this repo for me
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
ilia@Ilia-uh:~/work/bro-run$ ./bro config --help
Configure the sandbox, bro!.

Usage: bro config [sandbox_name]

Arguments:
  [sandbox_name]  Name of the sandbox [possible values: docker, libvirt]

Options:
  -h, --help  Print help
ilia@Ilia-uh:~/work/bro-run$ 
```


# demo run
After configuring sandbox
```
ilia@Ilia-uh:~/work/bro-run$ ./bro run https://github.com/IliaFeldgun/bro-run
Getting git repo https://github.com/IliaFeldgun/bro-run, bro!
git clone-ing, bro!
Cloning into 'bro-run'...
remote: Enumerating objects: 88, done.
remote: Counting objects: 100% (88/88), done.
remote: Compressing objects: 100% (50/50), done.
remote: Total 88 (delta 14), reused 81 (delta 8), pack-reused 0
Receiving objects: 100% (88/88), 17.46 KiB | 17.46 MiB/s, done.
Resolving deltas: 100% (14/14), done.
Running git repo scm_dir/bro-run, bro!
Bro, I'm loading detectors from: make.yaml
Bro, I'm loading detectors from: rustlang.yaml
Bro, I'm loading detectors from: python.yaml
Bro, I can make this, detected rustlang!
cargo build-ing, bro!
   Compiling version_check v0.9.4
   ...
   ..
   .
   ..
   ...
   Compiling bro-run v0.1.0 (/home/ilia/work/bro-run/scm_dir/bro-run)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.68s
Looks good, bro!
```



## More
later