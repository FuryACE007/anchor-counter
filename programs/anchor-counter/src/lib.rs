use anchor_lang::prelude::*;

declare_id!("6s254XijhqqRP6Yn9FpSgKrNNbGrj9cD64QuRfTXjici");

#[program]
pub mod anchor_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
