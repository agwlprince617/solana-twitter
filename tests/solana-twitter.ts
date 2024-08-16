import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaTwitter } from "../target/types/solana_twitter";
import { SystemProgram } from "@solana/web3.js";
import * as assert from "assert";

describe("solana-twitter", () => {
  // Configure the client to use the local cluster.

  //Setting the Provider object
  anchor.setProvider(anchor.AnchorProvider.env());

  //Use the registered provider to create a Program Object 
  const program = anchor.workspace.SolanaTwitter as Program<SolanaTwitter>;

  // it("Is initialized!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });
    it('can send a tweet', async () => {

      //Before sending the transacions to the blockchain
      const tweet = anchor.web3.Keypair.generate();
      await program.rpc.sendTweet('','Do, you know Solana?', {
        accounts: {
          //Accounts here
          tweet: tweet.publicKey,
          author: program.provider.wallet.publicKey ,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [tweet],
      });

      //Fetch the account details of the created tweet
      const tweetAccount = await program.account.tweet.fetch(tweet.publicKey);
      assert.equal(tweetAccount.author.toBase58(), program.provider.wallet.publicKey.toBase58());
      assert.equal(tweetAccount.topic, '');
      assert.equal(tweetAccount.content, 'Do, you know Solana?');
      assert.ok(tweetAccount.timestamp);
    });

    it('can send a tweet other user', async () => {

      const otherUser= anchor.web3.Keypair.generate(); 
      const signature = await program.provider.connection.requestAirdrop(otherUser.publicKey, 1000000000);
      await program.provider.connection.confirmTransaction(signature);
      

      //Before sending the transacions to the blockchain
      const tweet = anchor.web3.Keypair.generate();
      await program.rpc.sendTweet('','Do, you know Solana?', {
        accounts: {
          //Accounts here
          tweet: tweet.publicKey,
          author: otherUser.publicKey ,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [otherUser, tweet],
      });

      //Fetch the account details of the created tweet
      const tweetAccount = await program.account.tweet.fetch(tweet.publicKey);
      assert.equal(tweetAccount.author.toBase58(), otherUser.publicKey.toBase58());
      assert.equal(tweetAccount.topic, '');
      assert.equal(tweetAccount.content, 'Do, you know Solana?');
      assert.ok(tweetAccount.timestamp);
    });

});
