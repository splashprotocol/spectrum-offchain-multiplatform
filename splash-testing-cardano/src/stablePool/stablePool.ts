import { PoolT2tExactValidateStablePoolTransitionT2tExact } from "../../plutus.ts";
import { getCSAndSсript, getUtxoWithToken, getUtxoWithAda, stringifyBigIntReviewer } from "../balance/balancePool.ts";
import { getConfig } from "../config.ts";
import { getLucid } from "../lucid.ts";
import { Asset, BuiltValidators, asUnit } from "../types.ts";
import { setupWallet } from "../wallet.ts";
import { Unit, Datum, MintingPolicy, Data, Lucid} from "@lucid-evolution/lucid";
import { encoder } from 'npm:js-encoding-utils';

export const TokenB   = ""
export const TokenBCS = "f6099832f9563e4cf59602b3351c3c5a8a7dda2d44575ef69b82cf8d"

const lqFee = 100n
const treasuryFee = 0n

const initAN2N = 3200n

const startLovelaceValue = 100000000
const startTokenB        = 100000000

// do not touch
const lqEmission = 9223372036854775807n;
const nftEmission = 1n;

const nftTNBase16 = `6e6674`;
const lqTNBase16 = `6c71`;
const encodedTestB = TokenB;// stringToHex(TokenB);

export type StablePoolT2TConfig = {
    poolNft: Asset,
    an2n: bigint,
    poolX: Asset,
    poolY: Asset,
    multiplierX: bigint,
    multiplierY: bigint,
    poolLq: Asset,
    amplCoeffIsEditable: boolean,
    lpFeeIsEditable: boolean,
    lpFeeNum: bigint,
    protocolFeeNum: bigint,
    treasuryX: bigint,
    treasuryY: bigint,
    DAOPolicy: string,
    // treasuryAddress - is contract
    treasuryAddress: string,
}

function buildStablePoolT2TDatum(lucid: Lucid, conf: StablePoolT2TConfig): Datum {
    return Data.to({
        poolNft: conf.poolNft,
        an2n: conf.an2n,
        assetX: conf.poolX,
        assetY: conf.poolY,
        multiplierX: conf.multiplierX,
        multiplierY: conf.multiplierY,
        lpToken: conf.poolLq,
        amplCoeffIsEditable: false,
        lpFeeIsEditable: false,
        lpFeeNum: conf.lpFeeNum,
        protocolFeeNum: conf.protocolFeeNum,
        daoStabeProxyWitness: conf.DAOPolicy,
        treasuryAddress: conf.treasuryAddress,
        protocolFeesX: conf.treasuryX,
        protocolFeesY: conf.treasuryY
    }, PoolT2tExactValidateStablePoolTransitionT2tExact.inputDatum)
}

async function main() {

    const lucid = await getLucid();
    await setupWallet(lucid);

    const conf = await getConfig<BuiltValidators>();

    const utxos = (await lucid.wallet.getUtxos());

    const boxWithToken = await getUtxoWithToken(utxos, encodedTestB);
    const boxWithAda   = await getUtxoWithAda(utxos)

    if (!boxWithToken) {
        console.log("No box with token!");
        return
    }

    const nftInfo = await getCSAndSсript(boxWithToken.txHash, boxWithToken.outputIndex, nftTNBase16, `${nftEmission}`);
    const lqInfo  = await getCSAndSсript(boxWithToken.txHash, boxWithToken.outputIndex, lqTNBase16, `${lqEmission}`);

    console.log(`nft info: ${nftInfo}`);

    console.log(`address: ${await lucid.wallet.address()}`);

    const poolAddress = lucid.utils.credentialToAddress(
        { hash: conf.validators!.stablePoolT2T.hash, type: 'Script' },
    );

    const nftMintingPolicy: MintingPolicy =
        {
            type: "PlutusV2",
            script: nftInfo.script
        }

    const lqMintingPolicy: MintingPolicy =
        {
            type: "PlutusV2",
            script: lqInfo.script
        }

    const lqUnit: Unit  = `${lqInfo.policyId.concat(lqTNBase16)}`;
    const nftUnit: Unit = `${nftInfo.policyId.concat(nftTNBase16)}`;

    console.log(`lq: ${lqUnit}`);
    console.log(`nftUnit: ${nftUnit}`);

    const mintingLqAssets: Record<Unit | "lovelace", bigint> =
        {
            [lqUnit]: lqEmission
        }

    const mintingNftAssets: Record<Unit | "lovelace", bigint> =
        {
            [nftUnit]: nftEmission
        }

    const poolConfig = {
        poolNft: {
            policy: nftInfo.policyId,
            name: nftTNBase16,
        },
        an2n: initAN2N,
        poolX: {
            policy: "",
            name: "",
        },
        poolY: {
            policy: TokenBCS,
            name: encodedTestB,
        },
        multiplierX: 1n,
        multiplierY: 1n,
        poolLq: {
            policy: lqInfo.policyId,
            name: lqTNBase16,
        },
        amplCoeffIsEditable: false,
        lpFeeIsEditable: false,
        lpFeeNum: BigInt(lqFee),
        protocolFeeNum: BigInt(treasuryFee),
        treasuryX: 0n,
        treasuryY: 0n,
        DAOPolicy: "",
        // treasuryAddress - is contract
        treasuryAddress: "",
    }

    console.log(`mintingLqAssets: ${JSON.stringify(mintingLqAssets, stringifyBigIntReviewer)}`)
    console.log(`mintingNftAssets: ${JSON.stringify(mintingNftAssets, stringifyBigIntReviewer)}`)

    console.log(`poolConfig: ${JSON.stringify(poolConfig, stringifyBigIntReviewer)}`)

    const depositedValue = {
        lovelace: BigInt(startLovelaceValue),
        [asUnit(poolConfig.poolY)]: BigInt(startTokenB),
        [asUnit(poolConfig.poolNft)]: nftEmission,
        [asUnit(poolConfig.poolLq)]: (lqEmission - BigInt(startLovelaceValue * 2))
    }

    const tx = await lucid.newTx().collectFrom([boxWithToken!])
        .attachMintingPolicy(nftMintingPolicy)
        .mintAssets(mintingNftAssets, Data.to(0n))
        .attachMintingPolicy(lqMintingPolicy)
        .mintAssets(mintingLqAssets, Data.to(0n))
        .payToContract(poolAddress, { inline: buildStablePoolT2TDatum(lucid, poolConfig) }, depositedValue)
        .complete();

    const txId = await (await tx.sign().complete()).submit();

    console.log(`tx: ${txId}`)
}

main()