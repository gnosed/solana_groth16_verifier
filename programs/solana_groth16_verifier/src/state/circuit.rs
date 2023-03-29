use anchor_lang::prelude::*;
use num_derive::*;
use num_traits::*;

use ark_bn254::*;
use ark_ec::*;
use ark_ff::bytes::{FromBytes, ToBytes};
use std::ops::Neg;

#[account]
pub struct Circuit {
    verifying_key: Groth16Verifyingkey,
    public_inputs: [u8; 9 * 32],
}

impl Circuit {
    pub fn setup(
        &mut self,
        verifying_key: Groth16Verifyingkey,
        public_inputs: [u8; 9 * 32],
    ) -> Result<()> {
        self.verifying_key = verifying_key;
        self.public_inputs = public_inputs;

        Ok(());
    }

    type G1 = ark_ec::short_weierstrass_jacobian::GroupAffine<ark_bn254::g1::Parameters>;

    fn change_endianness(bytes: &[u8]) -> Vec<u8> {
        let mut vec = Vec::new();
        for b in bytes.chunks(32) {
            for byte in b.iter().rev() {
                vec.push(*byte);
            }
        }
        vec
    }

    pub fn verify_proof(proof: [u8; 256]) -> Result<()> {
        let mut public_inputs_vec = Vec::new();

        for input in self.public_inputs.chunks(32) {
            public_inputs_vec.push(input);
        }

        let proof_a: G1 =
            <G1 as FromBytes>::read(&*[&change_endianness(&proof[0..64])[..], &[0u8][..]].concat())
                .unwrap();
        let mut proof_a_neg = [0u8; 65];
        <G1 as ToBytes>::write(&proof_a.neg(), &mut proof_a_neg[..]).unwrap();

        let proof_a = change_endianness(&proof_a_neg[..64]).try_into().unwrap();
        let proof_b = proof[64..192].try_into().unwrap();
        let proof_c = proof[192..256].try_into().unwrap();

        let mut verifier = Groth16Verifier::new(
            &proof_a,
            &proof_b,
            &proof_c,
            public_inputs_vec.as_slice(),
            &self.verifying_key,
        )
        .unwrap();
        verifier.verify().unwrap();
    }
}