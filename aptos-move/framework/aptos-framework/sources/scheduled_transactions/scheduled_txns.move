module aptos_framework::scheduled_txns {
    use std::bcs;
    use std::error;
    use std::hash::sha3_256;
    use std::signer;
    use std::vector;
    use aptos_std::table;
    use aptos_std::table::Table;
    use aptos_framework::account;
    use aptos_framework::aptos_coin::AptosCoin;
    use aptos_framework::big_ordered_map::{Self, BigOrderedMap};
    use aptos_framework::coin;
    use aptos_framework::coin::ensure_paired_metadata;
    use aptos_framework::permissioned_signer;
    use aptos_framework::permissioned_signer::StorablePermissionedHandle;
    use aptos_framework::primary_fungible_store;
    use aptos_framework::system_addresses;
    use aptos_framework::timestamp;
    #[test_only]
    use aptos_framework::fungible_asset::Metadata;
    #[test_only]
    use aptos_framework::object::Object;

    friend aptos_framework::block;
    friend aptos_framework::transaction_validation;
    #[test_only]
    friend aptos_framework::test_scheduled_txns;

    /// Map key already exists
    const EINVALID_SIGNER: u64 = 1;

    /// Scheduled time is in the past
    const EINVALID_TIME: u64 = 2;

    const U64_MAX: u64 = 18446744073709551615;

    /// Conversion factor between our time granularity (100ms) and microseconds
    const MICRO_CONVERSION_FACTOR: u64 = 100000;

    /// Conversion factor between our time granularity (100ms) and milliseconds
    const MILLI_CONVERSION_FACTOR: u64 = 100;

    /// If we cannot schedule in 100 * time granularity (100ms, i.e 100 blocks), we will abort the txn
    const EXPIRY_DELTA: u64 = 100;

    /// SHA3-256 produces 32 bytes
    const TXN_ID_SIZE: u16 = 32;

    // Todo: Confirm this.
    /// The maximum size of a function in bytes
    const MAX_FUNC_SIZE: u16 = 1024;

    // Todo: Confirm this is a reasonable estimate
    /// The maximum size of a function in bytes
    const AVG_FUNC_SIZE: u16 = 128;
    const AVG_SCHED_TXN_SIZE: u16 = 128 + AVG_FUNC_SIZE; // strictly it is 112 + AVG_FUNC_SIZE

    /// ScheduledTransaction with permission signer handle, scheduled_time, gas params, and function
    struct ScheduledTransaction has copy, drop, store {
        /// 72 bytes (32 + 32 + 8)
        sender_handle: StorablePermissionedHandle,
        /// 100ms granularity
        scheduled_time: u64,
        /// Maximum gas to spend for this transaction
        max_gas_amount: u64,
        /// Charged @ lesser of {max_gas_unit_price, max_gas_unit_price other than this in the block executed}
        max_gas_unit_price: u64,
        /// txn to be rescheduled at scheduled_time + next_schedule_delta_time.
        /// Note: (1) Once set, the txn will be rescheduled at the same delta interval next time, and so on.
        ///       (2) Can be cancelled, with the same id returned in insert(), to stop the perpetual rescheduling.
        ///       (3) If one rescheduled fails or is expired, the perpetual rescheduling chain will be broken.
        ///       (4) If scheduled_time + next_schedule_delta_time < current_time, the txn reschedule will fail.
        next_schedule_delta_time: u64,
        /// Variables are captured in the closure; no arguments passed; no return
        f: |()|
    }

    /// We pass the id around instead re-computing it
    struct ScheduledTransactionWithId has copy, drop, store {
        txn: ScheduledTransaction,
        txn_id: TransactionId
    }

    /// SHA3-256
    struct TransactionId has copy, drop, store {
        hash: vector<u8>
    }

    /// First sorted in ascending order of time, then on gas priority, and finally on txn_id
    /// gas_priority = U64_MAX - gas_unit_price; we want higher gas_unit_price to come before lower gas_unit_price
    /// The goal is to have fixed size key, val entries in BigOrderedMap, hence we use txn_id as a key instead of
    /// having {time, gas_priority} --> List<txn_id>
    struct ScheduleMapKey has copy, drop, store {
        time: u64,
        gas_priority: u64,
        txn_id: TransactionId
    }

    /// Dummy struct to use as a value type in BigOrderedMap
    struct Empty has copy, drop, store {}

    struct ScheduleQueue has key {
        /// key_size = 48 bytes; value_size = 0
        schedule_map: BigOrderedMap<ScheduleMapKey, Empty>,
        /// lookup 'ScheduledTransaction' by txn_id
        /// Using a 'table' is costly because it creates a new slot for every <key, val>; hence using BigOrderedMap
        txn_tbl: BigOrderedMap<TransactionId, ScheduledTransaction>
    }

    /// BigOrderedMap has MAX_NODE_BYTES = 409600 (400KB), MAX_DEGREE = 4096, DEFAULT_TARGET_NODE_SIZE = 4096;
    const BIG_ORDRD_MAP_TGT_ND_SZ: u16 = 4096;
    const SCHEDULE_MAP_KEY_SIZE: u16 = 48;
    // Leaf node size = 48 * 80 (leaf_degree) = 3840 bytes (<= DEFAULT_TARGET_NODE_SIZE)
    // todo: check if it can be DEFAULT_TARGET_NODE_SIZE/SCHEDULE_MAP_KEY_SIZE; check if value size is indeed 0
    const SCHEDULE_MAP_LEAF_DEGREE: u16 = 80;

    /// Signer for the store for gas fee deposits
    // todo: check if this is secure
    struct GasFeeDepositStoreSignerCap has key {
        cap: account::SignerCapability
    }

    /// We want reduce the contention while scheduled txns are being executed
    // todo: check if 32 is a good number
    const TO_REMOVE_PARALLELISM: u64 = 32;
    struct ToRemoveTbl has key {
        remove_tbl: Table<u16, vector<TransactionId>>
    }

    /// Can be called only by the framework
    public fun initialize(framework: &signer) {
        system_addresses::assert_aptos_framework(framework);

        // Create owner account for handling deposits
        let owner_addr = @0xb; // Replace with your desired address
        let (owner_signer, owner_cap) =
            account::create_framework_reserved_account(owner_addr);

        // Initialize fungible store for the owner
        let metadata = ensure_paired_metadata<AptosCoin>();
        primary_fungible_store::ensure_primary_store_exists(
            signer::address_of(&owner_signer), metadata
        );

        // Store the capability
        move_to(framework, GasFeeDepositStoreSignerCap { cap: owner_cap });

        // Initialize queue
        let queue = ScheduleQueue {
            schedule_map: big_ordered_map::new_with_config(
                BIG_ORDRD_MAP_TGT_ND_SZ / SCHEDULE_MAP_KEY_SIZE,
                SCHEDULE_MAP_LEAF_DEGREE,
                true
            ),
            txn_tbl: big_ordered_map::new_with_config(
                (BIG_ORDRD_MAP_TGT_ND_SZ / TXN_ID_SIZE),
                (BIG_ORDRD_MAP_TGT_ND_SZ / (TXN_ID_SIZE + AVG_SCHED_TXN_SIZE)),
                true
            )
        };
        move_to(framework, queue);

        // Aggregator to keep count of how many entries to be removed from the queue
        move_to(
            framework,
            ToRemoveTbl {
                remove_tbl: table::new<u16, vector<TransactionId>>()
            }
        );
    }

    /// Stop, remove and refund all scheduled txns; can be called only by the framework
    public fun shutdown(
        framework: &signer
    ) acquires ScheduleQueue, ToRemoveTbl, GasFeeDepositStoreSignerCap {
        system_addresses::assert_aptos_framework(framework);

        // Make a list of txns to cancel and refund
        let txns_to_cancel = vector::empty<ScheduledTransactionWithId>();
        let queue = borrow_global<ScheduleQueue>(signer::address_of(framework));
        let iter = queue.txn_tbl.new_begin_iter();
        while (!iter.iter_is_end(&queue.txn_tbl)) {
            let txn_id = *iter.iter_borrow_key();
            let txn = iter.iter_borrow(&queue.txn_tbl);
            let scheduled_txn_with_id = ScheduledTransactionWithId { txn: *txn, txn_id };
            txns_to_cancel.push_back(scheduled_txn_with_id);
            iter = iter.iter_next(&queue.txn_tbl);
        };

        // Cancel all transactions
        while (!txns_to_cancel.is_empty()) {
            let txn_with_id = txns_to_cancel.pop_back();
            // Create a new signer from the stored handle
            let schedule_txn_signer =
                permissioned_signer::signer_from_storable_permissioned_handle(
                    &txn_with_id.txn.sender_handle
                );
            cancel(&schedule_txn_signer, txn_with_id.txn_id.hash);
        };

        // Remove and destroy resources
        let ScheduleQueue { schedule_map, txn_tbl } =
            move_from<ScheduleQueue>(signer::address_of(framework));
        schedule_map.destroy(|_| {});
        txn_tbl.destroy(|_| {});

        // Clean up ToRemoveTbl; we can only empty the table but not drop it!
        let ToRemoveTbl { remove_tbl } =
            borrow_global_mut<ToRemoveTbl>(signer::address_of(framework));
        let i = 0;
        while (i < TO_REMOVE_PARALLELISM) {
            if (remove_tbl.contains((i as u16))) {
                remove_tbl.remove((i as u16));
            };
            i = i + 1;
        };
    }

    /// todo: Do we need a function to pause ???

    /// Insert a scheduled transaction into the queue. Txn_id is returned to user, which can be used to cancel the txn.
    public fun insert(
        sender: &signer, txn: ScheduledTransaction
    ): vector<u8> acquires ScheduleQueue, GasFeeDepositStoreSignerCap {
        // todo: we should limit the size of the scheduled txn; NOTE that f is of variable size ???
        // Generate a unique transaction ID only once and only here. Because all periodically rescheduled txns (if any)
        // will have the same txn_id generated here (which is different from the sha3_256 of the rescheduled txn)
        let txn_id = TransactionId {
            hash: sha3_256(bcs::to_bytes(&txn))
        };
        insert_txn_with_id(sender, txn, txn_id)
    }

    /// Cancel a scheduled transaction, must be called by the signer who originally scheduled the transaction.
    public fun cancel(
        sender: &signer, txn_id: vector<u8>
    ) acquires ScheduleQueue, GasFeeDepositStoreSignerCap {
        let queue = borrow_global_mut<ScheduleQueue>(@aptos_framework);
        let txn_id = TransactionId { hash: txn_id };
        if (!queue.txn_tbl.contains(&txn_id)) {
            return;
        };

        let txn = queue.txn_tbl.borrow(&txn_id);
        let deposit_amt = txn.max_gas_amount * txn.max_gas_unit_price;

        // we expect the sender to be a permissioned signer
        let schedule_txn_signer =
            permissioned_signer::signer_from_storable_permissioned_handle(
                &txn.sender_handle
            );
        assert!(
            signer::address_of(sender) == signer::address_of(&schedule_txn_signer),
            error::permission_denied(EINVALID_SIGNER)
        );

        // Remove the transaction from the schedule_map & txn_tbl
        let key = ScheduleMapKey {
            time: txn.scheduled_time / MILLI_CONVERSION_FACTOR,
            gas_priority: U64_MAX - txn.max_gas_unit_price,
            txn_id
        };
        queue.schedule_map.remove(&key);
        queue.txn_tbl.remove(&txn_id);

        // Refund the deposit
        // Get owner signer from capability
        let gas_deposit_store_cap =
            borrow_global<GasFeeDepositStoreSignerCap>(@aptos_framework);
        let gas_deposit_store_signer =
            account::create_signer_with_capability(&gas_deposit_store_cap.cap);

        // Refund deposit from owner's store to sender
        coin::transfer<AptosCoin>(
            &gas_deposit_store_signer,
            signer::address_of(sender),
            deposit_amt
        );
    }

    /// Common function called for both insert and reschedule
    fun insert_txn_with_id(
        sender: &signer, txn: ScheduledTransaction, txn_id: TransactionId
    ): vector<u8> acquires ScheduleQueue, GasFeeDepositStoreSignerCap {
        // we expect the sender to be a permissioned signer
        let schedule_txn_signer =
            permissioned_signer::signer_from_storable_permissioned_handle(
                &txn.sender_handle
            );
        assert!(
            signer::address_of(sender) == signer::address_of(&schedule_txn_signer),
            error::permission_denied(EINVALID_SIGNER)
        );

        let queue = borrow_global_mut<ScheduleQueue>(@aptos_framework);
        if (queue.txn_tbl.contains(&txn_id)) {
            return txn_id.hash;
        };

        // Only schedule txns in the future
        let txn_time = txn.scheduled_time / MILLI_CONVERSION_FACTOR; // Round down to the nearest 100ms
        let block_time = timestamp::now_microseconds() / MICRO_CONVERSION_FACTOR;
        assert!(txn_time > block_time, error::invalid_argument(EINVALID_TIME));

        // We need inverse of gas_unit_price for ordering because ScheduleMapKey is sorted in ascending order time
        // first and then on gas_priority
        let gas_priority = U64_MAX - txn.max_gas_unit_price;
        let key = ScheduleMapKey { time: txn_time, gas_priority, txn_id };

        // Insert the transaction into the schedule_map & txn_tbl
        queue.schedule_map.add(key, Empty {});
        queue.txn_tbl.add(txn_id, txn);

        // Collect deposit
        // Get owner signer from capability
        let gas_deposit_store_cap =
            borrow_global<GasFeeDepositStoreSignerCap>(@aptos_framework);
        let gas_deposit_store_signer =
            account::create_signer_with_capability(&gas_deposit_store_cap.cap);
        let gas_deposit_store_addr = signer::address_of(&gas_deposit_store_signer);

        coin::transfer<AptosCoin>(
            sender,
            gas_deposit_store_addr,
            txn.max_gas_amount * txn.max_gas_unit_price
        );
        txn_id.hash
    }

    /// Gets txns due to be run; also expire txns that could not be run for a while (mostly due to low gas priority)
    fun get_ready_transactions(
        timestamp: u64, limit: u64
    ): vector<ScheduledTransactionWithId> acquires ScheduleQueue, GasFeeDepositStoreSignerCap {
        let queue = borrow_global<ScheduleQueue>(@aptos_framework);
        let block_time = timestamp / MILLI_CONVERSION_FACTOR;
        let scheduled_txns = vector::empty<ScheduledTransactionWithId>();
        let count = 0;
        let txns_to_expire = vector::empty<ScheduledTransactionWithId>();

        let iter = queue.schedule_map.new_begin_iter();
        while (!iter.iter_is_end(&queue.schedule_map) && count < limit) {
            let scheduled_key = iter.iter_borrow_key();
            if (scheduled_key.time > block_time) {
                return scheduled_txns;
            };
            let txn_id = scheduled_key.txn_id;
            let txn = *queue.txn_tbl.borrow(&txn_id);
            let scheduled_txn_with_id = ScheduledTransactionWithId { txn, txn_id };
            if (scheduled_key.time + EXPIRY_DELTA < block_time) {
                // Transaction has expired
                txns_to_expire.push_back(scheduled_txn_with_id);
            } else {
                scheduled_txns.push_back(scheduled_txn_with_id);
            };
            iter = iter.iter_next(&queue.schedule_map);
        };

        while (!txns_to_expire.is_empty()) {
            let txn_with_id = txns_to_expire.pop_back();
            // Create a new signer from the stored handle
            let schedule_txn_signer =
                permissioned_signer::signer_from_storable_permissioned_handle(
                    &txn_with_id.txn.sender_handle
                );
            cancel(&schedule_txn_signer, txn_with_id.txn_id.hash);
        };

        scheduled_txns
    }

    /// Increment after every scheduled transaction is run
    /// IMP: Make sure this does not affect parallel execution of txns
    public(friend) fun finish_execution(txn_id: TransactionId) acquires ToRemoveTbl {
        // Get first 8 bytes of the hash as u64 and then mod
        let hash_bytes = txn_id.hash;
        assert!(hash_bytes.length() == 32, 1); // SHA3-256 produces 32 bytes

        // Take first 8 bytes and convert to u64
        let value =
            ((hash_bytes[0] as u64) << 56) | ((hash_bytes[1] as u64) << 48)
                | ((hash_bytes[2] as u64) << 40) | ((hash_bytes[3] as u64) << 32)
                | ((hash_bytes[4] as u64) << 24) | ((hash_bytes[5] as u64) << 16)
                | ((hash_bytes[6] as u64) << 8) | (hash_bytes[7] as u64);

        // todo: check if it is efficient to compute tbl_idx in rust instead
        let tbl_idx = ((value % TO_REMOVE_PARALLELISM) as u16);
        let to_remove = borrow_global_mut<ToRemoveTbl>(@aptos_framework);

        if (!to_remove.remove_tbl.contains(tbl_idx)) {
            let txn_ids = vector::empty<TransactionId>();
            txn_ids.push_back(txn_id);
            to_remove.remove_tbl.add(tbl_idx, txn_ids);
        } else {
            let txn_ids = to_remove.remove_tbl.borrow_mut(tbl_idx);
            txn_ids.push_back(txn_id);
        };
    }

    /// Remove the txns that are run
    public(friend) fun remove_txns() acquires ToRemoveTbl, ScheduleQueue, GasFeeDepositStoreSignerCap {
        let to_remove = borrow_global_mut<ToRemoveTbl>(@aptos_framework);
        let queue = borrow_global_mut<ScheduleQueue>(@aptos_framework);
        let idx: u16 = 0;
        let txns_to_reschedule = vector::empty<ScheduledTransactionWithId>();

        while ((idx as u64) < TO_REMOVE_PARALLELISM) {
            if (to_remove.remove_tbl.contains(idx)) {
                let txn_ids = to_remove.remove_tbl.remove(idx);
                let txn_ids_len = txn_ids.length();
                let txn_idx = 0;

                while (txn_idx < txn_ids_len) {
                    let txn_id = *txn_ids.borrow(txn_idx);
                    // Remove transaction from txn_tbl
                    let txn = queue.txn_tbl.remove(&txn_id);
                    let key =
                        ScheduleMapKey {
                            time: txn.scheduled_time / MILLI_CONVERSION_FACTOR,
                            gas_priority: U64_MAX - txn.max_gas_unit_price,
                            txn_id
                        };

                    if (txn.next_schedule_delta_time > 0) {
                        // Reschedule the transaction
                        txn.scheduled_time =
                            txn.scheduled_time + txn.next_schedule_delta_time;
                        txns_to_reschedule.push_back(
                            ScheduledTransactionWithId { txn, txn_id }
                        );
                    };
                    // Remove transaction from schedule_map
                    queue.schedule_map.remove(&key);
                    txn_idx = txn_idx + 1;
                };
            };
            idx = idx + 1;
        };

        // Reinsert the transactions that need to be rescheduled
        while (!txns_to_reschedule.is_empty()) {
            let txn_with_id = txns_to_reschedule.pop_back();
            // Create a new signer from the stored handle
            let schedule_txn_signer =
                permissioned_signer::signer_from_storable_permissioned_handle(
                    &txn_with_id.txn.sender_handle
                );
            insert_txn_with_id(
                &schedule_txn_signer, txn_with_id.txn, txn_with_id.txn_id
            );
        }
    }

    ////////////////////////// TESTS //////////////////////////
    #[test_only]
    public fun get_num_txns(): u64 acquires ScheduleQueue {
        let queue = borrow_global<ScheduleQueue>(@aptos_framework);
        let num_txns = queue.txn_tbl.compute_length();
        num_txns
    }

    #[test_only]
    public fun get_ready_transactions_test(
        timestamp: u64, limit: u64
    ): vector<ScheduledTransactionWithId> acquires ScheduleQueue, GasFeeDepositStoreSignerCap {
        get_ready_transactions(timestamp, limit)
    }

    #[test_only]
    public fun create_transaction_id(hash: vector<u8>): TransactionId {
        let txn_id = TransactionId { hash };
        txn_id
    }

    #[test_only]
    public fun get_deposit_owner_signer(): signer acquires GasFeeDepositStoreSignerCap {
        let owner_cap = borrow_global<GasFeeDepositStoreSignerCap>(@aptos_framework);
        let owner_signer = account::create_signer_with_capability(&owner_cap.cap);
        owner_signer
    }

    #[test_only]
    public fun get_metadata_for_AptosCoin(): Object<Metadata> {
        coin::ensure_paired_metadata<AptosCoin>()
    }

    struct State has copy, drop, store {
        count: u64
    }

    #[persistent]
    fun step(state: State, _val: u64) {
        if (state.count < 10) {
            state.count = state.count + 1;
        }
    }

    #[test_only]
    public fun create_scheduled_txn(
        storable_perm_handle: StorablePermissionedHandle,
        scheduled_time: u64,
        max_gas_unit_price: u64,
        max_gas_amount: u64,
        next_schedule_delta_time: u64
    ): ScheduledTransaction {
        let state = State { count: 8 };
        let foo = || step(state, 5);

        ScheduledTransaction {
            sender_handle: storable_perm_handle,
            scheduled_time,
            max_gas_amount,
            max_gas_unit_price,
            next_schedule_delta_time,
            f: foo
        }
    }

    #[test_only]
    public fun shutdown_test(
        fx: &signer
    ) acquires ScheduleQueue, GasFeeDepositStoreSignerCap, ToRemoveTbl {
        shutdown(fx);
    }

    #[test(fx = @0x1, user = @0x1234)]
    fun test_basic(
        fx: &signer, user: signer
    ) acquires ScheduleQueue, GasFeeDepositStoreSignerCap, ToRemoveTbl {
        let user_addr = signer::address_of(&user);

        // Setup test environment
        let (burn, mint) = aptos_framework::aptos_coin::initialize_for_test(fx);
        aptos_framework::aptos_account::create_account(user_addr);
        initialize(fx);
        timestamp::set_time_has_started_for_testing(fx);

        // Fund user account
        let coin = coin::mint<AptosCoin>(1000000, &mint);
        coin::deposit(user_addr, coin);

        // Create permissioned handle and set permissions
        let storable_perm_handle =
            permissioned_signer::create_storable_permissioned_handle(&user, 60);
        let perm_signer =
            permissioned_signer::signer_from_storable_permissioned_handle(
                &storable_perm_handle
            );

        // Ensure metadata for AptosCoin
        let metadata = coin::ensure_paired_metadata<AptosCoin>();

        // Grant permissions to the permissioned signer
        primary_fungible_store::grant_permission(
            &user,
            &perm_signer,
            metadata,
            1000000 // Set a sufficient limit for withdrawals
        );

        // Create transactions with same scheduled_time but different gas prices
        let txn1 = create_scheduled_txn(storable_perm_handle, 1000000, 20, 100, 0); // time: 1s, gas: 20
        let txn2 = create_scheduled_txn(storable_perm_handle, 1000000, 30, 100, 0); // time: 1s, gas: 30
        let txn3 = create_scheduled_txn(storable_perm_handle, 1000000, 10, 100, 0); // time: 1s, gas: 10

        // Create transactions with same scheduled_time and gas price
        let txn4 = create_scheduled_txn(storable_perm_handle, 2000000, 20, 1000, 0); // time: 2s, gas: 20
        let txn5 = create_scheduled_txn(storable_perm_handle, 2000000, 20, 100, 3000000); // time: 2s, gas: 20
        let txn6 = create_scheduled_txn(storable_perm_handle, 2000000, 20, 200, 0); // time: 2s, gas: 20

        let txn7 = create_scheduled_txn(storable_perm_handle, 4000000, 20, 100, 0); // time: 2s, gas: 20
        let txn8 = create_scheduled_txn(storable_perm_handle, 4000000, 20, 200, 0); // time: 2s, gas: 20

        // Insert all transactions
        let txn1_id = insert(&user, txn1);
        let txn2_id = insert(&user, txn2);
        let txn3_id = insert(&user, txn3);
        let txn4_id = insert(&user, txn4);
        let txn5_id = insert(&user, txn5);
        let txn6_id = insert(&user, txn6);
        let txn7_id = insert(&user, txn7);
        let txn8_id = insert(&user, txn8);

        assert!(get_num_txns() == 8, get_num_txns());

        // Test get_ready_transactions at t=0.5s (should return empty)
        timestamp::update_global_time_for_test(500000);
        let ready_txns = get_ready_transactions(500000, 10);
        assert!(ready_txns.length() == 0, 2);

        // Test get_ready_transactions at t=1.5s (should return first 3 txns)
        let ready_txns = get_ready_transactions(1000020, 10);
        assert!(ready_txns.length() == 3, 3);
        assert!(ready_txns[0].txn_id == TransactionId { hash: txn2_id }, 3);

        // Remove transactions
        finish_execution(TransactionId { hash: txn1_id });
        finish_execution(TransactionId { hash: txn2_id });
        finish_execution(TransactionId { hash: txn3_id });
        remove_txns(); // Should remove first 3 txns
        assert!(get_num_txns() == 5, 4);

        // Test get_ready_transactions at t=2.5s (should return next 3 txns)
        let ready_txns = get_ready_transactions(2000040, 10);
        assert!(ready_txns.length() == 3, 5);

        // Execute and remove 2 transactions
        finish_execution(TransactionId { hash: txn4_id });
        finish_execution(TransactionId { hash: txn6_id });
        remove_txns(); // Should remove 2 txns
        assert!(get_num_txns() == 3, 6);

        let ready_txns = get_ready_transactions(2000050, 10);
        assert!(ready_txns.length() == 1, 7);

        // Execute and remove last 3 transactions
        finish_execution(TransactionId { hash: txn5_id });
        finish_execution(TransactionId { hash: txn7_id });
        finish_execution(TransactionId { hash: txn8_id });

        remove_txns(); // Should remove all but txn which has to be rescheduled
        assert!(get_num_txns() == 1, get_num_txns());
        assert!(get_ready_transactions(2000000, 10).length() == 0, 5);
        assert!(get_ready_transactions(5000020, 10).length() == 1, 5);

        // try expiring a txn by getting it late
        assert!(
            get_ready_transactions(6000000, 10).length() == 0,
            get_ready_transactions(6000000, 10).length()
        );
        assert!(get_num_txns() == 0, get_num_txns());

        coin::destroy_burn_cap(burn);
        coin::destroy_mint_cap(mint);
    }
}
