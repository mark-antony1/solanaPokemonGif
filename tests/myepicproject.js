const anchor = require('@project-serum/anchor');

const { SystemProgram } = anchor.web3;

const main = async() => {
  console.log("🚀 Starting test...")

  const provider = anchor.Provider.env();
  anchor.setProvider(provider);  
  
  const program = anchor.workspace.Myepicproject;
  const baseAccount = anchor.web3.Keypair.generate();

  const tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  console.log("baseAccount.publicKey",baseAccount.publicKey)
  console.log("provider.wallet.publicKey", provider.wallet.publicKey)
  console.log("SystemProgram.programId,", SystemProgram.programId)

  console.log("📝 Your transaction signature", tx);

  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);

  console.log('👀 GIF Count', account.totalGifs.toString())

  console.log("program.rpc", program.rpc)
  await program.rpc.addGif("link", {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  account = await program.account.baseAccount.fetch(baseAccount.publicKey);

  console.log('👀 GIF Count', account.totalGifs.toString())
  console.log('👀 GIF Gif List', account.gifList)

}

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