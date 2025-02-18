import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { KsgNft } from "../target/types/ksg_nft";
import { PublicKey } from "@solana/web3.js";

describe("ksg-nft", () => {
  const id = new anchor.BN(6);
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.KsgNft as Program<KsgNft>;

  it("Is initialized!", async () => {
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Is NFT created!", async () => {
    const tx = await program.methods.createSingleNft(id, `KSG NFT ${id}`, `KSG#${id}`, "https://moccasin-urgent-shrew-838.mypinata.cloud/ipfs/bafkreiceu3llxj3px4mtylh245xpskecz7qxfhp744et7w4wt6znqcvipm").rpc();
    console.log("Your transaction signature", tx);
  });

  it("Is NFT transferred!", async () => {
    const tx = await program.methods.transferNft(id).accounts({
      from: program.provider.publicKey,
      to: new PublicKey("TES831ca9od458WBCgxFqEJAzn7KKZwaDW8jePMJwyo"),
    }).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Is NFT updated!", async () => {
    const tx = await program.methods.updateNftMetadata(id, `KSG NFT ${id} Updated`, `KSG#${id}U`, "https://moccasin-urgent-shrew-838.mypinata.cloud/ipfs/bafkreiceu3llxj3px4mtylh245xpskecz7qxfhp744et7w4wt6znqcvipm").rpc();
    console.log("Your transaction signature", tx);
  });
});
