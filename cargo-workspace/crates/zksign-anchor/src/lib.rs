use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWFKyP7Z2BhVgkz3T6o3zZ8g4V5K");

#[program]
pub mod zksign_anchor {
    use super::*;

    pub fn init_attestation(ctx: Context<InitAttestation>, data: Vec<u8>) -> Result<()> {
        let attestation = &mut ctx.accounts.attestation;
        attestation.authority = *ctx.accounts.authority.key;
        attestation.data = data;
        Ok(())
    }

    pub fn verify_and_store(_ctx: Context<InitAttestation>, _proof: Vec<u8>) -> Result<()> {
        // Mock verification: in real code call a verification library or syscall
        // For scaffold, we simply return Ok(())
        Ok(())
    }
}

#[account]
pub struct Attestation {
    pub authority: Pubkey,
    pub data: Vec<u8>,
}

#[derive(Accounts)]
pub struct InitAttestation<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 1024)]
    pub attestation: Account<'info, Attestation>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
