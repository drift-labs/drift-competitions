use crate::*;

pub struct ContainerParams {
    pub program_id: Pubkey,
    pub winner_min_result: u128,
    pub winner_max_result: u128,
    pub prize_min_result: u128,
    pub prize_max_result: u128,
    pub competition_key: Pubkey,
    pub spot_market_key: Pubkey,
    pub if_vault_key: Pubkey,
}

impl ContainerParams {
    pub fn decode(container_params: &Vec<u8>) -> std::result::Result<Self, SwitchboardClientError> {
        let params = String::from_utf8(container_params.clone()).unwrap();

        let mut program_id: Pubkey = Pubkey::default();
        let mut winner_min_result: u128 = 0;
        let mut winner_max_result: u128 = 0;
        let mut prize_min_result: u128 = 0;
        let mut prize_max_result: u128 = 0;
        let mut competition_key: Pubkey = Pubkey::default();
        let mut spot_market_key: Pubkey = Pubkey::default();
        let mut if_vault_key: Pubkey = Pubkey::default();

        for env_pair in params.split(',') {
            let pair: Vec<&str> = env_pair.splitn(2, '=').collect();
            if pair.len() == 2 {
                match pair[0] {
                    "PID" => program_id = Pubkey::from_str(pair[1]).unwrap(),
                    "WINNER_MIN" => winner_min_result = pair[1].parse::<u128>().unwrap(),
                    "WINNER_MAX" => winner_max_result = pair[1].parse::<u128>().unwrap(),
                    "PRIZE_MIN" => prize_min_result = pair[1].parse::<u128>().unwrap(),
                    "PRIZE_MAX" => prize_max_result = pair[1].parse::<u128>().unwrap(),
                    "COMPETITION" => competition_key = Pubkey::from_str(pair[1]).unwrap(),
                    "SPOT_MARKET" => spot_market_key = Pubkey::from_str(pair[1]).unwrap(),
                    "IF_VAULT" => if_vault_key = Pubkey::from_str(pair[1]).unwrap(),
                    _ => {}
                }
            }
        }

        if program_id == Pubkey::default() {
            return Err(SwitchboardClientError::CustomMessage(
                "PID cannot be undefined".to_string(),
            ));
        }
        if winner_max_result == 0 {
            return Err(SwitchboardClientError::CustomMessage(
                "WINNER_MAX must be greater than 0".to_string(),
            ));
        }
        if prize_max_result == 0 {
            return Err(SwitchboardClientError::CustomMessage(
                "PRIZE_MAX must be greater than 0".to_string(),
            ));
        }
        if competition_key == Pubkey::default() {
            return Err(SwitchboardClientError::CustomMessage(
                "COMPETITION cannot be undefined".to_string(),
            ));
        }

        if spot_market_key == Pubkey::default() {
            return Err(SwitchboardClientError::CustomMessage(
                "SPOT_MARKET cannot be undefined".to_string(),
            ));
        }

        if if_vault_key == Pubkey::default() {
            return Err(SwitchboardClientError::CustomMessage(
                "IF_VAULT cannot be undefined".to_string(),
            ));
        }

        Ok(Self {
            program_id,
            winner_min_result,
            winner_max_result,
            prize_min_result,
            prize_max_result,
            competition_key,
            spot_market_key,
            if_vault_key,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_params_decode() {
        let request_params_string = format!(
            "PID={},WINNER_MIN={},WINNER_MAX={},PRIZE_MIN={},PRIZE_MAX={},COMPETITION={},SPOT_MARKET={},IF_VAULT={}",
            anchor_spl::token::ID,
            1,
            6,
            1,
            6,
            anchor_spl::token::ID,
            anchor_spl::token::ID,
            anchor_spl::token::ID
        );
        let request_params_bytes = request_params_string.into_bytes();

        let params = ContainerParams::decode(&request_params_bytes).unwrap();

        assert_eq!(params.program_id, anchor_spl::token::ID);
        assert_eq!(params.winner_min_result, 1);
        assert_eq!(params.winner_max_result, 6);
        assert_eq!(params.prize_min_result, 1);
        assert_eq!(params.prize_max_result, 6);
        assert_eq!(params.competition_key, anchor_spl::token::ID);
        assert_eq!(params.spot_market_key, anchor_spl::token::ID);
        assert_eq!(params.if_vault_key, anchor_spl::token::ID);
    }
}
