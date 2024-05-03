use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

pub enum MovieInstruction {
    AddMovieReview {
        rating: u8,
        title: String,
        description: String,
    },
    UpdateMovieReview {
        rating: u8,
        title: String,
        description: String,
    },
    AddComment {
        comment: String,
    },
}

#[derive(BorshDeserialize)]
struct MovieReviewPayload {
    rating: u8,
    title: String,
    description: String,
}

#[derive(BorshDeserialize)]
struct CommentPayload {
    comment: String,
}

impl MovieInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match variant {
            0 => {
                let payload = MovieReviewPayload::try_from_slice(rest).unwrap();
                Self::AddMovieReview {
                    rating: payload.rating,
                    title: payload.title,
                    description: payload.description,
                }
            }
            1 => {
                let payload = MovieReviewPayload::try_from_slice(rest).unwrap();
                Self::UpdateMovieReview {
                    rating: payload.rating,
                    title: payload.title,
                    description: payload.description,
                }
            }
            2 => {
                let payload = CommentPayload::try_from_slice(rest).unwrap();
                Self::AddComment {
                    comment: payload.comment,
                }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
