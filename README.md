<div class="oranda-hide">

# toml-path 

</div>

jq for tomls (library and binary)


## Description
The `toml-path` crate provides both a binary and a library for querying toml files. If you are familiar with [jq](https://github.com/jqlang/jq) you already know how to use toml-path.

## Language
Overview of the language. See sections [binary](#Binary) and [library](#Library) for more specific usage.





## Binary

### Install
```bash
cargo install toml-path
```

### Examples
```console
$ toml-path '.package["name", "description"]' ./Cargo.toml
["toml-path", "jq for tomls (library and binary)"]

```

- Access an element by index in an array
```console
$ echo '[10, 20, 30, 40]' | toml-path '.[2]'
30

```

- Access a specific field in an object
```console
$ echo '{"name": "Alice", "age": 25}' | toml-path '.["age"]'
25

```

- Access elements in a nested array
```console
$ echo '[[1, 2], [3, 4], [5, 6]]' | toml-path '.[1][0]'
3

```

- Slice an array
```console
$ echo '[10, 20, 30, 40, 50]' | toml-path '.[1:4]'
[20, 30, 40]

```

- Access last element in an array
```console
$ echo '[1, 2, 3, 4, 5]' | toml-path '.[-1]'
5

```

- Index multiple elements from an array
```console
$ echo '[100, 200, 300, 400, 500]' | toml-path '.[0, 2, 4]'
100
300
500

```

- Retrieve a range of indices from an array
```console
$ echo '[10, 20, 30, 40, 50, 60, 70]' | toml-path '.[2:6]'
[30, 40, 50, 60]

```

- Access nested fields in objects
```console
$ echo '{"user": {"name": "Bob", "address": {"city": "New York", "zip": 10001}}}' | toml-path '.user.address.city'
"New York"

```

- Index objects in an array of objects
```console
$ echo '[{"id": 1, "value": "A"}, {"id": 2, "value": "B"}, {"id": 3, "value": "C"}]' | toml-path '.[1].value'
"B"

```

- Indexing with string keys in an array of objects
```console
$ echo '[{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]' | toml-path '.[0]["name"]'
"Alice"

```



## Library

### Install
```bash
cargo add toml-path
```

### Examples
```rust
use toml_path::get_from_file;

let file = "Cargo.toml";
let tomlpath = "package.name";
let name = get_from_file(file, tomlpath).unwrap();
assert_eq!("toml-path", name);
```


