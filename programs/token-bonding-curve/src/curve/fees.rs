use crate::error::SwapError;
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
};

/// Encapsulates the fee information for swap operations
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Fees {
    /// Owner trading fee numerator (1%)
    pub owner_trade_fee_numerator: u64,
    /// Owner trading fee denominator (100%)
    pub owner_trade_fee_denominator: u64,
}

impl Fees {
    /// Calculate the owner trading fee in SOL
    pub fn owner_trading_fee(&self, sol_amount: u64) -> Option<u64> {
        sol_amount
            .checked_mul(self.owner_trade_fee_numerator)?
            .checked_div(self.owner_trade_fee_denominator)
    }

    /// Validate that the fee is reasonable (1% or less)
    pub fn validate(&self) -> Result<(), SwapError> {
        if self.owner_trade_fee_numerator > self.owner_trade_fee_denominator {
            return Err(SwapError::InvalidFee.into());
        }
        Ok(())
    }
}

/// IsInitialized is required to use `Pack::pack` and `Pack::unpack`
impl IsInitialized for Fees {
    fn is_initialized(&self) -> bool {
        true
    }
}

impl Sealed for Fees {}
impl Pack for Fees {
    const LEN: usize = 16;
    fn pack_into_slice(&self, output: &mut [u8]) {
        let output = array_mut_ref![output, 0, 16];
        let (owner_trade_fee_numerator, owner_trade_fee_denominator) =
            mut_array_refs![output, 8, 8];
        *owner_trade_fee_numerator = self.owner_trade_fee_numerator.to_le_bytes();
        *owner_trade_fee_denominator = self.owner_trade_fee_denominator.to_le_bytes();
    }

    fn unpack_from_slice(input: &[u8]) -> Result<Fees, ProgramError> {
        let input = array_ref![input, 0, 16];
        #[allow(clippy::ptr_offset_with_cast)]
        let (owner_trade_fee_numerator, owner_trade_fee_denominator) =
            array_refs![input, 8, 8];
        Ok(Self {
            owner_trade_fee_numerator: u64::from_le_bytes(*owner_trade_fee_numerator),
            owner_trade_fee_denominator: u64::from_le_bytes(*owner_trade_fee_denominator),
        })
    }
}