use crate::*;

pub struct ContainerParams {
    pub program_id: Pubkey,
    pub min_result: u32,
    pub max_result: u32,
    pub user_key: Pubkey,
}

impl ContainerParams {
    pub fn decode(container_params: &Vec<u8>) -> std::result::Result<Self, SwitchboardClientError> {
        let params = String::from_utf8(container_params.clone()).unwrap();

        let mut program_id: Pubkey = Pubkey::default();
        let mut min_result: u32 = 0;
        let mut max_result: u32 = 0;
        let mut user_key: Pubkey = Pubkey::default();

        for env_pair in params.split(',') {
            let pair: Vec<&str> = env_pair.splitn(2, '=').collect();
            if pair.len() == 2 {
                match pair[0] {
                    "PID" => program_id = Pubkey::from_str(pair[1]).unwrap(),
                    "MIN_RESULT" => min_result = pair[1].parse::<u32>().unwrap(),
                    "MAX_RESULT" => max_result = pair[1].parse::<u32>().unwrap(),
                    "USER" => user_key = Pubkey::from_str(pair[1]).unwrap(),
                    _ => {}
                }
            }
        }

        if program_id == Pubkey::default() {
            return Err(SwitchboardClientError::CustomMessage(
                "PID cannot be undefined".to_string(),
            ));
        }
        if max_result == 0 {
            return Err(SwitchboardClientError::CustomMessage(
                "MAX_GUESS must be greater than 0".to_string(),
            ));
        }
        if user_key == Pubkey::default() {
            return Err(SwitchboardClientError::CustomMessage(
                "USER_KEY cannot be undefined".to_string(),
            ));
        }

        Ok(Self {
            program_id,
            min_result,
            max_result,
            user_key,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_params_decode() {
        let request_params_string = format!(
            "PID={},MIN_GUESS={},MAX_GUESS={},USER={}",
            anchor_spl::token::ID,
            1,
            6,
            anchor_spl::token::ID
        );
        let request_params_bytes = request_params_string.into_bytes();

        let params = ContainerParams::decode(&request_params_bytes).unwrap();

        assert_eq!(params.program_id, anchor_spl::token::ID);
        assert_eq!(params.min_result, 1);
        assert_eq!(params.max_result, 6);
        assert_eq!(params.user_key, anchor_spl::token::ID);
    }
}
