# Examples of using serde to parse config files in Rust

This uses [Serde](https://serde.rs) and builds up examples of parsing a JSON config file.

Each step is fully written in the `examples` directory.
The code that starts in `src/main.rs` is the same as `examples/step1.rs`.

In each step, either make the changes as described below, or copy the file, e.g.:

```shell
cp examples/step2.rs src/main.rs
```

and then run with

```shell
cargo run
```

## Step 1 - Start with config1.json file  (but rename to main.rs)

Run it and see that it requires a config file to be passed:

```shell
cargo run
```

Let's make sure to pass `config1.json`:

```shell
cargo run config1.json
```

it should just print out the base JSON.

## Step 2 - Add config deserialisation

We want to read the JSON text into a struct, so let's define one.
We must add the `Debug` derive trait so that we can print it out, and the `Deserialize` trait so that it can automatically convert from text:

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    provider: String,
    service: String,
    users: Vec<String>,
}
```
and change the `println` line to:

```rust
let config: Config = serde_json::from_str(&config_string)?;
println!({config:?});
```

## Step 3 - Add printing the details

Now we have our data in as a real Rust struct, we can pull out the values and print them,
replacing the `println` above:

```rust
    let Config {
        provider,
        service,
        users,
    } = config;
    println!("In {provider}, create {service} for:");
    for u in users {
        println!("- {u}");
    }
```

## Step 4 - Read nested structures

We now move to `config2.json`.
The users entry should be more than just a string, so we make a new struct to hold the possible entries.

```rust
#[derive(Debug, Deserialize)]
struct Config {
    provider: String,
    service: String,
    users: Vec<User>,
}

#[derive(Debug, Deserialize)]
struct User {
    username: String,
    role: String,
}
```

## Step 5 - Make a `Role` `enum`

Here, we want to be able to specify the exact possible values for this entry so make an `enum` to hold i:.

```rust
#[derive(Debug, Deserialize)]
struct User {
    username: String,
    role: Role,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Role {
    Admin,
    User,
}
```

## Step 6 - Add optional setting

Moving on to `config3.json`, there's a tricky value that may or may not be present.
Allow this `LegacyType` to be optionally there, and if it is, print it.

```rust
#[serde(rename = "LegacyType")]
legacy_type: Option<String>,
```

```rust
if let Some(legacy) = legacy_type {
    println!("Also, legacy thing: {legacy}");
}
```

This code will now work with both `config2.json` and `config3.json`.

## Step 7 - Serialize to YAML

show that we can serialize back out by adding the `Serialize` derive to all structs and enums.
We can then print the YAML version of the config (add this straight after `serde_json::from_str`:

```rust
println!("{}", serde_yaml::to_string(&config)?);
```

And to make sure that the legacy value does not get written out, set:

```rust
skip_serializing_if = "Option::is_none"
```

in its `serde` setting.
