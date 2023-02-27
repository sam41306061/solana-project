use anchor_lang::prelude::*;

declare_id!("8UUgrNvtDKyGTVP91txbmh1uY8EtSfVKKjbJTvrVGJAt");

#[program]
pub mod mytestproject {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result<()> {
        //get a reference to the account
        //&mut is done to get a mutable reference to base_account - this will give us the power to edit
        let base_account = &mut ctx.accounts.base_account;
        //initialize total memes
        base_account.total_memes = 0;
        Ok(())
    }

    //function to add Meme
    pub fn add_meme(ctx: Context<AddMeme>, meme_link: String) -> Result<()> {
        //get referance to account and increment total_memes
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;
        //build struct
        let item = ItemStruct {
            meme_link: meme_link.to_string(),
            user_address: *user.to_account_info().key,
        };
        //adding to the list
        base_account.memes_list.push(item);
        base_account.total_memes += 1;
        Ok(())
    }
}

//attach certain variables to the startstuffoff context
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(
        init,
        payer = user,
        space = 9000
    )] //tells solana how we want to initialize BaseAccount
    //init - will tell solana to create a new account owned by current program
    //payer = user - tells who pays for the account. here it's the user who calls this function
    //space - it will assign how much space (in bytes) is required for this account
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>, //which is data passed into the program that proves to the program that the user calling this program actually owns their wallet account.
    pub system_program: Program<'info, System>, // This actually calls the system Program - which actually runs solana
    //
}

//specify what data is needed in the AddGif context
#[derive(Accounts)]
pub struct AddMeme<'info> {
    //let's us change the value of total_memes stored in BaseAccount
    #[account(mut)] //mutable ref
    pub base_account: Account<'info, BaseAccount>,
    //if the above is not done, it won't change the data in the account and will only change it within the function
    #[account(mut)]
    pub user: Signer<'info>,
}

//creating a custom struct
//the below code is used to tell anchor how to serialize and deserialize. Since data is stored in binary this needs to be done.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub meme_link: String,
    pub user_address: Pubkey,
}

//tells our program what kind of an account it can make and what to store inside of it
#[account]
pub struct BaseAccount {
    pub total_memes: u64,
    //vec is vector which is kind of like an array. here it holds an array of item Structs
    pub memes_list: Vec<ItemStruct>,
}