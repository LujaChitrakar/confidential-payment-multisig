import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { Multisig } from "../target/types/multisig";
import { assert } from "chai";
import { Keypair, PublicKey } from "@solana/web3.js";
import { BN } from "bn.js";
import bs58 from "bs58";
import { config } from "dotenv";
config({ path: "./tests/.env" });

describe("multisig_program", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Multisig as Program<Multisig>;

  const logTxnSignature = (tx: string) => {
    console.log(
      "Your transaction signature",
      `https://explorer.solana.com/tx/${tx}?cluster=devnet`
    );
  };

  const getGatewayAddress = (program: Program<Multisig>) => {
    const GATEWAY_SEED = "gateway";
    const [gatewayPublicKey] = PublicKey.findProgramAddressSync(
      [Buffer.from(GATEWAY_SEED)],
      program.programId
    );
    return gatewayPublicKey;
  };

  const multiSigName = "Test";

  const [multiSig] = web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("multi_sig"),
      program.provider.publicKey.toBytes(),
      anchor.utils.bytes.utf8.encode(multiSigName),
    ],
    program.programId
  );

  const [multisigAuthority] = web3.PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("authority"), multiSig.toBytes()],
    program.programId
  );

  async function getTransactionKey(create: boolean) {
    const multisigData = await program.account.multiSig.fetch(multiSig);
    const count = create
      ? (multisigData.transactionCount as number)
      : (multisigData.transactionCount as number) - 1;

    const [transactionPda] = web3.PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("transaction"),
        multiSig.toBytes(),
        new anchor.BN(count).toBuffer("le", 4),
      ],
      program.programId
    );

    return transactionPda;
  }

  //gateway authority
  const authorityKeypair = Keypair.fromSecretKey(
    bs58.decode(process.env.AUTHORITY_PRIVATE_KEY!)
  );
  const authority = authorityKeypair.publicKey;

  //bank admin
  const adminKeypair = Keypair.fromSecretKey(
    bs58.decode(process.env.ADMIN_PRIVATE_KEY!)
  );
  const admin = adminKeypair.publicKey;

  const tokenMint = new PublicKey(
    "FMdEtYLuweboBHGR4UiTnVXXcpCF2TKhoXj5uKtLNwTH"
  );

  //GATEWAY

  it.skip("Initialize Gateway", async () => {
    const amount = new BN(1000);
    const tokenMint = new PublicKey(
      "FMdEtYLuweboBHGR4UiTnVXXcpCF2TKhoXj5uKtLNwTH"
    );
    const tx = await program.methods
      .initializeGateway(admin)
      .accounts({ authority })
      .signers([authorityKeypair])
      .rpc();

    logTxnSignature(tx);
  });

  it.skip("Attest KYC", async () => {
    const bankId = new BN(1);
    const tokenMint = new PublicKey(
      "FMdEtYLuweboBHGR4UiTnVXXcpCF2TKhoXj5uKtLNwTH"
    );
    const tx = await program.methods
      .attestKyc(bankId)
      .accounts({ authority })
      .signers([authorityKeypair])
      .rpc();

    logTxnSignature(tx);
  });

  it.skip("Register Bank", async () => {
    const bankId = new BN(1);
    const tokenMint = new PublicKey(
      "FMdEtYLuweboBHGR4UiTnVXXcpCF2TKhoXj5uKtLNwTH"
    );

    const bankName = "Everest";
    const swiftCode = "EBL";

    const tx = await program.methods
      .registerBank(bankId, bankName, swiftCode)
      .accounts({ authority, usdcMint: tokenMint })
      .signers([authorityKeypair])
      .rpc();

    logTxnSignature(tx);
  });

  it.skip("Bank Deposit", async () => {
    const bankId = new BN(1);
    const amount = new BN(10);
    const tokenMint = new PublicKey(
      "FMdEtYLuweboBHGR4UiTnVXXcpCF2TKhoXj5uKtLNwTH"
    );

    const tx = await program.methods
      .bankDeposit(bankId, amount)
      .accounts({ admin, usdcMint: tokenMint })
      .signers([adminKeypair])
      .rpc();

    logTxnSignature(tx);
  });

  it.skip("Emergency Freeze", async () => {
    const bankId = new BN(1);

    const tx = await program.methods
      .emergencyFreeze(bankId)
      .accounts({ admin })
      .signers([adminKeypair])
      .rpc();

    logTxnSignature(tx);
  });

  it.skip("It creates multi-sig!", async () => {
    const tx = await program.methods
      .createMultisig(
        [
          {
            owners: [program.provider.publicKey],
            m: 1,
            active: false,
          },
        ],
        multiSigName
      )
      .accounts({
        multiSig,
      })
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it.skip("It creates transaction!", async () => {
    const transaction = await getTransactionKey(true);

    const tx = await program.methods
      .createTransaction(0, "Add a new owner to the multi-sig")
      .accounts({
        multiSig,
        transaction,
      })
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it.skip("It adds data to the transaction!", async () => {
    const transaction = await getTransactionKey(false);

    const [txData] = web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("data"), transaction.toBytes()],
      program.programId
    );

    const instruction = await createAddOwnerTransaction();

    const tx = await program.methods
      .createTxData([instruction])
      .accounts({
        multiSig,
        transaction,
        txData,
      })
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it.skip("It adds withdraw instruction to the transaction!", async () => {
    const transaction = await getTransactionKey(false);
    const [txData] = web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("data"), transaction.toBytes()],
      program.programId
    );

    let amount = new BN(10);
    let bank_id = new BN(1);
    let recipient = Keypair.generate().publicKey;

    const withdrawIx = await program.methods
      .bankWithdraw(bank_id, amount)
      .accounts({
        recipient: recipient,
        authority: multisigAuthority,
      })
      .instruction();

    const tx = await program.methods
      .createTxData([withdrawIx])
      .accounts({
        multiSig,
        transaction,
        txData,
      })
      .rpc();

    console.log("Withdraw instruction added", tx);
  });

  it.skip("It finalizes data to the transaction!", async () => {
    const transaction = await getTransactionKey(false);

    const [txData] = web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("data"), transaction.toBytes()],
      program.programId
    );

    const tx = await program.methods
      .finalizeTxData()
      .accounts({
        multiSig,
        transaction,
        txData,
      })
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it.skip("It votes for the transaction!", async () => {
    const transaction = await getTransactionKey(false);

    const [voteRecord] = web3.PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("vote"),
        transaction.toBytes(),
        program.provider.publicKey.toBytes(),
      ],
      program.programId
    );

    const tx = await program.methods
      .voteTransaction(0, true)
      .accounts({
        multiSig,
        transaction,
        voteRecord,
      })
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it.skip("It accepts the transaction!", async () => {
    const transaction = await getTransactionKey(false);

    const tx = await program.methods
      .acceptTransaction(0)
      .accounts({
        multiSig,
        transaction,
      })
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it.skip("It executes the transaction!", async () => {
    const transaction = await getTransactionKey(false);

    const [txData] = web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("data"), transaction.toBytes()],
      program.programId
    );

    const instructionData = await program.account.txData.fetch(txData);

    const remainingAccounts = [];

    const instructions = instructionData.instructions as any[];

    instructions.forEach((ix) => {
      remainingAccounts.push({
        pubkey: ix.programId,
        isSigner: false,
        isWritable: false,
      });

      ix.keys.forEach((key: any) => {
        if (key.pubkey.toBase58() === multisigAuthority.toBase58()) {
          key.isSigner = false;
        }

        remainingAccounts.push(key);
      });
    });

    const tx = await program.methods
      .executeTransaction(0)
      .accounts({
        multiSig,
        transaction,
        txData,
      })
      .remainingAccounts(remainingAccounts)
      .rpc();

    console.log("Your transaction signature", tx);
  });

  //Transaction Builder::
  async function createAddOwnerTransaction() {
    const anchor_tx = await program.methods
      .changeMultisigRealloc({
        addOwner: {
          owner: new web3.PublicKey(
            "7YfWWiuRXf1mjDBsLCpuhoDvGLG5ny91QtGbohLF45aG"
          ),
          stratum: 0,
        },
      })
      .accounts({
        multiSig,
        authority: multisigAuthority,
      })
      .instruction();

    return anchor_tx;
  }

  async function createRemoveOwnerTransaction() {
    const anchor_tx = await program.methods
      .changeMultisig(
        {
          removeOwner: {
            owner: new web3.PublicKey(
              "7YfWWiuRXf1mjDBsLCpuhoDvGLG5ny91QtGbohLF45aG"
            ),
          },
        },
        0
      )
      .accounts({
        multiSig,
        authority: multisigAuthority,
      })
      .instruction();

    return anchor_tx;
  }

  async function createDeactivateStratumTransaction() {
    const anchor_tx = await program.methods
      .changeMultisig({ deactivateStratum: {} }, 1)
      .accounts({
        multiSig,
        authority: multisigAuthority,
      })
      .instruction();

    return anchor_tx;
  }

  async function createActivateStratumTransaction() {
    const anchor_tx = await program.methods
      .changeMultisig({ activateStratum: {} }, 1)
      .accounts({
        multiSig,
        authority: multisigAuthority,
      })
      .instruction();

    return anchor_tx;
  }

  async function createChangeMTransaction() {
    const anchor_tx = await program.methods
      .changeMultisig({ changeM: { newM: 0 } }, 1)
      .accounts({
        multiSig,
        authority: multisigAuthority,
      })
      .instruction();

    return anchor_tx;
  }
});
