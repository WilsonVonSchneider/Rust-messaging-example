# Error handling

Errors are handled throughout the application inside this library.

The question mark (`?`) operator in Rust is used as an `error propagation alternative` to functions that return `Result` or `Option` types. The `?` operator is a shortcut as it reduces the amount of code needed to immediately return Err or None from the types `Result<T, Err>` or `Option` in a function.

By adding all of our `errors` in the `Error enum` inside of this library and properly implementing the traits, we can use the `?` to propagate errors all the way to the frontend.

[Back to master README](../../README.md#error-handling)

---

## How to add errors

In this example, lets say that you created a library `utilities` with some utility, helper functions to use in the app. Lets also assume that, for the sake of this example, the `utilities` library has an error defined inside `utilities::error::Error`.

Import the error inside of the library (`lib.rs` file)
```rust
use utilities::error::Error as UtilityError;
```

Add the `UtilityError` to the `Error` enum:
```rust
pub enum Error {
    ...
    Utility(UtilityError),
    ...
}
```

Add the `Utility` error to the `Display` trait implementation:
```rust
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ....
            Error::Utility(ref cause) => write!(f, "Utility Error: {}", cause),
            ...
        }
    }
}
```

Add the `Utility` error to the `StdError` implementation for the `Error`:
```rust
impl StdError for Error {
    fn cause(&self) -> Option<&dyn StdError> {
        match *self {
            ...
            Error::Utility(ref cause) => Some(cause),
            ...
        }
    }
}
```

Add the `From` trait implementation for the `Utility` error; `From Utility for Error`;
```rust
impl From<Utility> for Error {
    fn from(cause: Utility) -> Error {
        Error::Utility(cause)
    }
}
```

`IF` the `Utility` error can be mapped to an actix HttpResponse error, add it to the ResponseError implementation for the `Error` enum:
```rust
impl ResponseError for Error {
    ...
    let mut response = match self {
        ...
        Error::Utility(_) => {
            HttpResponse::BadRequest()
        }
        ...
    }
    ...
}
```

Add the error into the `Error implementation`, inside `into_error_body` function. This function is used to transform the error into a uniform `ErrorBody`.
```rust
impl Error {
    ...
    pub fn into_error_body(&self) -> ErrorBody {
        match *self {
            ...
            Error::Utility(ref cause) => ErrorBody {
                message: Some("Utility Error".to_stirng()),
                code: "utility".to_string(),
                cause: Some(cause.to_string()),
                payload: None
            }
            ...
        }
    }
    ...
}
```

[Back to TOP](#error-handling)

[Back to master README](../../README.md#error-handling)

---

## Error Index

| Error  | Source  | Usage  |
|---|---|---|
| `Error example` | example | example / example |

[Back to TOP](#error-handling)

[Back to master README](../../README.md#error-handling)

---