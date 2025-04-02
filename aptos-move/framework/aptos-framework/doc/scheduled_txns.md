
<a id="0x1_scheduled_txns"></a>

# Module `0x1::scheduled_txns`



-  [Struct `ScheduledTransaction`](#0x1_scheduled_txns_ScheduledTransaction)
-  [Struct `ScheduledTransactionWithId`](#0x1_scheduled_txns_ScheduledTransactionWithId)
-  [Struct `TransactionId`](#0x1_scheduled_txns_TransactionId)
-  [Struct `ScheduleMapKey`](#0x1_scheduled_txns_ScheduleMapKey)
-  [Resource `ScheduleQueue`](#0x1_scheduled_txns_ScheduleQueue)
-  [Resource `OwnerCapability`](#0x1_scheduled_txns_OwnerCapability)
-  [Resource `ToRemoveTbl`](#0x1_scheduled_txns_ToRemoveTbl)
-  [Struct `State`](#0x1_scheduled_txns_State)
-  [Constants](#@Constants_0)
-  [Function `initialize`](#0x1_scheduled_txns_initialize)
-  [Function `insert`](#0x1_scheduled_txns_insert)
-  [Function `cancel`](#0x1_scheduled_txns_cancel)
-  [Function `get_ready_transactions`](#0x1_scheduled_txns_get_ready_transactions)
-  [Function `finish_execution`](#0x1_scheduled_txns_finish_execution)
-  [Function `remove_txns`](#0x1_scheduled_txns_remove_txns)
-  [Function `step`](#0x1_scheduled_txns_step)


<pre><code><b>use</b> <a href="account.md#0x1_account">0x1::account</a>;
<b>use</b> <a href="aptos_coin.md#0x1_aptos_coin">0x1::aptos_coin</a>;
<b>use</b> <a href="../../aptos-stdlib/../move-stdlib/doc/bcs.md#0x1_bcs">0x1::bcs</a>;
<b>use</b> <a href="big_ordered_map.md#0x1_big_ordered_map">0x1::big_ordered_map</a>;
<b>use</b> <a href="coin.md#0x1_coin">0x1::coin</a>;
<b>use</b> <a href="../../aptos-stdlib/doc/debug.md#0x1_debug">0x1::debug</a>;
<b>use</b> <a href="fungible_asset.md#0x1_fungible_asset">0x1::fungible_asset</a>;
<b>use</b> <a href="../../aptos-stdlib/../move-stdlib/doc/hash.md#0x1_hash">0x1::hash</a>;
<b>use</b> <a href="object.md#0x1_object">0x1::object</a>;
<b>use</b> <a href="permissioned_signer.md#0x1_permissioned_signer">0x1::permissioned_signer</a>;
<b>use</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store">0x1::primary_fungible_store</a>;
<b>use</b> <a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">0x1::signer</a>;
<b>use</b> <a href="../../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string">0x1::string</a>;
<b>use</b> <a href="system_addresses.md#0x1_system_addresses">0x1::system_addresses</a>;
<b>use</b> <a href="../../aptos-stdlib/doc/table.md#0x1_table">0x1::table</a>;
<b>use</b> <a href="timestamp.md#0x1_timestamp">0x1::timestamp</a>;
<b>use</b> <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">0x1::vector</a>;
</code></pre>



<a id="0x1_scheduled_txns_ScheduledTransaction"></a>

## Struct `ScheduledTransaction`



<pre><code><b>struct</b> <a href="scheduled_txns.md#0x1_scheduled_txns_ScheduledTransaction">ScheduledTransaction</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>sender_handle: <a href="permissioned_signer.md#0x1_permissioned_signer_StorablePermissionedHandle">permissioned_signer::StorablePermissionedHandle</a></code>
</dt>
<dd>

</dd>
<dt>
<code>scheduled_time: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>max_gas_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>max_gas_unit_price: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>next_schedule_delta_time: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>f: |()| <b>with</b> <b>copy</b>+store</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_scheduled_txns_ScheduledTransactionWithId"></a>

## Struct `ScheduledTransactionWithId`



<pre><code><b>struct</b> <a href="scheduled_txns.md#0x1_scheduled_txns_ScheduledTransactionWithId">ScheduledTransactionWithId</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>txn: <a href="scheduled_txns.md#0x1_scheduled_txns_ScheduledTransaction">scheduled_txns::ScheduledTransaction</a></code>
</dt>
<dd>

</dd>
<dt>
<code>txn_id: <a href="scheduled_txns.md#0x1_scheduled_txns_TransactionId">scheduled_txns::TransactionId</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_scheduled_txns_TransactionId"></a>

## Struct `TransactionId`



<pre><code><b>struct</b> <a href="scheduled_txns.md#0x1_scheduled_txns_TransactionId">TransactionId</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../../aptos-stdlib/../move-stdlib/doc/hash.md#0x1_hash">hash</a>: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_scheduled_txns_ScheduleMapKey"></a>

## Struct `ScheduleMapKey`



<pre><code><b>struct</b> <a href="scheduled_txns.md#0x1_scheduled_txns_ScheduleMapKey">ScheduleMapKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>time: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>gas_priority: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_scheduled_txns_ScheduleQueue"></a>

## Resource `ScheduleQueue`



<pre><code><b>struct</b> <a href="scheduled_txns.md#0x1_scheduled_txns_ScheduleQueue">ScheduleQueue</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>schedule_map: <a href="big_ordered_map.md#0x1_big_ordered_map_BigOrderedMap">big_ordered_map::BigOrderedMap</a>&lt;<a href="scheduled_txns.md#0x1_scheduled_txns_ScheduleMapKey">scheduled_txns::ScheduleMapKey</a>, <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="scheduled_txns.md#0x1_scheduled_txns_TransactionId">scheduled_txns::TransactionId</a>&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>txn_tbl: <a href="big_ordered_map.md#0x1_big_ordered_map_BigOrderedMap">big_ordered_map::BigOrderedMap</a>&lt;<a href="scheduled_txns.md#0x1_scheduled_txns_TransactionId">scheduled_txns::TransactionId</a>, <a href="scheduled_txns.md#0x1_scheduled_txns_ScheduledTransaction">scheduled_txns::ScheduledTransaction</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_scheduled_txns_OwnerCapability"></a>

## Resource `OwnerCapability`



<pre><code><b>struct</b> <a href="scheduled_txns.md#0x1_scheduled_txns_OwnerCapability">OwnerCapability</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>cap: <a href="account.md#0x1_account_SignerCapability">account::SignerCapability</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_scheduled_txns_ToRemoveTbl"></a>

## Resource `ToRemoveTbl`



<pre><code><b>struct</b> <a href="scheduled_txns.md#0x1_scheduled_txns_ToRemoveTbl">ToRemoveTbl</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>remove_tbl: <a href="../../aptos-stdlib/doc/table.md#0x1_table_Table">table::Table</a>&lt;u16, <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="scheduled_txns.md#0x1_scheduled_txns_TransactionId">scheduled_txns::TransactionId</a>&gt;&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_scheduled_txns_State"></a>

## Struct `State`



<pre><code><b>struct</b> <a href="scheduled_txns.md#0x1_scheduled_txns_State">State</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>count: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_scheduled_txns_MICRO_CONVERSION_FACTOR"></a>

Conversion factor between our granularity (100ms) and microseconds


<pre><code><b>const</b> <a href="scheduled_txns.md#0x1_scheduled_txns_MICRO_CONVERSION_FACTOR">MICRO_CONVERSION_FACTOR</a>: u64 = 100000;
</code></pre>



<a id="0x1_scheduled_txns_AVG_FUNC_SIZE"></a>



<pre><code><b>const</b> <a href="scheduled_txns.md#0x1_scheduled_txns_AVG_FUNC_SIZE">AVG_FUNC_SIZE</a>: u16 = 64;
</code></pre>



<a id="0x1_scheduled_txns_AVG_SCHED_TXN_SIZE"></a>



<pre><code><b>const</b> <a href="scheduled_txns.md#0x1_scheduled_txns_AVG_SCHED_TXN_SIZE">AVG_SCHED_TXN_SIZE</a>: u16 = 192;
</code></pre>



<a id="0x1_scheduled_txns_BIG_ORDRD_MAP_TGT_ND_SZ"></a>



<pre><code><b>const</b> <a href="scheduled_txns.md#0x1_scheduled_txns_BIG_ORDRD_MAP_TGT_ND_SZ">BIG_ORDRD_MAP_TGT_ND_SZ</a>: u16 = 4096;
</code></pre>



<a id="0x1_scheduled_txns_MAX_FUNC_SIZE"></a>



<pre><code><b>const</b> <a href="scheduled_txns.md#0x1_scheduled_txns_MAX_FUNC_SIZE">MAX_FUNC_SIZE</a>: u16 = 1024;
</code></pre>



<a id="0x1_scheduled_txns_MAX_TXNS_PER_KEY"></a>



<pre><code><b>const</b> <a href="scheduled_txns.md#0x1_scheduled_txns_MAX_TXNS_PER_KEY">MAX_TXNS_PER_KEY</a>: u16 = 30;
</code></pre>



<a id="0x1_scheduled_txns_MILLI_CONVERSION_FACTOR"></a>

Conversion factor between our granularity (100ms) and milliseconds


<pre><code><b>const</b> <a href="scheduled_txns.md#0x1_scheduled_txns_MILLI_CONVERSION_FACTOR">MILLI_CONVERSION_FACTOR</a>: u64 = 100;
</code></pre>



<a id="0x1_scheduled_txns_SCHEDULE_MAP_KEY_SIZE"></a>



<pre><code><b>const</b> <a href="scheduled_txns.md#0x1_scheduled_txns_SCHEDULE_MAP_KEY_SIZE">SCHEDULE_MAP_KEY_SIZE</a>: u16 = 16;
</code></pre>



<a id="0x1_scheduled_txns_SCHEDULE_MAP_LEAF_DEGREE"></a>



<pre><code><b>const</b> <a href="scheduled_txns.md#0x1_scheduled_txns_SCHEDULE_MAP_LEAF_DEGREE">SCHEDULE_MAP_LEAF_DEGREE</a>: u16 = 4;
</code></pre>



<a id="0x1_scheduled_txns_TO_REMOVE_PARALLELISM"></a>



<pre><code><b>const</b> <a href="scheduled_txns.md#0x1_scheduled_txns_TO_REMOVE_PARALLELISM">TO_REMOVE_PARALLELISM</a>: u64 = 32;
</code></pre>



<a id="0x1_scheduled_txns_TXN_ID_SIZE"></a>



<pre><code><b>const</b> <a href="scheduled_txns.md#0x1_scheduled_txns_TXN_ID_SIZE">TXN_ID_SIZE</a>: u16 = 32;
</code></pre>



<a id="0x1_scheduled_txns_U64_MAX"></a>



<pre><code><b>const</b> <a href="scheduled_txns.md#0x1_scheduled_txns_U64_MAX">U64_MAX</a>: u64 = 18446744073709551615;
</code></pre>



<a id="0x1_scheduled_txns_initialize"></a>

## Function `initialize`



<pre><code><b>public</b> <b>fun</b> <a href="scheduled_txns.md#0x1_scheduled_txns_initialize">initialize</a>(framework: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="scheduled_txns.md#0x1_scheduled_txns_initialize">initialize</a>(framework: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>) {
    <a href="system_addresses.md#0x1_system_addresses_assert_aptos_framework">system_addresses::assert_aptos_framework</a>(framework);

    // Create owner <a href="account.md#0x1_account">account</a> for handling deposits
    <b>let</b> owner_addr = @0xb; // Replace <b>with</b> your desired <b>address</b>
    <b>let</b> (owner_signer, owner_cap) = <a href="account.md#0x1_account_create_framework_reserved_account">account::create_framework_reserved_account</a>(owner_addr);

    // Initialize fungible store for the owner
    <b>let</b> metadata = ensure_paired_metadata&lt;AptosCoin&gt;();
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_ensure_primary_store_exists">primary_fungible_store::ensure_primary_store_exists</a>(
        <a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(&owner_signer),
        metadata
    );

    // Store the <a href="../../aptos-stdlib/doc/capability.md#0x1_capability">capability</a>
    <b>move_to</b>(framework, <a href="scheduled_txns.md#0x1_scheduled_txns_OwnerCapability">OwnerCapability</a> { cap: owner_cap });

    // Initialize queue
    <b>let</b> queue = <a href="scheduled_txns.md#0x1_scheduled_txns_ScheduleQueue">ScheduleQueue</a> {
        schedule_map: <a href="big_ordered_map.md#0x1_big_ordered_map_new_with_config">big_ordered_map::new_with_config</a>(
            <a href="scheduled_txns.md#0x1_scheduled_txns_BIG_ORDRD_MAP_TGT_ND_SZ">BIG_ORDRD_MAP_TGT_ND_SZ</a> / <a href="scheduled_txns.md#0x1_scheduled_txns_SCHEDULE_MAP_KEY_SIZE">SCHEDULE_MAP_KEY_SIZE</a>, 4, <b>true</b>),
        txn_tbl: <a href="big_ordered_map.md#0x1_big_ordered_map_new_with_config">big_ordered_map::new_with_config</a>(
            (<a href="scheduled_txns.md#0x1_scheduled_txns_BIG_ORDRD_MAP_TGT_ND_SZ">BIG_ORDRD_MAP_TGT_ND_SZ</a> / <a href="scheduled_txns.md#0x1_scheduled_txns_TXN_ID_SIZE">TXN_ID_SIZE</a>),
            (<a href="scheduled_txns.md#0x1_scheduled_txns_BIG_ORDRD_MAP_TGT_ND_SZ">BIG_ORDRD_MAP_TGT_ND_SZ</a> / (<a href="scheduled_txns.md#0x1_scheduled_txns_TXN_ID_SIZE">TXN_ID_SIZE</a> + <a href="scheduled_txns.md#0x1_scheduled_txns_AVG_SCHED_TXN_SIZE">AVG_SCHED_TXN_SIZE</a>)), <b>true</b>),
    };
    <b>move_to</b>(framework, queue);

    // Aggregator <b>to</b> keep count of how many entries <b>to</b> be removed from the queue
    <b>move_to</b>(framework, <a href="scheduled_txns.md#0x1_scheduled_txns_ToRemoveTbl">ToRemoveTbl</a> {
        remove_tbl: <a href="../../aptos-stdlib/doc/table.md#0x1_table_new">table::new</a>&lt;u16, <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="scheduled_txns.md#0x1_scheduled_txns_TransactionId">TransactionId</a>&gt;&gt;(),
    });
}
</code></pre>



</details>

<a id="0x1_scheduled_txns_insert"></a>

## Function `insert`

Do we need a function to pause ???


<pre><code><b>public</b> <b>fun</b> <a href="scheduled_txns.md#0x1_scheduled_txns_insert">insert</a>(sender: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, txn: <a href="scheduled_txns.md#0x1_scheduled_txns_ScheduledTransaction">scheduled_txns::ScheduledTransaction</a>): <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="scheduled_txns.md#0x1_scheduled_txns_insert">insert</a>(sender: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, txn: <a href="scheduled_txns.md#0x1_scheduled_txns_ScheduledTransaction">ScheduledTransaction</a>): <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; <b>acquires</b> <a href="scheduled_txns.md#0x1_scheduled_txns_ScheduleQueue">ScheduleQueue</a>, <a href="scheduled_txns.md#0x1_scheduled_txns_OwnerCapability">OwnerCapability</a> {
    // we expect the sender <b>to</b> be a permissioned <a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>
    <b>let</b> schedule_txn_signer = <a href="permissioned_signer.md#0x1_permissioned_signer_signer_from_storable_permissioned_handle">permissioned_signer::signer_from_storable_permissioned_handle</a>(&txn.sender_handle);
    <b>assert</b>!(<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(sender) == <a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(&schedule_txn_signer), 1); // todo: throw an <a href="../../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error">error</a> or no-op instead of <b>assert</b>

    // todo: we should limit the size of the scheduled txn ???
    // Generate a unique transaction ID
    <b>let</b> txn_id = <a href="scheduled_txns.md#0x1_scheduled_txns_TransactionId">TransactionId</a> { <a href="../../aptos-stdlib/../move-stdlib/doc/hash.md#0x1_hash">hash</a>: sha3_256(<a href="../../aptos-stdlib/../move-stdlib/doc/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&txn)) };

    <b>let</b> queue = <b>borrow_global_mut</b>&lt;<a href="scheduled_txns.md#0x1_scheduled_txns_ScheduleQueue">ScheduleQueue</a>&gt;(@aptos_framework);
    <b>if</b> (queue.txn_tbl.contains(&txn_id)) {
        <b>return</b> txn_id.<a href="../../aptos-stdlib/../move-stdlib/doc/hash.md#0x1_hash">hash</a>;
    };

    // Only schedule txns in the future
    <b>let</b> txn_time = txn.scheduled_time / <a href="scheduled_txns.md#0x1_scheduled_txns_MILLI_CONVERSION_FACTOR">MILLI_CONVERSION_FACTOR</a>; // Round down <b>to</b> the nearest 100ms
    <b>let</b> block_time = <a href="timestamp.md#0x1_timestamp_now_microseconds">timestamp::now_microseconds</a>() / <a href="scheduled_txns.md#0x1_scheduled_txns_MICRO_CONVERSION_FACTOR">MICRO_CONVERSION_FACTOR</a>;
    <a href="../../aptos-stdlib/doc/debug.md#0x1_debug_print">debug::print</a>(&<a href="timestamp.md#0x1_timestamp_now_microseconds">timestamp::now_microseconds</a>());
    <b>assert</b>!(txn_time &gt; block_time, 2);

    // We need inverse of gas_unit_price for ordering because <a href="scheduled_txns.md#0x1_scheduled_txns_ScheduleMapKey">ScheduleMapKey</a> is sorted in ascending order time
    // first and then on gas_priority
    <b>let</b> gas_priority = <a href="scheduled_txns.md#0x1_scheduled_txns_U64_MAX">U64_MAX</a> - txn.max_gas_unit_price;
    <b>let</b> key = <a href="scheduled_txns.md#0x1_scheduled_txns_ScheduleMapKey">ScheduleMapKey</a> { time: txn_time, gas_priority };
    <a href="../../aptos-stdlib/doc/debug.md#0x1_debug_print">debug::print</a>(&key);

    // Insert the transaction into the schedule_map
    <b>let</b> scheduled_txns_at_key = <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_empty">vector::empty</a>&lt;<a href="scheduled_txns.md#0x1_scheduled_txns_TransactionId">TransactionId</a>&gt;();
    <b>if</b> (queue.schedule_map.contains(&key)) {
        scheduled_txns_at_key = queue.schedule_map.remove(&key);
        <b>assert</b>!(scheduled_txns_at_key.length() &lt; (<a href="scheduled_txns.md#0x1_scheduled_txns_MAX_TXNS_PER_KEY">MAX_TXNS_PER_KEY</a> <b>as</b> u64), 3); // todo: throw an <a href="../../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error">error</a> instead of <b>assert</b>
    };
    scheduled_txns_at_key.push_back(txn_id);
    queue.schedule_map.add(key, scheduled_txns_at_key);

    // Insert the transaction into the txn_tbl
    queue.txn_tbl.add(txn_id, txn);

    // Collect deposit
    // Get owner <a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a> from <a href="../../aptos-stdlib/doc/capability.md#0x1_capability">capability</a>
    <b>let</b> owner_cap = <b>borrow_global</b>&lt;<a href="scheduled_txns.md#0x1_scheduled_txns_OwnerCapability">OwnerCapability</a>&gt;(@aptos_framework);
    <b>let</b> owner_signer = <a href="account.md#0x1_account_create_signer_with_capability">account::create_signer_with_capability</a>(&owner_cap.cap);
    <b>let</b> owner_addr = <a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(&owner_signer);

    // Collect deposit into owner's store
    <a href="../../aptos-stdlib/doc/debug.md#0x1_debug_print">debug::print</a>(&std::string::utf8(b"insert............."));
    <a href="coin.md#0x1_coin_transfer">coin::transfer</a>&lt;AptosCoin&gt;(
        sender,
        owner_addr,
        txn.max_gas_amount * txn.max_gas_unit_price
    );
    txn_id.<a href="../../aptos-stdlib/../move-stdlib/doc/hash.md#0x1_hash">hash</a>
}
</code></pre>



</details>

<a id="0x1_scheduled_txns_cancel"></a>

## Function `cancel`



<pre><code><b>public</b> <b>fun</b> <a href="scheduled_txns.md#0x1_scheduled_txns_cancel">cancel</a>(sender: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, txn_id: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="scheduled_txns.md#0x1_scheduled_txns_cancel">cancel</a>(sender: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, txn_id: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;) <b>acquires</b> <a href="scheduled_txns.md#0x1_scheduled_txns_ScheduleQueue">ScheduleQueue</a>, <a href="scheduled_txns.md#0x1_scheduled_txns_OwnerCapability">OwnerCapability</a> {
    <a href="../../aptos-stdlib/doc/debug.md#0x1_debug_print">debug::print</a>(&std::string::utf8(b"cancel"));
    //<a href="../../aptos-stdlib/doc/debug.md#0x1_debug_print">debug::print</a>(&get_num_txns());
    <b>let</b> queue = <b>borrow_global_mut</b>&lt;<a href="scheduled_txns.md#0x1_scheduled_txns_ScheduleQueue">ScheduleQueue</a>&gt;(@aptos_framework);
    <b>let</b> txn_id = <a href="scheduled_txns.md#0x1_scheduled_txns_TransactionId">TransactionId</a> { <a href="../../aptos-stdlib/../move-stdlib/doc/hash.md#0x1_hash">hash</a>: txn_id };
    <b>if</b> (!queue.txn_tbl.contains(&txn_id)) {
        <b>return</b>;
    };

    <b>let</b> txn = queue.txn_tbl.borrow(&txn_id);
    <b>let</b> deposit_amt = txn.max_gas_amount * txn.max_gas_unit_price;
    // we expect the sender <b>to</b> be a permissioned <a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>
    <b>let</b> schedule_txn_signer = <a href="permissioned_signer.md#0x1_permissioned_signer_signer_from_storable_permissioned_handle">permissioned_signer::signer_from_storable_permissioned_handle</a>(&txn.sender_handle);
    <b>assert</b>!(<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(sender) == <a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(&schedule_txn_signer), 1); // todo: throw an <a href="../../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error">error</a> or no-op instead of <b>assert</b>

    <b>let</b> key = <a href="scheduled_txns.md#0x1_scheduled_txns_ScheduleMapKey">ScheduleMapKey</a> { time: txn.scheduled_time / <a href="scheduled_txns.md#0x1_scheduled_txns_MILLI_CONVERSION_FACTOR">MILLI_CONVERSION_FACTOR</a>, gas_priority: <a href="scheduled_txns.md#0x1_scheduled_txns_U64_MAX">U64_MAX</a> - txn.max_gas_unit_price };
    <a href="../../aptos-stdlib/doc/debug.md#0x1_debug_print">debug::print</a>(&key);
    // Remove the transaction from the txn_tbl
    queue.txn_tbl.remove(&txn_id);

    // Remove the transaction from the schedule_map
    <b>let</b> scheduled_txns_at_key = queue.schedule_map.remove(&key);
    <b>let</b> len = scheduled_txns_at_key.length();
    <b>let</b> idx = 0;
    <b>while</b> (idx &lt; len) {
        <b>if</b> (scheduled_txns_at_key.borrow(idx).<a href="../../aptos-stdlib/../move-stdlib/doc/hash.md#0x1_hash">hash</a> == txn_id.<a href="../../aptos-stdlib/../move-stdlib/doc/hash.md#0x1_hash">hash</a>) {
            scheduled_txns_at_key.remove(idx);
            <b>break</b>
        };
        idx = idx + 1;
    };
    queue.schedule_map.add(key, scheduled_txns_at_key);

    // Refund the deposit
    // Get owner <a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a> from <a href="../../aptos-stdlib/doc/capability.md#0x1_capability">capability</a>
    <b>let</b> owner_cap = <b>borrow_global</b>&lt;<a href="scheduled_txns.md#0x1_scheduled_txns_OwnerCapability">OwnerCapability</a>&gt;(@aptos_framework);
    <b>let</b> owner_signer = <a href="account.md#0x1_account_create_signer_with_capability">account::create_signer_with_capability</a>(&owner_cap.cap);

    // Refund deposit from owner's store <b>to</b> sender
    <a href="../../aptos-stdlib/doc/debug.md#0x1_debug_print">debug::print</a>(&std::string::utf8(b"cancel................"));
    <a href="coin.md#0x1_coin_transfer">coin::transfer</a>&lt;AptosCoin&gt;(
        &owner_signer,
        <a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(sender),
        deposit_amt
    );
}
</code></pre>



</details>

<a id="0x1_scheduled_txns_get_ready_transactions"></a>

## Function `get_ready_transactions`



<pre><code>#[view]
<b>public</b>(<b>friend</b>) <b>fun</b> <a href="scheduled_txns.md#0x1_scheduled_txns_get_ready_transactions">get_ready_transactions</a>(<a href="timestamp.md#0x1_timestamp">timestamp</a>: u64, limit: u64): <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="scheduled_txns.md#0x1_scheduled_txns_ScheduledTransactionWithId">scheduled_txns::ScheduledTransactionWithId</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="scheduled_txns.md#0x1_scheduled_txns_get_ready_transactions">get_ready_transactions</a>(<a href="timestamp.md#0x1_timestamp">timestamp</a>: u64, limit: u64): <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="scheduled_txns.md#0x1_scheduled_txns_ScheduledTransactionWithId">ScheduledTransactionWithId</a>&gt; <b>acquires</b> <a href="scheduled_txns.md#0x1_scheduled_txns_ScheduleQueue">ScheduleQueue</a> {
    <b>let</b> queue = <b>borrow_global</b>&lt;<a href="scheduled_txns.md#0x1_scheduled_txns_ScheduleQueue">ScheduleQueue</a>&gt;(@aptos_framework);
    <b>let</b> block_time = <a href="timestamp.md#0x1_timestamp">timestamp</a> / <a href="scheduled_txns.md#0x1_scheduled_txns_MILLI_CONVERSION_FACTOR">MILLI_CONVERSION_FACTOR</a>;
    <b>let</b> <a href="scheduled_txns.md#0x1_scheduled_txns">scheduled_txns</a> = <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_empty">vector::empty</a>&lt;<a href="scheduled_txns.md#0x1_scheduled_txns_ScheduledTransactionWithId">ScheduledTransactionWithId</a>&gt;();
    <b>let</b> count = 0;
    <b>let</b> iter = queue.schedule_map.new_begin_iter();
    <b>while</b> (!iter.iter_is_end(&queue.schedule_map) && count &lt; limit) {
        <b>let</b> scheduled_key = iter.iter_borrow_key();
        <b>if</b> (scheduled_key.time &gt; block_time) {
            <b>return</b> <a href="scheduled_txns.md#0x1_scheduled_txns">scheduled_txns</a>;
        };
        <b>let</b> txn_ids = iter.iter_borrow(&queue.schedule_map);
        <b>let</b> txn_ids_len = txn_ids.length();
        <b>let</b> idx = 0;
        <b>while</b> (idx &lt; txn_ids_len && count &lt; limit) {
            <b>let</b> txn_id = *txn_ids.borrow(idx);
            <b>let</b> txn = queue.txn_tbl.borrow(&txn_id);
            <b>let</b> txn_with_id = <a href="scheduled_txns.md#0x1_scheduled_txns_ScheduledTransactionWithId">ScheduledTransactionWithId</a> {
                txn: *txn,
                txn_id,
            };
            <a href="scheduled_txns.md#0x1_scheduled_txns">scheduled_txns</a>.push_back(txn_with_id);
            count = count + 1;
            idx = idx + 1;
        };
        iter = iter.iter_next(&queue.schedule_map);
    };
    <a href="scheduled_txns.md#0x1_scheduled_txns">scheduled_txns</a>
}
</code></pre>



</details>

<a id="0x1_scheduled_txns_finish_execution"></a>

## Function `finish_execution`

Increment after every scheduled transaction is run
IMP: Make sure this does not affect parallel execution of txns


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="scheduled_txns.md#0x1_scheduled_txns_finish_execution">finish_execution</a>(txn_id: <a href="scheduled_txns.md#0x1_scheduled_txns_TransactionId">scheduled_txns::TransactionId</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="scheduled_txns.md#0x1_scheduled_txns_finish_execution">finish_execution</a>(txn_id: <a href="scheduled_txns.md#0x1_scheduled_txns_TransactionId">TransactionId</a>) <b>acquires</b> <a href="scheduled_txns.md#0x1_scheduled_txns_ToRemoveTbl">ToRemoveTbl</a> {
    // Get first 8 bytes of the <a href="../../aptos-stdlib/../move-stdlib/doc/hash.md#0x1_hash">hash</a> <b>as</b> u64 and then mod
    <b>let</b> hash_bytes = txn_id.<a href="../../aptos-stdlib/../move-stdlib/doc/hash.md#0x1_hash">hash</a>;
    <b>assert</b>!(hash_bytes.length() == 32, 1); // SHA3-256 produces 32 bytes

    // Take first 8 bytes and convert <b>to</b> u64
    // Take first 8 bytes and convert <b>to</b> u64
    <b>let</b> value = ((hash_bytes[0] <b>as</b> u64) &lt;&lt; 56) |
        ((hash_bytes[1] <b>as</b> u64) &lt;&lt; 48) |
        ((hash_bytes[2] <b>as</b> u64) &lt;&lt; 40) |
        ((hash_bytes[3] <b>as</b> u64) &lt;&lt; 32) |
        ((hash_bytes[4] <b>as</b> u64) &lt;&lt; 24) |
        ((hash_bytes[5] <b>as</b> u64) &lt;&lt; 16) |
        ((hash_bytes[6] <b>as</b> u64) &lt;&lt; 8) |
        (hash_bytes[7] <b>as</b> u64);

    // todo: check <b>if</b> it is efficient <b>to</b> compute tbl_idx in rust instead
    <b>let</b> tbl_idx = ((value % <a href="scheduled_txns.md#0x1_scheduled_txns_TO_REMOVE_PARALLELISM">TO_REMOVE_PARALLELISM</a>) <b>as</b> u16);
    <b>let</b> to_remove = <b>borrow_global_mut</b>&lt;<a href="scheduled_txns.md#0x1_scheduled_txns_ToRemoveTbl">ToRemoveTbl</a>&gt;(@aptos_framework);

    <b>if</b> (!to_remove.remove_tbl.contains(tbl_idx)) {
        <b>let</b> txn_ids = <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_empty">vector::empty</a>&lt;<a href="scheduled_txns.md#0x1_scheduled_txns_TransactionId">TransactionId</a>&gt;();
        txn_ids.push_back(txn_id);
        to_remove.remove_tbl.add(tbl_idx, txn_ids);
    } <b>else</b> {
        <b>let</b> txn_ids = to_remove.remove_tbl.borrow_mut(tbl_idx);
        txn_ids.push_back(txn_id);
    };
}
</code></pre>



</details>

<a id="0x1_scheduled_txns_remove_txns"></a>

## Function `remove_txns`

Remove the txns that are run


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="scheduled_txns.md#0x1_scheduled_txns_remove_txns">remove_txns</a>()
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="scheduled_txns.md#0x1_scheduled_txns_remove_txns">remove_txns</a>() <b>acquires</b> <a href="scheduled_txns.md#0x1_scheduled_txns_ToRemoveTbl">ToRemoveTbl</a>, <a href="scheduled_txns.md#0x1_scheduled_txns_ScheduleQueue">ScheduleQueue</a> {
    <b>let</b> to_remove = <b>borrow_global_mut</b>&lt;<a href="scheduled_txns.md#0x1_scheduled_txns_ToRemoveTbl">ToRemoveTbl</a>&gt;(@aptos_framework);
    <b>let</b> queue = <b>borrow_global_mut</b>&lt;<a href="scheduled_txns.md#0x1_scheduled_txns_ScheduleQueue">ScheduleQueue</a>&gt;(@aptos_framework);
    <b>let</b> idx: u16 = 0;

    <b>while</b> ((idx <b>as</b> u64) &lt; <a href="scheduled_txns.md#0x1_scheduled_txns_TO_REMOVE_PARALLELISM">TO_REMOVE_PARALLELISM</a>) {
        <b>if</b> (to_remove.remove_tbl.contains(idx)) {
            <b>let</b> txn_ids = to_remove.remove_tbl.remove(idx);
            <b>let</b> txn_ids_len = txn_ids.length();
            <b>let</b> txn_idx = 0;

            <b>while</b> (txn_idx &lt; txn_ids_len) {
                <b>let</b> txn_id = *txn_ids.borrow(txn_idx);
                // Get transaction data before removing it
                <b>let</b> txn = queue.txn_tbl.borrow(&txn_id);
                <b>let</b> key = <a href="scheduled_txns.md#0x1_scheduled_txns_ScheduleMapKey">ScheduleMapKey</a> {
                    time: txn.scheduled_time / <a href="scheduled_txns.md#0x1_scheduled_txns_MILLI_CONVERSION_FACTOR">MILLI_CONVERSION_FACTOR</a>,
                    gas_priority: <a href="scheduled_txns.md#0x1_scheduled_txns_U64_MAX">U64_MAX</a> - txn.max_gas_unit_price
                };

                // Remove transaction from txn_tbl
                queue.txn_tbl.remove(&txn_id);

                // Remove transaction from schedule_map
                <b>let</b> <a href="scheduled_txns.md#0x1_scheduled_txns">scheduled_txns</a> = queue.schedule_map.remove(&key);
                <b>let</b> mut_len = <a href="scheduled_txns.md#0x1_scheduled_txns">scheduled_txns</a>.length();
                <b>let</b> mut_idx = 0;
                <b>let</b> remaining_txns = <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_empty">vector::empty</a>();

                <b>while</b> (mut_idx &lt; mut_len) {
                    <b>if</b> (<a href="scheduled_txns.md#0x1_scheduled_txns">scheduled_txns</a>.borrow(mut_idx).<a href="../../aptos-stdlib/../move-stdlib/doc/hash.md#0x1_hash">hash</a> != txn_id.<a href="../../aptos-stdlib/../move-stdlib/doc/hash.md#0x1_hash">hash</a>) {
                        remaining_txns.push_back(*<a href="scheduled_txns.md#0x1_scheduled_txns">scheduled_txns</a>.borrow(mut_idx));
                    };
                    mut_idx = mut_idx + 1;
                };

                // Add back non-empty vectors <b>to</b> schedule_map
                <b>if</b> (!remaining_txns.is_empty()) {
                    queue.schedule_map.add(key, remaining_txns);
                };
                txn_idx = txn_idx + 1;
            };
        };
        idx = idx + 1;
    };
}
</code></pre>



</details>

<a id="0x1_scheduled_txns_step"></a>

## Function `step`



<pre><code>#[persistent]
<b>fun</b> <a href="scheduled_txns.md#0x1_scheduled_txns_step">step</a>(state: <a href="scheduled_txns.md#0x1_scheduled_txns_State">scheduled_txns::State</a>, val: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="scheduled_txns.md#0x1_scheduled_txns_step">step</a>(state: <a href="scheduled_txns.md#0x1_scheduled_txns_State">State</a>, val: u64) {
    <b>if</b> (state.count &lt; 10) {
        state.count = state.count + 1;
        <a href="../../aptos-stdlib/doc/debug.md#0x1_debug_print">debug::print</a>(&state.count);
    }
}
</code></pre>



</details>


[move-book]: https://aptos.dev/move/book/SUMMARY
