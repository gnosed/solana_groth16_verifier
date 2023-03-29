import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaGroth16Verifier } from "../target/types/solana_groth16_verifier";

describe("solana_groth16_verifier", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaGroth16Verifier as Program<SolanaGroth16Verifier>;

  it("Is initialized!", async () => {
    // Add your test here.
    
  });
});
