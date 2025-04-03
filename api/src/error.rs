use steel::*;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum ExampleProgramError {
    #[error("This is a dummy error")]
    Dummy = 0,

    #[error("Signer account does not match authority")]
    WrongSigner = 1,
}

error!(ExampleProgramError);
