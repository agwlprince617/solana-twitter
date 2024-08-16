use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_lang::solana_program::entrypoint::ProgramResult;



declare_id!("MdueQEiG1zRdsjZXhazkLshBF7J9NagdNZFGBXsWY8p");

// [error_code]
// pub enum ErrorCode {
//     #[msg("Provided topic is too long")]
//     TopicTooLong,
//     #[msg("Provided content too long")]
//     ContentTooLong,
// }

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn send_tweet(ctx: Context<SendTweet>, topic: String, content: String) -> ProgramResult {
        let tweet: &mut Account<Tweet> = &mut ctx.accounts.tweet;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();

        // if topic.chars().count()>50 {
        //     return Err(ErrorCode::TopicTooLong.into())
        // }

        // if content.chars().count()>280 {
        //     return Err(ErrorCode::ContentTooLong.into())
        // }

        tweet.author = *author.key;
        tweet.timestamp= clock.unix_timestamp;
        tweet.topic=topic;
        tweet.content=content;
        Ok(())
    }
}

// #[derive(Accounts)]
// pub struct Initialize {}


//Define the context {all the accounts that are necessary for the instruction to do the job}
#[derive(Accounts)]
pub struct SendTweet<'info> {
    #[account(init, payer=author, space=Tweet::LEN)]
    pub tweet: Account<'info, Tweet>,
    #[account(mut)]
    pub author: Signer<'info>,  
    #[account(address = system_program::ID)]
    pub system_program: Program<'info,System>,
}

//1. Define the structure of tweet account
#[account]
pub struct Tweet{
    pub author: Pubkey,
    pub timestamp: i64,
    pub topic:String,
    pub content: String, 
}


//2. Adding useful constants for sizing properties
const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGHT_PREFIX: usize = 4;
const MAX_TOPIC_LENGHT: usize = 50*4;
const MAX_CONTENT_LENGTH: usize = 280*4;


//3. Add a constant in the tweet that provides the total size 
impl Tweet {
    const LEN: usize = DISCRIMINATOR_LENGTH
    + PUBLIC_KEY_LENGTH
    + TIMESTAMP_LENGTH
    + STRING_LENGHT_PREFIX + MAX_TOPIC_LENGHT
    + STRING_LENGHT_PREFIX + MAX_CONTENT_LENGTH;
}



