A simple error construct with a better `Display` implementation that prints nested errors via the `source` method.

```rust
fn main() -> Result<(), fullerror::FullError> {} //all you need
```
