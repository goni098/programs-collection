use anchor_lang::{prelude::*, solana_program::program::invoke_signed};
use anchor_spl::token::{Mint, Token};
use mpl_token_metadata::{
    instructions::{CreateMetadataAccountV3, CreateMetadataAccountV3InstructionArgs},
    types::DataV2,
};

#[derive(Accounts)]
#[instruction(
    params: Metadata
)]
pub struct InitToken<'info> {
    /// CHECK: New Metaplex Account being created
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    #[account(
        init,
        seeds = [b"mint"],
        bump,
        payer = payer,
        mint::decimals = params.decimals,
        mint::authority = mint,
    )]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    /// CHECK: account constraint checked in account trait
    #[account(address = mpl_token_metadata::ID)]
    pub token_metadata_program: UncheckedAccount<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct Metadata {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
}

pub fn handler(ctx: Context<InitToken>, metadata: Metadata) -> Result<()> {
    let seeds = &["mint".as_bytes(), &[ctx.bumps.mint]];
    let signer = [&seeds[..]];

    let account_info = vec![
        ctx.accounts.metadata.to_account_info(),
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.token_metadata_program.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.rent.to_account_info(),
    ];

    invoke_signed(
        &CreateMetadataAccountV3 {
            metadata: ctx.accounts.metadata.key(),
            system_program: ctx.accounts.system_program.key(),
            mint: ctx.accounts.mint.key(),
            mint_authority: ctx.accounts.payer.key(),
            payer: ctx.accounts.payer.key(),
            rent: None,
            update_authority: (ctx.accounts.payer.key(), false),
        }
        .instruction(CreateMetadataAccountV3InstructionArgs {
            collection_details: None,
            data: DataV2 {
                collection: None,
                creators: None,
                name: metadata.name,
                uri: metadata.uri,
                seller_fee_basis_points: 0,
                symbol: metadata.symbol,
                uses: None,
            },
            is_mutable: true,
        }),
        account_info.as_slice(),
        &signer,
    )?;

    msg!("Token mint created successfully.");

    Ok(())
}
