const anchor = require('@project-serum/anchor');
//getting the system program
const { SystemProgram } = anchor.web3;

const main = async () => {
  console.log('Starting test.....!');

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Mytestproject;

  //create an account keypair for our program - create credentials for the base account we are creating
  const baseAccount = anchor.web3.Keypair.generate();

  //call startstuffoff and give it the params
  let tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  console.log('TX hash ', tx);

  //fetch data from account - account in solana is actually a place to write and read data
  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('Meme Count', account.totalMemes.toString());

  await program.rpc.addMeme("insertamemelinkhere",{
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('Meme Count', account.totalMemes.toString());

  console.log("memes list", account.memesList)
};

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();
