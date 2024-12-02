// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use aptos_native_interface::{
    safely_pop_arg, RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeResult,
};
use aptos_types::dkg::real_dkg::rounding::DKGRounding;
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{
    loaded_data::runtime_types::Type,
    values::{Struct, Value},
};
use smallvec::{smallvec, SmallVec};
use std::collections::vec_deque::VecDeque;

pub fn make_all(
    builder: &SafeNativeBuilder,
) -> impl Iterator<Item = (String, NativeFunction)> + '_ {
    let mut natives = vec![];

    natives.extend([
        // BLS over BLS12-381
        (
            "rounding_v0_internal",
            rounding_v0_internal as RawSafeNative,
        ),
    ]);

    builder.make_named_natives(natives)
}

use fixed::types::U64F64;

pub fn rounding_v0_internal(
    _context: &mut SafeNativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let fast_secrecy_thresh_raw = safely_pop_arg!(arguments, u128);
    let recon_thresh_raw = safely_pop_arg!(arguments, u128);
    let secrecy_thresh_raw = safely_pop_arg!(arguments, u128);
    let stakes = safely_pop_arg!(arguments, Vec<u64>);
    let secrecy_thresh = U64F64::from_bits(secrecy_thresh_raw);
    let recon_thresh = U64F64::from_bits(recon_thresh_raw);
    let fast_secrecy_thresh = if fast_secrecy_thresh_raw == 0 {
        None
    } else {
        Some(U64F64::from_bits(fast_secrecy_thresh_raw))
    };
    let result = DKGRounding::new(&stakes, secrecy_thresh, recon_thresh, fast_secrecy_thresh);
    Ok(smallvec![Value::struct_(Struct::pack(vec![
        Value::u64(result.profile.ideal_total_weight),
        Value::vector_u64(result.profile.validator_weights.clone()),
        Value::u128(result.profile.reconstruct_threshold_in_weights as u128),
        Value::struct_(Struct::pack(
            result
                .profile
                .fast_reconstruct_threshold_in_weights
                .map(|x| Value::u128(x as u128))
                .into_iter()
                .collect::<Vec<_>>()
        )),
    ]))])
}
