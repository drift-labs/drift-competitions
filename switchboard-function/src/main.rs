use std::str::FromStr;

pub use switchboard_solana::get_ixn_discriminator;
pub use switchboard_solana::prelude::*;

mod params;
pub use params::*;

#[tokio::main(worker_threads = 12)]
async fn main() {
    // First, initialize the runner instance with a freshly generated Gramine keypair
    let runner = FunctionRunner::new_from_cluster(Cluster::Devnet, None).unwrap();

    // parse and validate user provided request params
    let params = ContainerParams::decode(
        &runner
            .function_request_data
            .as_ref()
            .unwrap()
            .container_params,
    )
    .unwrap();

    // Generate our random result
    let winner_result = generate_randomness(params.min_result, params.max_result);
    let mut winner_result_bytes = winner_result.to_le_bytes().to_vec();

    let prize_result = generate_randomness(params.min_result, params.max_result);
    let mut prize_result_bytes = prize_result.to_le_bytes().to_vec();

    // IXN DATA:
    // LEN: 12 bytes
    // [0-8]: Anchor Ixn Discriminator
    // [9-12]: Random Result as u32
    let mut ixn_data = get_ixn_discriminator("receive_randomness").to_vec();
    ixn_data.append(&mut winner_result_bytes);
    ixn_data.append(&mut prize_result_bytes);

    // ACCOUNTS:
    // 1. Competition (mut)
    // 2. Switchboard Function
    // 3. Switchboard Function Request
    // 4. Enclave Signer (signer): our Gramine generated keypair
    let receive_randomness_ixn = Instruction {
        program_id: params.program_id,
        data: ixn_data,
        accounts: vec![
            AccountMeta::new(params.competition_key, false),
            AccountMeta::new_readonly(runner.function, false),
            AccountMeta::new_readonly(runner.function_request_key.unwrap(), false),
            AccountMeta::new_readonly(runner.signer, true),
        ],
    };

    // Then, write your own Rust logic and build a Vec of instructions.
    // Should  be under 700 bytes after serialization
    let ixs: Vec<solana_program::instruction::Instruction> = vec![receive_randomness_ixn];

    // Finally, emit the signed quote and partially signed transaction to the functionRunner oracle
    // The functionRunner oracle will use the last outputted word to stdout as the serialized result. This is what gets executed on-chain.
    runner.emit(ixs).await.unwrap();
}

fn generate_randomness(min: u32, max: u32) -> u32 {
    if min == max {
        return min;
    }
    if min > max {
        return generate_randomness(max, min);
    }

    // We add one so its inclusive [min, max]
    let window = (max + 1) - min;

    let mut bytes: [u8; 4] = [0u8; 4];
    Gramine::read_rand(&mut bytes).expect("gramine failed to generate randomness");
    let raw_result: &[u32] = bytemuck::cast_slice(&bytes[..]);

    (raw_result[0] % window) + min
}

#[cfg(test)]
mod tests {
    use super::*;

    // 1. Check when lower_bound is greater than upper_bound
    #[test]
    fn test_generate_randomness_with_flipped_bounds() {
        let min = 100;
        let max = 50;

        let result = generate_randomness(100, 50);
        assert!(result >= max && result < min);
    }

    // 2. Check when lower_bound is equal to upper_bound
    #[test]
    fn test_generate_randomness_with_equal_bounds() {
        let bound = 100;
        assert_eq!(generate_randomness(bound, bound), bound);
    }

    // 3. Test within a range
    #[test]
    fn test_generate_randomness_within_bounds() {
        let min = 100;
        let max = 200;

        let result = generate_randomness(min, max);

        assert!(result >= min && result < max);
    }

    // 4. Test randomness distribution (not truly deterministic, but a sanity check)
    #[test]
    fn test_generate_randomness_distribution() {
        let min = 0;
        let max = 9;

        let mut counts = vec![0; 10];
        for _ in 0..1000 {
            let result = generate_randomness(min, max);
            let index: usize = result as usize;
            counts[index] += 1;
        }

        // Ensure all counts are non-zero (probabilistically should be the case)
        for count in counts.iter() {
            assert!(*count > 0);
        }
    }
}
