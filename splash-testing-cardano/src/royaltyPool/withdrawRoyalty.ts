import {getLucid} from "../lucid.ts";
import {getPrivateKey, setupWallet} from "../wallet.ts";
import {getConfig} from "../config.ts";
import {BuiltValidators} from "../types.ts";
import * as CML from '@anastasia-labs/cardano-multiplatform-lib-nodejs';
import {Data, Datum, fromHex, Lucid, toHex} from "@lucid-evolution/lucid";
import {
    RoyaltyPoolWithdrawDummyValidate,
    RoyaltyPoolWithdrawValidate
} from "../../plutus.ts";
import {credentialToAddress} from "@lucid-evolution/utils";

const nftCSBase16 = `e5d9944fec15ac918115b737d749de4c10ec515c41f8b839b051d13e`;
const nftTNBase16 = `6e6674`;
const toWithdrawX = 1_300_000n;
const toWithdrawY = 0n;
const startLovelaceValue = 10_000_000n;
const fee = 1_500_000n;

export type WithdrawRoyalty = {
    poolnft: { policy: string; name: string };
    withdrawroyaltyx: bigint;
    withdrawroyaltyy: bigint;
    royaltyaddress: string;
    royaltypubkey: string;
    fee: bigint,
    signature: string;
}

function createConfig(
    privateKey: CML.Bip32PrivateKey
): WithdrawRoyalty {

    let dataToSign = Data.to({
        poolnft: { policy: nftCSBase16, name: nftTNBase16 },
        withdrawroyaltyx: toWithdrawX,
        withdrawroyaltyy: toWithdrawY,
        royaltyaddress: privateKey.to_public().to_raw_key().hash().to_hex(),
        royaltypubkey: toHex(privateKey.to_public().to_raw_key().to_raw_bytes()),
        exfee: fee,
    }, RoyaltyPoolWithdrawDummyValidate.conf)

    let dataToSignHex = fromHex(dataToSign)

    let signature = privateKey.to_raw_key().sign(dataToSignHex).to_hex()

    return {
        poolnft: { policy: nftCSBase16, name: nftTNBase16 },
        withdrawroyaltyx: toWithdrawX,
        withdrawroyaltyy: toWithdrawY,
        royaltyaddress: privateKey.to_public().to_raw_key().hash().to_hex(),
        royaltypubkey: toHex(privateKey.to_public().to_raw_key().to_raw_bytes()),
        fee,
        signature: signature,
    }
}

function buildRoyaltyWithdrawDatum(lucid: Lucid, conf: WithdrawRoyalty): Datum {
    return Data.to({
        royaltydata: {
            poolnft: conf.poolnft,
            withdrawroyaltyx: conf.withdrawroyaltyx,
            withdrawroyaltyy: conf.withdrawroyaltyy,
            royaltyaddress: conf.royaltyaddress,
            royaltypubkey: conf.royaltypubkey,
            exfee: conf.fee,
        },
        signature: conf.signature
    }, RoyaltyPoolWithdrawValidate.conf)
}

async function main() {

    const lucid = await getLucid();
    await setupWallet(lucid);

    let privateKey = await getPrivateKey();

    const conf = await getConfig<BuiltValidators>();

    const utxos = (await lucid.wallet().getUtxos());

    const poolAddress = credentialToAddress(
        "Preprod",
        { hash: conf.validators!.royaltyWithdraw.hash, type: 'Script' },
    );

    const depositedValue = {
        lovelace: BigInt(startLovelaceValue),
    }

    const withdrawConf: WithdrawRoyalty = createConfig(privateKey)

    let cfg = buildRoyaltyWithdrawDatum(lucid, withdrawConf)

    const tx = await lucid.newTx()
        .pay.ToContract(
            poolAddress,
            { kind: "inline", value: cfg },
            depositedValue
        ).complete();

    const txId = await (await tx.sign.withWallet().complete()).submit();

    console.log(`tx: ${txId}`)

    await lucid.awaitTx(txId);
}

main()