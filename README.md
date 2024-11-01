### Example

```rust
use garde::Validate;

use rusqlite_derive::{FromSql, ToSql};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Validate, FromSql, ToSql)]
#[garde(transparent)]
#[serde(transparent)]
pub struct Username(
    #[garde(length(min = 3, max = 24), alphanumeric)]
    //
    String,
);
```
