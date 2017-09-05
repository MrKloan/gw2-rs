# gw2-rs
> Rust wrapper for the Guild Wars 2 REST and MumbleLink API.

```toml
[dependencies]
gw2 = { git = "https://github.com/mrkloan/gw2-rs" }
```

## REST API

The `api` module can be devided in 3 parts: `core`, `v1` and `v2`.

### Core

The core API module will give you access to all the utilities used to interact with the Guild Wars 2 API. You can import
it by using `gw2::api::prelude::*`.

Use the `Builder` structure to create an instance of `Client`, the default `Requester` implementor. Moreover, you can
use the `Requester` trait to create your own implementation of the API client.

```rust
use gw2::api::prelude::*;

// Each structure returned by a `Requester.request()` call must derive `Deserialize`.
#[derive(Deserialize)]
pub struct Build {
    pub id: i32
}

#[test]
fn test() {
    // Create a `Client` instance for the API v2.
    let client = Builder::new(Version::V2).into();
    
    // Make a call to any `/v2` endpoint (eg. /v2/build) and try to deserialize the result using the provided structure.
    let build = client.request::<Build>("build");
    
    assert!(build.is_ok());
    assert!(build.unwrap().id > 0);
}
```

### API v1
> The API v1 implementation is still a **WORK IN PROGRESS**

The `v1` module implements the specifications of the [Guild Wars 2 API v1](https://wiki.guildwars2.com/wiki/API:1).

All the required models have been rewritten as Rust structures, and the `/v1` endpoints can be used just as the REST API
would be (eg. `api.build()` is equivalent to `/v1/build.json`).

```rust
use gw2::api::v1::prelude::*;

#[test]
fn test() {
    let api = Api::new(Api::builder().into());
    let build = api.build();
    
    assert!(build.is_ok());
    assert!(*build.unwrap() > 0);
}
```

### API v2
> The API v2 implementation is still a **WORK IN PROGRESS**

The `v2` module implements the specifications of the [Guild Wars 2 API v2](https://wiki.guildwars2.com/wiki/API:2).

All the required models have been rewritten as Rust structures, and the `/v2` endpoints can be used just as the REST API
would be (eg. `api.quaggan("box")` is equivalent to `/v2/quaggans/box`).

```rust
use gw2::api::v2::prelude::*;

#[test]
fn test() {
    let api = Api::default();
    let quaggan = api.quaggan("box");
    
    assert!(quaggan.is_ok());
    assert_eq!(quaggan.unwrap().id, "box");
}
```

## MumbleLink API
> The MumbleLink API implementation can currently only be used on Windows

> A small shift in the LinkedMem struct's bytes seems to be present at the moment... Investigation in progress.

```rust
use gw2::link::GW2;

#[test]
fn test() {
    let mut gw2 = GW2::new().expect("Unable to link to Guild Wars 2");
    
    loop {
        if let Some(link) = gw2.tick() {
            // Do something with the current `LinkedMem` instance
        }
    }
}
```