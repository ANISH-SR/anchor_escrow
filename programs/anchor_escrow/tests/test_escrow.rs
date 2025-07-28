#![allow(unexpected_cfgs)]
#![allow(deprecated)]
#![allow(unused)]

#[cfg(test)]
mod tests {
    use anchor_escrow::{instruction, Escrow};
    use anchor_lang::{prelude::*, InstructionData};
    use anchor_spl::{
        associated_token::spl_associated_token_account,
        token_interface::{Mint, TokenAccount},
    };
    use mollusk_svm::{program, result::Check, Mollusk};
    use mollusk_svm_bencher::MolluskComputeUnitBencher;
    use solana_sdk::{
        account::{Account, WritableAccount},
        instruction::{AccountMeta, Instruction},
        native_token::LAMPORTS_PER_SOL,
        pubkey,
        pubkey::Pubkey,
    };

    const ID: Pubkey = pubkey!("Eo9M2qhnJuRkZZsqGKhY9bxue7kSzowu5ppzgo5c2GWC");
    const MAKER: Pubkey = Pubkey::new_from_array([0x01; 32]);

    const SEED: u64 = 12345;
    const DEPOSIT_AMOUNT: u64 = 1_000_000;
    const RECEIVE_AMOUNT: u64 = 2_000_000;

    #[test]
    fn test_make() {
        let mollusk = Mollusk::new(&ID, "../../target/deploy/anchor_escrow");

        let (escrow_pda, escrow_bump) = Pubkey::find_program_address(
            &[b"escrow", MAKER.as_ref(), SEED.to_le_bytes().as_ref()],
            &ID,
        );

        let (system_program, system_account) = program::keyed_account_for_system_program();
        let token_program = anchor_spl::token::ID;
        let associated_token_program = anchor_spl::associated_token::ID;

        let mint_a = Pubkey::new_unique();
        let mint_a_account =
            Account::new(mollusk.sysvars.rent.minimum_balance(82), 82, &token_program);

        let mint_b = Pubkey::new_unique();
        let mint_b_account =
            Account::new(mollusk.sysvars.rent.minimum_balance(82), 82, &token_program);

        let maker_ata_a =
            anchor_spl::associated_token::get_associated_token_address(&MAKER, &mint_a);

        let maker_ata_a_account = Account::new(
            mollusk.sysvars.rent.minimum_balance(165), //Standard SPL token account size
            165,
            &token_program,
        );

        let vault =
            anchor_spl::associated_token::get_associated_token_address(&escrow_pda, &mint_a);

        let vault_account = Account::new(
            mollusk.sysvars.rent.minimum_balance(165),
            165,
            &token_program,
        );

        // Create Make Ix data
        let make_ix_data = instruction::Make {
            seed: SEED,
            receive: RECEIVE_AMOUNT,
            deposit: DEPOSIT_AMOUNT,
        }
        .data();

        let make_ix_accs = vec![
            AccountMeta::new(MAKER, true),
            AccountMeta::new_readonly(mint_a, false),
            AccountMeta::new_readonly(mint_b, false),
            AccountMeta::new(maker_ata_a, false),
            AccountMeta::new(escrow_pda, false),
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(associated_token_program, false),
        ];

        // Create the Make instruction
        let make_ix = Instruction::new_with_bytes(ID, &make_ix_data, make_ix_accs);

        let make_tx_accounts = &vec![
            (
                MAKER,
                Account::new(5 * LAMPORTS_PER_SOL, 0, &system_program),
            ),
            (mint_a, mint_a_account),
            (mint_b, mint_b_account),
            (maker_ata_a, maker_ata_a_account),
            (escrow_pda, Account::new(0, 0, &system_program)),
            (vault, vault_account),
            (system_program, system_account),
            (token_program, Account::new(1_000_000, 0, &system_program)),
            (
                associated_token_program,
                Account::new(1_000_000, 0, &system_program),
            ),
        ];

        let result = mollusk.process_and_validate_instruction(
            &make_ix,
            &make_tx_accounts,
            &[Check::success()],
        );

        MolluskComputeUnitBencher::new(mollusk)
            .bench(("Make", &make_ix, &make_tx_accounts))
            .must_pass(true)
            .out_dir("benches/")
            .execute();
    }
}

// MolluskComputeUnitBencher::new(mollusk)
//             .bench(("Initialize Mint A", &init_mint_a, &tx_accounts))
//             .bench(("Initialize Mint B", &init_mint_b, &tx_accounts))
//             .bench(("Create Maker ATA", &create_maker_ata, &tx_accounts))
//             .bench(("Mint to Maker", &mint_to_maker, &tx_accounts))
//             .bench(("Make Escrow", &make_ix, &tx_accounts))
//             .must_pass(true)
//             .out_dir("benches/")
//             .execute();

// #![allow(unexpected_cfgs)]
// #![allow(deprecated)]
// #![allow(unused)]

// #[cfg(test)]
// mod tests {
//     use anchor_escrow::{instruction, Escrow};
//     use anchor_lang::{prelude::*, InstructionData};
//     use anchor_spl::{
//         associated_token::spl_associated_token_account,
//         token_interface::{Mint, TokenAccount},
//     };
//     use mollusk_svm::{program, result::Check, Mollusk};
//     use mollusk_svm_bencher::MolluskComputeUnitBencher;
//     use solana_program::program_pack::Pack;
//     use solana_sdk::{
//         account::{Account, WritableAccount},
//         instruction::{AccountMeta, Instruction},
//         native_token::LAMPORTS_PER_SOL,
//         pubkey,
//         pubkey::Pubkey,
//     };
//     use spl_token::solana_program::program_option::COption;
//     use spl_token::state::{Account as SplAccount, AccountState, Mint as SplMint};

//     const ID: Pubkey = pubkey!("Eo9M2qhnJuRkZZsqGKhY9bxue7kSzowu5ppzgo5c2GWC");
//     const MAKER: Pubkey = Pubkey::new_from_array([0x01; 32]);

//     const SEED: u64 = 12345;
//     const DEPOSIT_AMOUNT: u64 = 1_000_000;
//     const RECEIVE_AMOUNT: u64 = 2_000_000;

//     #[test]
//     fn test_make() {
//         let mollusk = Mollusk::new(&ID, "../../target/deploy/anchor_escrow");

//         let (escrow_pda, _escrow_bump) = Pubkey::find_program_address(
//             &[b"escrow", MAKER.as_ref(), SEED.to_le_bytes().as_ref()],
//             &ID,
//         );

//         let (system_program, system_account) = program::keyed_account_for_system_program();
//         let token_program = anchor_spl::token::ID;
//         let associated_token_program = anchor_spl::associated_token::ID;

//         // Mint A
//         let mint_a = Pubkey::new_unique();
//         let mut mint_a_account =
//             Account::new(mollusk.sysvars.rent.minimum_balance(82), 82, &token_program);
//         let mint_a_data = SplMint {
//             mint_authority: COption::None,
//             supply: 0,
//             decimals: 6,
//             is_initialized: true,
//             freeze_authority: COption::None,
//         };
//         mint_a_data.pack_into_slice(&mut mint_a_account.data_as_mut_slice());

//         // Mint B
//         let mint_b = Pubkey::new_unique();
//         let mut mint_b_account =
//             Account::new(mollusk.sysvars.rent.minimum_balance(82), 82, &token_program);
//         let mint_b_data = SplMint {
//             mint_authority: COption::None,
//             supply: 0,
//             decimals: 6,
//             is_initialized: true,
//             freeze_authority: COption::None,
//         };
//         mint_b_data.pack_into_slice(&mut mint_b_account.data_as_mut_slice());

//         // Maker ATA A
//         let maker_ata_a =
//             anchor_spl::associated_token::get_associated_token_address(&MAKER, &mint_a);

//         let mut maker_ata_a_account = Account::new(
//             mollusk.sysvars.rent.minimum_balance(165),
//             165,
//             &token_program,
//         );

//         let maker_ata_data = SplAccount {
//             mint: mint_a,
//             owner: MAKER,
//             amount: DEPOSIT_AMOUNT,
//             delegate: COption::None,
//             state: AccountState::Initialized,
//             is_native: COption::None,
//             delegated_amount: 0,
//             close_authority: COption::None,
//         };
//         maker_ata_data.pack_into_slice(&mut maker_ata_a_account.data_as_mut_slice());

//         // Vault
//         let vault =
//             anchor_spl::associated_token::get_associated_token_address(&escrow_pda, &mint_a);

//         let mut vault_account = Account::new(
//             mollusk.sysvars.rent.minimum_balance(165),
//             165,
//             &token_program,
//         );

//         let vault_data = SplAccount {
//             mint: mint_a,
//             owner: escrow_pda,
//             amount: 0,
//             delegate: COption::None,
//             state: AccountState::Initialized,
//             is_native: COption::None,
//             delegated_amount: 0,
//             close_authority: COption::None,
//         };

//         vault_data.pack_into_slice(&mut vault_account.data_as_mut_slice());

//         // Escrow
//         let escrow_account = Account::new(
//             mollusk.sysvars.rent.minimum_balance(8 + Escrow::INIT_SPACE),
//             8 + Escrow::INIT_SPACE,
//             &ID,
//         );

//         // Make instruction data
//         let make_ix_data = instruction::Make {
//             seed: SEED,
//             receive: RECEIVE_AMOUNT,
//             deposit: DEPOSIT_AMOUNT,
//         }
//         .data();

//         let make_ix_accs = vec![
//             AccountMeta::new(MAKER, true),
//             AccountMeta::new_readonly(mint_a, false),
//             AccountMeta::new_readonly(mint_b, false),
//             AccountMeta::new(maker_ata_a, false),
//             AccountMeta::new(escrow_pda, false),
//             AccountMeta::new(vault, false),
//             AccountMeta::new_readonly(system_program, false),
//             AccountMeta::new_readonly(token_program, false),
//             AccountMeta::new_readonly(associated_token_program, false),
//         ];

//         let make_ix = Instruction::new_with_bytes(ID, &make_ix_data, make_ix_accs);

//         let make_tx_accounts = &vec![
//             (
//                 MAKER,
//                 Account::new(5 * LAMPORTS_PER_SOL, 0, &system_program),
//             ),
//             (mint_a, mint_a_account),
//             (mint_b, mint_b_account),
//             (maker_ata_a, maker_ata_a_account),
//             (escrow_pda, escrow_account),
//             (vault, vault_account),
//             (system_program, system_account),
//             (token_program, Account::new(1_000_000, 0, &system_program)),
//             (
//                 associated_token_program,
//                 Account::new(1_000_000, 0, &system_program),
//             ),
//         ];

//         let result = mollusk.process_and_validate_instruction(
//             &make_ix,
//             &make_tx_accounts,
//             &[Check::success()],
//         );

//         MolluskComputeUnitBencher::new(mollusk)
//             .bench(("Make", &make_ix, &make_tx_accounts))
//             .must_pass(true)
//             .out_dir("benches/")
//             .execute();
//     }
// }
