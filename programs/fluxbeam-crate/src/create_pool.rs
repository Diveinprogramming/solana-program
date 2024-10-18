use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_lang::system_program::{create_account, CreateAccount};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::instruction::Instruction;

pub struct CreatePoolAccountsSimple<'info> {
    pub payer: AccountInfo<'info>,
    pub pool: AccountInfo<'info>,
    pub pool_authority: AccountInfo<'info>,
    pub token_account_a: AccountInfo<'info>,
    pub token_account_b: AccountInfo<'info>,
    pub pool_lp_mint: AccountInfo<'info>,
    pub fee_account: AccountInfo<'info>,
    pub token_account_pool: AccountInfo<'info>,
    pub pool_token_program_id: AccountInfo<'info>,
    pub swap_program_id: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

pub struct CreatePoolAccounts<'info> {
    pub authority: AccountInfo<'info>,
    pub payer: AccountInfo<'info>,

    pub pool: AccountInfo<'info>,
    pub pool_authority: AccountInfo<'info>,

    pub mint_a: AccountInfo<'info>,
    pub mint_b: AccountInfo<'info>,

    pub token_account_a: AccountInfo<'info>,
    pub token_account_b: AccountInfo<'info>,
    pub pool_lp_mint: AccountInfo<'info>,
    pub fee_account: AccountInfo<'info>,

    pub token_account_pool: AccountInfo<'info>,

    pub mint_a_token_program: AccountInfo<'info>,
    pub mint_b_token_program: AccountInfo<'info>,
    pub pool_token_program_id: AccountInfo<'info>,

    pub swap_program_id: AccountInfo<'info>,
    pub associated_token_program: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CreatePoolArgs {
    pub instruction: u8,
    pub trade_fee_numerator: u64,
    pub trade_fee_denominator: u64,
    pub owner_trade_fee_numerator: u64,
    pub owner_trade_fee_denominator: u64,
    pub owner_withdraw_fee_numerator: u64,
    pub owner_withdraw_fee_denominator: u64,
    pub host_fee_numerator: u64,
    pub host_fee_denominator: u64,
    pub curve_type: u8,
    pub curve_parameters: [u8; 32],
}

pub fn create_pool_instruction(accounts: &CreatePoolAccountsSimple, args: &CreatePoolArgs) -> Instruction {
    let data = args.try_to_vec().unwrap();

    Instruction {
        program_id: *accounts.swap_program_id.key,
        accounts: vec![
            AccountMeta::new(*accounts.pool.key, false),
            AccountMeta::new_readonly(*accounts.pool_authority.key, false),
            AccountMeta::new_readonly(*accounts.token_account_a.key, false),
            AccountMeta::new_readonly(*accounts.token_account_b.key, false),
            AccountMeta::new(*accounts.pool_lp_mint.key, false),
            AccountMeta::new_readonly(*accounts.fee_account.key, false),
            AccountMeta::new(*accounts.token_account_pool.key, false),
            AccountMeta::new_readonly(*accounts.pool_token_program_id.key, false),
        ],
        data,
    }
}

pub fn create_pool(accounts: &CreatePoolAccountsSimple, args: &CreatePoolArgs, _bump_seed: u8) -> Result<()> {

    //Init the swap pool account
    create_account(
        CpiContext::new(
            accounts.system_program.to_account_info(),
            CreateAccount {
                from: accounts.payer.to_account_info(),
                to: accounts.pool.to_account_info(),
            },
        ),
        Rent::default().minimum_balance(324),
        324,
        accounts.swap_program_id.key,
    )?;

    let ix = create_pool_instruction(accounts, args);
    solana_program::program::invoke(
        &ix,
        &[
            //   0. `[writable, signer]` New Token-swap to create.
            accounts.pool.clone(),
            //   1. `[]` swap authority derived from `create_program_address(&[Token-swap account])`
            accounts.pool_authority.clone(),
            //   2. `[]` token_a Account. Must be non zero, owned by swap authority.
            accounts.token_account_a.clone(),
            //   3. `[]` token_b Account. Must be non zero, owned by swap authority.
            accounts.token_account_b.clone(),
            //   4. `[writable]` Pool Token Mint. Must be empty, owned by swap authority.
            accounts.pool_lp_mint.clone(),
            //   5. `[]` Pool Token Account to deposit trading and withdraw fees. Must be empty, not owned by swap authority
            accounts.fee_account.clone(),
            //   6. `[writable]` Pool Token Account to deposit the initial pool token supply.  Must be empty, not owned by swap authority.
            accounts.token_account_pool.clone(),
            //   7. `[]` Pool Token program id
            accounts.pool_token_program_id.clone(),
        ],
    )?;

    Ok(())
}


//TODO Complete - Requires all accounts to be passed uninitialised
pub fn create_pool_accounts() {
    //
    // msg!("Create Pool Token A {} - {}", accounts.token_account_a.key.to_string(), accounts.mint_a.key.to_string());
    //
    // msg!("Initialize Pool Token A (Quote Mint) {}", accounts.token_account_a.key.to_string());
    // initialize_account3(
    //     CpiContext::new(
    //         accounts.mint_a_token_program.to_account_info(),
    //         InitializeAccount3 {
    //             account: accounts.token_account_a.to_account_info(),
    //             mint: accounts.mint_a.to_account_info(),
    //             authority: accounts.pool_authority.to_account_info(),
    //         })
    // )?;
    //
    // msg!("Create Pool Token B (Base Mint)");
    // associated_token::create(CpiContext::new(
    //     accounts.associated_token_program.to_account_info(), //TODO Add ATA program to ix
    //     associated_token::Create {
    //         payer: accounts.payer.to_account_info(),
    //         associated_token: accounts.associated_token_program.to_account_info(),
    //         authority: accounts.token_account_b.to_account_info(),
    //         mint: accounts.mint_b.to_account_info(),
    //         system_program: accounts.system_program.to_account_info(),
    //         token_program: accounts.mint_b_token_program.to_account_info(),
    //     },
    // ))?;
    // initialize_account3(
    //     CpiContext::new(
    //         accounts.mint_b_token_program.to_account_info(),
    //         InitializeAccount3 {
    //             account: accounts.token_account_b.to_account_info(),
    //             mint: accounts.mint_b.to_account_info(),
    //             authority: accounts.pool_authority.to_account_info(),
    //         })
    // )?;
    //
    // msg!("Create Pool LP Mint");
    // initialize_mint2(
    //     CpiContext::new(
    //         accounts.pool_token_program_id.to_account_info(),
    //         InitializeMint2 {
    //             mint: accounts.token_pool.to_account_info(),
    //         }),
    //     0,
    //     accounts.pool_authority.key,
    //     None,
    // )?;
    //
    //
    // msg!("Create Pool LP");
    // associated_token::create(CpiContext::new(
    //     accounts.associated_token_program.to_account_info(), //TODO Add ATA program to ix
    //     associated_token::Create {
    //         payer: accounts.payer.to_account_info(),
    //         associated_token: accounts.associated_token_program.to_account_info(),
    //         authority: accounts.token_account_pool.to_account_info(),
    //         mint: accounts.token_pool.to_account_info(),
    //         system_program: accounts.system_program.to_account_info(),
    //         token_program: accounts.pool_token_program_id.to_account_info(),
    //     },
    // ))?;
    //
    //
    // initialize_account3(
    //     CpiContext::new(
    //         accounts.pool_token_program_id.to_account_info(),
    //         InitializeAccount3 {
    //             account: accounts.token_account_pool.to_account_info(),
    //             mint: accounts.token_pool.to_account_info(),
    //             authority: accounts.pool_authority.to_account_info(),
    //         })
    // )?;


    //TODO Test it works same as hoisted version

    // anchor_spl::associated_token::create(CpiContext::new(
    //     accounts.associated_token_program.to_account_info(),
    //     Create {
    //         payer: accounts.payer.to_account_info(),
    //         associated_token: accounts.token_account_a.to_account_info(),
    //         authority: accounts.pool_authority.to_account_info(),
    //         mint: accounts.mint_a.to_account_info(),
    //         system_program: accounts.system_program.to_account_info(),
    //         token_program: accounts.mint_a_token_program.to_account_info(),
    //     },
    // ))?;
}