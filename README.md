# HDK Unit Testing
This crate provides a library of [mocked hdk functions](src/mock_hdk.rs) to help with unit testing your zome functions.

## usage
In your tests, instantiate a mutable `MockHdkT` object and pass a mutable reference of this into the mocked hdk functions.
```rust
let mut mock_hdk = MockHdkT::new();
let mock_hdk_ref = &mut mock_hdk;
```
All mocked functions take `mock_hdk_ref` as an input as well as the expected input and output of the called hdk function.
```rust
use hdk_unit_testing::mock_hdk::*;

mock_create(
    mock_hdk_ref,
    expected_input,
    expected_output,
);

```

See an example usage in [Acorn](https://github.com/h-be/acorn/blob/c877964162346864bbe2995fec951d83182b518c/dna/tests/src/profiles/profile.rs#L17-L29).