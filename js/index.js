const { PublicKey, Connection } = require("@solana/web3.js");
const { performance } = require('perf_hooks');

const RPC_URL = "https://api.mainnet-beta.solana.com";
const SOLEND_PROGRAM_ID = "So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo";
const LENDING_MARKET_MAIN = "4UpD2fh7xH3VP9QQaXtsS1YY3bxzWhtfpks7FatyKvdY";
const OBLIGATION_LEN = 1300;


async function main() {
    console.log("=======================================");
    console.log("Running JS benchmark");
    var startTime = performance.now()
    const accounts = await getObligations();
    var endTime = performance.now()
    console.log(`Retrieved ${accounts.length} accounts in ${(endTime - startTime).toFixed(0)}ms.`);
}

async function getObligations() {
    const client = new Connection(RPC_URL, "confirmed");
    const solend_program_pk = new PublicKey(SOLEND_PROGRAM_ID);
    const config = {
        filters: [
            {
                memcmp: {
                    offset: 10,
                    bytes: LENDING_MARKET_MAIN,
                },
            },
            {
                dataSize: OBLIGATION_LEN,
            },
        ],
        encoding: "base64",
        commitment: client.commitment,
    };

    const accounts = await client.getProgramAccounts(solend_program_pk, config);
    return accounts;
}

main();