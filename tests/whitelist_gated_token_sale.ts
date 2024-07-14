import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { WhitelistGatedTokenSale } from "../target/types/whitelist_gated_token_sale";

describe("whitelist_gated_token_sale", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .WhitelistGatedTokenSale as Program<WhitelistGatedTokenSale>;
  const wallet = provider.wallet;

  const [salePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from(anchor.utils.bytes.utf8.encode("sale")),
      Buffer.from(anchor.utils.bytes.utf8.encode("SHIN")),
    ],
    program.programId
  );

  const [buyerPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from(anchor.utils.bytes.utf8.encode("buyer_info")),
      wallet.publicKey.toBuffer(),
    ],
    program.programId
  );

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize("SHIN", new anchor.BN(1000), new anchor.BN(100))
      .rpc();
    console.log("Your transaction signature", tx);
    const account = await program.account.sale.fetch(salePda);

    console.log(
      `
        token_name: ${account.tokenName}
        price_per_token: ${account.pricePerToken.toString()}
        sale_amount: ${account.saleAmount.toString()}
        sold_amount: ${account.soldAmount.toString()}
      `
    );
  });

  it("adds to whitelist", async () => {
    const tx = await program.methods.addToWhitelist(wallet.publicKey).rpc();

    console.log("Your transaction signature", tx);

    const account = await program.account.buyerInfo.fetch(buyerPda);
    console.log(`
      buyer: ${account.buyer.toBase58()}
      is_whitelisted: ${account.isWhitelisted}
    `);
  });

  it("purchases token", async () => {
    const tx = await program.methods.buyTokens("SHIN", new anchor.BN(10)).rpc();

    console.log("Your transaction signature", tx);
    const account = await program.account.sale.fetch(salePda);

    console.log(
      `
        token_name: ${account.tokenName}
        price_per_token: ${account.pricePerToken.toString()}
        sale_amount: ${account.saleAmount.toString()}
        sold_amount: ${account.soldAmount.toString()}
      `
    );
  });

  it("removes from whitelist", async () => {
    const tx = await program.methods
      .removeFromWhitelist(wallet.publicKey)
      .rpc();
    console.log("Your transaction signature", tx);

    const account = await program.account.buyerInfo.fetch(buyerPda);
    console.log(`
      buyer: ${account.buyer.toBase58()}
      is_whitelisted: ${account.isWhitelisted}
    `);
  });
});
