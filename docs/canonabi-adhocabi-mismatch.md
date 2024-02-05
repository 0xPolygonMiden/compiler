TL;DR: The compiler will recognize the functions with a mismatch between the canonical ABI and the tx kernel ad-hoc ABI and make the compiler generate an adapter function that will call the tx kernel function and convert function arguments and result. For most TX kernel functions, the adapter function can be generated automatically. See below for the functions that require manual adapter functions.

# Canonical ABI vs Miden (tx kernel) ABI mismatch and how to resolve it.

From the analisys of all the functions in the tx kernel API the Canonical ABI rule that mostly causes the mismatch between the Canonical ABI and the Miden ABI is that anything larger than 8 bytes (i64) is returned via a pointer passed as an argument.

We want to recognize the functions with a mismatch between the Canonical ABI and the Miden ABI and make the compiler generate an adapter function that will call the tx kernel function and convert function arguments and result. 

For the complete list of the tx kernel functions in WIT format, see the [miden.wit](https://github.com/0xPolygonMiden/compiler/blob/18ead77410b27d97e96c96d36b573e289323f737/tests/rust-apps-wasm/sdk/sdk/wit/miden.wit)
For most TX kernel functions, the adapter function can be generated automatically using the pattern recognition and adapter functions below. 

## Required changes in other parts of the compiler

To make compiler aware of tx kernel function signatures they will be passed along the MAST hash root for every import in the Wasm component.

## Adapters generation

The compiler will analyze every component import to recognize the Miden ABI pattern and generate an adapter function if needed. This can be done in a transformation pass or as part of the MASM code generation.

## Miden ABI pattern recognition

The following pseudo-code can be used to recognize the Miden ABI pattern:

```rust
pub enum MidenAbiPattern {
    ReturnViaPointer,
    /// The Wasm core function type is the same as the tx kernel ad-hoc signature
    /// The tx kernel function can be called directly without any modifications.
    NoAdapterNeeded,
}

pub struct MidenAbiPatternRecognition {
    pattern: Option<MidenAbiPattern>,
    component_function: ComponentFunctionType,
    wasm_core_func: Signature,
    tx_kernel_function: Signature,
}

pub fn recognize_miden_abi_pattern(
    component_function: &ComponentFunctionType,
    wasm_core_func: &Signature,
    tx_kernel_func: &Signature) -> MidenAbiPatternRecognition {
    if wasm_core_func == tx_kernel_func {
        return MidenAbiPatternRecognition {
            pattern: Some(NoAdapterNeeded),
            component_function,
            wasm_core_function,
            tx_kernel_function,
        };
    } else if component_function.returns[0].byte_size > 8 && wasm_core_func.params.last() == I32 {
        return MidenAbiPatternRecognition {
            pattern: Some(ReturnViaPointer),
            component_function,
            wasm_core_function,
            tx_kernel_function,
        };
    } else {
        return MidenAbiPatternRecognition {
            pattern: None,
            component_function,
            wasm_core_function,
            tx_kernel_function,
        };
    }
}
```

## Adapter function code generation

The following pseudo-code can be used to generate the adapter function:

```rust
pub fn generate_adapter(recognition: MidenAbiPatternRecognition) {
    match recognition.pattern {
        Some(pattern) => generate_adapter(
            pattern, 
            recognition.component_function,
            recognition.wasm_core_function,
            recognition.tx_kernel_function
        ),
        None => use_manual_adapter(
            recognition.component_function,
            recognition.wasm_core_function,
            recognition.tx_kernel_function
        ),
    }
}

/// Escape hatch for the cases when the compiler can't generate an adapter function automatically
/// and we need to provide the adapter function manually.
pub fn use_manual_adapter(...) {
    // Find and use the manual adapter in the adapter library for the tx_kernel_function
}
```

The manual adapter library is a collection of adapter functions that are used when the compiler can't generate an adapter function automatically so its expected to be provided. The manual adapter library is a part of the Miden compiler.


### Return-via-pointer Adapter

The return value is expected to be returned by storing its flattened representation in a pointer passed as an argument.

Recognize this Miden ABI pattern by looking at the Wasm component function type. If the return value is bigger than 64 bits, expect the last argument in the Wasm core(HIR) signature to be `i32` (a pointer).

The adapter function calls the tx kernel function and stores the result in the provided pointer(the last argument of the wasm core function).

Here is the pseudo-code for generating the adapter function for the Return-via-pointer Miden ABI pattern:
```rust
    let ptr = wasm_core_function.params.last();
    let adapter_function = FunctionBuilder::new(wasm_core_function.clone());
    let tx_kernel_function_params = wasm_core_function.params.drop_last();
    let tx_kernel_func_val = adapter_function.call(tx_kernel_function, tx_kernel_function_params);
    adapter_function.store(tx_kernel_func_val, ptr);
    adapter_function.build();
```

Here is how the adapter might look like in a pseudo-code for the `add_asset` function:
```
/// Takes an Asset as an argument and returns a new Asset
func wasm_core_add_asset(v0: f64, v1: f64, v2: f64, v3: f64, ptr: i32) {
    v4 = call tx_kernel_add_asset(v0, v1, v2, v3);
    // v4 is a tuple of 4 f64 values
    store v4 in ptr;
}
```

### No-adapter-needed

No adapter is needed. The Wasm core function type is the same as the tx kernel ad-hoc signature.

This Miden ABI pattern is selected if no other Miden ABI pattern is applicable and the wasm core function signature is the same as the tx kernel ad-hoc signature.

For example, the `get_id` function falls under this Miden ABI pattern and its calls will be translated to the tx kernel function calls without any modifications.


## Transaction kernel functions that require manual adapter functions:
// TODO: check if the note inputs limit is 16 and make `get_inputs` return a big tuple

`get_assets:func() -> list<core-asset>` in the `note` interface is the only function that requires attention. In Canonical ABI, any function that returns a dynamic list of items needs to allocate memory in the caller's module due to the shared-nothing nature of the Wasm component model. For this case, a `realloc` function is passed as a part of lift/lower Canonical ABI options for the caller to allocate memory in the caller's module. 

Here are the signatures of the `get_assets` function in the WIT, core Wasm, and the tx kernel ad-hoc ABI:
Comment from the `miden-base`
```
#! Writes the assets of the currently executing note into memory starting at the specified address.
#!
#! Inputs: [dest_ptr]
#! Outputs: [num_assets, dest_ptr]
#!
#! - dest_ptr is the memory address to write the assets.
#! - num_assets is the number of assets in the currently executing note.
```

Wasm component function type:
`get-assets: func() -> list<core-asset>;`

Wasm core signature:
`wasm_core_get_assets(i32) -> ()`

If we add a new `get_assets_count: func() -> u32;` function to the tx kernel and add the assets count parameter to the `get_assets` function (`get_assets: func(assets_count: u32) -> list<core-asset>;`) we should have everything we need to manually write the adapter function for the `get_assets` function. 

The list is expected to be returned by storing the pointer to its first item in a `ptr` pointer passed as an argument and item count at `ptr + 4 bytes` address (`ptr` points to two pointers).

We could try to recognize this Miden ABI pattern by looking at the Wasm component function type. If the return value is a list, expect the last argument in the Wasm core(HIR) signature to be `i32` (a pointer). The problem is recognizing the list count parameter in the Wasm core(HIR) signature.

The adapter function calls allocates `asset_count * item_size` memory via the `realloc` call and passes the pointer to the newly allocated memory to the tx kernel function.

Here is how the adapter function might look like in a pseudo-code for the `get_assets` function:
```
func wasm_core_get_assets(asset_count: u32, ptr_ptr: i32) {
    mem_size = asset_count * item_size;
    ptr = realloc(mem_size);
    (actual_asset_count, ptr) = call tx_kernel_get_assets(ptr);
    assert(actual_asset_count == asset_count);
    store ptr in ptr_ptr;
    store account_count in ptr_ptr + 4;

}
```

**Since the `get_assets` tx kernel function in the current form can trash the provided memory if the actual assets count differs from the returned by `get_assets_count`, we can introduce the asset count parameter to the `get_assets` tx kernel function and check that it the same as the actual assets count written to memory.**


## The example of some functions signatures 

### `add_asset` (return-via-pointer Miden ABI pattern)
Comment from the `miden-base`
```
#! Add the specified asset to the vault.
#!
#! Panics:
#! - If the asset is not valid.
#! - If the total value of two fungible assets is greater than or equal to 2^63.
#! - If the vault already contains the same non-fungible asset.
#!
#! Stack: [ASSET]
#! Output: [ASSET']
#!
#! - ASSET' final asset in the account vault is defined as follows:
#!   - If ASSET is a non-fungible asset, then ASSET' is the same as ASSET.
#!   - If ASSET is a fungible asset, then ASSET' is the total fungible asset in the account vault
#!     after ASSET was added to it.
```

Wasm component function type:
`add-asset(core-asset) -> core-asset`

Wasm core signature:
`wasm_core_add_asset(f64, f64, f64, f64, i32) -> ()`
The last `i32` is a pointer to a returned value (`word`)

Tx kernel ad-hoc signature:
`tx_kernel_add_asset(felt, felt, felt, felt) -> (felt, felt, felt, felt)`


### `get_id` (no-adapter-needed Miden ABI pattern)
Comment from the `miden-base`
```
#! Returns the account id.
#!
#! Stack: []
#! Output: [acct_id]
#!
#! - acct_id is the account id.
```

Wasm component function type:
`get-id() -> account-id`

Wasm core signature:
`wasm_core_get_id() -> f64`

Tx kernel ad-hoc signature:
`tx_kernel_get_id() -> felt`
