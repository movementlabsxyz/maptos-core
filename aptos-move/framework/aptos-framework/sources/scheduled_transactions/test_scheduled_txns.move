#[test_only]
module aptos_framework::test_scheduled_txns {
    use std::signer;
    use aptos_framework::timestamp;
    use aptos_framework::coin::{Self};
    use aptos_framework::aptos_coin::AptosCoin;
    use aptos_framework::permissioned_signer;
    use aptos_framework::primary_fungible_store;
    use aptos_framework::scheduled_txns::{Self, create_transaction_id};
    use aptos_framework::transaction_fee;
    use aptos_framework::transaction_validation;

    #[test(fx = @0x1, user = @0x123)]
    fun test_scheduled_txn_flow(fx: &signer, user: signer) {
        let user_addr = signer::address_of(&user);

        // Setup
        let (burn, mint) = aptos_framework::aptos_coin::initialize_for_test(fx);
        transaction_fee::store_aptos_coin_burn_cap_for_test(fx, burn);
        transaction_fee::store_aptos_coin_mint_cap_for_test(fx, mint);
        aptos_framework::aptos_account::create_account(user_addr);
        scheduled_txns::initialize(fx);
        let fa_store_signer = scheduled_txns::get_deposit_owner_signer();
        let fa_store_addr = signer::address_of(&fa_store_signer);
        assert!(coin::balance<AptosCoin>(fa_store_addr) == 0, 0);
        timestamp::set_time_has_started_for_testing(fx);

        // Fund user account
        let coin = coin::mint<AptosCoin>(1000000, &mint);
        coin::deposit(user_addr, coin);
        let pre_balance = coin::balance<AptosCoin>(user_addr);

        // Create permissioned handle and set permissions
        let storable_perm_handle =
            permissioned_signer::create_storable_permissioned_handle(&user, 60);
        let perm_signer =
            permissioned_signer::signer_from_storable_permissioned_handle(
                &storable_perm_handle
            );

        // Ensure metadata for AptosCoin
        let metadata = scheduled_txns::get_metadata_for_AptosCoin();

        // Grant permissions to the permissioned signer
        primary_fungible_store::grant_permission(
            &user,
            &perm_signer,
            metadata,
            1000000 // Set a sufficient limit for withdrawals
        );

        // Create test transactions
        let txn1 =
            scheduled_txns::create_scheduled_txn(
                storable_perm_handle, 1000000, 20, 100, 0
            );
        let txn2 =
            scheduled_txns::create_scheduled_txn(
                storable_perm_handle, 1000000, 30, 100, 2000
            );

        // Insert transactions
        let txn1_id = scheduled_txns::insert(&user, txn1);
        let tId1 = create_transaction_id(txn1_id);
        let txn2_id = scheduled_txns::insert(&user, txn2);
        let tId2 = create_transaction_id(txn2_id);

        // Check initial state
        assert!(scheduled_txns::get_num_txns() == 2, 1);

        // Move time forward and get ready transactions
        timestamp::update_global_time_for_test(1000020);
        let ready_txns = scheduled_txns::get_ready_transactions_test(1000020, 10);
        assert!(ready_txns.length() == 2, 2);

        // Execute and verify transaction epilogue
        transaction_validation::scheduled_txn_epilogue_test_helper(
            &fa_store_signer,
            user_addr,
            tId1,
            10, // storage_fee_refund
            20, // gas_price
            100, // max_gas_units
            2000, // scheduling_deposit
            50 // gas_remaining
        );

        let post_txn1_balance = coin::balance<AptosCoin>(user_addr);
        assert!(
            (post_txn1_balance + 50 * 20 - 10 + 3000) == pre_balance,
            3
        );

        // Cleanup
        transaction_validation::scheduled_txn_epilogue_test_helper(
            &fa_store_signer,
            user_addr,
            tId2,
            2000, // storage_fee_refund
            20, // gas_price
            100, // max_gas_units
            3000, // scheduling_deposit
            50 // gas_remaining
        );
        let post_txn2_balance = coin::balance<AptosCoin>(user_addr);
        //debug::print(&std::string::utf8(b"post_txn2_balance."));
        //debug::print(&post_txn2_balance);
        assert!(
            (post_txn2_balance + 50 * 20 * 2 - 2010) == pre_balance,
            4
        );

        // check reschedule
        scheduled_txns::remove_txns();
        assert!(scheduled_txns::get_num_txns() == 1, 4);

        scheduled_txns::shutdown_test(fx);

        assert!(
            coin::balance<AptosCoin>(fa_store_addr) == 0,
            coin::balance<AptosCoin>(fa_store_addr)
        );
        coin::destroy_burn_cap(burn);
        coin::destroy_mint_cap(mint);
    }
}
