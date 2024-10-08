import {getLucid} from "../lucid.ts";
import {getPrivateKey, setupWallet} from "../wallet.ts";
import {getConfig} from "../config.ts";
import {asUnit, BuiltValidators} from "../types.ts";
import {getUtxoWithToken} from "../balance/balancePool.ts";
import * as CML from '@anastasia-labs/cardano-multiplatform-lib-nodejs';
import {Data, Datum, fromHex, Lucid, toHex} from "@lucid-evolution/lucid";
import {RoyaltyPoolPoolValidatePool, RoyaltyPoolWithdrawValidate} from "../../plutus.ts";
import {RoyaltyPoolConfig} from "./deployPool.ts";
import {credentialToAddress} from "@lucid-evolution/utils";
import { encoder } from 'js-encoding-utils';
import stringToArrayBuffer = encoder.stringToArrayBuffer;
import {
    BigInteger,
    ConstrPlutusData,
    PlutusData,
    PlutusDataList
} from "@anastasia-labs/cardano-multiplatform-lib-nodejs";

const nftCSBase16 = `8c46dee6b9a36488cdf2bcb8d6258f6c1e8d7a65bb11ef116ef94738`;
const nftTNBase16 = `6e6674`;
const toWithdraw = 100n;
const startLovelaceValue = 10_000_000n;
const current_royalty_nonce = 0n;

export type WithdrawRoyalty = {
    poolnft: { policy: string; name: string };
    withdrawroyaltyx: bigint;
    withdrawroyaltyy: bigint;
    royaltyaddress: string;
    royaltypubkey: string;
    signature: string;
}

//d8798018641864581c597010ddc9ddf8994029f988ca72642eeadffbc37da5823a43408da25820feaf992325b43acc825e88263fd32230e7ecc3ab1effa338af121d762de305bf00
//d87982581c8c46dee6b9a36488cdf2bcb8d6258f6c1e8d7a65bb11ef116ef94738436e667418641864581c597010ddc9ddf8994029f988ca72642eeadffbc37da5823a43408da25820feaf992325b43acc825e88263fd32230e7ecc3ab1effa338af121d762de305bf00

function createConfig(
    privateKey: CML.Bip32PrivateKey
): WithdrawRoyalty {

    let nftList = PlutusDataList.new()
    nftList.add(PlutusData.new_bytes(fromHex(nftCSBase16)))
    nftList.add(PlutusData.new_bytes(fromHex(nftTNBase16)))

    let nftData = CML.PlutusData.new_constr_plutus_data(
        ConstrPlutusData.new(
            0n,
            nftList
        )
    )

    let toSign = Uint8Array.from([
        ...nftData.to_cbor_bytes(),
        ...CML.PlutusData.new_integer(BigInteger.from_str(toWithdraw.toString())).to_cbor_bytes(),
        ...CML.PlutusData.new_integer(BigInteger.from_str(toWithdraw.toString())).to_cbor_bytes(),
        ...CML.PlutusData.new_bytes(privateKey.to_public().to_raw_key().hash().to_raw_bytes()).to_cbor_bytes(),
        ...CML.PlutusData.new_bytes(privateKey.to_public().to_raw_key().to_raw_bytes()).to_cbor_bytes(),
        ...CML.PlutusData.new_integer(BigInteger.from_str(current_royalty_nonce.toString())).to_cbor_bytes()
        ]
    )
    console.log(`toSign: ${toHex(toSign)}`)
    console.log(`toHex(privateKey.to_public().to_raw_bytes()): ${toHex(privateKey.to_public().to_raw_key().to_raw_bytes())}`)

    let signature = privateKey.to_raw_key().sign(toSign).to_hex()

    return {
        poolnft: { policy: nftCSBase16, name: nftTNBase16 },
        withdrawroyaltyx: toWithdraw,
        withdrawroyaltyy: toWithdraw,
        royaltyaddress: privateKey.to_public().to_raw_key().hash().to_hex(),
        royaltypubkey: toHex(privateKey.to_public().to_raw_key().to_raw_bytes()),
        signature: signature,
    }
}

function buildRoyaltyWithdrawDatum(lucid: Lucid, conf: WithdrawRoyalty): Datum {
    return Data.to({
        poolnft: conf.poolnft,
        withdrawroyaltyx: conf.withdrawroyaltyx,
        withdrawroyaltyy: 0n,
        royaltyaddress: conf.royaltyaddress,
        royaltypubkey: conf.royaltypubkey,
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

    const tx = await lucid.newTx()
        .pay.ToContract(
            poolAddress,
            { kind: "inline", value: buildRoyaltyWithdrawDatum(lucid, withdrawConf) },
            depositedValue
        ).complete();

    const txId = await (await tx.sign.withWallet().complete()).submit();

    console.log(`tx: ${txId}`)

    await lucid.awaitTx(txId);
}

main()