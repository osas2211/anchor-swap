use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenAccount, TokenInterface, TransferChecked, transfer_checked, Mint};

pub fn transfer_token<'info>(
  from: &InterfaceAccount<'info, TokenAccount>,
  to: &InterfaceAccount<'info, TokenAccount>,
  amount: &u64,
  mint: &InterfaceAccount<'info, Mint>,
  authority: &Signer<'info>,
  token_program: &Interface<'info, TokenInterface>
)-> Result<()>{
  let transfer_accounts_args = TransferChecked{
    from: from.to_account_info(),
    to: to.to_account_info(),
    mint: mint.to_account_info(),
    authority: authority.to_account_info(),
  };

  let cpi_context = CpiContext::new(token_program.to_account_info(), transfer_accounts_args);
  match transfer_checked(cpi_context, *amount, mint.decimals){
     Ok(_)=> return Ok(()),
     Err(err)=> err
  };
  Ok(())
}