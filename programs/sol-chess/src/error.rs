use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("User Already In Game")]
    UserAlreadyInGame,
    #[msg("Color Not Available")]
    ColorNotAvailable,
    #[msg("Invalid Game State")]
    InvalidGameState,
    #[msg("Not User's Turn")]
    NotUsersTurn,
}
