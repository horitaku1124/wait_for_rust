# wait_for_rust

`wait_for_rust` is a CLI command waits for any non-synchronous event. This command aims to wait synchronously a command.

### build

```
cargo build --release
```

### run

```
# wait to create some file then exit
./wait_for_rust -f some.pid


# wait to launch web server then exit
./wait_for_rust -p localhost:80
```
