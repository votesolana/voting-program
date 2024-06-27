import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VoteProgramSolana } from "../target/types/vote_program_solana";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { createMint, getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";

describe("vote-program-solana", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(anchor.AnchorProvider.env());
  const payer = provider.wallet as anchor.Wallet;
  const connection = new Connection("http://127.0.0.1:8899", "confirmed");
  const mintKeypair = Keypair.fromSecretKey(new Uint8Array([
    106, 161, 133, 170,  47,  84,  34,  10,  51,  38, 126,
     28, 165,  73, 222,  35,  53, 223, 147, 161,  27,  93,
    127,  21,  79, 214, 100, 117,  39, 240,   7, 203, 122,
    197,  84, 167, 209, 141, 227, 190,  41, 174, 122, 255,
     21, 102,  98,  37, 100,  19, 144,  78, 251, 179, 167,
    150, 118, 127,   0, 232, 131,  29,   3, 244
  ]));


  const program = anchor.workspace.VoteProgramSolana as Program<VoteProgramSolana>;

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

    let [treasuryAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from("vote_vaulttremp")],
      program.programId
    )

    console.log(treasuryAccount)
    const tx = await program.methods.initialize()
    .accounts({
      signer: payer.publicKey,
      treasury_account: treasuryAccount,
      mint: mintKeypair.publicKey
    } )
    .rpc();
    console.log("Your transaction signature", tx);
  });

  it("vote", async() => {
    let userVotewiftrempAccount = await  getOrCreateAssociatedTokenAccount(
      connection,
      payer.payer,
      mintKeypair.publicKey,
      payer.publicKey
    );

   /* await mintTo(
      connection,
      payer.payer,
      mintKeypair.publicKey,
      userVotewiftrempAccount.address,
      payer.payer,
      1e8 //100 and 1e7 is 10
    ) */

    let [voteInfo] = PublicKey.findProgramAddressSync(
      [Buffer.from("votewiftremp_info"), payer.publicKey.toBuffer()],
      program.programId
    )

    let [voteAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from("votewiftremptoken"), payer.publicKey.toBuffer()],
      program.programId
    )

    const bob = await getOrCreateAssociatedTokenAccount(
      connection,
      payer.payer,
      mintKeypair.publicKey,
      payer.publicKey
    )
    console.log("voteaccount", voteAccount)
    console.log("userVotewiftrempAccount", userVotewiftrempAccount)
    const tx = await program.methods
      .vote(new anchor.BN(101)) //number in here is amount of tokens to stake 100=100
      .signers([payer.payer])
      .accounts({
        voteInfoAcount: voteInfo,
        voteAccount: voteAccount,
        userVotewiftrempAccount: userVotewiftrempAccount.address,
        mint: mintKeypair.publicKey,
        signer: payer.publicKey
      })
      .rpc();    
  

      console.log("Your transaction signature place vote", tx);
  });


  
  it("collectVote", async () => {

    let [treasuryAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from("vote_vaulttremp")],
      program.programId
    )

    let userVotewiftrempAccount = await  getOrCreateAssociatedTokenAccount(
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
      treasury: treasuryAccount,
      voteInfoAcount: voteInfo,
      voteAccount: voteAccount,
      userVotewiftrempAccount: userVotewiftrempAccount.address,
      mint: mintKeypair.publicKey,
      signer: payer.publicKey
    })
    .rpc();    
    console.log("Your transaction signature collect vote", tx);
  })

 
});



