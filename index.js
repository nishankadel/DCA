const sol = require("@solana/web3.js");
const BufferLayout = require("buffer-layout");
const spl = require("@solana/spl-token");

const cluster = sol.clusterApiUrl("devnet", true);
const conn = new sol.Connection(cluster);

const programAddress = "9DmcPZacbsvfCFsNLQxr3KKrs56k7e48wWCe7CCogoXA";

let admin = sol.Keypair.generate();
let user = sol.Keypair.generate();

const depositLayout = BufferLayout.struct([
  BufferLayout.u8("instruction"),
  BufferLayout.blob(8, "amount"),
]);

const withdrawLayout = BufferLayout.struct([
  BufferLayout.u8("instruction"),
  BufferLayout.blob(8, "amount"),
]);

const depositStream = async (connection) => {
  var data = Buffer.alloc(depositLayout.span);
  depositLayout.encode(
    {
      instruction: 0,
      amount: new spl.u64(10000).toBuffer(),
    },
    data
  );
  // pda is a new keypair where the funds are sent, and program metadata
  // is kept and updated by the program.
    const pda = new sol.Keypair();

  console.log(pda);
  console.log("ADMIN: %s", admin.publicKey.toBase58());
  console.log("USER:   %s", user.publicKey.toBase58());
  console.log("PDA:   %s", pda.publicKey.toBase58());
  console.log("DATA:", data);

  const instruction = new sol.TransactionInstruction({
    keys: [
      {
        // user is the stream sender.
        pubkey: user.publicKey,
        isSigner: true,
        isWritable: true,
      },
      {
        // admin is the stream recipient.
        pubkey: admin.publicKey,
        isSigner: false,
        isWritable: true,
      },
      // pda to store data
      {
        pubkey: pda.publicKey,
        isSigner: true,
        isWritable: true,
      },
      // withdraw data
      {
        pubkey: "GpGUwJfyTrmkLVTiJY6QD6dPFde4qgdehd3m4mF7p53D",
        isSigner: false,
        isWritable: true,
      },
      {
        // This is the system program public key.
        pubkey: sol.SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
    ],
    programId: new sol.PublicKey(programAddress),
    data: data,
  });
  // Transaction signed by Alice and the new pda.
  tx = new sol.Transaction().add(instruction);
  return await sol.sendAndConfirmTransaction(connection, tx, [admin, pda]);
};

// const withdrawStream = async (connection) => {
//   var data = Buffer.alloc(withdrawLayout.span);
//   withdrawLayout.encode(
//     {
//       // 1 means withdraw in the Rust program.
//       instruction: 1,
//       // When amount is 0 lamports, then withdraw everything
//       // that is unlocked on the stream. Otherwise, arbitrary
//       // values are allowed.
//       amount: new spl.u64(7).toBuffer(),
//     },
//     data
//   );
//   const p = new sol.Keypair();
//   let pda = await sol.PublicKey.findProgramAddress(
//     [p.Buffer()],
//     programAddress
//   );

//   console.log("ADMIN: %s", admin.publicKey.toBase58());
//   console.log("USER:   %s", user.publicKey.toBase58());
//   console.log("PDA:   %s", pda.publicKey.toBase58());
//   console.log("DATA:", data);

//   const instruction = new sol.TransactionInstruction({
//     keys: [
//       {
//         // withdraw from admin account.
//         pubkey: admin.publicKey,
//         isSigner: false,
//         isWritable: true,
//       },
//       {
//         // withdraw by user account.
//         pubkey: user.publicKey,
//         isSigner: true,
//         isWritable: true,
//       },
//       {
//         // master pda
//         pubkey: pda,
//         isSigner: false,
//         isWritable: true,
//       },
//       {
//         // data storage pda
//         pubkey: "AJdf7xNM1MG9z3Sqtjr8Rjxtk1Bh3jsJJdCmthKJoW4Z",
//         isSigner: false,
//         isWritable: true,
//       },
//       {
//         // This is the system program public key.
//         pubkey: sol.SystemProgram.programId,
//         isSigner: false,
//         isWritable: false,
//       },
//     ],
//     programId: new sol.PublicKey(programAddress),
//     data: data,
//   });
//   // Transaction signed by Bob.
//   tx = new sol.Transaction().add(instruction);
//   return await sol.sendAndConfirmTransaction(connection, tx, [user]);
// };

depositStream(conn);
// withdrawStream(conn);
