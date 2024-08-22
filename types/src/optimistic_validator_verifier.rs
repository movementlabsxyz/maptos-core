// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::{
    account_address::AccountAddress, aggregate_signature::{AggregateSignature, PartialSignatures}, ledger_info::LedgerInfo, validator_verifier::{ValidatorVerifier, VerifyError}
};
use anyhow::Result;
use aptos_crypto::bls12381;
// use rayon::iter::IntoParallelRefIterator;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::Serialize;
use std::{
    collections::HashMap,
    sync::Arc,
};
use lru::LruCache;

pub enum VerificationResult<VoteType> {
    Verified((HashMap<AccountAddress, VoteType>, AggregateSignature)),
    NotEnoughVotes,
    AggregatedBefore,
    DuplicateVote,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct SignatureData<VoteType> {
    // Question: Should we allow multiple votes from the same author?
    unverified_votes: HashMap<AccountAddress, (bls12381::Signature, VoteType)>,
    verified_votes: HashMap<AccountAddress, VoteType>,
    // The above verified votes are aggregated into this signature
    aggregated_signature: Option<AggregateSignature>,
    // Timestamp at which the first vote was received for this message
    first_vote_timestamp_usecs: u64,
    // Timestamp at which the last vote was received for this message
    last_vote_timestamp_usecs: u64,
}

#[derive(Debug)]
pub struct OptimisticValidatorVerifier<VoteType> {
    validator_verifier: Arc<ValidatorVerifier>,
    vote_data: HashMap<LedgerInfo, SignatureData<VoteType>>,
    // Cache of the most recent aggregated messages. If more votes are received for these messages, 
    // we can ignore the votes.
    recent_aggregated_blocks: LruCache<LedgerInfo, ()>,
    verification_frequency: u64,
}

// TODO: How does garbage collection happen?
// TODO: How do we handle when a vote verification fails and a validator becomes untrusted?
// TODO: After an aggregate signature is formed for a message, should we remove immediately? How to handle the next set of votes received for the same message?
// TODO: Need to make sure the verificaiton can be done in parallel. This may not be the case when having mut signature_data.

impl<VoteType: Sized + Clone + PartialEq> OptimisticValidatorVerifier<VoteType> {
    pub fn new(
        validator_verifier: Arc<ValidatorVerifier>,
        verification_frequency: u64,
    ) -> Self {
        Self {
            validator_verifier,
            vote_data: HashMap::new(),
            recent_aggregated_blocks: LruCache::new(50),
            verification_frequency,
        }
    }

    pub async fn verify(
        &mut self,
        author: AccountAddress,
        block: &LedgerInfo,
        signature: &bls12381::Signature,
        vote: &VoteType,
    ) -> Result<VerificationResult<VoteType>, VerifyError> {
        // Check if the block is already present in recent_aggregated_blocks
        if self.recent_aggregated_blocks.contains(block) {
            return Ok(VerificationResult::AggregatedBefore);
        }

        if self.validator_verifier.get_voting_power(&author).is_none() {
            return Err(VerifyError::UnknownAuthor);
        }
        
        let signature_data = self.vote_data.entry(block.clone()).or_insert_with(|| SignatureData {
            unverified_votes: HashMap::new(),
            verified_votes: HashMap::new(),
            aggregated_signature: None,
            first_vote_timestamp_usecs: aptos_infallible::duration_since_epoch().as_micros() as u64,
            last_vote_timestamp_usecs: aptos_infallible::duration_since_epoch().as_micros() as u64,
        });
        
        // Check if a verified signature is already received for the author.
        if signature_data.verified_votes.contains_key(&author) {
            return Ok(VerificationResult::DuplicateVote);
        }

        // If there is an unverified signature from the author, check if the new signature is the same.
        if signature_data.verified_votes.contains_key(&author) {
            let (old_signature, old_vote) = signature_data.unverified_votes.get(&author).unwrap();
            if old_signature == signature && *old_vote == *vote {
                return Ok(VerificationResult::DuplicateVote);
            } else if old_signature != signature {
                // Verify both the signatures
            }
            return Ok(VerificationResult::DuplicateVote);
        }

        signature_data.unverified_votes.insert(author, (signature.clone(), vote.clone()));

        // If there are enough votes, aggregate the unverified votes and verify the signature.
        let voted_authors = signature_data.verified_votes.keys().chain(signature_data.unverified_votes.keys());
        let has_enough_voting_power = self.validator_verifier.check_voting_power(voted_authors, true).is_ok();
        if has_enough_voting_power || signature_data.unverified_votes.len() as u64 >= self.verification_frequency {
            let aggregated_signature = self.validator_verifier.aggregate_signatures(
                &PartialSignatures::new(signature_data.unverified_votes.iter().map(|(account_address, (signature, _))| (*account_address, signature.clone())).collect()),
                signature_data.aggregated_signature.clone()
            )?;
            match self.validator_verifier.verify_multi_signatures(block, &aggregated_signature) {
                Ok(_) => {
                    signature_data.aggregated_signature = Some(aggregated_signature.clone());
                    signature_data.verified_votes.extend(signature_data.unverified_votes.iter().map(|(account_address, (_signature, vote))| (*account_address, vote.clone())).collect::<Vec<_>>());
                    signature_data.unverified_votes.clear();
                },
                Err(err) => {
                    // TODO: Need to return/print this error.
                    println!("Failed to verify aggregated signature {:?}", err);
                    let unverified_signatures = signature_data.unverified_votes.iter().map(|(account_address, (signature, _vote))| (*account_address, signature.clone())).collect::<Vec<_>>();
                    let verified_votes = unverified_signatures.into_par_iter().flat_map(|(account_address, signature)| {
                        match self.validator_verifier.verify(account_address, block, &signature) {
                            Ok(_) => Some((account_address, signature)),
                            Err(_) => None,
                        }
                    }).collect::<Vec<_>>();
                    let aggregated_signature = self.validator_verifier.aggregate_signatures(
                        &PartialSignatures::new(verified_votes.iter().cloned().collect()),
                        signature_data.aggregated_signature.clone()
                    )?;
                    signature_data.aggregated_signature = Some(aggregated_signature.clone());
                    for (author, _) in verified_votes {
                        let (_, vote) = signature_data.unverified_votes.remove(&author).unwrap();
                        signature_data.verified_votes.insert(author, vote);
                    }
                    signature_data.unverified_votes.clear();
                }
            }

            if self.validator_verifier.check_voting_power(signature_data.verified_votes.keys(), true).is_ok() {
                self.recent_aggregated_blocks.put(block.clone(), ());
                return Ok(VerificationResult::Verified((signature_data.verified_votes.clone(), aggregated_signature)));
            } else {
                return Ok(VerificationResult::NotEnoughVotes);
            }
        }
        Ok(VerificationResult::NotEnoughVotes)
    }
}
