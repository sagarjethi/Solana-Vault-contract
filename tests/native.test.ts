
// Manually initialize variables that are automatically defined in Playground
const PROGRAM_ID = new web3.PublicKey("CfQvX79UamyFzp7WvhLQwnEJvvKA43y3Qceh8CrPK5sA");
const connection = new web3.Connection("https://api.devnet.solana.com", "confirmed");
const wallet = { keypair: web3.Keypair.generate() };

describe("Test", () => {
  it("initialize", async () => {
  });
});
