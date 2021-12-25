use anchor_lang::prelude::*;

declare_id!("7efLeRxvLmFrQcGr9TUCC6vzJTEGa1q8689eoKvYnT3r");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    base_account.total_gifs = 0;
    Ok(())
  }

  pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

    let item_struct = ItemStruct {
      gif_link: gif_link.to_string(),
      user_address: *user.to_account_info().key,
      upvotes: 0,
      id: base_account.total_gifs.to_string(),
    };


    base_account.total_gifs += 1;
    base_account.gif_list.push(item_struct);
    Ok(())
  }

  pub fn update_item(ctx: Context<UpdateItem>, id: String) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    let index = id.parse::<i32>().unwrap();
    base_account.gif_list[index as usize].upvotes += 1;
    Ok(())
  }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
  #[account(init, payer = user, space = 9000)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateItem<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
  pub gif_link: String,
  pub user_address: Pubkey,
  pub upvotes: u64,
  pub id: String,
}

#[account]
pub struct BaseAccount {
  pub total_gifs: u64,
  pub gif_list: Vec<ItemStruct>,
}
