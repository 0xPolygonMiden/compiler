#![no_std]
#![no_main]

use basic_wallet;
use basic_wallet::MyWallet;
use miden::account;

// WASI runtime expects the following function to be "main"
#[no_mangle]
pub extern "C" fn __main_void() -> i32 {
    /*

    Expected Miden program:

    #! Helper procedure to add all assets of a note to an account.
    #!
    #! Inputs: []
    #! Outputs: []
    #!
    proc.add_note_assets_to_account
        push.0 exec.note::get_assets
        # => [num_of_assets, 0 = ptr, ...]

        # compute the pointer at which we should stop iterating
        dup.1 add
        # => [end_ptr, ptr, ...]

        # pad the stack and move the pointer to the top
        padw movup.5
        # => [ptr, 0, 0, 0, 0, end_ptr, ...]

        # compute the loop latch
        dup dup.6 neq
        # => [latch, ptr, 0, 0, 0, 0, end_ptr, ...]

        while.true
            # => [ptr, 0, 0, 0, 0, end_ptr, ...]

            # save the pointer so that we can use it later
            dup movdn.5
            # => [ptr, 0, 0, 0, 0, ptr, end_ptr, ...]

            # load the asset and add it to the account
            mem_loadw call.wallet::receive_asset
            # => [ASSET, ptr, end_ptr, ...]

            # increment the pointer and compare it to the end_ptr
            movup.4 add.1 dup dup.6 neq
            # => [latch, ptr+1, ASSET, end_ptr, ...]
        end

        # clear the stack
        drop dropw drop
    end


    #! Pay-to-ID script: adds all assets from the note to the account, assuming ID of the account
    #! matches target account ID specified by the note inputs.
    #!
    #! Requires that the account exposes: miden::wallets::basic::receive_asset procedure.
    #!
    #! Inputs: []
    #! Outputs: []
    #!
    #! Note inputs are assumed to be as follows:
    #! - target_account_id is the ID of the account for which the note is intended.
    #!
    #! FAILS if:
    #! - Account does not expose miden::wallets::basic::receive_asset procedure.
    #! - Account ID of executing account is not equal to the Account ID specified via note inputs.
    #! - The same non-fungible asset already exists in the account.
    #! - Adding a fungible asset would result in amount overflow, i.e., the total amount would be
    #!   greater than 2^63.
    export.p2id
        # load the note inputs to memory starting at address 0
        push.0 exec.note::get_inputs
        # => [inputs_ptr]

        # read the target account id from the note inputs
        mem_load
        # => [target_account_id]

        exec.account::get_id
        # => [account_id, target_account_id, ...]

        # ensure account_id = target_account_id, fails otherwise
        assert_eq
        # => [...]

        exec.add_note_assets_to_account
        # => [...]
    end
         */

    // TODO: implement the missing parts of the program and SDK
    // let inputs = miden::sat::note::get_inputs();
    // let target_account_id = todo!("read target account id from inputs");
    // let account_id = miden::sat::account::get_id();
    // assert_eq!(account_id, target_account_id);
    // let assets = miden::sat::note::get_assets();
    // for asset in assets {
    //     // TODO: should be invoked via `call` op
    //     MyWallet.receive_asset(asset);
    // }
    0
}
