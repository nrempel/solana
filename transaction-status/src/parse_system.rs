use {
    crate::parse_instruction::{
        check_num_accounts, ParsableProgram, ParseInstructionError, ParsedInstructionEnum,
    },
    bincode::deserialize,
    serde_json::json,
    solana_sdk::{
        instruction::CompiledInstruction, pubkey::Pubkey, system_instruction::SystemInstruction,
    },
};

pub fn parse_system(
    instruction: &CompiledInstruction,
    account_keys: &[Pubkey],
) -> Result<ParsedInstructionEnum, ParseInstructionError> {
    let system_instruction: SystemInstruction = deserialize(&instruction.data)
        .map_err(|_| ParseInstructionError::InstructionNotParsable(ParsableProgram::System))?;
    match instruction.accounts.iter().max() {
        Some(index) if (*index as usize) < account_keys.len() => {}
        _ => {
            // Runtime should prevent this from ever happening
            return Err(ParseInstructionError::InstructionKeyMismatch(
                ParsableProgram::System,
            ));
        }
    }
    match system_instruction {
        SystemInstruction::CreateAccount {
            lamports,
            space,
            owner,
        } => {
            check_num_system_accounts(&instruction.accounts, 2)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "createAccount".to_string(),
                info: json!({
                    "source": account_keys[instruction.accounts[0] as usize].to_string(),
                    "newAccount": account_keys[instruction.accounts[1] as usize].to_string(),
                    "lamports": lamports,
                    "space": space,
                    "owner": owner.to_string(),
                }),
            })
        }
        SystemInstruction::Assign { owner } => {
            check_num_system_accounts(&instruction.accounts, 1)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "assign".to_string(),
                info: json!({
                    "account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "owner": owner.to_string(),
                }),
            })
        }
        SystemInstruction::Transfer { lamports } => {
            check_num_system_accounts(&instruction.accounts, 2)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "transfer".to_string(),
                info: json!({
                    "source": account_keys[instruction.accounts[0] as usize].to_string(),
                    "destination": account_keys[instruction.accounts[1] as usize].to_string(),
                    "lamports": lamports,
                }),
            })
        }
        SystemInstruction::CreateAccountWithSeed {
            base,
            seed,
            lamports,
            space,
            owner,
        } => {
            check_num_system_accounts(&instruction.accounts, 2)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "createAccountWithSeed".to_string(),
                info: json!({
                    "source": account_keys[instruction.accounts[0] as usize].to_string(),
                    "newAccount": account_keys[instruction.accounts[1] as usize].to_string(),
                    "base": base.to_string(),
                    "seed": seed,
                    "lamports": lamports,
                    "space": space,
                    "owner": owner.to_string(),
                }),
            })
        }
        SystemInstruction::AdvanceNonceAccount => {
            check_num_system_accounts(&instruction.accounts, 3)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "advanceNonce".to_string(),
                info: json!({
                    "nonceAccount": account_keys[instruction.accounts[0] as usize].to_string(),
                    "recentBlockhashesSysvar": account_keys[instruction.accounts[1] as usize].to_string(),
                    "nonceAuthority": account_keys[instruction.accounts[2] as usize].to_string(),
                }),
            })
        }
        SystemInstruction::WithdrawNonceAccount(lamports) => {
            check_num_system_accounts(&instruction.accounts, 5)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "withdrawFromNonce".to_string(),
                info: json!({
                    "nonceAccount": account_keys[instruction.accounts[0] as usize].to_string(),
                    "destination": account_keys[instruction.accounts[1] as usize].to_string(),
                    "recentBlockhashesSysvar": account_keys[instruction.accounts[2] as usize].to_string(),
                    "rentSysvar": account_keys[instruction.accounts[3] as usize].to_string(),
                    "nonceAuthority": account_keys[instruction.accounts[4] as usize].to_string(),
                    "lamports": lamports,
                }),
            })
        }
        SystemInstruction::InitializeNonceAccount(authority) => {
            check_num_system_accounts(&instruction.accounts, 3)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "initializeNonce".to_string(),
                info: json!({
                    "nonceAccount": account_keys[instruction.accounts[0] as usize].to_string(),
                    "recentBlockhashesSysvar": account_keys[instruction.accounts[1] as usize].to_string(),
                    "rentSysvar": account_keys[instruction.accounts[2] as usize].to_string(),
                    "nonceAuthority": authority.to_string(),
                }),
            })
        }
        SystemInstruction::AuthorizeNonceAccount(authority) => {
            check_num_system_accounts(&instruction.accounts, 2)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "authorizeNonce".to_string(),
                info: json!({
                    "nonceAccount": account_keys[instruction.accounts[0] as usize].to_string(),
                    "nonceAuthority": account_keys[instruction.accounts[1] as usize].to_string(),
                    "newAuthorized": authority.to_string(),
                }),
            })
        }
        SystemInstruction::UpgradeNonceAccount => {
            check_num_system_accounts(&instruction.accounts, 1)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "upgradeNonce".to_string(),
                info: json!({
                    "nonceAccount": account_keys[instruction.accounts[0] as usize].to_string(),
                }),
            })
        }
        SystemInstruction::Allocate { space } => {
            check_num_system_accounts(&instruction.accounts, 1)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "allocate".to_string(),
                info: json!({
                    "account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "space": space,
                }),
            })
        }
        SystemInstruction::AllocateWithSeed {
            base,
            seed,
            space,
            owner,
        } => {
            check_num_system_accounts(&instruction.accounts, 2)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "allocateWithSeed".to_string(),
                info: json!({
                    "account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "base": base.to_string(),
                    "seed": seed,
                    "space": space,
                    "owner": owner.to_string(),
                }),
            })
        }
        SystemInstruction::AssignWithSeed { base, seed, owner } => {
            check_num_system_accounts(&instruction.accounts, 2)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "assignWithSeed".to_string(),
                info: json!({
                    "account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "base": base.to_string(),
                    "seed": seed,
                    "owner": owner.to_string(),
                }),
            })
        }
        SystemInstruction::TransferWithSeed {
            lamports,
            from_seed,
            from_owner,
        } => {
            check_num_system_accounts(&instruction.accounts, 3)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "transferWithSeed".to_string(),
                info: json!({
                    "source": account_keys[instruction.accounts[0] as usize].to_string(),
                    "sourceBase": account_keys[instruction.accounts[1] as usize].to_string(),
                    "destination": account_keys[instruction.accounts[2] as usize].to_string(),
                    "lamports": lamports,
                    "sourceSeed": from_seed,
                    "sourceOwner": from_owner.to_string(),
                }),
            })
        }
    }
}

fn check_num_system_accounts(accounts: &[u8], num: usize) -> Result<(), ParseInstructionError> {
    check_num_accounts(accounts, num, ParsableProgram::System)
}

#[cfg(test)]
mod test {
    use {
        super::*,
        solana_sdk::{message::Message, pubkey::Pubkey, system_instruction},
    };

    #[test]
    #[allow(clippy::same_item_push)]
    fn test_parse_system_instruction() {
        let mut keys: Vec<Pubkey> = vec![];
        for _ in 0..6 {
            keys.push(solana_sdk::pubkey::new_rand());
        }

        let lamports = 55;
        let space = 128;

<<<<<<< HEAD
        let instruction =
            system_instruction::create_account(&keys[0], &keys[1], lamports, space, &keys[2]);
        let message = Message::new(&[instruction], None);
=======
        let instruction = system_instruction::create_account(
            &from_pubkey,
            &to_pubkey,
            lamports,
            space,
            &owner_pubkey,
        );
        let mut message = Message::new(&[instruction], None);
>>>>>>> 7b786ff33 (RPC instruction parser tests are missing some cases (#25951))
        assert_eq!(
            parse_system(&message.instructions[0], &keys[0..2]).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "createAccount".to_string(),
                info: json!({
                    "source": keys[0].to_string(),
                    "newAccount": keys[1].to_string(),
                    "lamports": lamports,
                    "owner": keys[2].to_string(),
                    "space": space,
                }),
            }
        );
<<<<<<< HEAD
        assert!(parse_system(&message.instructions[0], &keys[0..1]).is_err());

        let instruction = system_instruction::assign(&keys[0], &keys[1]);
        let message = Message::new(&[instruction], None);
=======
        assert!(parse_system(
            &message.instructions[0],
            &AccountKeys::new(&message.account_keys[0..1], None)
        )
        .is_err());
        let keys = message.account_keys.clone();
        message.instructions[0].accounts.pop();
        assert!(parse_system(&message.instructions[0], &AccountKeys::new(&keys, None)).is_err());
    }

    #[test]
    fn test_parse_system_assign_ix() {
        let account_pubkey = Pubkey::new_unique();
        let owner_pubkey = Pubkey::new_unique();
        let instruction = system_instruction::assign(&account_pubkey, &owner_pubkey);
        let mut message = Message::new(&[instruction], None);
>>>>>>> 7b786ff33 (RPC instruction parser tests are missing some cases (#25951))
        assert_eq!(
            parse_system(&message.instructions[0], &keys[0..1]).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "assign".to_string(),
                info: json!({
                    "account": keys[0].to_string(),
                    "owner": keys[1].to_string(),
                }),
            }
        );
<<<<<<< HEAD
        assert!(parse_system(&message.instructions[0], &[]).is_err());

        let instruction = system_instruction::transfer(&keys[0], &keys[1], lamports);
        let message = Message::new(&[instruction], None);
=======
        assert!(parse_system(&message.instructions[0], &AccountKeys::new(&[], None)).is_err());
        let keys = message.account_keys.clone();
        message.instructions[0].accounts.pop();
        assert!(parse_system(&message.instructions[0], &AccountKeys::new(&keys, None)).is_err());
    }

    #[test]
    fn test_parse_system_transfer_ix() {
        let lamports = 55;
        let from_pubkey = Pubkey::new_unique();
        let to_pubkey = Pubkey::new_unique();
        let instruction = system_instruction::transfer(&from_pubkey, &to_pubkey, lamports);
        let mut message = Message::new(&[instruction], None);
>>>>>>> 7b786ff33 (RPC instruction parser tests are missing some cases (#25951))
        assert_eq!(
            parse_system(&message.instructions[0], &keys[0..2]).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "transfer".to_string(),
                info: json!({
                    "source": keys[0].to_string(),
                    "destination": keys[1].to_string(),
                    "lamports": lamports,
                }),
            }
        );
<<<<<<< HEAD
        assert!(parse_system(&message.instructions[0], &keys[0..1]).is_err());
=======
        assert!(parse_system(
            &message.instructions[0],
            &AccountKeys::new(&message.account_keys[0..1], None)
        )
        .is_err());
        let keys = message.account_keys.clone();
        message.instructions[0].accounts.pop();
        assert!(parse_system(&message.instructions[0], &AccountKeys::new(&keys, None)).is_err());
    }
>>>>>>> 7b786ff33 (RPC instruction parser tests are missing some cases (#25951))

        let seed = "test_seed";
        let instruction = system_instruction::create_account_with_seed(
            &keys[0], &keys[2], &keys[1], seed, lamports, space, &keys[3],
        );
        let mut message = Message::new(&[instruction], None);
        assert_eq!(
            parse_system(&message.instructions[0], &keys[0..3]).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "createAccountWithSeed".to_string(),
                info: json!({
                    "source": keys[0].to_string(),
                    "newAccount": keys[2].to_string(),
                    "lamports": lamports,
                    "base": keys[1].to_string(),
                    "seed": seed,
                    "owner": keys[3].to_string(),
                    "space": space,
                }),
            }
        );

<<<<<<< HEAD
        let seed = "test_seed";
        let instruction = system_instruction::create_account_with_seed(
            &keys[0], &keys[1], &keys[0], seed, lamports, space, &keys[3],
        );
        let message = Message::new(&[instruction], None);
        assert_eq!(
            parse_system(&message.instructions[0], &keys[0..2]).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "createAccountWithSeed".to_string(),
                info: json!({
                    "source": keys[0].to_string(),
                    "newAccount": keys[1].to_string(),
                    "lamports": lamports,
                    "base": keys[0].to_string(),
                    "seed": seed,
                    "owner": keys[3].to_string(),
                    "space": space,
                }),
            }
        );
        assert!(parse_system(&message.instructions[0], &keys[0..1]).is_err());

        let instruction = system_instruction::allocate(&keys[0], space);
        let message = Message::new(&[instruction], None);
=======
        assert!(parse_system(
            &message.instructions[0],
            &AccountKeys::new(&message.account_keys[0..1], None)
        )
        .is_err());
        let keys = message.account_keys.clone();
        message.instructions[0].accounts.pop();
        message.instructions[0].accounts.pop();
        assert!(parse_system(&message.instructions[0], &AccountKeys::new(&keys, None)).is_err());
    }

    #[test]
    fn test_parse_system_allocate_ix() {
        let space = 128;
        let account_pubkey = Pubkey::new_unique();
        let instruction = system_instruction::allocate(&account_pubkey, space);
        let mut message = Message::new(&[instruction], None);
>>>>>>> 7b786ff33 (RPC instruction parser tests are missing some cases (#25951))
        assert_eq!(
            parse_system(&message.instructions[0], &keys[0..1]).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "allocate".to_string(),
                info: json!({
                    "account": keys[0].to_string(),
                    "space": space,
                }),
            }
        );
<<<<<<< HEAD
        assert!(parse_system(&message.instructions[0], &[]).is_err());

        let instruction =
            system_instruction::allocate_with_seed(&keys[1], &keys[0], seed, space, &keys[2]);
        let message = Message::new(&[instruction], None);
=======
        assert!(parse_system(&message.instructions[0], &AccountKeys::new(&[], None)).is_err());
        let keys = message.account_keys.clone();
        message.instructions[0].accounts.pop();
        assert!(parse_system(&message.instructions[0], &AccountKeys::new(&keys, None)).is_err());
    }

    #[test]
    fn test_parse_system_allocate_with_seed_ix() {
        let space = 128;
        let seed = "test_seed";
        let account_pubkey = Pubkey::new_unique();
        let base_pubkey = Pubkey::new_unique();
        let owner_pubkey = Pubkey::new_unique();
        let instruction = system_instruction::allocate_with_seed(
            &account_pubkey,
            &base_pubkey,
            seed,
            space,
            &owner_pubkey,
        );
        let mut message = Message::new(&[instruction], None);
>>>>>>> 7b786ff33 (RPC instruction parser tests are missing some cases (#25951))
        assert_eq!(
            parse_system(&message.instructions[0], &keys[0..2]).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "allocateWithSeed".to_string(),
                info: json!({
                    "account": keys[1].to_string(),
                    "base": keys[0].to_string(),
                    "seed": seed,
                    "owner": keys[2].to_string(),
                    "space": space,
                }),
            }
        );
<<<<<<< HEAD
        assert!(parse_system(&message.instructions[0], &keys[0..1]).is_err());

        let instruction = system_instruction::assign_with_seed(&keys[1], &keys[0], seed, &keys[2]);
        let message = Message::new(&[instruction], None);
=======
        assert!(parse_system(
            &message.instructions[0],
            &AccountKeys::new(&message.account_keys[0..1], None)
        )
        .is_err());
        let keys = message.account_keys.clone();
        message.instructions[0].accounts.pop();
        assert!(parse_system(&message.instructions[0], &AccountKeys::new(&keys, None)).is_err());
    }

    #[test]
    fn test_parse_system_assign_with_seed_ix() {
        let seed = "test_seed";
        let account_pubkey = Pubkey::new_unique();
        let base_pubkey = Pubkey::new_unique();
        let owner_pubkey = Pubkey::new_unique();
        let instruction = system_instruction::assign_with_seed(
            &account_pubkey,
            &base_pubkey,
            seed,
            &owner_pubkey,
        );
        let mut message = Message::new(&[instruction], None);
>>>>>>> 7b786ff33 (RPC instruction parser tests are missing some cases (#25951))
        assert_eq!(
            parse_system(&message.instructions[0], &keys[0..2]).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "assignWithSeed".to_string(),
                info: json!({
                    "account": keys[1].to_string(),
                    "base": keys[0].to_string(),
                    "seed": seed,
                    "owner": keys[2].to_string(),
                }),
            }
        );
<<<<<<< HEAD
        assert!(parse_system(&message.instructions[0], &keys[0..1]).is_err());
=======
        assert!(parse_system(
            &message.instructions[0],
            &AccountKeys::new(&message.account_keys[0..1], None)
        )
        .is_err());
        let keys = message.account_keys.clone();
        message.instructions[0].accounts.pop();
        assert!(parse_system(&message.instructions[0], &AccountKeys::new(&keys, None)).is_err());
    }
>>>>>>> 7b786ff33 (RPC instruction parser tests are missing some cases (#25951))

        let instruction = system_instruction::transfer_with_seed(
            &keys[1],
            &keys[0],
            seed.to_string(),
            &keys[3],
            &keys[2],
            lamports,
        );
        let mut message = Message::new(&[instruction], None);
        assert_eq!(
            parse_system(&message.instructions[0], &keys[0..3]).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "transferWithSeed".to_string(),
                info: json!({
                    "source": keys[1].to_string(),
                    "sourceBase": keys[0].to_string(),
                    "sourceSeed": seed,
                    "sourceOwner": keys[3].to_string(),
                    "lamports": lamports,
                    "destination": keys[2].to_string()
                }),
            }
        );
<<<<<<< HEAD
        assert!(parse_system(&message.instructions[0], &keys[0..2]).is_err());
=======
        assert!(parse_system(
            &message.instructions[0],
            &AccountKeys::new(&message.account_keys[0..2], None)
        )
        .is_err());
        let keys = message.account_keys.clone();
        message.instructions[0].accounts.pop();
        assert!(parse_system(&message.instructions[0], &AccountKeys::new(&keys, None)).is_err());
>>>>>>> 7b786ff33 (RPC instruction parser tests are missing some cases (#25951))
    }

    #[test]
    #[allow(clippy::same_item_push)]
    fn test_parse_system_instruction_nonce() {
        let mut keys: Vec<Pubkey> = vec![];
        for _ in 0..5 {
            keys.push(solana_sdk::pubkey::new_rand());
        }

<<<<<<< HEAD
        let instruction = system_instruction::advance_nonce_account(&keys[1], &keys[0]);
        let message = Message::new(&[instruction], None);
=======
        let instruction =
            system_instruction::advance_nonce_account(&nonce_pubkey, &authorized_pubkey);
        let mut message = Message::new(&[instruction], None);
>>>>>>> 7b786ff33 (RPC instruction parser tests are missing some cases (#25951))
        assert_eq!(
            parse_system(&message.instructions[0], &keys[0..3]).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "advanceNonce".to_string(),
                info: json!({
                    "nonceAccount": keys[1].to_string(),
                    "recentBlockhashesSysvar": keys[2].to_string(),
                    "nonceAuthority": keys[0].to_string(),
                }),
            }
        );
<<<<<<< HEAD
        assert!(parse_system(&message.instructions[0], &keys[0..2]).is_err());

        let lamports = 55;
        let instruction =
            system_instruction::withdraw_nonce_account(&keys[1], &keys[0], &keys[2], lamports);
        let message = Message::new(&[instruction], None);
=======
        assert!(parse_system(
            &message.instructions[0],
            &AccountKeys::new(&message.account_keys[0..2], None)
        )
        .is_err());
        let keys = message.account_keys.clone();
        message.instructions[0].accounts.pop();
        assert!(parse_system(&message.instructions[0], &AccountKeys::new(&keys, None)).is_err());
    }

    #[test]
    fn test_parse_system_withdraw_nonce_account_ix() {
        let nonce_pubkey = Pubkey::new_unique();
        let authorized_pubkey = Pubkey::new_unique();
        let to_pubkey = Pubkey::new_unique();

        let lamports = 55;
        let instruction = system_instruction::withdraw_nonce_account(
            &nonce_pubkey,
            &authorized_pubkey,
            &to_pubkey,
            lamports,
        );
        let mut message = Message::new(&[instruction], None);
>>>>>>> 7b786ff33 (RPC instruction parser tests are missing some cases (#25951))
        assert_eq!(
            parse_system(&message.instructions[0], &keys[0..5]).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "withdrawFromNonce".to_string(),
                info: json!({
                    "nonceAccount": keys[1].to_string(),
                    "destination": keys[2].to_string(),
                    "recentBlockhashesSysvar": keys[3].to_string(),
                    "rentSysvar": keys[4].to_string(),
                    "nonceAuthority": keys[0].to_string(),
                    "lamports": lamports
                }),
            }
        );
<<<<<<< HEAD
        assert!(parse_system(&message.instructions[0], &keys[0..4]).is_err());

        let instructions =
            system_instruction::create_nonce_account(&keys[0], &keys[1], &keys[4], lamports);
        let message = Message::new(&instructions, None);
=======
        assert!(parse_system(
            &message.instructions[0],
            &AccountKeys::new(&message.account_keys[0..4], None)
        )
        .is_err());
        let keys = message.account_keys.clone();
        message.instructions[0].accounts.pop();
        assert!(parse_system(&message.instructions[0], &AccountKeys::new(&keys, None)).is_err());
    }

    #[test]
    fn test_parse_system_initialize_nonce_ix() {
        let lamports = 55;
        let from_pubkey = Pubkey::new_unique();
        let nonce_pubkey = Pubkey::new_unique();
        let authorized_pubkey = Pubkey::new_unique();

        let instructions = system_instruction::create_nonce_account(
            &from_pubkey,
            &nonce_pubkey,
            &authorized_pubkey,
            lamports,
        );
        let mut message = Message::new(&instructions, None);
>>>>>>> 7b786ff33 (RPC instruction parser tests are missing some cases (#25951))
        assert_eq!(
            parse_system(&message.instructions[1], &keys[0..4]).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "initializeNonce".to_string(),
                info: json!({
                    "nonceAccount": keys[1].to_string(),
                    "recentBlockhashesSysvar": keys[2].to_string(),
                    "rentSysvar": keys[3].to_string(),
                    "nonceAuthority": keys[4].to_string(),
                }),
            }
        );
<<<<<<< HEAD
        assert!(parse_system(&message.instructions[1], &keys[0..3]).is_err());

        let instruction = system_instruction::authorize_nonce_account(&keys[1], &keys[0], &keys[2]);
        let message = Message::new(&[instruction], None);
=======
        assert!(parse_system(
            &message.instructions[1],
            &AccountKeys::new(&message.account_keys[0..3], None)
        )
        .is_err());
        let keys = message.account_keys.clone();
        message.instructions[0].accounts.pop();
        assert!(parse_system(&message.instructions[0], &AccountKeys::new(&keys, None)).is_err());
    }

    #[test]
    fn test_parse_system_authorize_nonce_account_ix() {
        let nonce_pubkey = Pubkey::new_unique();
        let authorized_pubkey = Pubkey::new_unique();
        let new_authority_pubkey = Pubkey::new_unique();

        let instruction = system_instruction::authorize_nonce_account(
            &nonce_pubkey,
            &authorized_pubkey,
            &new_authority_pubkey,
        );
        let mut message = Message::new(&[instruction], None);
>>>>>>> 7b786ff33 (RPC instruction parser tests are missing some cases (#25951))
        assert_eq!(
            parse_system(&message.instructions[0], &keys[0..2]).unwrap(),
            ParsedInstructionEnum {
                instruction_type: "authorizeNonce".to_string(),
                info: json!({
                    "nonceAccount": keys[1].to_string(),
                    "newAuthorized": keys[2].to_string(),
                    "nonceAuthority": keys[0].to_string(),
                }),
            }
        );
<<<<<<< HEAD
        assert!(parse_system(&message.instructions[0], &keys[0..1]).is_err());
=======
        assert!(parse_system(
            &message.instructions[0],
            &AccountKeys::new(&message.account_keys[0..1], None)
        )
        .is_err());
        let keys = message.account_keys.clone();
        message.instructions[0].accounts.pop();
        assert!(parse_system(&message.instructions[0], &AccountKeys::new(&keys, None)).is_err());
>>>>>>> 7b786ff33 (RPC instruction parser tests are missing some cases (#25951))
    }
}
