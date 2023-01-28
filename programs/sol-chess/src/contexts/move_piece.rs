use crate::*;

#[derive(Accounts)]
pub struct MovePiece<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,address=User::pda(payer.key()).0)]
    pub user: Account<'info, User>,
    #[account(mut)]
    pub game: Account<'info, Game>,
}
impl<'info> MovePiece<'info> {
    pub fn process(&mut self, from: Square, to: Square) -> Result<()> {
        let Self { user, game, .. } = self;

        require!(
            user.key() == game.get_current_player_turn(),
            CustomError::NotUsersTurn
        );

        game.move_piece(from, to);

        Ok(())
    }
}
