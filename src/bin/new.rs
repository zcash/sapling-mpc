extern crate pairing;
extern crate phase2;
extern crate sapling_crypto;

use std::fs::File;
use std::io::BufWriter;

fn main() {
    let jubjub_params = sapling_crypto::jubjub::JubjubBls12::new();

    let params = File::create("params").unwrap();
    let mut params = BufWriter::with_capacity(1024 * 1024, params);

    // Sapling spend circuit
    phase2::MPCParameters::new(sapling_crypto::circuit::sapling::Spend {
        params: &jubjub_params,
        value_commitment: None,
        proof_generation_key: None,
        payment_address: None,
        commitment_randomness: None,
        ar: None,
        auth_path: vec![None; 32], // Tree depth is 32 for sapling
        anchor: None,
    })
    .unwrap()
    .write(&mut params)
    .unwrap();

    // Sapling output circuit
    phase2::MPCParameters::new(sapling_crypto::circuit::sapling::Output {
        params: &jubjub_params,
        value_commitment: None,
        payment_address: None,
        commitment_randomness: None,
        esk: None,
    })
    .unwrap()
    .write(&mut params)
    .unwrap();

    // Sprout joinsplit circuit
    phase2::MPCParameters::new(sapling_crypto::circuit::sprout::JoinSplit {
        vpub_old: None,
        vpub_new: None,
        h_sig: None,
        phi: None,
        inputs: vec![
            sapling_crypto::circuit::sprout::JSInput {
                value: None,
                a_sk: None,
                rho: None,
                r: None,
                auth_path: [None; 29], // Depth is 29 for Sprout
            },
            sapling_crypto::circuit::sprout::JSInput {
                value: None,
                a_sk: None,
                rho: None,
                r: None,
                auth_path: [None; 29], // Depth is 29 for Sprout
            },
        ],
        outputs: vec![
            sapling_crypto::circuit::sprout::JSOutput {
                value: None,
                a_pk: None,
                r: None,
            },
            sapling_crypto::circuit::sprout::JSOutput {
                value: None,
                a_pk: None,
                r: None,
            },
        ],
        rt: None,
    })
    .unwrap()
    .write(&mut params)
    .unwrap();
}
