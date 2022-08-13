import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Te1 } from "../target/types/te1";

describe("te1", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Te1 as Program<Te1>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
