import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { TestApp } from "../target/types/test_app";

describe("testApp", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.TestApp as Program<TestApp>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.doMsg({});
    console.log("Your transaction signature", tx);
  });
});
