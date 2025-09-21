import {
  AddressLookupTableAccount,
  BlockhashWithExpiryBlockHeight,
  ComputeBudgetProgram,
  Connection,
  PublicKey,
  Transaction,
  TransactionInstruction,
  TransactionMessage,
  VersionedTransaction,
} from "@solana/web3.js";

export async function retryTxn(
  transaction: Transaction,
  blockhashContext: BlockhashWithExpiryBlockHeight,
  connection: Connection
) {
  const { lastValidBlockHeight } = blockhashContext;
  let blockheight = await connection.getBlockHeight();

  let txn = "";

  while (blockheight < lastValidBlockHeight) {
    txn = await connection.sendRawTransaction(transaction.serialize(), {
      skipPreflight: true,
      maxRetries: 0,
    });
    await new Promise((r) => setTimeout(r, 1000));
    const status = await connection
      .getSignatureStatuses([txn])
      .then((statuses) => statuses.value[0]);

    if (
      status?.confirmationStatus === "confirmed" ||
      status?.confirmationStatus === "finalized"
    ) {
      if (status.err) {
        const error =
          typeof status.err === "string"
            ? status.err
            : JSON.stringify(status.err);

        throw new Error(error);
      } else {
        return txn;
      }
    }

    blockheight = await connection.getBlockHeight();
  }

  throw new Error(`Failed to confirm transaction : ${txn}`);
}

const API_ENDPOINT = "https://lite-api.jup.ag/swap/v1";

export const getQuote = async (
  fromMint: PublicKey,
  toMint: PublicKey,
  amount: number
) => {
  const url = `${API_ENDPOINT}/quote?outputMint=${toMint.toBase58()}&inputMint=${fromMint.toBase58()}&amount=${amount}&slippageBps=50&onlyDirectRoutes=true&dynamicSlippage=true`;

  return fetch(url).then((response) => response.json());
};

export const getSwapIx = async (
  user: PublicKey,
  outputAccount: PublicKey,
  quote: any
) => {
  const data = {
    quoteResponse: quote,
    userPublicKey: user.toBase58(),
    wrapAndUnwrapSol: false,
    destinationTokenAccount: outputAccount.toBase58(),
    useSharedAccounts: true,
  };
  return fetch(`${API_ENDPOINT}/swap-instructions`, {
    method: "POST",
    headers: {
      Accept: "application/json",
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  }).then((response) => response.json());
};

export const getAddressLookupTableAccounts = async (
  keys: string[],
  connection: Connection
): Promise<AddressLookupTableAccount[]> => {
  const addressLookupTableAccountInfos =
    await connection.getMultipleAccountsInfo(
      keys.map((key) => new PublicKey(key))
    );

  return addressLookupTableAccountInfos.reduce((acc, accountInfo, index) => {
    const addressLookupTableAddress = keys[index];
    if (accountInfo) {
      const addressLookupTableAccount = new AddressLookupTableAccount({
        key: new PublicKey(addressLookupTableAddress),
        state: AddressLookupTableAccount.deserialize(accountInfo.data),
      });
      acc.push(addressLookupTableAccount);
    }

    return acc;
  }, new Array<AddressLookupTableAccount>());
};

export const createSmartTxn = async (
  instructions: Array<TransactionInstruction>,
  payer: PublicKey,
  addressLookupTableAccounts: AddressLookupTableAccount[],
  connection: Connection
) => {
  const blockhashWithExpiryBlockHeight = await connection.getLatestBlockhash(
    "confirmed"
  );

  instructions.push(
    ComputeBudgetProgram.setComputeUnitLimit({ units: 500_000 })
  );

  const transaction = new VersionedTransaction(
    new TransactionMessage({
      instructions,
      payerKey: payer,
      recentBlockhash: blockhashWithExpiryBlockHeight.blockhash,
    }).compileToV0Message(addressLookupTableAccounts)
  );

  return {
    transaction,
    blockhashContext: blockhashWithExpiryBlockHeight,
  };
};
