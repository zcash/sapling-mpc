extern crate blake2_rfc;
extern crate dirs;
extern crate pairing;
extern crate phase2;
extern crate sapling_crypto;

use blake2_rfc::blake2b::Blake2b;
use phase2::MPCParameters;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::PathBuf;

fn load_combined_params(params: PathBuf) -> (MPCParameters, MPCParameters, MPCParameters) {
    let params = File::open(params).expect("couldn't open combined params file");
    let mut params = BufReader::with_capacity(1024 * 1024, params);

    let sapling_spend = phase2::MPCParameters::read(&mut params, true)
        .expect("couldn't deserialize Sapling Spend params");

    let sapling_output = phase2::MPCParameters::read(&mut params, true)
        .expect("couldn't deserialize Sapling Output params");

    let sprout_joinsplit = phase2::MPCParameters::read(&mut params, true)
        .expect("couldn't deserialize Sprout JoinSplit params");

    (sapling_spend, sapling_output, sprout_joinsplit)
}

fn load_one(params: PathBuf) -> io::Result<MPCParameters> {
    println!("Loading {}", params.to_string_lossy());
    let params = File::open(params)?;
    let mut params = BufReader::with_capacity(1024 * 1024, params);
    phase2::MPCParameters::read(&mut params, true)
}

fn load_individual_params(
    params_dir: &PathBuf,
) -> io::Result<(MPCParameters, MPCParameters, MPCParameters)> {
    let sapling_spend = load_one(params_dir.join("sapling-spend.params"))?;
    let sapling_output = load_one(params_dir.join("sapling-output.params"))?;
    let sprout_groth16 = load_one(params_dir.join("sprout-groth16.params"))?;
    Ok((sapling_spend, sapling_output, sprout_groth16))
}

fn main() {
    let jubjub_params = sapling_crypto::jubjub::JubjubBls12::new();

    let combined_params = PathBuf::from("params");
    let unix_params_dir = dirs::home_dir().map(|path| path.join(".zcash-params"));
    let win_osx_params_dir = dirs::data_dir().map(|path| path.join("ZcashParams"));
    let (sapling_spend, sapling_output, sprout_joinsplit) = if combined_params.exists() {
        load_combined_params(combined_params)
    } else {
        match (unix_params_dir, win_osx_params_dir) {
            (Some(ref params_dir), _) if params_dir.exists() => {
                match load_individual_params(params_dir) {
                    Ok(params) => params,
                    Err(e) => {
                        println!("Failed to load parameters: {}", e);
                        return;
                    }
                }
            }
            (_, Some(ref params_dir)) if params_dir.exists() => {
                match load_individual_params(params_dir) {
                    Ok(params) => params,
                    Err(e) => {
                        println!("Failed to load parameters: {}", e);
                        return;
                    }
                }
            }
            _ => {
                println!("Cannot locate the Zcash parameters.");
                println!("Please run zcash-fetch-params or fetch-params.sh to download the parameters, and then re-run this tool.");
                return;
            }
        }
    };

    println!("Verifying Sapling spend parameters...");
    let sapling_spend_contributions = sapling_spend
        .verify(sapling_crypto::circuit::sapling::Spend {
            params: &jubjub_params,
            value_commitment: None,
            proof_generation_key: None,
            payment_address: None,
            commitment_randomness: None,
            ar: None,
            auth_path: vec![None; 32], // Tree depth is 32 for sapling
            anchor: None,
        })
        .expect("parameters are invalid");

    println!("Verifying Sapling output parameters...");
    let sapling_output_contributions = sapling_output
        .verify(sapling_crypto::circuit::sapling::Output {
            params: &jubjub_params,
            value_commitment: None,
            payment_address: None,
            commitment_randomness: None,
            esk: None,
        })
        .expect("parameters are invalid");

    println!("Verifying Sprout parameters...");
    let sprout_joinsplit_contributions = sprout_joinsplit
        .verify(sapling_crypto::circuit::sprout::JoinSplit {
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
        .expect("parameters are invalid");

    println!("Parameters verified! Here is the list of contribution hashes:");
    for ((a, b), c) in sapling_spend_contributions
        .into_iter()
        .zip(sapling_output_contributions.into_iter())
        .zip(sprout_joinsplit_contributions)
    {
        let mut h = Blake2b::new(64);
        h.update(&a);
        h.update(&b);
        h.update(&c);
        let h = h.finalize();

        println!("{}", into_hex(h.as_ref()));
    }
}

fn into_hex(h: &[u8]) -> String {
    let mut f = String::new();

    for byte in &h[..] {
        f += &format!("{:02x}", byte);
    }

    f
}
