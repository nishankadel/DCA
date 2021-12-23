//! Program state processor

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{
    instruction::{ProcessDeposit, ProcessWithdraw, TokenInstruction},
    state::DCA,
    utils::{assert_keys_equal, get_master_address_and_bump_seed},
};

use spl_associated_token_account::get_associated_token_address;

pub struct Processor {}

impl Processor {
    pub fn process_deposit(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        ////
        let account_info_iter = &mut accounts.iter();
        let sender_account = next_account_info(account_info_iter)?; // sender
        let admin_pda = next_account_info(account_info_iter)?; // admin pda
        let token_program = next_account_info(account_info_iter)?;
        let token_mint = next_account_info(account_info_iter)?; //////////is mint address sent in account or as Pubkey???
        let system_program = next_account_info(account_info_iter)?;
        let rent_account = next_account_info(account_info_iter)?;
        let pda_associated_info = next_account_info(account_info_iter)?; // Associated token of pda
        let associated_token_info = next_account_info(account_info_iter)?; // Associated token master
        let associated_token_address = next_account_info(account_info_iter)?;

        let (account_address, _bump_seed) =
            get_master_address_and_bump_seed(sender_account.key, program_id);
        //Was the transaction signed by sender account's public key
        if !sender_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let pda_token = get_associated_token_address(&account_address, token_mint.key); //creating pda token as per token available

        //comparing admin_pda and pda
        //comparing mint addresses
        //return error
        if *admin_pda.key != account_address
            && spl_token::id() != *token_program.key
            && pda_token != *pda_associated_info.key
        {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if pda_associated_info.data_is_empty() {
            invoke(
                &spl_associated_token_account::create_associated_token_account(
                    sender_account.key,
                    admin_pda.key,
                    token_mint.key,
                ),
                &[
                    sender_account.clone(),
                    pda_associated_info.clone(),
                    admin_pda.clone(),
                    token_mint.clone(),
                    token_program.clone(),
                    rent_account.clone(),
                    associated_token_info.clone(),
                    system_program.clone(),
                ],
            )?
        }

        invoke(
            &spl_token::instruction::transfer(
                token_program.key,
                associated_token_address.key,
                pda_associated_info.key,
                sender_account.key,
                &[sender_account.key],
                amount,
            )?,
            &[
                token_program.clone(),
                associated_token_address.clone(),
                pda_associated_info.clone(),
                sender_account.clone(),
                system_program.clone(),
            ],
        )?;

        let mut dca = DCA::try_from_slice(&admin_pda.data.borrow())?;
        dca.amount += amount;
        dca.sender_account = *sender_account.key;
        dca.admin_pda = *admin_pda.key;
        dca.mint_address = *token_mint.key;
        dca.serialize(&mut &mut admin_pda.data.borrow_mut()[..])?;

        Ok(())
    }

    pub fn process_withdraw(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let sender_account = next_account_info(account_info_iter)?; // sender
        let admin_pda = next_account_info(account_info_iter)?; // admin pda
        let token_program = next_account_info(account_info_iter)?;
        let token_mint = next_account_info(account_info_iter)?; //////////is mint address sent in account or as Pubkey???
        let system_program = next_account_info(account_info_iter)?;
        let _rent_account = next_account_info(account_info_iter)?;
        let pda_associated_info = next_account_info(account_info_iter)?; // Associated token of pda
        let _associated_token_info = next_account_info(account_info_iter)?; // Associated token master
        let associated_token_address = next_account_info(account_info_iter)?;

        let (account_address, bump_seed) =
            get_master_address_and_bump_seed(sender_account.key, program_id);
        let pda_signer_seeds: &[&[_]] = &[&sender_account.key.to_bytes(), &[bump_seed]];
        let pda_associated_token = get_associated_token_address(&account_address, token_mint.key);
        let source_associated_token =
            get_associated_token_address(&sender_account.key, token_mint.key);
        assert_keys_equal(source_associated_token, *associated_token_address.key)?;
        assert_keys_equal(spl_token::id(), *token_program.key)?;
        assert_keys_equal(account_address, *admin_pda.key)?;
        assert_keys_equal(pda_associated_token, *pda_associated_info.key)?;
        if !sender_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        invoke_signed(
            &spl_token::instruction::transfer(
                token_program.key,
                pda_associated_info.key,
                associated_token_address.key,
                admin_pda.key,
                &[admin_pda.key],
                amount,
            )?,
            &[
                token_program.clone(),
                pda_associated_info.clone(),
                associated_token_address.clone(),
                admin_pda.clone(),
                system_program.clone(),
            ],
            &[&pda_signer_seeds[..]],
        )?;

        let mut dca = DCA::try_from_slice(&admin_pda.data.borrow())?;
        dca.amount -= amount;
        dca.sender_account = *sender_account.key;
        dca.admin_pda = *admin_pda.key;
        dca.mint_address = *token_mint.key;
        dca.serialize(&mut &mut admin_pda.data.borrow_mut()[..])?;

        Ok(())
    }

    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        let instruction = TokenInstruction::unpack(input)?;
        match instruction {
            TokenInstruction::ProcessDeposit(ProcessDeposit { amount }) => {
                msg!("Instruction: Deposit token");
                Self::process_deposit(program_id, accounts, amount)
            }
            TokenInstruction::ProcessWithdraw(ProcessWithdraw { amount }) => {
                msg!("Instruction: Withdraw token");
                Self::process_withdraw(program_id, accounts, amount)
            }
        }
    }
}
