import BN from "bn.js";
import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import type { Gastochain } from "../target/types/gastochain";

// Configure the client to use the local cluster
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.Gastochain as anchor.Program<Gastochain>;


async function run() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Gastochain;
  const gasto = anchor.web3.Keypair.generate();

  console.log("📍 Cuenta gasto:", gasto.publicKey.toString());

  const sigCrear = await program.methods
    .crearGasto(new anchor.BN(50), "Comida", "Alimentos")
    .accounts({
      gasto: gasto.publicKey,
      signer: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([gasto])
    .rpc();

  console.log("✅ Gasto creado. Tx:", sigCrear);

  await provider.connection.confirmTransaction(sigCrear, "confirmed");

  const cuenta = await program.account.gasto.fetch(gasto.publicKey);
  console.log("📖 Monto:", cuenta.monto.toString());
  console.log("📖 Descripción:", cuenta.descripcion);
  console.log("📖 Categoría:", cuenta.categoria);
}

run();
