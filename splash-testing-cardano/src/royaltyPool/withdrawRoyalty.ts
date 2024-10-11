import {getLucid} from "../lucid.ts";
import {getPrivateKey, setupWallet} from "../wallet.ts";
import {getConfig} from "../config.ts";
import {asUnit, BuiltValidators} from "../types.ts";
import {getUtxoWithToken} from "../balance/balancePool.ts";
import * as CML from '@anastasia-labs/cardano-multiplatform-lib-nodejs';
import {Data, Datum, fromHex, Lucid, toHex} from "@lucid-evolution/lucid";
import {
    RoyaltyPoolPoolValidatePool,
    RoyaltyPoolWithdrawDummyValidate,
    RoyaltyPoolWithdrawValidate
} from "../../plutus.ts";
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


// d879 82 581cc703911186f724e178366f9b021b74409d562165a901360cd724368f436e6674    1a0013d62000581ca05e925159cc1495383854bb450c20bc27ff8ecd9f8ada459b99c78d582019c350e960c14d184d4d5c09dca6d2cfcbe120517dc90985a3dedcd73600a031001a0016e360
// d879 9f 581cc703911186f724e178366f9b021b74409d562165a901360cd724368f436e6674 ff 1a0013d62000581ca05e925159cc1495383854bb450c20bc27ff8ecd9f8ada459b99c78d582019c350e960c14d184d4d5c09dca6d2cfcbe120517dc90985a3dedcd73600a031001a0016e360
const nftCSBase16 = `e5d9944fec15ac918115b737d749de4c10ec515c41f8b839b051d13e`;
const nftTNBase16 = `6e6674`;
const toWithdrawX = 1_300_000n;
const toWithdrawY = 0n;
const startLovelaceValue = 10_000_000n;
const current_royalty_nonce = 0n;
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

//d8798018641864581c597010ddc9ddf8994029f988ca72642eeadffbc37da5823a43408da25820feaf992325b43acc825e88263fd32230e7ecc3ab1effa338af121d762de305bf00
//d87982581c8c46dee6b9a36488cdf2bcb8d6258f6c1e8d7a65bb11ef116ef94738436e667418641864581c597010ddc9ddf8994029f988ca72642eeadffbc37da5823a43408da25820feaf992325b43acc825e88263fd32230e7ecc3ab1effa338af121d762de305bf00

function createConfig(
    privateKey: CML.Bip32PrivateKey
): WithdrawRoyalty {

    let nftAC = PlutusDataList.new()
    nftAC.add(PlutusData.new_bytes(fromHex(nftCSBase16)))
    nftAC.add(PlutusData.new_bytes(fromHex(nftTNBase16)))

    let nftData = CML.PlutusData.new_constr_plutus_data(
        ConstrPlutusData.new(
            0n,
            nftAC
        )
    )

    let nftListData = PlutusDataList.new()
    nftListData.add(nftData)
    nftListData.add(CML.PlutusData.new_integer(BigInteger.from_str(toWithdrawX.toString())))
    nftListData.add(CML.PlutusData.new_integer(BigInteger.from_str(toWithdrawY.toString())))
    nftListData.add(CML.PlutusData.new_bytes(privateKey.to_public().to_raw_key().hash().to_raw_bytes()))
    nftListData.add(CML.PlutusData.new_bytes(privateKey.to_public().to_raw_key().to_raw_bytes()))
    nftListData.add(CML.PlutusData.new_integer(BigInteger.from_str(current_royalty_nonce.toString())))
    nftListData.add(CML.PlutusData.new_integer(BigInteger.from_str(fee.toString())))

    let toSignData = CML.PlutusData.new_constr_plutus_data(
        ConstrPlutusData.new(
            0n,
            nftListData
        )
    )

    let test = Data.to({
        poolnft: { policy: nftCSBase16, name: nftTNBase16 },
        withdrawroyaltyx: toWithdrawX,
        withdrawroyaltyy: toWithdrawY,
        royaltyaddress: privateKey.to_public().to_raw_key().hash().to_hex(),
        royaltypubkey: toHex(privateKey.to_public().to_raw_key().to_raw_bytes()),
        exfee: fee,
    }, RoyaltyPoolWithdrawDummyValidate.conf)

    console.log(`newVersion: ${test}`)

    let testHex = fromHex(test)

    let toSign = toSignData.to_cbor_bytes()
    console.log(`toSign: ${toHex(toSign)}`)
    console.log(`toHex(privateKey.to_public().to_raw_bytes()): ${toHex(privateKey.to_public().to_raw_key().to_raw_bytes())}`)

    let signature = privateKey.to_raw_key().sign(testHex).to_hex()

    let finalDataList = PlutusDataList.new()
    finalDataList.add(toSignData)
    finalDataList.add(CML.PlutusData.new_bytes(privateKey.to_raw_key().sign(testHex).to_raw_bytes()))

    let dataWithSignature = CML.PlutusData.new_constr_plutus_data(
        ConstrPlutusData.new(
            0n,
            finalDataList
        )
    )

    console.log(`final: ${toHex(dataWithSignature.to_cbor_bytes())}`)
    // d87982d87987d87982581c13d621f934e250898102c99d4c6916503d360a375b20e756ec51d61f436e66741a0013d62000581ca05e925159cc1495383854bb450c20bc27ff8ecd9f8ada459b99c78d582019c350e960c14d184d4d5c09dca6d2cfcbe120517dc90985a3dedcd73600a031001a0016e36058402dfffbfdc9b81b64881951b571fc33dc6b6a7135524863780aa33f14fe32b47004be55583c1e02a5e97acf1a1e2d7f70e2ff14276a93b5122f6ab5b60351ba03
    // d87982581c13d621f934e250898102c99d4c6916503d360a375b20e756ec51d61f436e66741a0013d62000581ca05e925159cc1495383854bb450c20bc27ff8ecd9f8ada459b99c78d582019c350e960c14d184d4d5c09dca6d2cfcbe120517dc90985a3dedcd73600a031001a0016e360

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

    console.log(`last cfg: ${cfg}`)

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