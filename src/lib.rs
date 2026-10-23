use anchor_lang::prelude::*;

declare_id!("3qSXF7xaYya1vgmMjXfZn9bMMN1Uj4ShgbbTo6te2vEA");

#[program]
pub mod guestbook_test {
    use super::*;

    // Sign the guestbook: creates one Entry account per signature.
    // `id` is chosen by the client (e.g. a timestamp) so entries never collide.
    pub fn sign(ctx: Context<Sign>, id: u64, name: String, message: String) -> Result<()> {
        let name = name.trim().to_string();
        let message = message.trim().to_string();
        require!(!name.is_empty() && !message.is_empty(), GuestbookError::Empty);
        require!(name.len() <= 60, GuestbookError::NameTooLong);
        require!(message.len() <= 280, GuestbookError::MessageTooLong);

        let e = &mut ctx.accounts.entry;
        e.author = ctx.accounts.author.key();
        e.id = id;
        e.timestamp = Clock::get()?.unix_timestamp;
        e.name = name;
        e.message = message;
        msg!("{} signed the guestbook: {}", e.author, e.message);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct Sign<'info> {
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(
        init,
        payer = author,
        space = 8 + 32 + 8 + 8 + (4 + 60) + (4 + 280),
        seeds = [b"entry", author.key().as_ref(), &id.to_le_bytes()],
        bump
    )]
    pub entry: Account<'info, Entry>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Entry {
    pub author: Pubkey,
    pub id: u64,
    pub timestamp: i64,
    pub name: String,
    pub message: String,
}

#[error_code]
pub enum GuestbookError {
    #[msg("Name and message are required")]
    Empty,
    #[msg("Name must be 60 bytes or fewer")]
    NameTooLong,
    #[msg("Message must be 280 bytes or fewer")]
    MessageTooLong,
}
