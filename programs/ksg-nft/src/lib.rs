use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::{
    create_master_edition_v3,
    create_metadata_accounts_v3,
    CreateMasterEditionV3,
    CreateMetadataAccountsV3,
    update_metadata_accounts_v2,
    Metadata,
};
use anchor_spl::token::{ self, mint_to, Mint, MintTo, Token, TokenAccount };
use mpl_token_metadata::types::{ /* Collection, */ Creator, DataV2 };

declare_id!("67uqmtwUpEk6w8Nnopv3FoZxs3j2stbbKqabE3wNGbZ1");

#[program]
pub mod ksg_nft {
    use anchor_spl::metadata::UpdateMetadataAccountsV2;

    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Initialized!");
        Ok(())
    }
    pub fn create_single_nft(
        ctx: Context<CreateNFT>,
        id: u64,
        name: String,
        symbol: String,
        uri: String
    ) -> Result<()> {
        msg!("Creating seeds");
        let id_bytes = id.to_le_bytes();
        let seeds = &["mint".as_bytes(), id_bytes.as_ref(), &[ctx.bumps.mint]];

        msg!("Run mint_to");

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    authority: ctx.accounts.signer.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                },
                &[&seeds[..]]
            ),
            1 // 1 token
        )?;

        msg!("Run create metadata accounts v3");

        create_metadata_accounts_v3(
            CpiContext::new_with_signer(
                ctx.accounts.metadata_program.to_account_info(),
                CreateMetadataAccountsV3 {
                    payer: ctx.accounts.signer.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    metadata: ctx.accounts.nft_metadata.to_account_info(),
                    mint_authority: ctx.accounts.signer.to_account_info(),
                    update_authority: ctx.accounts.signer.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
                &[&seeds[..]]
            ),
            DataV2 {
                name,
                symbol,
                uri,
                seller_fee_basis_points: 500,
                creators: Some(
                    vec![Creator {
                        address: ctx.accounts.signer.key(),
                        verified: true,
                        share: 100,
                    }]
                ),
                collection: None,
                uses: None,
            },
            true,
            true,
            None
        )?;

        msg!("Run create master edition v3");

        create_master_edition_v3(
            CpiContext::new_with_signer(
                ctx.accounts.metadata_program.to_account_info(),
                CreateMasterEditionV3 {
                    edition: ctx.accounts.master_edition_account.to_account_info(),
                    payer: ctx.accounts.signer.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    metadata: ctx.accounts.nft_metadata.to_account_info(),
                    mint_authority: ctx.accounts.signer.to_account_info(),
                    update_authority: ctx.accounts.signer.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
                &[&seeds[..]]
            ),
            Some(1)
        )?;

        msg!("Minted NFT successfully");

        Ok(())
    }

    pub fn transfer_nft(ctx: Context<TransferNFT>, _id: u64) -> Result<()> {
        let cpi_accounts = token::Transfer {
            from: ctx.accounts.from_ata.to_account_info(),
            to: ctx.accounts.to_ata.to_account_info(),
            authority: ctx.accounts.from.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, 1)?;

        Ok(())
    }

    pub fn update_nft_metadata(
        ctx: Context<UpdateNFTMetadata>,
        id: u64,
        name: String,
        symbol: String,
        uri: String
    ) -> Result<()> {
        msg!("Creating seeds");
        let id_bytes = id.to_le_bytes();
        let seeds = &["mint".as_bytes(), id_bytes.as_ref(), &[ctx.bumps.mint]];

        msg!("Updating metadata");

        update_metadata_accounts_v2(
            CpiContext::new_with_signer(
                ctx.accounts.metadata_program.to_account_info(),
                UpdateMetadataAccountsV2 {
                    metadata: ctx.accounts.nft_metadata.to_account_info(),
                    update_authority: ctx.accounts.signer.to_account_info(),
                },
                &[&seeds[..]]
            ),
            Some(ctx.accounts.signer.key()),
            Some(DataV2 {
                name,
                symbol,
                uri,
                seller_fee_basis_points: 500,
                creators: Some(
                    vec![Creator {
                        address: ctx.accounts.signer.key(),
                        verified: true,
                        share: 100,
                    }]
                ),
                collection: None,
                uses: None,
            }),
            None,
            Some(true)
        )?;

        Ok(())
    }

    // pub fn mint_to_collection(
    //     ctx: Context<MintToCollection>,
    //     id_collection: u64,
    //     id_nft: u64,
    //     name: String,
    //     symbol: String,
    //     uri: String,
    // ) -> Result<()> {
    //     msg!("Creating seeds");
    //     let id_bytes = id_collection.to_le_bytes();
    //     let id_nft_bytes = id_nft.to_le_bytes();
    //     let seeds = &[
    //         "mint".as_bytes(),
    //         id_bytes.as_ref(),
    //         id_nft_bytes.as_ref(),
    //         &[ctx.bumps.mint],
    //     ];

    //     msg!("Run mint_to");

    //     mint_to(
    //         CpiContext::new_with_signer(
    //             ctx.accounts.token_program.to_account_info(),
    //             MintTo {
    //                 authority: ctx.accounts.authority.to_account_info(),
    //                 to: ctx.accounts.token_account.to_account_info(),
    //                 mint: ctx.accounts.mint.to_account_info(),
    //             },
    //             &[&seeds[..]]
    //         ),
    //         1 // 1 token
    //     )?;

    //     msg!("Run create metadata accounts v3");

    //     create_metadata_accounts_v3(
    //         CpiContext::new_with_signer(
    //             ctx.accounts.metadata_program.to_account_info(),
    //             CreateMetadataAccountsV3 {
    //                 payer: ctx.accounts.payer.to_account_info(),
    //                 mint: ctx.accounts.mint.to_account_info(),
    //                 metadata: ctx.accounts.nft_metadata.to_account_info(),
    //                 mint_authority: ctx.accounts.authority.to_account_info(),
    //                 update_authority: ctx.accounts.authority.to_account_info(),
    //                 system_program: ctx.accounts.system_program.to_account_info(),
    //                 rent: ctx.accounts.rent.to_account_info(),
    //             },
    //             &[&seeds[..]]
    //         ),
    //         DataV2 {
    //             name,
    //             symbol,
    //             uri,
    //             seller_fee_basis_points: 0,
    //             creators: Some(
    //                 vec![Creator {
    //                     address: ctx.accounts.payer.key(),
    //                     verified: true,
    //                     share: 100,
    //                 }]
    //             ),
    //             collection: Some(Collection {
    //                 key: ctx.accounts.collection.key(),
    //                 verified: false,
    //             }),
    //             uses: None,
    //         },
    //         true,
    //         true,
    //         None
    //     )?;

    //     msg!("Run create master edition v3");

    //     create_master_edition_v3(
    //         CpiContext::new_with_signer(
    //             ctx.accounts.metadata_program.to_account_info(),
    //             CreateMasterEditionV3 {
    //                 edition: ctx.accounts.master_edition_account.to_account_info(),
    //                 payer: ctx.accounts.payer.to_account_info(),
    //                 mint: ctx.accounts.mint.to_account_info(),
    //                 metadata: ctx.accounts.nft_metadata.to_account_info(),
    //                 mint_authority: ctx.accounts.authority.to_account_info(),
    //                 update_authority: ctx.accounts.authority.to_account_info(),
    //                 system_program: ctx.accounts.system_program.to_account_info(),
    //                 token_program: ctx.accounts.token_program.to_account_info(),
    //                 rent: ctx.accounts.rent.to_account_info(),
    //             },
    //             &[&seeds[..]]
    //         ),
    //         Some(1)
    //     )?;

    //     msg!("Minted NFT successfully");

    //     Ok(())
    // }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CreateNFT<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 0,
        mint::authority = signer,
        mint::freeze_authority = signer,
        seeds = ["mint".as_bytes(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition".as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    /// CHECK:
    pub master_edition_account: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    /// CHECK:
    pub nft_metadata: UncheckedAccount<'info>,
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct TransferNFT<'info> {
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = from
    )]
    pub from_ata: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = from,
        associated_token::mint = mint,
        associated_token::authority = to
    )]
    pub to_ata: Account<'info, TokenAccount>,
    /// CHECK:
    #[account(mut)]
    pub to: AccountInfo<'info>,
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(seeds = ["mint".as_bytes(), id.to_le_bytes().as_ref()], bump)]
    pub mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct UpdateNFTMetadata<'info> {
    #[account(seeds = ["mint".as_bytes(), id.to_le_bytes().as_ref()], bump)]
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    /// CHECK:
    pub nft_metadata: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub metadata_program: Program<'info, Metadata>,
}

// #[derive(Accounts)]
// #[instruction(id_collection: u64, id_nft: u64)]
// pub struct MintToCollection<'info> {
//     #[account(mut)]
//     pub authority: Signer<'info>,
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     #[account(
//         init,
//         payer = payer,
//         mint::decimals = 0,
//         mint::authority = authority,
//         mint::freeze_authority = authority,
//         seeds = [
//             "mint".as_bytes(),
//             id_collection.to_le_bytes().as_ref(),
//             id_nft.to_le_bytes().as_ref(),
//         ],
//         bump
//     )]
//     pub mint: Account<'info, Mint>,
//     #[account(
//         init_if_needed,
//         payer = payer,
//         associated_token::mint = mint,
//         associated_token::authority = payer
//     )]
//     pub token_account: Account<'info, TokenAccount>,
//     pub associated_token_program: Program<'info, AssociatedToken>,
//     pub rent: Sysvar<'info, Rent>,
//     pub system_program: Program<'info, System>,
//     pub token_program: Program<'info, Token>,
//     pub metadata_program: Program<'info, Metadata>,
//     #[account(
//         mut,
//         seeds = [
//             b"metadata".as_ref(),
//             metadata_program.key().as_ref(),
//             mint.key().as_ref(),
//             b"edition".as_ref(),
//         ],
//         bump,
//         seeds::program = metadata_program.key()
//     )]
//     /// CHECK:
//     pub master_edition_account: UncheckedAccount<'info>,
//     #[account(
//         mut,
//         seeds = [
//             b"metadata".as_ref(),
//             metadata_program.key().as_ref(),
//             mint.key().as_ref(),
//         ],
//         bump,
//         seeds::program = metadata_program.key()
//     )]
//     /// CHECK:
//     pub nft_metadata: UncheckedAccount<'info>,
//     /// CHECK:
//     pub collection: UncheckedAccount<'info>,
// }