#![allow(unexpected_cfgs)]
#![allow(deprecated)]
#![allow(unused)]

#[cfg(test)]
mod tests {
    use anchor_escrow::{instruction, Escrow};
    use anchor_lang::{prelude::*, InstructionData};
    use mollusk_svm::Mollusk;
    use solana_sdk::{
        account::{Account, WritableAccount},
        instruction::{AccountMeta, Instruction},
        native_token::LAMPORTS_PER_SOL,
        pubkey,
    };

    use mollusk_svm_bencher::MolluskComputeUnitBencher;

    const ID: pubkey::Pubkey = pubkey!("Eo9M2qhnJuRkZZsqGKhY9bxue7kSzowu5ppzgo5c2GWC");

    const USER: pubkey::Pubkey = pubkey::Pubkey::new_from_array([0x01; 32]);

    #[test]
    fn test_make() {
        let mollusk = Mollusk::new(&ID, "../../target/deploy/anchor_escrow");

        let (state_pda, state_bump) = 
            pubkey::Pubkey::find_program_address(&[b"state", USER.as_ref()], &ID);

        let (state_pda, state_bump) = 
            pubkey::Pubkey::find_program_address(&[b"state", USER.as_ref()], &ID);


    }
}
