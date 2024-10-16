import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VoteProgramSolana } from "../target/types/vote_program_solana";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { createMint, getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import { u64 } from '@solana/buffer-layout-utils';
import { struct } from '@solana/buffer-layout';



describe("vote-program-solana", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(anchor.AnchorProvider.env());
  const payer = provider.wallet as anchor.Wallet;
  const connection = new Connection("https://api.mainnet-beta.solana.com", "confirmed");
  ;


  console.log(payer.publicKey)


  const program = anchor.workspace.VoteProgramSolana as Program<VoteProgramSolana>;



  let walletAddy = new PublicKey("Ew824EPiCGbFnL6jkY1JP8qY6n5oRu1DhSLo59fmwkdb");

 /*let bob = async () => {
    let userVotewiftrempAccountOtherWallet = await getOrCreateAssociatedTokenAccount(
      connection,
      payer.payer,
      mintKeypair.publicKey,
      walletAddy
    );
    console.log("token wallet", userVotewiftrempAccountOtherWallet)
  
    await mintTo(
      connection,
      payer.payer,
      mintKeypair.publicKey,
      userVotewiftrempAccountOtherWallet.address,
      payer.payer,
      1e12 //100 and 1e7 is 10
    )
  }
  
  //bob(); */
  

  let mintAddyVote = new PublicKey("7xyVxmGWot6kWD3Su7g717UU4JiBWxsKGfzNtn61vbcV");



  async function createMintToken() {
    const mint = await createMint(
      connection,
      payer.payer,
      payer.publicKey,
      payer.publicKey,
      6,
      mintKeypair

    )

  }

  it("Is initialized!", async () => {
    //await createMintToken();
    //console.log(mintKeypair.publicKey)
    //console.log("mint")



  

    let [treasuryAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from("vote_vaulttremp")],
      program.programId
    )

    let [globalVoteAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from("votewiftrempglobal")],
      program.programId
    )

    console.log("global vote", globalVoteAccount)
    console.log("treasuryAccount", treasuryAccount)

    const tx = await program.methods.initialize()
      .accounts({
        signer: payer.publicKey,
        globalVoteAccount: globalVoteAccount,
        treasury_account: treasuryAccount,
        mint: mintAddyVote,
      })
      .rpc();
    console.log("Your transaction signature", tx);


  });

/*
  it("vote", async () => {

    // Define the enumeration
    enum TimeLength {
      OneMinute = 0,
      Medium = 1,
      Long = 2,
      VeryLong = 3,
    }


    //get info from globalvoteaccount
    const fetchAndParseMint = async (globalVoteAccount, connection) => {
      try {
        console.log(`Step - 1: Fetching Account Data for ${globalVoteAccount.toBase58()}`);
        let { data } = await connection.getAccountInfo(globalVoteAccount) || {};
        console.log(data)
        if (!data) return;

        console.log(`Step - 2: Deserializing Found Account Data`);
        const deserialized = globalVoteLayout.decode(data);
        console.log(deserialized);
      } catch {
        return null;
      }
    }



   


    let [treasuryAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from("vote_vaulttremp")],
      program.programId
    )

    await mintTo(
      connection,
      payer.payer,
      mintKeypair.publicKey,
      treasuryAccount,
      payer.payer,
      1e13 //100 1e8 and 1e7 is 10
    )




    let [voteInfo] = PublicKey.findProgramAddressSync(
      [Buffer.from("votewiftremp_info"), payer.publicKey.toBuffer()],
      program.programId
    )

    let [globalVoteAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from("votewiftrempglobal")],
      program.programId
    )

    fetchAndParseMint(globalVoteAccount, connection);

    let [voteAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from("votewiftremptoken"), payer.publicKey.toBuffer()],
      program.programId
    )

    console.log("voteaccount", voteAccount)
    console.log("voteInfoaccount", voteInfo)
    console.log("userVotewiftrempAccount", userVotewiftrempAccount)
    console.log("treasuryAccount", treasuryAccount)
    console.log("global vote", globalVoteAccount)

    const tx = await program.methods
      .vote(new anchor.BN(1000000), true, { fiveMonths: {} }) //number in here is amount of tokens to stake 100=100
      .signers([payer.payer])
      .accounts({
        voteInfoAcount: voteInfo,
        treasury_account: treasuryAccount,
        globalVoteAccount: globalVoteAccount,
        voteAccount: voteAccount,
        userVotewiftrempAccount: userVotewiftrempAccount.address,
        mint: mintKeypair.publicKey,
        signer: payer.publicKey,
      })
      .rpc();


    console.log("Your transaction signature place vote", tx);
  });



  it("collectVote", async () => {

    //get info from globalvoteaccount
    const fetchAndParseMint = async (globalVoteAccount, connection) => {
      try {
        console.log(`Step - 1: Fetching Account Data for ${globalVoteAccount.toBase58()}`);
        let { data } = await connection.getAccountInfo(globalVoteAccount) || {};
        if (!data) return;

        console.log(`Step - 2: Deserializing Found Account Data`);
        const deserialized = globalVoteLayout.decode(data);
        console.log(deserialized);
      } catch {
        return null;
      }
    }


    let userVotewiftrempAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      payer.payer,
      mintKeypair.publicKey,
      payer.publicKey
    );


    let [voteInfo] = PublicKey.findProgramAddressSync(
      [Buffer.from("votewiftremp_info"), payer.publicKey.toBuffer()],
      program.programId
    )

    let [voteAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from("votewiftremptoken"), payer.publicKey.toBuffer()],
      program.programId
    )


    const tx = await program.methods
      .collectVote()
      .signers([payer.payer])
      .accounts({
        voteInfoAcount: voteInfo,
        voteAccount: voteAccount,
        userVotewiftrempAccount: userVotewiftrempAccount.address,
        mint: mintKeypair.publicKey,
        signer: payer.publicKey
      })
      .rpc();
    console.log("Your transaction signature collect vote", tx);
  })

*/

});



