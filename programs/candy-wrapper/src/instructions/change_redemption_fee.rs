use anchor_lang::prelude::*;

use crate::{error::CustomError, state::Authority};

#[derive(Accounts)]
pub struct RedemptionFeeCtx<'info> {
    #[account(
        mut,
        constraint = payer.key() == authority.load()?.admin,
    )]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub authority: AccountLoader<'info, Authority>,
}

pub fn change_redemption_fee_handler(
    ctx: Context<RedemptionFeeCtx>,
    fee_basis_pts: u16,
) -> Result<()> {
    let authority = &mut ctx.accounts.authority.load_mut()?;
    require!(
        fee_basis_pts <= 100,
        CustomError::RedemptionFeeBasisPtsCannotExceed100
    );
    require!(authority.mutable == 1, CustomError::MintIsImmutable);
    authority.redemption_fee_basis_pts = fee_basis_pts;
    Ok(())
}
