// deno-lint-ignore-file
import {
  applyDoubleCborEncoding,
  applyParamsToScript,
  Data,
  Validator,
} from "@lucid-evolution/lucid";

export interface IBeaconBeacon {
  new (
    ref: { transactionId: { hash: string }; outputIndex: bigint },
  ): Validator;
  _: Data;
}

export const BeaconBeacon = Object.assign(
  function (ref: { transactionId: { hash: string }; outputIndex: bigint }) {
    return {
      type: "PlutusV2",
      script: applyParamsToScript(
        applyDoubleCborEncoding(
          "5901e301000032323232323232222533300432533233006300130073754004264a6646601066e212000332232533300b3370e900118061baa0011480004dd6980818069baa00132533300b3370e900118061baa00114c103d87a80001323300100137566022601c6ea8008894ccc040004530103d87a8000132323253330103371e911010100375c602200626012660286ea00052f5c026600a00a0046eb4c044008c050008c048004c8cc00400400c894ccc03c0045300103d87a80001323232533300f3371e00c6eb8c04000c4c020cc04cdd3000a5eb804cc014014008dd598080011809801180880099198008009bab300e300f300f300f300f300b3754600660166ea8018894ccc03400452f5bded8c0264646464a66601c66e3d2201000021003133012337606ea4008dd3000998030030019bab300f003375c601a0046022004601e0026eb8c034c028dd50020992999804980218051baa00113375e600660166ea8c038c02cdd50008040b1999180080091129998070010a6103d87a800013232533300d300800313006330110024bd70099980280280099b8000348004c04800cc040008dd6180118051baa3002300a375400a90001ba548000528918060009b874800058c024c028c018dd50008a4c26cacae6955ceaab9e5573eae815d0aba201",
        ),
        [ref],
        {
          "dataType": "list",
          "items": [{
            "title": "OutputReference",
            "description":
              "An `OutputReference` is a unique reference to an output on-chain. The `output_index`\n corresponds to the position in the output list of the transaction (identified by its id)\n that produced that output",
            "anyOf": [{
              "title": "OutputReference",
              "dataType": "constructor",
              "index": 0,
              "fields": [{
                "title": "transactionId",
                "description":
                  "A unique transaction identifier, as the hash of a transaction body. Note that the transaction id\n isn't a direct hash of the `Transaction` as visible on-chain. Rather, they correspond to hash\n digests of transaction body as they are serialized on the network.",
                "anyOf": [{
                  "title": "TransactionId",
                  "dataType": "constructor",
                  "index": 0,
                  "fields": [{ "dataType": "bytes", "title": "hash" }],
                }],
              }, { "dataType": "integer", "title": "outputIndex" }],
            }],
          }],
        },
      ),
    };
  },
  { _: { "title": "Data", "description": "Any Plutus data." } },
) as unknown as IBeaconBeacon;

export interface ILiquidityLockerLiquidityLocker {
  new (): Validator;
  conf: { lockedUntil: bigint; redeemer: string };
  successorIx: bigint;
}

export const LiquidityLockerLiquidityLocker = Object.assign(
  function () {
    return {
      type: "PlutusV2",
      script:
        "5904040100003232323232323223232322322533300832323253323300c3001300d37540042646464a66601e601a60206ea80044c8c8c8c94ccc04cc044c050dd500089919299980a9991191980080080191299980e0008a50132533301a3371e6eb8c07c008010528899801801800980f8009bac301a301b301b301b301b301b301b301b301b301737546012602e6ea8038dd71803180b9baa014100114a0a66602866ebcc020c058dd50021804180b1baa00113253330153370e9002180b1baa00113232323253330193233001001323300100137566018603a6ea802c894ccc07c00452f5c0264666444646600200200644a66604a00220062646604e6e9ccc09cdd4803198139ba9375c60480026604e6ea0dd69812800a5eb80cc00c00cc0a4008c09c004dd7180f0009bab301f001330030033023002302100122533301e00114a2264a666038646466e24dd6981198120009991192999810980b18111baa0011480004dd6981318119baa0013253330213016302237540022980103d87a8000132330010013756604e60486ea8008894ccc098004530103d87a8000132323253330263371e00e6eb8c09c00c4c064cc0a8dd4000a5eb804cc014014008dd698138011815001181400099198008008051129998128008a6103d87a8000132323253330253371e00e6eb8c09800c4c060cc0a4dd3000a5eb804cc014014008dd59813001181480118138009bae3023002375c604600260460026eb0c0840084cc00c00c004528181080088008a50337126eb4c030c068dd500b9bad300c301a37540066eacc020c064dd50021809800980d180b9baa001163003301637540022664464a66602e601860306ea80044c94ccc060c94ccc070c06c00454ccc064c038c0680045288a99980c980b980d0008a5016163754601260346ea8c030c068dd5002099b880030011337120060026eb4c070c064dd50008a50300a30183754601460306ea8008c064c068c068c068c068c068c068c068c058dd50059bad3008301637540266030602a6ea800458ccc8c0040048894ccc0600085300103d87a800013232533301730150031300a3301b0024bd70099980280280099b8000348004c07000cc068008dd61800980a1baa00900c2301730183018001300130123754602a60246ea80088c054c05800458cc88c8cc00400400c894ccc054004530103d87a80001323253330143375e6010602c6ea80080144c01ccc0600092f5c02660080080026032004602e0026eb0c008c040dd5002980998081baa004374a9000118090009b874800858c03cc040008c038004c028dd50008a4c26cac6eb4004c00400c94ccc010c008c014dd5000899191919299980598070010a4c2c6eb8c030004c030008dd6980500098031baa00116370e90002b9a5573aaae7955cfaba05742ae89",
    };
  },
  {
    conf: {
      "title": "Config",
      "anyOf": [{
        "title": "Config",
        "dataType": "constructor",
        "index": 0,
        "fields": [{ "dataType": "integer", "title": "lockedUntil" }, {
          "dataType": "bytes",
          "title": "redeemer",
        }],
      }],
    },
  },
  { successorIx: { "dataType": "integer" } },
) as unknown as ILiquidityLockerLiquidityLocker;

export interface IAuctionAuction {
  new (): Validator;
  conf: {
    base: { policy: string; name: string };
    quote: { policy: string; name: string };
    priceStart: { num: bigint; denom: bigint };
    startTime: bigint;
    stepLen: bigint;
    steps: bigint;
    priceDacayNum: bigint;
    feePerQuote: { num: bigint; denom: bigint };
    redeemer: string;
  };
  action: { Exec: { spanIx: bigint; successorIx: bigint } } | "Cancel";
}

export const AuctionAuction = Object.assign(
  function () {
    return {
      type: "PlutusV2",
      script:
        "590717010000323232323232322323223232253330083232533300a3008300b375400c2646464646464a666020601660226ea80044c8c94ccc048c040c04cdd5000899191919191919191919299980e180d180e9baa001132323232323232323232323232323232323232533302f0171533302f0011533302f0051533302f002100614a029405280a5032533302f302d303037540022a66605e66e3cdd7181398189baa02e375c606860626ea80044c0b4034528099baf30263031375402c604c60626ea804cc094c0c0dd5181298181baa0123370e66e08018ccc004008dd6980b18179baa3013302f37540584466e0800920d00f33704a66605a010266e04cdc0806805802899b8100d00b333001002375a6048605e6ea8c04cc0bcdd50161119b82002375a6068606a606a606a606a606a606a60626ea80b8888c8ccc00400401000c8894ccc0d400840044ccc00c00cc0e0008cc010004dd6981b8011919800800a400044a66605a66e2008800452f5c02660626ea0004cc008008cdc0000a400466e2120000035333029533302900514a2200829444cdc49919b8130013756602660586ea8044c004dd5980998161baa00e233300b0014881004881000013370666e08004dd6980f98151baa002375a602260546ea80094ccc09c00c4cdc199b823370200800c6eb4c040c0a4dd500099b80375a602060526ea8004dd6980f18149baa00113370200800c602260506ea8094cdc78042441003371e00c91010033300437566018604a6ea801c014dd7180618129baa300c302537540446660066eacc02cc090dd50030029bae300b30243754603260486ea8084ccc008dd5980518119baa008003375c601460466ea8c028c08cdd50101998009bab30093022375400e0066eb8c024c088dd5180b98111baa01f222325333023301e302437540022900009bad30283025375400264a666046603c60486ea80045300103d87a80001323300100137566052604c6ea8008894ccc0a0004530103d87a8000132323253330283371e00e6eb8c0a400c4c060cc0b0dd4000a5eb804cc014014008dd698148011816001181500099198008008021129998138008a6103d87a8000132323253330273371e00e6eb8c0a000c4c05ccc0acdd3000a5eb804cc014014008dd59814001181580118148009bae301530203754600e60406ea8074dd7180a180f9baa3014301f37540386042603c6ea800458ccc8c0040048894ccc0840085300103d87a8000132325333020301e00313010330240024bd70099980280280099b8000348004c09400cc08c008dd61800980e9baa00d00f23020302130210013002301b3754603c60366ea80214ccc060cdc40069bad301d301e301e301e301e301e301a375402e2a66603064a666032602860346ea80044c94ccc068c94ccc078c07400454ccc06cc058c0700045288a99980d980c980e0008a5016163754600660386ea8c044c070dd5002099b8800700113371200e0026eb4c078c06cdd50008a50300f301a3754601e60346ea80084c94ccc064c050c068dd5000899299980d19299980f180e8008a99980d980b180e0008a511533301b3019301c00114a02c2c6ea8c00cc070dd51801980e1baa00413371000200c266e24004018dd6980f180d9baa00114a0601e60346ea8c004c068dd50010a5014a04603a603c002600260306ea80208c06cc070c070c070c070c070c070c070004cdc000080119b80375a6030603260326032602a6ea8048cdc10008041bad30173018301830183018301437540222c6644646600200200644a6660300022980103d87a80001323253330173375e601c60326ea80080144c01ccc06c0092f5c0266008008002603800460340026eb0c020c04cdd5001980b18099baa002374a90000b180a180a801180980098079baa006375a602260240046eb4c040004c030dd50030999119198008008019129998088008a50132533300f3371e6eb8c050008010528899801801800980a0009bac3002300c3754600260186ea800cdd7180118061baa0092300f0012300e300f300f300f300f300f300f300f300f00114984d958c94ccc01cc0140044c8c8c8c94ccc038c04400852616375a601e002601e0046eb4c034004c024dd50018a99980398010008a99980518049baa00314985858c01cdd50011b8748008c8c94ccc014c00cc018dd50020991919191919191919191919191919191919299980d180e80109919191924c602c00c602a01e602a02060280222c6eb8c06c004c06c008c064004c064008dd6980b800980b8011bad30150013015002375a602600260260046eb4c044004c044008c03c004c03c008c034004c034008c02c004c01cdd50020b12999802980198031baa001132323232533300c300f002149858dd6980680098068011bad300b001300737540022c4a6660086004600a6ea80044c8c8c8c94ccc02cc03800852616375c601800260180046eb8c028004c018dd50008b1b87480015cd2ab9d5573caae7d5d02ba157441",
    };
  },
  {
    conf: {
      "title": "Config",
      "anyOf": [{
        "title": "Config",
        "dataType": "constructor",
        "index": 0,
        "fields": [
          {
            "title": "base",
            "anyOf": [{
              "title": "Asset",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "bytes", "title": "policy" }, {
                "dataType": "bytes",
                "title": "name",
              }],
            }],
          },
          {
            "title": "quote",
            "anyOf": [{
              "title": "Asset",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "bytes", "title": "policy" }, {
                "dataType": "bytes",
                "title": "name",
              }],
            }],
          },
          {
            "title": "priceStart",
            "anyOf": [{
              "title": "Rational",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "integer", "title": "num" }, {
                "dataType": "integer",
                "title": "denom",
              }],
            }],
          },
          { "dataType": "integer", "title": "startTime" },
          { "dataType": "integer", "title": "stepLen" },
          { "dataType": "integer", "title": "steps" },
          { "dataType": "integer", "title": "priceDacayNum" },
          {
            "title": "feePerQuote",
            "anyOf": [{
              "title": "Rational",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "integer", "title": "num" }, {
                "dataType": "integer",
                "title": "denom",
              }],
            }],
          },
          { "dataType": "bytes", "title": "redeemer" },
        ],
      }],
    },
  },
  {
    action: {
      "title": "Action",
      "anyOf": [{
        "title": "Exec",
        "description": "Execute order",
        "dataType": "constructor",
        "index": 0,
        "fields": [{ "dataType": "integer", "title": "spanIx" }, {
          "dataType": "integer",
          "title": "successorIx",
        }],
      }, {
        "title": "Cancel",
        "dataType": "constructor",
        "index": 1,
        "fields": [],
      }],
    },
  },
) as unknown as IAuctionAuction;

export interface IGridGridNative {
  new (): Validator;
  state: {
    beacon: string;
    token: { policy: string; name: string };
    buyShiftFactor: { num: bigint; denom: bigint };
    sellShiftFactor: { num: bigint; denom: bigint };
    maxLovelaceOffer: bigint;
    lovelaceOffer: bigint;
    price: { num: bigint; denom: bigint };
    side: boolean;
    budgetPerTransaction: bigint;
    minMarginalOutputLovelace: bigint;
    minMarginalOutputToken: bigint;
    redeemerAddress: {
      paymentCredential: { VerificationKeyCredential: [string] } | {
        ScriptCredential: [string];
      };
      stakeCredential: {
        Inline: [
          { VerificationKeyCredential: [string] } | {
            ScriptCredential: [string];
          },
        ];
      } | {
        Pointer: {
          slotNumber: bigint;
          transactionIndex: bigint;
          certificateIndex: bigint;
        };
      } | null;
    };
    cancellationPkh: string;
  };
  action: { Execute: { successorOutIndex: bigint } } | "Close";
}

export const GridGridNative = Object.assign(
  function () {
    return {
      type: "PlutusV2",
      script:
        "5908350100003232323232323223232322322533300832323232323232533300f300c3010375400a264646464a666026602260286ea80384c8c94ccc054c04cc058dd500089919299980b980a980c1baa0011323232323232323232323232323232323232323232323232323232323232323232323232533303b3370e9002181e1baa00113232533303d006100114a064a646466607e02e26464646464a66608866e2002d20001533304400415333044003100114a0294052819baf0083032330473038304537540846608e04e6608e04a6608e0466608e6ea0084cc11cdd419b8001f00a3304753330430011330073006375a6070608a6ea8094c014dd6981998229baa025101d330473330433330430014a094530103d87a80004c0103d87980003304737500326608e6ea005ccc11cdd400a99823809998239ba90124bd7019b89375a608e6090609060906090609060886ea810400ccdc49801801180200499b89012008337029000003099191919299982199b8800b4800054ccc10c01054ccc10c00c40045280a5014a066ebc01cc0c4cc118c0dcc110dd5020998230131982301219823011198231ba80203304637500406608ca66608400226600c600a6eb4c0dcc110dd501118021bad30323044375404420386608c66608400298103d87a80004c0103d87980003304637500306608c6ea0058cc118dd400a19823009198231ba90114bd70181f80599b893370466e052000008375a606a60846ea8068cdc10039bad30303042375403466e2404c018dc11bad302e304037540306e08dd69819181f9baa01722302d330423750004660846ea00052f5c06080607a6ea800458c0b4c0f0dd501199b80337026eb4c0ecc0f8dd5981d981f0109bad303b303e37566076607c04801e66e04008cccc00cc0f808c01401120063034333300201e375c607803a91010048018cccc00407400c0092006222232333001001005002222533303c3371090000008a99981f8010a40002646464a66607e66e3cdd718200018048991919299982119b8f375c608600601620022a66608a004290000a999822982400109919299982219b8f375c608a00401a2002290001bad3045001304700216375a6086004608c0046088002266600c00c00466e00011200137566080004608600660820042c6eb8c0e4c0e8008dd7181c000981a1baa3022303437540626eb8c0d8c0dc008c0d4004c0d4008dd6981980098198011bad30310013031002375a605e002605e00466e21200030293754605a002605a004605600260560046eb4c0a4004c0a4008dd698138009813801181280098128011811800981180118108009810800980e1baa019301f0013756601060346ea8004c070c064dd50008b19991800800911299980e0010a60103d87a800013232533301b30190031300a3301f0024bd70099980280280099b8000348004c08000cc07800802000cdd59802980b9baa3005301737546034602e6ea800458cc008020014dd6980c180a9baa00e132325333015301230163754002264a66602c6028602e6ea80044c8c8c94ccc06401854ccc06400840045280a503375e601a60346ea8008c074c078c078c078c078c078c078c078c078c078c078c078c068dd500b99baf300730193754600e60326ea8c070c064dd50011803980c9baa001301b0081633003009301a301737540022c6008602c6ea8034cc88c8cc00400400c894ccc068004528099299980c19b8f375c603a00400829444cc00c00c004c074004dd6180c180c980c980c980c980c980c980c980c980a9baa00a375c6030603260326032603260326032603260326032603260326032602a6ea804888c8cc00400400c894ccc064004530103d87a80001323253330183375e601a60346ea80080144c01ccc0700092f5c0266008008002603a00460360026e95200023016301700130143011375400a2c6eb0c004c040dd500291809980a180a0009bac3001300e375400646022002601e6020004601c00260146ea800452613656325333007300500113232533300c300f002149858dd6980680098049baa0021533300730040011533300a300937540042930b0b18039baa0013232533300630043007375400a26464646464646464646464646464646464646464646464646464a666046604c00426464646464932999812181118129baa007132323232533302b302e00213232498c94ccc0a8c0a00044c8c94ccc0bcc0c80084c92632533302d302b0011323253330323035002132498c0ac00458c0cc004c0bcdd50010a99981698150008991919191919299981b181c8010a4c2c6eb4c0dc004c0dc008dd6981a800981a8011bad3033001302f37540042c605a6ea800458c0c0004c0b0dd50018a99981518138008a99981698161baa00314985858c0a8dd500118120018b18160009816001181500098131baa00716301e010301d015301c0165333020301e3021375402e264646464a66604e60540042930b1bae30280013028002375c604c00260446ea805c5858dd718120009812001181100098110011bad30200013020002375a603c002603c0046eb4c070004c070008c94ccc064c06000454ccc058c04cc05c0045288a99980b180a180b8008a501616375460340026034004603000260300046eb4c058004c058008dd6980a000980a0011809000980900118080009808001180700098070011bae300c0013008375400a2c4a66600c6008600e6ea80044c8c8c8c94ccc034c04000852616375a601c002601c0046eb4c030004c020dd50008b1192999803180200089919299980598070010a4c2c6eb8c030004c020dd50010a999803180180089919299980598070010a4c2c6eb8c030004c020dd50010b18031baa001370e90011b87480015cd2ab9d5573caae7d5d02ba157441",
    };
  },
  {
    state: {
      "title": "GridStateNative",
      "anyOf": [{
        "title": "GridStateNative",
        "dataType": "constructor",
        "index": 0,
        "fields": [
          { "dataType": "bytes", "title": "beacon" },
          {
            "title": "token",
            "anyOf": [{
              "title": "Asset",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "bytes", "title": "policy" }, {
                "dataType": "bytes",
                "title": "name",
              }],
            }],
          },
          {
            "title": "buyShiftFactor",
            "anyOf": [{
              "title": "Rational",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "integer", "title": "num" }, {
                "dataType": "integer",
                "title": "denom",
              }],
            }],
          },
          {
            "title": "sellShiftFactor",
            "anyOf": [{
              "title": "Rational",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "integer", "title": "num" }, {
                "dataType": "integer",
                "title": "denom",
              }],
            }],
          },
          { "dataType": "integer", "title": "maxLovelaceOffer" },
          { "dataType": "integer", "title": "lovelaceOffer" },
          {
            "title": "price",
            "anyOf": [{
              "title": "Rational",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "integer", "title": "num" }, {
                "dataType": "integer",
                "title": "denom",
              }],
            }],
          },
          {
            "title": "side",
            "anyOf": [{
              "title": "False",
              "dataType": "constructor",
              "index": 0,
              "fields": [],
            }, {
              "title": "True",
              "dataType": "constructor",
              "index": 1,
              "fields": [],
            }],
          },
          { "dataType": "integer", "title": "budgetPerTransaction" },
          { "dataType": "integer", "title": "minMarginalOutputLovelace" },
          { "dataType": "integer", "title": "minMarginalOutputToken" },
          {
            "title": "redeemerAddress",
            "description":
              "A Cardano `Address` typically holding one or two credential references.\n\n Note that legacy bootstrap addresses (a.k.a. 'Byron addresses') are\n completely excluded from Plutus contexts. Thus, from an on-chain\n perspective only exists addresses of type 00, 01, ..., 07 as detailed\n in [CIP-0019 :: Shelley Addresses](https://github.com/cardano-foundation/CIPs/tree/master/CIP-0019/#shelley-addresses).",
            "anyOf": [{
              "title": "Address",
              "dataType": "constructor",
              "index": 0,
              "fields": [{
                "title": "paymentCredential",
                "description":
                  "A general structure for representing an on-chain `Credential`.\n\n Credentials are always one of two kinds: a direct public/private key\n pair, or a script (native or Plutus).",
                "anyOf": [{
                  "title": "VerificationKeyCredential",
                  "dataType": "constructor",
                  "index": 0,
                  "fields": [{ "dataType": "bytes" }],
                }, {
                  "title": "ScriptCredential",
                  "dataType": "constructor",
                  "index": 1,
                  "fields": [{ "dataType": "bytes" }],
                }],
              }, {
                "title": "stakeCredential",
                "anyOf": [{
                  "title": "Some",
                  "description": "An optional value.",
                  "dataType": "constructor",
                  "index": 0,
                  "fields": [{
                    "description":
                      "Represent a type of object that can be represented either inline (by hash)\n or via a reference (i.e. a pointer to an on-chain location).\n\n This is mainly use for capturing pointers to a stake credential\n registration certificate in the case of so-called pointer addresses.",
                    "anyOf": [{
                      "title": "Inline",
                      "dataType": "constructor",
                      "index": 0,
                      "fields": [{
                        "description":
                          "A general structure for representing an on-chain `Credential`.\n\n Credentials are always one of two kinds: a direct public/private key\n pair, or a script (native or Plutus).",
                        "anyOf": [{
                          "title": "VerificationKeyCredential",
                          "dataType": "constructor",
                          "index": 0,
                          "fields": [{ "dataType": "bytes" }],
                        }, {
                          "title": "ScriptCredential",
                          "dataType": "constructor",
                          "index": 1,
                          "fields": [{ "dataType": "bytes" }],
                        }],
                      }],
                    }, {
                      "title": "Pointer",
                      "dataType": "constructor",
                      "index": 1,
                      "fields": [
                        { "dataType": "integer", "title": "slotNumber" },
                        { "dataType": "integer", "title": "transactionIndex" },
                        { "dataType": "integer", "title": "certificateIndex" },
                      ],
                    }],
                  }],
                }, {
                  "title": "None",
                  "description": "Nothing.",
                  "dataType": "constructor",
                  "index": 1,
                  "fields": [],
                }],
              }],
            }],
          },
          { "dataType": "bytes", "title": "cancellationPkh" },
        ],
      }],
    },
  },
  {
    action: {
      "title": "Action",
      "anyOf": [{
        "title": "Execute",
        "dataType": "constructor",
        "index": 0,
        "fields": [{ "dataType": "integer", "title": "successorOutIndex" }],
      }, {
        "title": "Close",
        "dataType": "constructor",
        "index": 1,
        "fields": [],
      }],
    },
  },
) as unknown as IGridGridNative;

export interface ILimitOrderBatchWitness {
  new (): Validator;
  _: Data;
}

export const LimitOrderBatchWitness = Object.assign(
  function () {
    return {
      type: "PlutusV2",
      script:
        "59076e01000032323232323232225333003325332330053001300637546004600e6ea800c4c8c8c8c8cccc8888c8cccc004004014011289111299980a001880089919299980b0020a501323333007007002301a0055333014004133300800300100914a060300086030008602c0066eb0c010c02cdd50019bac3001300b37540066eb0c008c02cdd5001991919191919191111919299980a9808980b1baa00113253330163375e60366eb0c060c8cdd81ba83018001374e60320026ea800530102410000132323232323232323232323232323232323232323232323232533302f53233303000313253330313371266e08024dd6981618199baa00733704a666062026266e00cdc000600400508061bad302e3033375400e200229414ccc0c004c4c004cdc019b80008007009153330300121300100815333030300100813371266e05200000a3370000e01229404c94ccc0c4c0b4c0c8dd50008991919191919191919191919299981e99b89337040126eb4c0e0c0fcdd500999b82006375a6074607e6ea804c54ccc0f401454ccc0f401054ccc0f400c54ccc0f400840045280a5014a0294052819baf00b323232323232323374a9000198239824003998239824003198239824002998239ba801033047304800433047304800333047304800233047304800133047375001c609260920026090002608e002608c002608a608a00260880026086002607c6ea809ccdd7808007a99981d19b89375a607e6080608060806080608060786ea809400c5288806a99981c80e0980519b803370000a0060242a6660720362601400a2a666072601400a266e24cdc0a400002666e0000c04852819b890023370666e0801003c0414ccc0dc0644cdc019b8001200101010123370201a0026eb4c0b8c0dcdd500219b8100c001375a6048606a6ea8008c094004c0d8c0ccdd50008b181418191baa01f371266e054ccc0bc04840384cccc08403c0580552006533302f012100b1333302100c0160154801840045281929998190008a51132323300100100322533303500114a0264a66606666e3cdd7181c0010020a511330030030013038001375c606803a6eb0c0ccc0d0c0d0c0d0c0d0c0d0c0d0c0d0c0d0c0d0c0d0c0d0c0c0dd500c99baf0013032303330333033303330333033303330333033302f3754030604e605c6ea806cc098c0b4dd500c1817981818181818181818181818181818161baa015375a604460566ea8050dd6980c98151baa013375a6058605a605a605a605a60526ea8048cdc080100299b81533302500710011333301700200a009480194ccc09401c40104cccc05c0140280252006375a604c60526eacc098c0a4008c0a4004dd5980f98121baa011375a6046604c6eacc08cc098008c098004dd5980e18109baa00c3371e006911003371e008910100375c6032603c6ea8010dd7180b180e9baa003375c602e60386ea800cdd7180a180d9baa002301d301e301e301e301e301e301e301a3754006601e60326ea8008c024004528980d180b9baa00114a26018602c6ea8004c040c054dd50019180a980b180b180b000911119199800800802801111299980a99b884800000454ccc06000852000132323253330183371e6eb8c06400c0244c8c8c94ccc06ccdc79bae301c00300b10011533301e00214800054ccc078c0840084c8c94ccc074cdc79bae301e00200d1001148000dd6980f00098100010b1bad301c002301f002301d00113330060060023370000890009bab3019002301c003301a002162533300e3005300f37540022646464646464646464646464646464646464646464646464a6660526058004264646464649319198008008031129998178008a4c2646600600660660046eb8c0c40054ccc0a4c080c0a8dd5004099191919299981818198010991924c64a66605e604c00226464a666068606e00426493192999819181480089919299981b981d00109924c60520022c607000260686ea800854ccc0c8c0a00044c8c8c8c8c8c94ccc0ecc0f800852616375a607800260780046eb4c0e8004c0e8008dd6981c000981a1baa00216303237540022c606a00260626ea800c54ccc0bcc09400454ccc0c8c0c4dd50018a4c2c2c605e6ea8008c08800c58c0c4004c0c4008c0bc004c0acdd50040b2999814180f98149baa00b132323232533302f3032002149858dd6981800098180011bad302e001302a37540162c603601860340262c6eb0c0a8004c0a8008dd718140009814001181300098130011bad302400130240023022001302200230200013020002375a603c002603c0046eb4c070004c070008dd6980d000980d001180c000980c0011bae30160013016002375c602800260206ea80045894ccc034c010c038dd5000899191919299980a180b8010a4c2c6eb8c054004c054008dd7180980098079baa00116232533300d30040011323253330123015002149858dd7180980098079baa0021533300d30030011323253330123015002149858dd7180980098079baa00216300d37540026e1d2002370e900011807180798078009180698071807180718071807180718071807000980098041baa0042300b001370e90020b1180498050008a4c26cacae6955ceaab9e5573eae815d0aba21",
    };
  },
  { _: { "title": "Data", "description": "Any Plutus data." } },
) as unknown as ILimitOrderBatchWitness;

export interface ILimitOrderLimitOrder {
  new (
    witness: {
      Inline: [
        { VerificationKeyCredential: [string] } | {
          ScriptCredential: [string];
        },
      ];
    } | {
      Pointer: {
        slotNumber: bigint;
        transactionIndex: bigint;
        certificateIndex: bigint;
      };
    },
  ): Validator;
  conf: {
    tag: string;
    beacon: string;
    input: { policy: string; name: string };
    tradableInput: bigint;
    costPerExStep: bigint;
    minMarginalOutput: bigint;
    output: { policy: string; name: string };
    basePrice: { num: bigint; denom: bigint };
    fee: bigint;
    redeemerAddress: {
      paymentCredential: { VerificationKeyCredential: [string] } | {
        ScriptCredential: [string];
      };
      stakeCredential: {
        Inline: [
          { VerificationKeyCredential: [string] } | {
            ScriptCredential: [string];
          },
        ];
      } | {
        Pointer: {
          slotNumber: bigint;
          transactionIndex: bigint;
          certificateIndex: bigint;
        };
      } | null;
    };
    cancellationPkh: string;
    permittedExecutors: Array<string>;
  };
  action: boolean;
}

export const LimitOrderLimitOrder = Object.assign(
  function (
    witness: {
      Inline: [
        { VerificationKeyCredential: [string] } | {
          ScriptCredential: [string];
        },
      ];
    } | {
      Pointer: {
        slotNumber: bigint;
        transactionIndex: bigint;
        certificateIndex: bigint;
      };
    },
  ) {
    return {
      type: "PlutusV2",
      script: applyParamsToScript(
        applyDoubleCborEncoding(
          "5904020100003232323232323222323232232253330093232533300b0041323300100137566022602460246024602460246024601c6ea8008894ccc040004528099299980719baf00d300f301300214a226600600600260260022646464a66601c6014601e6ea80044c94ccc03cc030c040dd5000899191929998090038a99980900108008a5014a066ebcc020c04cdd5001180b180b980b980b980b980b980b980b980b980b98099baa00f3375e600860246ea8c010c048dd5180a98091baa00230043012375400260286eb0c050c054c054c044dd50028b1991191980080080191299980a8008a6103d87a80001323253330143375e6016602c6ea80080144cdd2a40006603000497ae0133004004001301900230170013758600a60206ea8010c04cc040dd50008b180098079baa0052301230130013322323300100100322533301200114a0264a66602066e3cdd7180a8010020a5113300300300130150013758602060226022602260226022602260226022601a6ea8004dd71808180898089808980898089808980898089808980898069baa0093001300c37540044601e00229309b2b19299980598050008a999804180218048008a51153330083005300900114a02c2c6ea8004c8c94ccc01cc010c020dd50028991919191919191919191919191919191919191919191919299981118128010991919191924c646600200200c44a6660500022930991980180198160011bae302a0015333022301f30233754010264646464a666052605800426464931929998141812800899192999816981800109924c64a666056605000226464a66606060660042649318140008b181880098169baa0021533302b3027001132323232323253330343037002149858dd6981a800981a8011bad30330013033002375a6062002605a6ea800858c0acdd50008b181700098151baa0031533302830240011533302b302a37540062930b0b18141baa002302100316302a001302a0023028001302437540102ca666042603c60446ea802c4c8c8c8c94ccc0a0c0ac00852616375a605200260520046eb4c09c004c08cdd50058b180d006180c8098b1bac30230013023002375c60420026042004603e002603e0046eb4c074004c074008c06c004c06c008c064004c064008dd6980b800980b8011bad30150013015002375a60260026026004602200260220046eb8c03c004c03c008dd7180680098049baa0051625333007300430083754002264646464a66601c60220042930b1bae300f001300f002375c601a00260126ea8004588c94ccc01cc0100044c8c94ccc030c03c00852616375c601a00260126ea800854ccc01cc00c0044c8c94ccc030c03c00852616375c601a00260126ea800858c01cdd50009b8748008dc3a4000ae6955ceaab9e5573eae815d0aba201",
        ),
        [witness],
        {
          "dataType": "list",
          "items": [{
            "title": "Referenced",
            "description":
              "Represent a type of object that can be represented either inline (by hash)\n or via a reference (i.e. a pointer to an on-chain location).\n\n This is mainly use for capturing pointers to a stake credential\n registration certificate in the case of so-called pointer addresses.",
            "anyOf": [{
              "title": "Inline",
              "dataType": "constructor",
              "index": 0,
              "fields": [{
                "description":
                  "A general structure for representing an on-chain `Credential`.\n\n Credentials are always one of two kinds: a direct public/private key\n pair, or a script (native or Plutus).",
                "anyOf": [{
                  "title": "VerificationKeyCredential",
                  "dataType": "constructor",
                  "index": 0,
                  "fields": [{ "dataType": "bytes" }],
                }, {
                  "title": "ScriptCredential",
                  "dataType": "constructor",
                  "index": 1,
                  "fields": [{ "dataType": "bytes" }],
                }],
              }],
            }, {
              "title": "Pointer",
              "dataType": "constructor",
              "index": 1,
              "fields": [{ "dataType": "integer", "title": "slotNumber" }, {
                "dataType": "integer",
                "title": "transactionIndex",
              }, { "dataType": "integer", "title": "certificateIndex" }],
            }],
          }],
        },
      ),
    };
  },
  {
    conf: {
      "title": "LimitOrderConfig",
      "anyOf": [{
        "title": "LimitOrderConfig",
        "dataType": "constructor",
        "index": 0,
        "fields": [
          { "dataType": "bytes", "title": "tag" },
          { "dataType": "bytes", "title": "beacon" },
          {
            "title": "input",
            "anyOf": [{
              "title": "Asset",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "bytes", "title": "policy" }, {
                "dataType": "bytes",
                "title": "name",
              }],
            }],
          },
          { "dataType": "integer", "title": "tradableInput" },
          { "dataType": "integer", "title": "costPerExStep" },
          { "dataType": "integer", "title": "minMarginalOutput" },
          {
            "title": "output",
            "anyOf": [{
              "title": "Asset",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "bytes", "title": "policy" }, {
                "dataType": "bytes",
                "title": "name",
              }],
            }],
          },
          {
            "title": "basePrice",
            "anyOf": [{
              "title": "Rational",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "integer", "title": "num" }, {
                "dataType": "integer",
                "title": "denom",
              }],
            }],
          },
          { "dataType": "integer", "title": "fee" },
          {
            "title": "redeemerAddress",
            "description":
              "A Cardano `Address` typically holding one or two credential references.\n\n Note that legacy bootstrap addresses (a.k.a. 'Byron addresses') are\n completely excluded from Plutus contexts. Thus, from an on-chain\n perspective only exists addresses of type 00, 01, ..., 07 as detailed\n in [CIP-0019 :: Shelley Addresses](https://github.com/cardano-foundation/CIPs/tree/master/CIP-0019/#shelley-addresses).",
            "anyOf": [{
              "title": "Address",
              "dataType": "constructor",
              "index": 0,
              "fields": [{
                "title": "paymentCredential",
                "description":
                  "A general structure for representing an on-chain `Credential`.\n\n Credentials are always one of two kinds: a direct public/private key\n pair, or a script (native or Plutus).",
                "anyOf": [{
                  "title": "VerificationKeyCredential",
                  "dataType": "constructor",
                  "index": 0,
                  "fields": [{ "dataType": "bytes" }],
                }, {
                  "title": "ScriptCredential",
                  "dataType": "constructor",
                  "index": 1,
                  "fields": [{ "dataType": "bytes" }],
                }],
              }, {
                "title": "stakeCredential",
                "anyOf": [{
                  "title": "Some",
                  "description": "An optional value.",
                  "dataType": "constructor",
                  "index": 0,
                  "fields": [{
                    "description":
                      "Represent a type of object that can be represented either inline (by hash)\n or via a reference (i.e. a pointer to an on-chain location).\n\n This is mainly use for capturing pointers to a stake credential\n registration certificate in the case of so-called pointer addresses.",
                    "anyOf": [{
                      "title": "Inline",
                      "dataType": "constructor",
                      "index": 0,
                      "fields": [{
                        "description":
                          "A general structure for representing an on-chain `Credential`.\n\n Credentials are always one of two kinds: a direct public/private key\n pair, or a script (native or Plutus).",
                        "anyOf": [{
                          "title": "VerificationKeyCredential",
                          "dataType": "constructor",
                          "index": 0,
                          "fields": [{ "dataType": "bytes" }],
                        }, {
                          "title": "ScriptCredential",
                          "dataType": "constructor",
                          "index": 1,
                          "fields": [{ "dataType": "bytes" }],
                        }],
                      }],
                    }, {
                      "title": "Pointer",
                      "dataType": "constructor",
                      "index": 1,
                      "fields": [
                        { "dataType": "integer", "title": "slotNumber" },
                        { "dataType": "integer", "title": "transactionIndex" },
                        { "dataType": "integer", "title": "certificateIndex" },
                      ],
                    }],
                  }],
                }, {
                  "title": "None",
                  "description": "Nothing.",
                  "dataType": "constructor",
                  "index": 1,
                  "fields": [],
                }],
              }],
            }],
          },
          { "dataType": "bytes", "title": "cancellationPkh" },
          {
            "dataType": "list",
            "items": { "dataType": "bytes" },
            "title": "permittedExecutors",
          },
        ],
      }],
    },
  },
  {
    action: {
      "title": "Bool",
      "anyOf": [{
        "title": "False",
        "dataType": "constructor",
        "index": 0,
        "fields": [],
      }, {
        "title": "True",
        "dataType": "constructor",
        "index": 1,
        "fields": [],
      }],
    },
  },
) as unknown as ILimitOrderLimitOrder;

export interface IRoyaltyPoolPoolValidatePool {
  new (): Validator;
  conf: {
    poolnft: { policy: string; name: string };
    poolx: { policy: string; name: string };
    poolY: { policy: string; name: string };
    poolLq: { policy: string; name: string };
    feenum: bigint;
    treasuryFee: bigint;
    royaltyFee: bigint;
    treasuryx: bigint;
    treasuryy: bigint;
    royaltyx: bigint;
    royaltyy: bigint;
    daoPolicy: Array<
      {
        Inline: [
          { VerificationKeyCredential: [string] } | {
            ScriptCredential: [string];
          },
        ];
      } | {
        Pointer: {
          slotNumber: bigint;
          transactionIndex: bigint;
          certificateIndex: bigint;
        };
      }
    >;
    treasuryAddress: string;
    royaltyPubKeyHash_256: string;
    royaltyNonce: bigint;
  };
  action: Data;
}

export const RoyaltyPoolPoolValidatePool = Object.assign(
  function () {
    return {
      type: "PlutusV2",
      script:
        "590e0e590e0b010000323232323232323232323232323232323232323232323232323232323232323232322225333021323232323232323232533302a3370e9001001099191929980e19808801981780389919191919191919191919191919191919191919299982099b87480100084c8c8c8c8c94cc0d4cdc38058060a99982319b8753330463370e6eb4c120095200014800054ccc118cdc39bad30480254800852002153330463370e6eb4c120095200414801054ccc118cdc39bad304802548018520061480212000132323253330493370e900200109929981c99b87330492253330410011480004cdc0181b1bab3050304e0013002304d001008330492253330410011480004cdc0181b1bab3050304e0013002304d00100715330393375e0020122a660726605c00a00c2a6607266e24cdc119b8101101e0153370466e040500540784cdc499b823370202203c02666e08cdc080900980f18258008b182600118218009baa00a153330463370ea66608c66e1cdd69824012a4000290000a99982319b87375a609004a90010a40042a66608c66e1cdd69824012a4008290020a99982319b87375a609004a90030a400c29004240042646464a66609266e1d20040021325330393370e6609244a66608200229000099b803036375660a0609c0026004609a0020106609244a66608200229000099b803036375660a0609c0026004609a00200e2a6607266ebc00402454cc0e4cc0b801401854cc0e4cdc499b823370202203c02a66e08cdc080a00a80f099b893370466e0404407804ccdc119b8101201301e304b00116304c002304300137540142a66608c66e1d4ccc118cdc39bad30480254800052000153330463370e6eb4c120095200214800854ccc118cdc39bad30480254801052004153330463370e6eb4c120095200614801852008480104c8c8c94cc0e0cdc3998241129998200008a4000266e00c0d4dd59827982680098011826000803998241129998200008a4000266e00c0d4dd598279826800980118260008030a9981c198168020028a9981c19b873370202003a90000a9981c299982499b8848000cdc080980a099b89302c3370466e04044048cdc019b8201402e3370466e0404c050cdc099b810030020013370402466e08cdc080980a19b8133702006004002266e24c0b0cdc119b810130143370066e080480b8cdc119b810110123370266e0400c008004cdc100a19b823370202202466e04cdc0801801000899991119191919191919191919191919191919192998261982099ba548000cc150c17c044cc150c17c040cc150c17c03ccc150c17c038cc150dd41bad305f00d3305437506eb4c17c030cc150dd41bad305f00b3305437506eb4c17c00ccc150dd41bad305f0023305437506eb4c17c004cc150dd41bad305f305e00133054374e6eb0c17c018cc150dd49bae305f0053305437526eb8c17c010cc150dd41bad305f305e004056014153304c53304c3371266e081094ccc174cdc424000026266e04dd6982f8009bad305f0081337026eb4c17cc178004dd6982f803a99982e99b884800004c4cdc10099bad305f00b1337040246eb4c17c02c54cc130cdc4299982e99b884800004c4cdc10099bad305f00b1337040246eb4c17c02ccdc102119b80533305d337109000009899b81375a60be0026eb4c17c0204cdc09bad305f305e001375a60be00e90010a9982619b8933704084a6660ba66e2120000131337026eb4c17c00cdd6982f805099b81375a60be0046eb4c17c0254ccc174cdc424000026266e0804cdd6982f806099b82012375a60be018266e214ccc174cdc424000026266e0804cdd6982f806099b82012375a60be01866e08108cdc0299982e99b884800004c4cdc09bad305f003375a60be014266e04dd6982f8011bad305f0094800854ccc174cdc4240000262a6609866e1cdd6982f8049bad305f00213370e6eb4c17c01cdd6982f982f0008a9982619b87375a60be0146eb4c17c00c4cdc39bad305f008375a60be00260ba00260b8002608060ba02260b400260b200260b000260ae00260ac00260aa00260a800260a600260a400260a200260a0002609e002609c002609e05c64646464018a66609866e1d200000213232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232533307e3370e6e340052038132323232323232324994ccc1e800452616308501003375a0026108020026104020066eb8004c20404004c1fc00c58dd7000983f000983e001999183d112999839000883b09983a1801983f8009801183f000919191919002a99983f19b87480000084c8c8c8c8c8c8c9265333079001149858c210040194ccc20804cdc3a400000426464a6661080266e1cdc6800a40702646493299983d0008a4c2c610a020062c6eb8004c2100400454ccc20804cdc3a400400426464a6661080266e1cdc6800a40702646493299983d0008a4c2c610a020062c6eb8004c2100400458c21404008c1f0004dd50009840008008a99983f19b87480080084c8c8c8c8c8c8c8c8c8c926533307c001149858c21c0400cdd68009843008009842008019bad001308301001308101003375a0026100020022c61020200460f00026ea8004dd6000983d800983c8019bad00130780013076003375a00260ea00260e60066eb4004c1c8004c1c000cdd6800983780098368019bad001306c001306a003375a00260d200260ce0066eb4004c198004c1900194ccc188cdc3a40000042646464a6660caa660a466e1c005200013370e002901c0991919299983419b89371a00290200991924ca6660bc0022930b18348018b1bae0013068001306600416371a0026eb8004c19000458c194008c170004dd50009830000982f003299982e19b87480000084c8c8c94ccc17d4cc130cdc3800a4000266e1c005203813232325333062337126e340052040132324994ccc16000452616306300316375c00260c400260c00082c6e34004dd7000982f0008b182f801182b0009baa001305a001305800653330563370e90000010991919299982ca9982319b87001480004cdc3800a40702646464a6660b866e24dc6800a4080264649329998290008a4c2c60ba0062c6eb8004c170004c16801058dc68009bae00130580011630590023050001375400260a800260a400ca6660a066e1d2000002132323253330535330403370e0029000099b87001480e04c8c8c94ccc158cdc49b8d001481004c8c926533304c001149858c15c00c58dd7000982b000982a0020b1b8d001375c00260a40022c60a600460940026ea8004c13800458c13c008c118004dd500419b81013014337020220246eb4c128c0bcc12c0a8dd69824981798250149bad3048302f3049028153330463370ea66608c66e1cdd69824012a4000290000a99982319b87375a609004a90010a40042a66608c66e1cdd69824012a4008290020a99982319b87375a609004a90030a400c290042400c264646464660024944528199199111982611299982200089128008a99982799baf304b3051001004130053051001130023050001001232223002003375a609e00200260960020046eb0c128c8c128c128c128c128c0bc004c12c0a8dd5982498171825000982401209991191919191919b87304937540029001191919299982799b87480000085854ccc13ccdc79bae30510014891c11b6d33ba98aa6305948d1da3144b9b76c42c516c2d016b91b08da7f001323374a6660a000290012400009266e1cc0e4021200416305200230490013754002609a609c0026098609a0026096609460980026608a66e052002002001375a6090608e04a044608e02a608c0246eacc114048dd5982200798218008b1822001181d8009baa001303f303e00c303e303d009375a607a606a607c00e6eb4c0f0c0d0c0f403cdd6981d981a181e0029bad303a3039303b004375a6072607060740186eb4c0e0c0e4008dd6981b981c0051980f80b002981a000981a981a181980618190009819800998171129998130008b0a99981899b873302232375660686066606a0026066002006900109819800898011819000801181818188081bac302f302e302e00a375a605c604e605e0026602c01a605a605800a60580022c605a00460480026ea8c0a4c0a0014c0a4004cc088dd6981398130020009bac302600130263025001302500230240021498588cdc0a40000024466ebcdd398108011ba730210014830268308c070c0080048c06cc0080048c068c0080048c064c048004cc0548894ccc03800440084cc00ccdc00012400460340029000111919191919191919ba548000cc054dd419b81337026601e002604000e6eb4c080010dd698100011980a9ba83370266e04cc03c004c080018dd698100019bad3020301f00233015375066e04028cc03c004c080014cc054dd4299980f299805991929980799b8f375c604460420046eb8c088c0840044cdc79bae3022002375c6044002604402a6042604000e26464a6601e66e3cdd7181118108011bae3022302100113371e6eb8c088008dd71811000981100a981098100030a400026601e00202802e6eacc07cc078c08001cc074004c070004c050c06c004c068004c064004c060c06800920feffffffffffffffff0123301400100214a244666026004002006294088c8ccc01000cdd7180a0009bae3014301300130140012223333004002480008cccc014009200075a6eac00400c8c008dd480091111980791299980380088028a99980919baf300e301400100613004301630140011300230130010015573e66e952000330013752004660026ea400800d5d0245004bd70118041801000918039803800aab9d2253330053371000490000b0998018010009800911299980299b87002480004c01c0044cc00ccdc080124004600c002464600446600400400246004466004004002ae695d12ba1230023754002aae781",
    };
  },
  {
    conf: {
      "title": "Config",
      "anyOf": [{
        "title": "Config",
        "dataType": "constructor",
        "index": 0,
        "fields": [
          {
            "title": "poolnft",
            "anyOf": [{
              "title": "Asset",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "bytes", "title": "policy" }, {
                "dataType": "bytes",
                "title": "name",
              }],
            }],
          },
          {
            "title": "poolx",
            "anyOf": [{
              "title": "Asset",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "bytes", "title": "policy" }, {
                "dataType": "bytes",
                "title": "name",
              }],
            }],
          },
          {
            "title": "poolY",
            "anyOf": [{
              "title": "Asset",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "bytes", "title": "policy" }, {
                "dataType": "bytes",
                "title": "name",
              }],
            }],
          },
          {
            "title": "poolLq",
            "anyOf": [{
              "title": "Asset",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "bytes", "title": "policy" }, {
                "dataType": "bytes",
                "title": "name",
              }],
            }],
          },
          { "dataType": "integer", "title": "feenum" },
          { "dataType": "integer", "title": "treasuryFee" },
          { "dataType": "integer", "title": "royaltyFee" },
          { "dataType": "integer", "title": "treasuryx" },
          { "dataType": "integer", "title": "treasuryy" },
          { "dataType": "integer", "title": "royaltyx" },
          { "dataType": "integer", "title": "royaltyy" },
          {
            "dataType": "list",
            "items": {
              "title": "Referenced",
              "description":
                "Represent a type of object that can be represented either inline (by hash)\n or via a reference (i.e. a pointer to an on-chain location).\n\n This is mainly use for capturing pointers to a stake credential\n registration certificate in the case of so-called pointer addresses.",
              "anyOf": [{
                "title": "Inline",
                "dataType": "constructor",
                "index": 0,
                "fields": [{
                  "description":
                    "A general structure for representing an on-chain `Credential`.\n\n Credentials are always one of two kinds: a direct public/private key\n pair, or a script (native or Plutus).",
                  "anyOf": [{
                    "title": "VerificationKeyCredential",
                    "dataType": "constructor",
                    "index": 0,
                    "fields": [{ "dataType": "bytes" }],
                  }, {
                    "title": "ScriptCredential",
                    "dataType": "constructor",
                    "index": 1,
                    "fields": [{ "dataType": "bytes" }],
                  }],
                }],
              }, {
                "title": "Pointer",
                "dataType": "constructor",
                "index": 1,
                "fields": [{ "dataType": "integer", "title": "slotNumber" }, {
                  "dataType": "integer",
                  "title": "transactionIndex",
                }, { "dataType": "integer", "title": "certificateIndex" }],
              }],
            },
            "title": "daoPolicy",
          },
          { "dataType": "bytes", "title": "treasuryAddress" },
          { "dataType": "bytes", "title": "royaltyPubKeyHash_256" },
          { "dataType": "integer", "title": "royaltyNonce" },
        ],
      }],
    },
  },
  { action: { "title": "Data", "description": "Any Plutus data." } },
) as unknown as IRoyaltyPoolPoolValidatePool;

export interface IRoyaltyPoolWithdrawValidate {
  new (): Validator;
  conf: {
    poolnft: { policy: string; name: string };
    withdrawroyaltyx: bigint;
    withdrawroyaltyy: bigint;
    royaltyaddress: string;
    royaltypubkey: string;
    signature: string;
  };
  action: Data;
}

export const RoyaltyPoolWithdrawValidate = Object.assign(
  function () {
    return {
      type: "PlutusV2",
      script:
        "590d77590d74010000323232323232323232323232323232323232323232323232323222253330193232323232323232323232323232323253330293370e90010010991919191919191919191919191919191919191919191919191929981899b87533304353303c32325330333371e6eb8c118c11c008dd718231823800899b8f375c608c0046eb8c118004c11c0bcc118c1100504c8c94cc0cccdc79bae30463047002375c608c608e002266e3cdd718230011bae3046001304702f304630440131480004cdc09981a9bab304401702e330353756608800a05c90000a9981899baf3044018304401815330313370e646608644a66608200229000099b80303337566094608e002600460900020026eacc11005cc8cc10c894ccc1040045200013370060666eacc128c11c004c008c120004004dd598220028a9981899b87330353756608800a608802a90010a99818a9981899b89375a60880506eb4c11003054cc0c4cdc49bad3044027375a60880162a6606266e1cdd6982200199b81375a60880186eb4c1100a054cc0c4cdc39bad30443045003337026eb4c11002cdd698220138a9981899b87337026606a6eacc11005cc110050dd698220141981a9bab3044005304401415330313370e66e04cc0d4dd5982200b98220099bad3044027330353756608800a6088026266e1ccc0d4dd5982200b98220091981a9bab304400530440121533031533031337126eb4c1100a0cc0d4004c1100504cdc49bad3044027330350013044013153303133372a6eb8c110094cdc51bb33044015337146eccdd41bad3044028337146eccdd41bad3044027337146eccdd49bae3044026337146eccdd49bae304402537666ea0dd6982218228041bae3044304502515330313370e606004490020a9981899b8730300214801054cc0c4cc0ccc11006cc110c11807854cc0c4cc0cccdd2a400066076608802a660766088028660766088026660766088024660766ea0dd698220089981d9ba8375a6088020660766ea0dd698220079981d9ba8375a608801c660766ea0dd698220069981d9ba8375a6088006660766ea0dd6982218228019981d9ba737586088014660766ea4dd718220049981d9ba9375c6088010660766ea0cdc01bad30443045008480080e80104cdc79b94375c608804a6eb8c110020cc88c8c8c94ccc11cc8c8c94ccc128cdc3a40040042646464a66609a66e1d200000214a0266ebcdd38021ba700130500023047001375400e2646464a66609a66e1d200200214a0266ebcdd38021ba700130500023047001375400e609a00460880026ea8004400858c8c8c8c8c94ccc12ccdc3a4000004264646464a66609e66e1d200000213232323253330533370e90010010b099ba548000004c158008c134004dd500098280008b182900118248009baa001304c00113374a9001021182700118228009baa00130483049304a001304730490053756646464a66609066e1d200200216153330483371e6eb8c1240040184c124c128c12c01c58c12c008c108004dd50009918231824000982298238019bae30430253374a900101c9981f1bad3042304301d01f30313230433043304330433043303200130430013232323253330433370e900200109919191919002a99982399b87480000084c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c94ccc1e4cdc39b8d001480e04c8c8c8c8c8c8c8c926533307b001149858c2040400cdd6800983f000983f0019bae001307b001307b00316375c00260f000260f0006660d646464646400aa6660f266e1d20000021323232323232324994ccc1e800452616308001006533307d3370e900000109919299983f99b87371a002901c0991924ca6660f60022930b1840808018b1bae001307e0011533307d3370e900100109919299983f99b87371a002901c0991924ca6660f60022930b1840808018b1bae001307e001163080010023077001375400260f40022a6660f266e1d20020021323232323232323232324994ccc1f400452616308301003375a0026100020026100020066eb4004c1f4004c1f400cdd6800983d0008b183e00118398009baa001375800260ea00260ea0066eb4004c1c8004c1c800cdd6800983780098378019bad001306c001306c003375a00260d200260d20066eb4004c198004c19800cdd6800983180098318019bad00130600013060006533305d3370e90000010991919299983029982c99b87001480004cdc3800a40702646464a6660c666e24dc6800a40802646493299982f8008a4c2c60ca0062c6eb8004c188004c18801058dc68009bae001305e0011630600023057001375400260b400260b400ca6660ae66e1d20000021323232533305a5330533370e0029000099b87001480e04c8c8c94ccc174cdc49b8d001481004c8c9265333059001149858c17c00c58dd7000982e000982e0020b1b8d001375c00260b00022c60b400460a20026ea8004c150004c1500194ccc144cdc3a40000042646464a6660a8a6609a66e1c005200013370e002901c0991919299982b99b89371a00290200991924ca6660a60022930b182c8018b1bae0013056001305600416371a0026eb8004c14800458c150008c12c004dd500098270009827003299982599b87480000084c8c8c94ccc1394cc11ccdc3800a4000266e1c005203813232325333051337126e340052040132324994ccc13400452616305300316375c00260a000260a00082c6e34004dd700098260008b182700118228009baa001304800116304a0023041001375400260880022c608c004607a0026ea8004c100c104004c100004c100004cc8c0f0894ccc0e80045854ccc0fccdc399818991bab3041304230430013040001003480084c1000044c008c104004c0f4088068c0f4004c0f0004c0ec004c0e8004c0e4004c0e0004c0dc004c0d8004c0d4004c0d0004c0cc004c0c8004c0c4004c0c4004c8c8c8c94ccc0c4cdc3a40080042646464646400aa66606a66e1d20000021323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323253330673370e6e340052038132323232323232324994ccc1a400452616306f003375a00260d800260d80066eb8004c1a4004c1a400c58dd7000983300098330019982c919191919002a99983399b87480000084c8c8c8c8c8c8c9265333068001149858c1b80194ccc1accdc3a400000426464a6660da66e1cdc6800a4070264649329998348008a4c2c60de0062c6eb8004c1b000454ccc1accdc3a400400426464a6660da66e1cdc6800a4070264649329998348008a4c2c60de0062c6eb8004c1b000458c1b8008c194004dd500098340008a99983399b87480080084c8c8c8c8c8c8c8c8c8c926533306b001149858c1c400cdd6800983700098370019bad001306b001306b003375a00260d00022c60d400460c20026ea8004dd6000983180098318019bad00130600013060003375a00260ba00260ba0066eb4004c168004c16800cdd6800982b800982b8019bad00130540013054003375a00260a200260a20066eb4004c138004c1380194ccc12ccdc3a40000042646464a66609ca6608e66e1c005200013370e002901c0991919299982899b89371a00290200991924ca66609a0022930b18298018b1bae0013050001305000416371a0026eb8004c13000458c138008c114004dd500098240009824003299982299b87480000084c8c8c94ccc1214cc104cdc3800a4000266e1c00520381323232533304b337126e340052040132324994ccc11c00452616304d00316375c002609400260940082c6e34004dd700098230008b1824001181f8009baa00130420013042006533303f3370e90000010991919299982129981d99b87001480004cdc3800a40702646464a66608a66e24dc6800a4080264649329998208008a4c2c608e0062c6eb8004c110004c11001058dc68009bae0013040001163042002303900137540026078002607800ca66607266e1d20000021323232533303c5330353370e0029000099b87001480e04c8c8c94ccc0fccdc49b8d001481004c8c926533303b001149858c10400c58dd7000981f000981f0020b1b8d001375c00260740022c607800460660026ea8004c0d800458c0e0008c0bc004dd500098190008b181a00118158009baa001302e302f001302e001302e302c302d001302d00133026375a605400a0102c605800460460026ea8c09cc0a0034cc088dd69813001002181300098130061bac302300237586044004604460440026044604000c6040002603e002603c002603a002603a00860380022930b19ba548000cc034dd4800998069ba900100c4881003301222253330110011002133003337000049001180c000a400044666028004002006294088cdd79ba73015002374e602a00246024602400244646660080066eb8c048004dd71809180980098098009111999802001240004666600a00490003ad3756002006460046ea40048888cc038894ccc030004401454ccc044cdd79806980900080309802180a98090008980118098008009180511299980400088020998029801980700098011807800a5eb815d01198040008010a515573eaae74894ccc014cdc4001240002c2660060040026002444a66600a66e1c0092000130060011330033370200490011803800919180111980100100091801119801001000ab9a5742ae888c008dd5000aab9e01",
    };
  },
  {
    conf: {
      "title": "Config",
      "anyOf": [{
        "title": "Config",
        "dataType": "constructor",
        "index": 0,
        "fields": [
          {
            "title": "poolnft",
            "anyOf": [{
              "title": "Asset",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "bytes", "title": "policy" }, {
                "dataType": "bytes",
                "title": "name",
              }],
            }],
          },
          { "dataType": "integer", "title": "withdrawroyaltyx" },
          { "dataType": "integer", "title": "withdrawroyaltyy" },
          { "dataType": "bytes", "title": "royaltyaddress" },
          { "dataType": "bytes", "title": "royaltypubkey" },
          { "dataType": "bytes", "title": "signature" },
        ],
      }],
    },
  },
  { action: { "title": "Data", "description": "Any Plutus data." } },
) as unknown as IRoyaltyPoolWithdrawValidate;

export interface IRoyaltyPoolDepositValidate {
  new (): Validator;
  conf: {
    poolnft: { policy: string; name: string };
    x: { policy: string; name: string };
    y: { policy: string; name: string };
    lq: { policy: string; name: string };
    exFee: bigint;
    rewardPkh: string;
    stakePkh: string | null;
    collateralAda: bigint;
  };
  action: Data;
}

export const RoyaltyPoolDepositValidate = Object.assign(
  function () {
    return {
      type: "PlutusV2",
      script:
        "59085d59085a0100003232323232323232323232323232323232323232323232323232323222253330173232323232323232323232323253330243370ea66604866e1cdd698149815001a4000290000a4004900009919191919191919191919191919191919299981a99b87480080084c8c8c8c8c94cc0a4cdc399816808181f811240042a6605266ebcc0fc018c0fc02854cc0a4cdc399981c111299981e000880109980199b8000248008c10c00520000164801054cc0a54ccc0e8cdc38010008a511533303a33710004002266666604e026607e04000200400600a266666604e026607e04200400200800a266e254ccc0e8cdc48010008801080099816809981f80f999999813804181f00f8010021bad303e01d375a607c607e03666666604c00e607a03e0040066eb4c0f4070dd6981e981f00d19b81337026605401a607803a6eb4c0f0028dd6981e181e80499b813370266052018607603a6eb4c0ec028dd6981d80419b81483fbfffffffffffffffc04cc0a002cc0e806858c0f0008c0b8004dd5181b981c00d9bab323037303830390013036303700130370013302b375a606801e016606800260660026460666066605c00260660026464646464a66606066e1d20040021323232323200553330343370e900000109919191919191919191919191919191919191919191919191919191919191919191919191919191919191919191919191919299983319b87371a002901c0991919191919191924ca6660d80022930b18390019bad001306f001306f003375c00260d800260d80062c6eb8004c1a4004c1a400ccc8c184894ccc190004417c4cc170c00cc1a8004c008c1ac0048c8c8c8c80154ccc198cdc3a40000042646464646464649329998358008a4c2c60e200ca6660d466e1d200000213232533306c3370e6e340052038132324994ccc1b000452616307200316375c00260de0022a6660d466e1d200200213232533306c3370e6e340052038132324994ccc1b000452616307200316375c00260de0022c60e200460c60026ea8004c1ac00454ccc198cdc3a40040042646464646464646464649329998370008a4c2c60e80066eb4004c1c4004c1c400cdd6800983700098370019bad001306b00116306d002305f00137540026eb0004c198004c19800cdd6800983180098318019bad00130600013060003375a00260ba00260ba0066eb4004c168004c16800cdd6800982b800982b8019bad00130540013054003375a00260a200260a200ca66609466e1d20000021323232533304d5330453370e0029000099b87001480e04c8c8c94ccc140cdc49b8d001481004c8c9265333050001149858c15800c58dd7000982980098298020b1b8d001375c002609e0022c60a200460860026ea8004c12c004c12c0194ccc110cdc3a40000042646464a66608ea6607e66e1c005200013370e002901c0991919299982519b89371a00290200991924ca6660940022930b18280018b1bae001304d001304d00416371a0026eb8004c12400458c12c008c0f4004dd500098228009822803299981f19b87480000084c8c8c94ccc1054cc0e4cdc3800a4000266e1c005203813232325333044337126e340052040132324994ccc11000452616304a00316375c002608e002608e0082c6e34004dd700098218008b1822801181b8009baa001303f001303f00653330383370e90000010991919299981da9981999b87001480004cdc3800a40702646464a66607c66e24dc6800a40802646493299981f0008a4c2c60880062c6eb8004c104004c10401058dc68009bae001303d00116303f0023031001375400260720022c6076004605a0026ea8004c0d400458c0dc008c0a4004dd5000981898181819800981818188011bab32303030313032001302f3030001303000133024375a605a0120086644646464a666058646464a66605e66e1d2002002132323253330323370e90000010a5013375e6e9c010dd3800981c80118158009baa007132323253330323370e90010010a5013375e6e9c010dd3800981c80118158009baa00730360023028001375400220042c6464646464a66606066e1d200000213232323253330343370e9000001099191919299981c19b8748008008584cdd2a4000002607e00460620026ea8004c0e400458c0ec008c0b4004dd5000981a800899ba5480080a8c0dc008c0a4004dd5000981898191819800981818190029bab323232533302d3370e90010010b0a99981699b8f375c606400200c260646066606800e2c6068004604c0026ea8004c8c0bcc0c4004c0b8c0c000cdd718160051816004998111bad302b005001375860540046eb0c0a40084cc88cc8c094894ccc0a00045280a99981499baf302e00100314a226004605e0026ea4004008dd61814981298150009bae3029007302930290013029302700b30270013026001302600a30240013023001302200130210013020001301f001301f004301e001149858888888cdc499b833370466e0401000c008004cc030018014888888c8cdc199b825333019323253300a3371e6eb8c080c084008dd718101810800899b8f375c60400046eb8c080004c08402cc0800184cdc099b8100100300210010040053300b0060052233301300200100314a066e9520003300637520026600c6ea40040252201002232333004003375c602a0026eb8c054c058004c058004888cccc01000920002333300500248001d69bab00100323002375200244446601844a66601e002200a2a66602066ebcc02cc0540040184c010c060c0540044c008c0580040055d01198048008010a514bd702ab9d2253330063371000490000b0998018010009800911299980319b87002480004c02c0044cc00ccdc0801240046018002464600446600400400246004466004004002aae7d5cd118031801000918029801000918021801000918019801800aba15744460046ea800555cf01",
    };
  },
  {
    conf: {
      "title": "DepositConfig",
      "anyOf": [{
        "title": "Config",
        "dataType": "constructor",
        "index": 0,
        "fields": [
          {
            "title": "poolnft",
            "anyOf": [{
              "title": "Asset",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "bytes", "title": "policy" }, {
                "dataType": "bytes",
                "title": "name",
              }],
            }],
          },
          {
            "title": "x",
            "anyOf": [{
              "title": "Asset",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "bytes", "title": "policy" }, {
                "dataType": "bytes",
                "title": "name",
              }],
            }],
          },
          {
            "title": "y",
            "anyOf": [{
              "title": "Asset",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "bytes", "title": "policy" }, {
                "dataType": "bytes",
                "title": "name",
              }],
            }],
          },
          {
            "title": "lq",
            "anyOf": [{
              "title": "Asset",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "bytes", "title": "policy" }, {
                "dataType": "bytes",
                "title": "name",
              }],
            }],
          },
          { "dataType": "integer", "title": "exFee" },
          { "dataType": "bytes", "title": "rewardPkh" },
          {
            "title": "stakePkh",
            "anyOf": [{
              "title": "Some",
              "description": "An optional value.",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "bytes" }],
            }, {
              "title": "None",
              "description": "Nothing.",
              "dataType": "constructor",
              "index": 1,
              "fields": [],
            }],
          },
          { "dataType": "integer", "title": "collateralAda" },
        ],
      }],
    },
  },
  { action: { "title": "Data", "description": "Any Plutus data." } },
) as unknown as IRoyaltyPoolDepositValidate;

export interface IRoyaltyPoolRedeemValidate {
  new (): Validator;
  conf: {
    poolnft: { policy: string; name: string };
    x: { policy: string; name: string };
    y: { policy: string; name: string };
    lq: { policy: string; name: string };
    exFee: bigint;
    rewardPkh: string;
    stakePkh: string | null;
  };
  action: Data;
}

export const RoyaltyPoolRedeemValidate = Object.assign(
  function () {
    return {
      type: "PlutusV2",
      script:
        "5908fa5908f70100003232323232323232323232323232323232323232323232323232323232222533301c3232323232323232323232323253330293370e00290000991919191919191919191919191919299981c19b87480080084c8c8c8c94cc094cdc399814991bab303e303f3040001303d303e00e303d01f4800854cc094cc88cdd79ba73041002374e6082002607a00a607a0102a6604a66e1cccc0d88894ccc0e800440084cc00ccdc00012400460820029000009a40082a6604aa6604a66e24ccccc09000c004c8dd5981f181f9820000981e981f007181e80f19b80375a607a0186eb4c0f4028dd6981e981f801099b8933333024003001323756607c607e6080002607a607c01c607a03a66e00dd6981e8059bad303d303e00a375a607a607c607e004266e24cdc01bad303d303b303f002004302d01033028323756607a607c607e0026078607a00e607803666644464646464a6660826464a6605866e3cdd7182218228011bae3044304500113371e6eb8c110008dd718220009822816982200389998169ba800237500066ea000854ccc104c8c94cc0b0cdc79bae30443045002375c6088608a002266e3cdd718220011bae3044001304502d3044006133302d37500086ea0004dd400089998169ba800437500066ea120003370200400866e0400800ccc0ac04800ccc0a804400cc0ec070c0ec06c008cdc0a41fdfffffffffffffffe026604c646eacc0ecc0f0c0f4004c0e8c0ec02cc0e8064cdc09814991bab303a303b303c0013039303a004375a607202e2c6076004605a0026ea8c0d8c0dc064c0dc004cc0acdd6981a007805181a00098198009918199819981680098198009919191919299981a19b87480100084c8c8c8c8c80154ccc0e0cdc3a400000426464646464646464646464646464646464646464646464646464646464646464646464646464646464646464646464646464a6660d466e1cdc6800a4070264646464646464649329998360008a4c2c60e40066eb4004c1bc004c1bc00cdd7000983600098360018b1bae001306900130690033323061225333064001105f13305c3003306a0013002306b001232323232005533306a3370e900000109919191919191924ca6660d60022930b1838803299983719b87480000084c8c94ccc1c0cdc39b8d001480e04c8c926533306c001149858c1c800c58dd700098378008a99983719b87480080084c8c94ccc1c0cdc39b8d001480e04c8c926533306c001149858c1c800c58dd700098378008b183880118318009baa001306b0011533306a3370e900100109919191919191919191924ca6660dc0022930b183a0019bad00130710013071003375a00260dc00260dc0066eb4004c1ac00458c1b4008c17c004dd50009bac00130660013066003375a00260c600260c60066eb4004c180004c18000cdd6800982e800982e8019bad001305a001305a003375a00260ae00260ae0066eb4004c150004c15000cdd680098288009828803299982719b87480000084c8c8c94ccc1454cc114cdc3800a4000266e1c005203813232325333054337126e340052040132324994ccc14000452616305600316375c00260a600260a60082c6e34004dd700098278008b182880118218009baa001304b001304b00653330483370e900000109919192999825a9981f99b87001480004cdc3800a40702646464a66609c66e24dc6800a4080264649329998250008a4c2c60a00062c6eb8004c134004c13401058dc68009bae001304900116304b002303d0013754002608a002608a00ca66608466e1d2000002132323253330455330393370e0029000099b87001480e04c8c8c94ccc120cdc49b8d001481004c8c9265333044001149858c12800c58dd7000982380098238020b1b8d001375c00260860022c608a004606e0026ea8004c0fc004c0fc0194ccc0f0cdc3a40000042646464a66607ea6606666e1c005200013370e002901c0991919299982119b89371a00290200991924ca66607c0022930b18220018b1bae0013041001304100416371a0026eb8004c0f400458c0fc008c0c4004dd5000981c8008b181d80118168009baa0013035001163037002302900137540026062605e60660026060606200260620026604a6eb4c0b8028010cc88c8c8c94ccc0c4c8c8c94ccc0d0cdc3a40040042646464a66606e66e1d200000214a0266ebcdd38021ba7001303a002302c001375400e2646464a66606e66e1d200200214a0266ebcdd38021ba7001303a002302c001375400e606e00460520026ea8004400858c8c8c8c8c94ccc0d4cdc3a4000004264646464a66607266e1d2000002132323232533303d3370e90010010b099ba548000004c100008c0c8004dd5000981d0008b181e00118170009baa001303600113374a9001015981c00118150009baa001303230333034001303130330053756646464a66606466e1d200200216153330323371e6eb8c0cc0040184c0ccc0d0c0d401c58c0d4008c09c004dd50009918181819000981798188019bae302d00a302d302e00a33023375a605800c0026eb0c0ac00cdd618150018999119918131129998148008a501533302e3375e605e00200629444c008c0c0004dd48008011bac302a3025302b002375c605400ea66605066e1cdd698149815001a4000290000a4004605260520026052604e014604e002604c002604c012604800260460026044002604200260400026040008603e0022930b1111119b833370400866e04cc02800c00800401488ccc06400800400c52819ba548000cc020dd4803998041ba900700b2223374a900019805001998050011980500080691191998020019bae3017001375c602e60300026030002444666600800490001199980280124000eb4dd5800801918011ba900122223300e2253330110011005153330163375e601a602e00200c260086034602e0022600460300020024a66601600229000099980819baf3007301100137520046eb4c050c044dd5980a1808800a4000910100574046601a00200429452f5c0aae74894ccc028cdc4001240002c2660060040026002444a66601466e1c00920001300b0011330033370200490011806000919180111980100100091801119801001000aab9f2300730020012300630020012300530020012300430040015734ae855d1118011baa0015573c1",
    };
  },
  {
    conf: {
      "title": "RedeemConfig",
      "anyOf": [{
        "title": "Config",
        "dataType": "constructor",
        "index": 0,
        "fields": [
          {
            "title": "poolnft",
            "anyOf": [{
              "title": "Asset",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "bytes", "title": "policy" }, {
                "dataType": "bytes",
                "title": "name",
              }],
            }],
          },
          {
            "title": "x",
            "anyOf": [{
              "title": "Asset",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "bytes", "title": "policy" }, {
                "dataType": "bytes",
                "title": "name",
              }],
            }],
          },
          {
            "title": "y",
            "anyOf": [{
              "title": "Asset",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "bytes", "title": "policy" }, {
                "dataType": "bytes",
                "title": "name",
              }],
            }],
          },
          {
            "title": "lq",
            "anyOf": [{
              "title": "Asset",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "bytes", "title": "policy" }, {
                "dataType": "bytes",
                "title": "name",
              }],
            }],
          },
          { "dataType": "integer", "title": "exFee" },
          { "dataType": "bytes", "title": "rewardPkh" },
          {
            "title": "stakePkh",
            "anyOf": [{
              "title": "Some",
              "description": "An optional value.",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "bytes" }],
            }, {
              "title": "None",
              "description": "Nothing.",
              "dataType": "constructor",
              "index": 1,
              "fields": [],
            }],
          },
        ],
      }],
    },
  },
  { action: { "title": "Data", "description": "Any Plutus data." } },
) as unknown as IRoyaltyPoolRedeemValidate;

export interface IRoyaltyPoolFeeSwitchValidate {
  new (): Validator;
  action: Data;
}

export const RoyaltyPoolFeeSwitchValidate = Object.assign(
  function () {
    return {
      type: "PlutusV2",
      script:
        "59112259111f010000323232323232323232323232323232323232323232323232322253330153232323253330193370e900200109919191919191919191919191919191919191919191919191919191919191919191919191919191929981919b87333040222533304000110021330033370000490011824000a400004c90020a99819299981e80f0a51133223330430020014a06ae8cc114078cdd79821982200f1ba948810015330325330323375e608803060880182a6606466ebcc11005cc11002c54cc0c8cdd7982200b18220050a9981919baf3044015304400915330323375e6088024608800c2a6606466ebcc110034c1100044cdd798221822806982218228008a9981919b8948028ccc8c1048894ccc10400440084cc00ccc010008c120004c124004894ccc10ccc8c10c894ccc1080045280a99982319baf304900100314a22600460940026ea40040984cdc0001240042004900025eb1411c3703634e3e34c0e7fd9a7348ad213f9272279535c73f4ec00efe00bd00811cf3a12554ca0ccd1a220b1de839a1940bc1006f08768b636fa98c66c900811c518a9c32deedc0b82604972692a2b7eb6c10b020d77c3c72e764b15600811c68aa59a87dbdbf8f78386dc6b83e63149d8c13939a1b0cda39707f9a00811c1826edf8011dc6a084214b87a4ff690665692f4243e5bb5ff5aad53600811cd350803d45e327f8808469d5dde9e0f6fdc6e6637d85ed44cc37a12c000a99982099b8753330413370e6eb4c110c1180b9200014800054ccc104cdc39bad3044304602e4800852002153330413370e6eb4c110c1180b9200414801054ccc104cdc39bad3044304602e4801852006153330413370e6eb4c110c1180b920081480205200a4800054cc0c8cdc39bad3044013375a608800e2a6606466ebcc11003cc11000c54cc0c8cdd7982200718220010a998191981880d00c8a9981919191929981aa9981a99b873303702430470183303701e304701815330353370e66e04cc0dc078c11c068cc0dc090c11c068cdc09bad3047008375a608e028266e1ccdc09981b80f182380c9981b812182380c99b81375a608e00e6eb4c11c04c54cc0d54cc0d4cdc39981b800982380d181919b81375a608e0106eb4c11c0504cdc39981b800982380c981919b81375a608e00e6eb4c11c04c54cc0d4cdc79bae3047011375c608e00a2a6606a66e1ccc0dc09000d200215330353371290001bad304700813371290001bad30470073756608c608e6090002664608444a6660820022c264646464a66609266e1d200000213006304d005153330493371e6eb8c13000401c4c1300144c018c134014c138008c124004dd518249825800991824982580098240009bae304500f02630443042304602e13370e6eb4c110050dd698220040a99982099b8753330413370e6eb4c110c1180b9200014800054ccc104cdc39bad3044304602e4800852002153330413370e6eb4c110c1180b9200414801054ccc104cdc39bad3044304602e4801852006153330413370e6eb4c110c1180b920081480205200a480084c8c94cc0d0cdc39bad3046015375a608c0122a66068a6606866ebcc11804cc11801c4cdd7982300918230030a9981a19baf3046011304600515330343375e608c020608c0082a660686606404603a2a66068646464a66608c66e1d2002002132323253330493370e90000010a5013375e6e9c010dd3800982700118248009baa004132323253330493370e90010010a5013375e6e9c010dd3800982700118248009baa004304b00230460013754004266e1cdd6982300b1bad304600a3045304701a3044304601a153330413370ea66608266e1cdd69822182301724000290000a99982099b87375a6088608c05c90010a40042a66608266e1cdd69822182301724008290020a99982099b87375a6088608c05c90030a400c2a66608266e1cdd69822182301724010290040a401490020a9981929981919baf3044011304400513375e608802060880082a6606466ebcc11003cc11000c54cc0c8cdd7982200718220010a998192998191981801080d89981880d00c8a9981919b87375a60880286eb4c11002054cc0c8cdc49bad304400748302e0084cdc4a40046eb4c11001c54ccc104cdc3a99982099b87375a6088608c05c90000a40002a66608266e1cdd69822182301724004290010a99982099b87375a6088608c05c90020a40082a66608266e1cdd6982218230172400c290030a99982099b87375a6088608c05c90040a4010290052400c2a6606466e1cdd698220099bad304400715330325330323375e6088022608800a266ebcc110040c11001054cc0c8cdd7982200798220018a998192998191981801080d89981880d00c899b87375a60880286eb4c11002054ccc104cdc3a99982099b87375a6088608c05c90000a40002a66608266e1cdd69822182301724004290010a99982099b87375a6088608c05c90020a40082a66608266e1cdd6982218230172400c290030a99982099b87375a6088608c05c90040a401029005240102a6606466e1cdd698220099bad304400715330325330323375e6088022608800a266ebcc110040c11001054cc0c8cdd7982200718220010a998192998191981801080d89981880d00c899b87375a60880286eb4c11002054cc0c9288a9981919b87375a60880266eb4c11001c54cc0c94cc0c8cdd798220089822002899baf3044010304400415330323375e608801c60880042a6606466ebcc11003cc11000c54cc0c94cc0c8cc0c008406c4cc0c406806454cc0c8cdc49bad3044008482fa68304cdc4a40046eb4c110020c110004c10c004c0fcc108004c104004c100004c0fc004c0f8004c0f4004c0f0004c0ec004c0e8004c0e8040c0e0004c0dc004c0ccc0d8004c0d4004c0d0004c0cc004c0c8004c0c4004c0c0004c0bc004c0b8004c0b8020c0acc0b4010c8c0acc0b4004c0a8c0ac020dd598149815181580119191919299981499b87480100084c8c8c8c8c80154ccc0b4cdc3a400000426464646464646464646464646464646464646464646464646464646464646464646464646464646464646464646464646464a6660be66e1cdc6800a4070264646464646464649329998310008a4c2c60d20066eb4004c198004c19800cdd7000983180098318018b1bae0013060001306000333052232323232005533305f3370e900000109919191919191924ca6660c20022930b1834003299983199b87480000084c8c94ccc194cdc39b8d001480e04c8c9265333062001149858c1a400c58dd700098330008a99983199b87480080084c8c94ccc194cdc39b8d001480e04c8c9265333062001149858c1a400c58dd700098330008b183400118318009baa00130620011533305f3370e900100109919191919191919191924ca6660c80022930b18358019bad00130680013068003375a00260ca00260ca0066eb4004c18800458c190008c17c004dd50009bac001305d001305d003375a00260b400260b40066eb4004c15c004c15c00cdd6800982a000982a0019bad00130510013051003375a002609c002609c0066eb4004c12c004c12c00cdd680098240009824003299982199b87480000084c8c8c94ccc1194cc100cdc3800a4000266e1c005203813232325333049337126e340052040132324994ccc11800452616304d00316375c002609400260940082c6e34004dd700098230008b182400118218009baa00130420013042006533303d3370e90000010991919299982029981d19b87001480004cdc3800a40702646464a66608666e24dc6800a4080264649329998200008a4c2c608e0062c6eb8004c110004c11001058dc68009bae0013040001163042002303d00137540026078002607800ca66606e66e1d20000021323232533303a5330343370e0029000099b87001480e04c8c8c94ccc0f4cdc49b8d001481004c8c926533303a001149858c10400c58dd7000981f000981f0020b1b8d001375c00260740022c6078004606e0026ea8004c0d8004c0d80194ccc0c4cdc3a40000042646464a666068a6605c66e1c005200013370e002901c0991919299981b99b89371a00290200991924ca6660680022930b181d8018b1bae0013038001303800416371a0026eb8004c0d000458c0d8008c0c4004dd500098180008b181900118168009baa001302c00116302e002302900137540026050604c6054002664604844a6660460022c2a66604e66e1ccc068c8dd59815981618168009815000801a400426054002260046056002604e604a60520220106eacc8c09cc0a0c0a4004c098c09cc0a0004cc07ccdc0a40046eb4c094c098c09c03c01cc8c8c8c8c94ccc098cdc3a40080042646464646400aa66605466e1d200000213232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232323232533305c3370e6e340052038132323232323232324994ccc17c004526163066003375a00260c600260c60066eb8004c180004c18000c58dd7000982e800982e80199827919191919002a99982e19b87480000084c8c8c8c8c8c8c926533305e001149858c1940194ccc180cdc3a400000426464a6660c466e1cdc6800a40702646493299982f8008a4c2c60cc0062c6eb8004c18c00454ccc180cdc3a400400426464a6660c466e1cdc6800a40702646493299982f8008a4c2c60cc0062c6eb8004c18c00458c194008c180004dd5000982f8008a99982e19b87480080084c8c8c8c8c8c8c8c8c8c9265333061001149858c1a000cdd6800983280098328019bad00130620013062003375a00260be0022c60c200460b80026ea8004dd6000982d000982d0019bad00130570013057003375a00260a800260a80066eb4004c144004c14400cdd6800982700098270019bad001304b001304b003375a002609000260900066eb4004c114004c1140194ccc100cdc3a40000042646464a666086a6607a66e1c005200013370e002901c0991919299982319b89371a00290200991924ca6660860022930b18250018b1bae0013047001304700416371a0026eb8004c10c00458c114008c100004dd5000981f800981f803299981d19b87480000084c8c8c94ccc0f54cc0dccdc3800a4000266e1c005203813232325333040337126e340052040132324994ccc0f400452616304400316375c002608200260820082c6e34004dd7000981e8008b181f801181d0009baa0013039001303900653330343370e90000010991919299981ba9981899b87001480004cdc3800a40702646464a66607466e24dc6800a40802646493299981b8008a4c2c607c0062c6eb8004c0ec004c0ec01058dc68009bae0013037001163039002303400137540026066002606600ca66605c66e1d20000021323232533303153302b3370e0029000099b87001480e04c8c8c94ccc0d0cdc49b8d001481004c8c9265333031001149858c0e000c58dd7000981a800981a8020b1b8d001375c00260620022c6066004605c0026ea8004c0b400458c0bc008c0a8004dd500098148008b181580118130009baa001302530233027001302430250023756646048604a604c002604660480026048002660366eb4c084c088c08c02c00cdd6181019181118111811180f80098108019bac301f0023758603c004603c603c002603c60380082c603c00460320026ea8c064c068004c0680045261623370290000009119baf374c0046e9800488cdd79ba73017002374e602e00244666022004002006294088c8ccc01000cdd718098009bae3013301400130140012223333004002480008cccc014009200075a6eac00400c8c008dd480091111980691299980600088028a99980819baf3012301300100613004301630130011300230140010012300922533300800110041330053003300f001300230100014bd702ba023300700100214a2aae7c894ccc014cdc4001240002c2660060040026002444a66600a66e1c0092000130080011330033370200490011804800919180111980100100091801119801001000ab9a2300430040015573aae855d1118011baa0015573d",
    };
  },
  { action: { "title": "Data", "description": "Any Plutus data." } },
) as unknown as IRoyaltyPoolFeeSwitchValidate;

export interface IDepositDeposit {
  new (): Validator;
  datum: {
    poolNft: { policy: string; name: string };
    redeemer: string;
    minExpectedLpAmount: bigint;
  };
  action: {
    ApplyOrder: {
      redeemerInIx: bigint;
      redeemerOutIx: bigint;
      poolInIx: bigint;
    };
  } | "CancelOrder";
}

export const DepositDeposit = Object.assign(
  function () {
    return {
      type: "PlutusV2",
      script:
        "5904bc0100003232323232323223232322323225333009323232533300c300a300d375400e26464646464646464646464a66602e602a60306ea80044c8c8c8c8c8c8c94ccc078c070c07cdd5000899191919299981119b8748010c08cdd50008992999811981098121baa00113232323232323232323232323232323232323232323232323232533304030430021323232323232323232323253330483046304937540022646464646464a66609c609066600605c0660642a66609c0042002294052819192999827982698281baa00113371e0886eb8c150c144dd5000899baf300230513754606260a26ea80e0014c004c140dd5180098281baa00623053001337120666660026eacc144c14800c01c018888c94ccc13cc124c140dd50008a400026eb4c150c144dd5000992999827982498281baa00114c103d87a800013233001001375660aa60a46ea8008894ccc150004530103d87a8000132323253330543371e00e6eb8c15400c4c104cc160dd4000a5eb804cc014014008dd6982a801182c001182b00099198008008021129998298008a6103d87a8000132323253330533371e00e6eb8c15000c4c100cc15cdd3000a5eb804cc014014008dd5982a001182b801182a800982780098259baa001304d304a37540022c6606406a0726eb8c12cc130008dd7182500098231baa018330050092375a0026600801846eb8004c0f8054cc0080588dd68009980080b9181e8009119198008008019129998228008a4c2646600600660920046006608e00260740322c6eb4c104004c104008dd6181f800981f8011bae303d001303d0023758607600260760046eb4c0e4004c0e4008dd6981b800981b8011bad3035001303500232533303230310011533302f3029303000114a22a66605e605a606000229405858dd518198009819801181880098188011bac302f001302f0023758605a002605a0046eb4c0ac004c0ac008c0a4004c094dd50008b181398121baa00116302630270023756604a002604a60426ea8c004c084dd5181218109baa002230243025001163300800c00e375c604260440046eb8c080004c070dd5180f8011bad301e301f301f001301a375402e603860326ea800458cc004014dd6980d8051800800911299980d0010a60103d87a80001323253330193017003130063301d0024bd70099980280280099b8000348004c07800cc070008dd2a40006eb0c05cc060c060008dd6180b00098091baa007375a6028602a0046eb4c04c004c04c004c038dd5003899198008008019129998088008a50132533300f3371e6eb8c050008010528899801801800980a0009bae30103011300d37540146eb0c03cc040c040c040c040c040c040c040c040c030dd5000980718059baa00114984d958c94ccc020c0180044c8c8c8c8c8c94ccc044c05000852616375a602400260240046eb4c040004c040008dd6980700098051baa0031533300830020011533300b300a37540062930b0b18041baa002370e90012999802180118029baa0031323232323232533300d3010002132498c01c01458dd6980700098070011bae300c001300c002300a001300637540062c4a6660086004600a6ea80044c8c8c8c94ccc02cc03800852616375c601800260180046eb8c028004c018dd50008b1b87480015cd2ab9d5573caae7d5d02ba15745",
    };
  },
  {
    datum: {
      "title": "DepositData",
      "description": "AMM-orders data.",
      "anyOf": [{
        "title": "DepositData",
        "dataType": "constructor",
        "index": 0,
        "fields": [
          {
            "title": "poolNft",
            "anyOf": [{
              "title": "Asset",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "bytes", "title": "policy" }, {
                "dataType": "bytes",
                "title": "name",
              }],
            }],
          },
          { "dataType": "bytes", "title": "redeemer" },
          { "dataType": "integer", "title": "minExpectedLpAmount" },
        ],
      }],
    },
  },
  {
    action: {
      "title": "OrderAction",
      "description": "Order action types.",
      "anyOf": [{
        "title": "ApplyOrder",
        "dataType": "constructor",
        "index": 0,
        "fields": [{ "dataType": "integer", "title": "redeemerInIx" }, {
          "dataType": "integer",
          "title": "redeemerOutIx",
        }, { "dataType": "integer", "title": "poolInIx" }],
      }, {
        "title": "CancelOrder",
        "dataType": "constructor",
        "index": 1,
        "fields": [],
      }],
    },
  },
) as unknown as IDepositDeposit;

export interface IPoolValidatePool {
  new (daoVotingWitness: string): Validator;
  inputDatum: {
    poolNft: { policy: string; name: string };
    n: bigint;
    tradableAssets: Array<{ policy: string; name: string }>;
    tradableTokensMultipliers: Array<bigint>;
    lpToken: { policy: string; name: string };
    lpFeeIsEditable: boolean;
    amplCoeff: bigint;
    lpFeeNum: bigint;
    protocolFeeNum: bigint;
    daoStabeProxyWitness: Array<string>;
    treasuryAddress: string;
    protocolFees: Array<bigint>;
    inv: bigint;
  };
  redeemer: {
    poolInIx: bigint;
    poolOutIx: bigint;
    action:
      | { AMMAction: { optionInt0: bigint; optionInt1: bigint } }
      | "PDAOAction";
  };
}

export const PoolValidatePool = Object.assign(
  function (daoVotingWitness: string) {
    return {
      type: "PlutusV2",
      script: applyParamsToScript(
        applyDoubleCborEncoding(
          "590e600100003232323232323223223232323232322322533300d3232323232325333013300c30143754008264646464646464646464a66603a6032603c6ea80044c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c94ccc0d0c0c0c0d4dd500089919191919299981c99b8748010c0e8dd5001099191919191919191919191919191919191919299982619baf03902e1533304c0071533304c0041533304c0031533304c002100114a029405280a5014a0a666096608e60986ea80d04c8ccc8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c888c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c94ccc234054ccc23404cdc38111bad3092010151533308d013370e0400262a66611a0266ebcdd380f1ba701113371e03801e29405280a50100114a0a66611802666118026110020129412889919191919191919299984a009999981d8048188058050098a99984a008038a99984a008018a99984a0080108008a5014a0294052819b8733702900000819b833370466e040c807ccdc0a41fdfffffffffffffffffffffffffffffffffe0e02203e666660726607464666600200200e0066607200a466e0ccdc119b82031029001005222253330990100314bd7009919299984d808020a5eb804c8c94ccc2740401452f5c026613c026ea0cdc099b81004002375a613e0200a666601001000600261400200a613e0200a6eb4c27404010c27404010dd6984d8080181581780480401899baf374e0126e9c004cc0d80088cdc199b823370405c04800200466e08cdc12401060ec05890604d0619981e0011981a004119b833370400205e03644a6661200266e200040084cdc0801000899b81001002333330343303500102602a00400302d330350070191323232533308f01308b01309001375400226464a66612202611a026124026ea80044c8c94ccc24c04c23c04c25004dd500089919299984a80984880984b009baa00113232533309701309301309801375400226464a66613202612a026134026ea80044c8c8c8c8c8c8c8c8c8c94ccc28c04cdc4982319b803370401c012600e06c66e092004009153330a301008153330a301006153330a301005153330a301004153330a301002100114a029405280a5014a02940ccccccc10cc8ccc004005200001822253330a80100114bd70099854809ba83253330a6013370e00608a2600c66e0ccdc019b823370600201a018601407601820026eb4c2a804004ccc00c00cc154008c2ac040040b80fc064060c0080fc104ccccccc1080580800f806005cc0040f8100dc100399b8700833700018014a66613c0266e1ccc1000488c26c04004cdc001da4006266e1c03c03452819b8930403370266e0802800cc004cdc0a4181341806466e092004003370400a66e1ccc0f404c8c26004004c208040e0cdc099b814830268300bc0b4dd6984f00984d809baa001163303c031037375a6138026132026ea800458cc0e80240d4dd6984d00984b809baa001163303800c033375a613002612a026ea800458cc0d80180c4dd6984b009849809baa0011633034003030375a6128026122026ea800458cc0c80080b8cc0d8020024cc0d4014018cc0cc010090cdc11bad308f010210013303b02602633031008015330300020073302f0020133302f02004f3302e01f0643370266607809a01a0180026660760c40180166eb4c21804c21c04008dd61842808009842808011bae30830100130830100237586102020026102020046eb4c1fc004c1fc004c1f8c1f8c1f8c1f8c1f8c1f8c1f8c1e8dd501f9bae307c307d002375c60f600260ee6ea8c1e8034dd6983c983d0011bac30780013078002375c60ec00260ec0046eb0c1d0004c1d0008dd6983900098390011bad30700013070001306f306f001306e002375860d800260d80046eb0c1a8004c1a8008dd69834000983418321baa05f22222223232323232323232533306e306a306f3754002264646464a6660e466e20020ccccc07c03c048028dd6983980119b8100b0061337100100022940ccccc078038044024dd6983980099b8000a005337606ea0cdc1180380099b81002005375066e08c01c004cdc000100299b83009001375a60e660e06ea800458cc044038020cdc100080499b8200a0073333301700700a0023001004003370400e66034010603000e66e08010c048020c04801c88ccc050009200022533306230030021301200110012533305e337100029000099b81480000044004c0040048894ccc1840085300103d87a8000132325333060305c0031304833064375000497ae0133300500500130470033065003375a60c600444646600200200644a6660c2002297ae0133062375060066eb4c18c004cc008008c19000488888c8c8c8c94ccc188cdc4001199998088040031824802802001899b8800200114a06666602000e00a60220080060046666601e00c00800600400266e0800cc028018cdc10019805002911998040010009119b8200200122333007002001223370200400244646600200200644a6660ba002297ae013305e3750646660280086eb8c180004dd718301830800982e1baa305f00133002002306000123330070014800088cdc00010009199803191980080080111299982d0008a5eb804c8c94ccc164c148c168dd500089980200200109982e982f182d9baa0013300400400232533305933305930550014a09444c104cc174dd4000a5eb80530103d87a8000375a60ba00460ba00290011119b82002001222223232533305a33710002004266e040080044cdc080080119b800020053370066e08014010cdc180180111119199800800802001911299982d8010a5eb804c8c94ccc17400c52f5c02660bc6ea0cc018008dd6982f8019998028028009830001982f8019bad305d002222223232533305833710002004266e040080044cdc080080119b800020043370066e0801000ccdc19980400198030028011b8048008888ccc01800c00888cc00c004008c0040048894ccc140cdc4000a4000290000a99982818260008a40042a646660a2609a66e1800920041333004004300100333706004900209800999802002180080199b83303800248010dc100111119199800800802001911299982a80108008999801801982c001198021bad3057002001375a60a20026eb4c144c148004c134dd501a0992999826182418269baa0011323232533304f330023756600660a26ea8104dd7182a18289baa004100114a0660026eacc008c140dd50200261119191980080080211299982a8008a5013253330533375e00860a860b000429444cc00c00c004c160004c0dccc14ccdd2a4004660a66ea40052f5c097ae0230523053305330533053305330530011633323001001222533305100214c0103d87a8000132325333050304c0031303833054375200497ae0133300500500130370033055003375c60a600403a900019baf374e0306e9cc0640514ccc124cdd781318270070a99982499b8702400c153330493375e6e9c088dd38050a99982499baf374e0406e9c02054ccc124cdd780f0030a99982480e08028999824802a504a229405280a5014a02940c104ccc004048dd718268011bae304d304e00222232533304b3044304c37540022900009bad3050304d375400264a666096608860986ea8004530103d87a800013233001001375660a2609c6ea8008894ccc140004530103d87a8000132323253330503371e00e6eb8c14400c4c0e0cc150dd4000a5eb804cc014014008dd69828801182a001182900099198008008021129998278008a6103d87a80001323232533304f3371e00e6eb8c14000c4c0dccc14cdd3000a5eb804cc014014008dd598280011829801182880098241baa0233375e04801a66e2120003045375460926094004609000260900046eb0c118004c118008dd6182200098220011bad30420013042001303d37540046064002607c60766ea800858c0f4c0e8dd5181e802181e181e8011bab303b001303b001303637546072606c6ea800458cc060084074c0040488c008004c004004894ccc0d000452f5c026606a6064606c00266004004606e0026eb0c0ccc0d0c0d0c0d0c0d0008cdc424000605c6ea8c0c8004c0c8008c0c0004c0c0008dd6181700098170011bac302c001302c002375a60540026054004605000260486ea807cc098c08cdd518130011bab30253026001302137546048604a0046046002603e6ea8c088c07cdd50008b198008059bad30210083001001222533302000214c0103d87a800013232533301f301b00313007330230024bd7009998028028009803001981200198110011b8048004dd2a40006038603a0046eb4c06c004c06c004c058dd5005180c180a9baa004163758602e603060300046eb0c058004c048dd5001180a180a801180980098079baa00114984d9594ccc02cc01cc030dd50008991919191919299980a180b80109924c64a666024601c002264646464a66603260380042930b1bad301a001301a002375a603000260286ea800854ccc048c02c00454ccc054c050dd50010a4c2c2c60246ea800458c054004c054008dd6980980098098011bad3011001300d37540022c600200c4a666012600a60146ea80044c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c94ccc098c0a40084c8c8c8c8c8c926330220082375a0026604201646eb8004c084050cc07c0548dd68009980f00b11810000980f00c8b1bad302700130270023758604a002604a0046eb8c08c004c08c008dd6181080098108011bad301f001301f002375a603a002603a0046eb4c06c004c06c008c94ccc060c05c00454ccc054c038c0580045288a99980a9808980b0008a501616375460320026032004602e002602e0046eb0c054004c054008dd6180980098098011bad30110013011002300f001300b37540022c6e1d200222323300100100322533300d00114984c8cc00c00cc044008c00cc03c00494ccc018c008c01cdd5000899191919299980698080010a4c2c6eb8c038004c038008dd7180600098041baa00116370e90001bae0015734aae7555cf2ab9f5740ae855d11",
        ),
        [daoVotingWitness],
        { "dataType": "list", "items": [{ "dataType": "bytes" }] },
      ),
    };
  },
  {
    inputDatum: {
      "title": "PoolData",
      "description": "Pool data.",
      "anyOf": [{
        "title": "PoolData",
        "dataType": "constructor",
        "index": 0,
        "fields": [
          {
            "title": "poolNft",
            "anyOf": [{
              "title": "Asset",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "bytes", "title": "policy" }, {
                "dataType": "bytes",
                "title": "name",
              }],
            }],
          },
          { "dataType": "integer", "title": "n" },
          {
            "dataType": "list",
            "items": {
              "title": "Asset",
              "anyOf": [{
                "title": "Asset",
                "dataType": "constructor",
                "index": 0,
                "fields": [{ "dataType": "bytes", "title": "policy" }, {
                  "dataType": "bytes",
                  "title": "name",
                }],
              }],
            },
            "title": "tradableAssets",
          },
          {
            "dataType": "list",
            "items": { "dataType": "integer" },
            "title": "tradableTokensMultipliers",
          },
          {
            "title": "lpToken",
            "anyOf": [{
              "title": "Asset",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "bytes", "title": "policy" }, {
                "dataType": "bytes",
                "title": "name",
              }],
            }],
          },
          {
            "title": "lpFeeIsEditable",
            "anyOf": [{
              "title": "False",
              "dataType": "constructor",
              "index": 0,
              "fields": [],
            }, {
              "title": "True",
              "dataType": "constructor",
              "index": 1,
              "fields": [],
            }],
          },
          { "dataType": "integer", "title": "amplCoeff" },
          { "dataType": "integer", "title": "lpFeeNum" },
          { "dataType": "integer", "title": "protocolFeeNum" },
          {
            "dataType": "list",
            "items": { "dataType": "bytes" },
            "title": "daoStabeProxyWitness",
          },
          { "dataType": "bytes", "title": "treasuryAddress" },
          {
            "dataType": "list",
            "items": { "dataType": "integer" },
            "title": "protocolFees",
          },
          { "dataType": "integer", "title": "inv" },
        ],
      }],
    },
  },
  {
    redeemer: {
      "title": "PoolRedeemer",
      "anyOf": [{
        "title": "PoolRedeemer",
        "dataType": "constructor",
        "index": 0,
        "fields": [{ "dataType": "integer", "title": "poolInIx" }, {
          "dataType": "integer",
          "title": "poolOutIx",
        }, {
          "title": "action",
          "description": "Pool action types.",
          "anyOf": [{
            "title": "AMMAction",
            "dataType": "constructor",
            "index": 0,
            "fields": [{ "dataType": "integer", "title": "optionInt0" }, {
              "dataType": "integer",
              "title": "optionInt1",
            }],
          }, {
            "title": "PDAOAction",
            "dataType": "constructor",
            "index": 1,
            "fields": [],
          }],
        }],
      }],
    },
  },
) as unknown as IPoolValidatePool;

export interface IProxyDaoStablePoolDao {
  new (): Validator;
  datum: { poolNft: { policy: string; name: string } };
  action: {
    poolInIx: bigint;
    poolOutIx: bigint;
    daoInIx: bigint;
    daoOutIx: bigint;
    daoActionIx: bigint;
  };
}

export const ProxyDaoStablePoolDao = Object.assign(
  function () {
    return {
      type: "PlutusV2",
      script:
        "5908a3010000323232323232322323232322322533300932323253323300d3001300e3754004264646464646464646464646464646464a66603a6036603c6ea80044c8c8c8c8c8c94cc8cc090c004c094dd5001099192999813181218139baa00113232323232533302b3029302c375400226464646464a666060605c60626ea80044c8c8c8c8c94ccc0d4c048c0d8dd500089919299981b981a981c1baa0011323232533303a3038303b375400226464646464a66607e603860806ea80044c8c8c8c8c8c94ccc114cdd781c182518239baa01a153330450041533304500315333045002100114a029405280a50323232323232323232323232323232323232323232323232323232323253330613370e6eb4c198c19c044dd6983318338020a99983080108008a5014a0a64646464646464646660d060cc0a026464a6660d40422a6660d40042002294052829998348038a9998348030a9998348040a9998348028a9998348020a99983480108018a5014a029405280a5014a0a6660d06010020266e2404120c0b80214a02a646660d260ba0a2264a6660d4a6660d46014020266e2404120be9a0c14a0200229414ccc1a401c54ccc1a401854ccc1a402054ccc1a401454ccc1a400454ccc1a400c40085280a5014a029405280a50153330693046051153330690071533306900615333069008153330690051533306900115333069004100214a029405280a5014a0294054ccc1a4cdc3828a400c26464a6660d6a6660d60142a6660d600e2a6660d60062a6660d600c2a6660d600a200829405280a5014a02940400452819baf374e660486604a00206e6604a0020926e9ccc090030064dd618371837983798359baa045153330693370e0a290040a9998348038a9998348030a9998348040a9998348028a9998348008a99983480208018a5014a029405280a5014a02a6660d266e1c145200a132533306a533306a300a01413371202890504e008a50100114a0a6660d200e2a6660d200c2a6660d20102a6660d20022a6660d20082a6660d2006200429405280a5014a029405280a99983499b870514803054ccc1a401c54ccc1a401854ccc1a401454ccc1a400454ccc1a401054ccc1a400c40085280a5014a029405280a5014a066e1c040070cdd79ba7375860d80186e9cdd6183600c99b8f375c60d60146eb8c1ac05ccdc38059bad306a0183370e6eb4c1a4068038cdd79ba6042374c06066ebcdd38089ba70043375e0760546e2520023370e66603607c0180166660360580180166eb0c18c004c18c004c188004c184008dd6982f800982f8011bad305d001305d002375a60b600260b660b660b660b660b660b660ae6ea8080dd7182c982d0011bae30580013054375460ae0166eb0c158004c158004c154004c150004c14c008dd698288009828800982800119b8848000c128dd518270009827000982698269826982698249baa0232232333001001003002222533304e00214bd700991929998280018a5eb804cc144dd419b81002375a60a400666600a00a00260a600660a40066eb4c14000888c8cc00400400c894ccc13000452f5c026609a6ea0c8ccc018010dd718278009bae304f3050001304b3754609c00266004004609e00244464a666090607860926ea8004520001375a609a60946ea8004c94ccc120c0f0c124dd50008a6103d87a8000132330010013756609c60966ea8008894ccc134004530103d87a80001323232533304d3371e00e6eb8c13800c4c0d0cc144dd4000a5eb804cc014014008dd698270011828801182780099198008008021129998260008a6103d87a80001323232533304c3371e00e6eb8c13400c4c0cccc140dd3000a5eb804cc014014008dd598268011828001182700099baf374c02a6e98018cdd782080199baf014006303c0013044304137540022c608660880046eacc108004c108008c100004c0f0dd5181f981e1baa001163301e0290223010003303c303937540022c603260706ea8014c0e8c0dcdd50008b181c981d0011bab30380013038002303600130323754606a60646ea800458cc05007c070dd59819981a001181900098171baa300f302e37540026060605a6ea800458cc03c06c054c00401494ccc0a4c09cc0a8dd50008991919191919191919191919191919191919191919191919191929998231824801099191919191924c6604201046eb4004cc08002c8dd7000982180a1980f00a91bad0013301d01623042001304001916375a608e002608e0046eb0c114004c114008dd7182180098218011bac30410013041002375a607e002607e0046eb4c0f4004c0f4008dd6981d800981d80119299981c181b8008a99981a9814981b0008a51153330353033303600114a02c2c6ea8c0e4004c0e4008c0dc004c0dc008dd6181a800981a8011bac30330013033002375a60620026062004605e00260566ea80045888c8cc00400400c894ccc0b800452613233003003303200230033030001302b302837540022c6010604e6ea8018c0a4c098dd50011b874801058c09cc0a0008dd598130009813001181200098101baa300130203754604660406ea80088c08cc09000458cc004034dd69810805980080091129998100010a60103d87a800013232533301f301d00313006330230024bd70099980280280099b8000348004c09000cc088008dd2a40006eb4c074c078008dd6980e000980e0011bad301a001301a002375a6030002603000260266ea802cdd6180a980b180b0011bac3014001301037540086024601e6ea8008dc3a40042c60206022004601e00260166ea8004526136565333007300530083754002264646464646464646464a666028602e0042930b1bad30150013015002375a602600260260046eb4c044004c044008dd6980780098078011bad300d001300937540022c60020084a66600a6006600c6ea80044c8c94ccc028c0340084c926300400116300b001300737540022c4a6660086004600a6ea80044c8c8c8c94ccc02cc03800852616375c601800260180046eb8c028004c018dd50008b1b87480015cd2ab9d5573caae7d5d02ba15745",
    };
  },
  {
    datum: {
      "title": "DAOData",
      "description": "DAO contract config (congig is immutable).",
      "anyOf": [{
        "title": "DAOData",
        "dataType": "constructor",
        "index": 0,
        "fields": [{
          "title": "poolNft",
          "anyOf": [{
            "title": "Asset",
            "dataType": "constructor",
            "index": 0,
            "fields": [{ "dataType": "bytes", "title": "policy" }, {
              "dataType": "bytes",
              "title": "name",
            }],
          }],
        }],
      }],
    },
  },
  {
    action: {
      "title": "DAOAction",
      "description": "DAO action types:",
      "anyOf": [{
        "title": "DAOAction",
        "dataType": "constructor",
        "index": 0,
        "fields": [
          { "dataType": "integer", "title": "poolInIx" },
          { "dataType": "integer", "title": "poolOutIx" },
          { "dataType": "integer", "title": "daoInIx" },
          { "dataType": "integer", "title": "daoOutIx" },
          { "dataType": "integer", "title": "daoActionIx" },
        ],
      }],
    },
  },
) as unknown as IProxyDaoStablePoolDao;

export interface IRedeemRedeem {
  new (): Validator;
  datum: {
    poolNft: { policy: string; name: string };
    redeemer: string;
    expectedAssets: Array<{ policy: string; name: string }>;
    minExpectedReceivedAssetsBalances: Array<bigint>;
    minExpectedLpChange: bigint;
  };
  action: {
    ApplyOrder: {
      redeemerInIx: bigint;
      redeemerOutIx: bigint;
      poolInIx: bigint;
    };
  } | "CancelOrder";
}

export const RedeemRedeem = Object.assign(
  function () {
    return {
      type: "PlutusV2",
      script:
        "5905b401000032323232323232232323232232322533300a323232533300d300b300e375400e26464646464646464646464a666030602c60326ea80044c8c8c8c8c8c8c8c8c8c8c94ccc08cc084c090dd5000899191919299981399b8748010c0a0dd50008992999814181318149baa0011323232323232323232323232323232323232323232323232323253330453048002132323232323232323232533304c304a304d3754002264646464646464a6660a6609866600805c0660642a6660a60062a6660a6004200229405280a5032325333054305230553754002266e3c120dd7182c982b1baa00113375e600460ac6ea8c0c4c158dd501e0039800982a9baa006230580013370e66600400601000e066646600200264666002002646600200207044a6660ae002297ae013305837506466600c00e6eb8c168004dd7182d182d800982b1baa305900133002002305a001035222533305700214bd7009919299982c8018a5eb804cc168ccc158cdc49bad305b0030024c0103d87a80004c0103d8798000333005005001305c003305b003375a60b200444a6660aa00229444c94ccc14d4ccc14ccdc42400060a86ea8c1600085288999829a514a09444cc00c00c004528182c0009111929998299826182a1baa0011480004dd6982c182a9baa001325333053304c305437540022980103d87a800013233001001375660b260ac6ea8008894ccc160004530103d87a8000132323253330583371e00e6eb8c16400c4c110cc170dd4000a5eb804cc014014008dd6982c801182e001182d000991980080080211299982b8008a6103d87a8000132323253330573371e00e6eb8c16000c4c10ccc16cdd3000a5eb804cc014014008dd5982c001182d801182c8009bab305330540023052001304e375460a2609c6ea800458cc0d40e00f0dd7182798280011bae304e001304a375402e6608601046eb4004cc10802c8dd7000982100a1982000a91bad0013303f01623041001303f01916375a608c002608c0046eb0c110004c110008dd7182100098210011bac30400013040002375a607c002607c0046eb4c0f0004c0f0008dd6981d000981d00119299981b981b0008a99981a1816981a8008a51153330343032303500114a02c2c6ea8c0e0004c0e0008c0d8004c0d8008dd6181a000981a0011bac30320013032002375a60600026060004605c00260546ea800458c0b0c0a4dd50008b181598160011bab302a001302a302637546002604c6ea8c0a4c098dd50011181498150008b198060080091bae30263027002375c604a00260426ea8c090018dd6981198120011bac30220013022002375860400026040604000260366ea8060c074c068dd50008b198008029bad301c00a3001001222533301b00214c0103d87a800013232533301a3018003130063301e0024bd70099980280280099b8000348004c07c00cc074008dd2a40006eb0c060c064c064008dd6180b80098099baa007375a602a602c0046eb4c050004c050004c03cdd5003899198008008019129998090008a5013253330103371e6eb8c054008010528899801801800980a8009bae30113012300e37540166eb0c040c044c044c044c044c044c044c044c044c034dd5000980798061baa00114984d958c94ccc024c01c0044c8c8c8c8c8c94ccc048c05400852616375a602600260260046eb4c044004c044008dd6980780098059baa0031533300930020011533300c300b37540062930b0b18049baa002370e90012999802980198031baa004132323232323232323232533301230150021323232498cc0340148dd6800998060031180700098060048b1bad301300130130023758602200260220046eb0c03c004c03c008dd718068009806801180580098039baa0041622323300100100322533300b00114984c8cc00c00cc03c008c00cc03400494ccc010c008c014dd5000899191919299980598070010a4c2c6eb8c030004c030008dd7180500098031baa00116370e90002b9a5573aaae7955cfaba05742ae881",
    };
  },
  {
    datum: {
      "title": "RedeemData",
      "anyOf": [{
        "title": "RedeemData",
        "dataType": "constructor",
        "index": 0,
        "fields": [
          {
            "title": "poolNft",
            "anyOf": [{
              "title": "Asset",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "bytes", "title": "policy" }, {
                "dataType": "bytes",
                "title": "name",
              }],
            }],
          },
          { "dataType": "bytes", "title": "redeemer" },
          {
            "dataType": "list",
            "items": {
              "title": "Asset",
              "anyOf": [{
                "title": "Asset",
                "dataType": "constructor",
                "index": 0,
                "fields": [{ "dataType": "bytes", "title": "policy" }, {
                  "dataType": "bytes",
                  "title": "name",
                }],
              }],
            },
            "title": "expectedAssets",
          },
          {
            "dataType": "list",
            "items": { "dataType": "integer" },
            "title": "minExpectedReceivedAssetsBalances",
          },
          { "dataType": "integer", "title": "minExpectedLpChange" },
        ],
      }],
    },
  },
  {
    action: {
      "title": "OrderAction",
      "description": "Order action types.",
      "anyOf": [{
        "title": "ApplyOrder",
        "dataType": "constructor",
        "index": 0,
        "fields": [{ "dataType": "integer", "title": "redeemerInIx" }, {
          "dataType": "integer",
          "title": "redeemerOutIx",
        }, { "dataType": "integer", "title": "poolInIx" }],
      }, {
        "title": "CancelOrder",
        "dataType": "constructor",
        "index": 1,
        "fields": [],
      }],
    },
  },
) as unknown as IRedeemRedeem;

export interface IRedeemUniformRedeemUniform {
  new (): Validator;
  datum: {
    poolNft: { policy: string; name: string };
    redeemer: string;
    minExpectedReceivedAssetsBalances: Array<bigint>;
  };
  action: {
    ApplyOrder: {
      redeemerInIx: bigint;
      redeemerOutIx: bigint;
      poolInIx: bigint;
    };
  } | "CancelOrder";
}

export const RedeemUniformRedeemUniform = Object.assign(
  function () {
    return {
      type: "PlutusV2",
      script:
        "59056601000032323232323232232323232232322533300a323232533300d300b300e375400e26464646464646464646464a666030602c60326ea80044c8c8c8c8c8c8c94ccc07cc074c080dd5000899191919299981199b8748010c090dd50008992999812181118129baa001132323232323232323232323232323232323232323232323232325333041304400213232323232323253330453043304637540022646464646464a666096608866600605405e05c2a6660960042002294052819192999826182518269baa00113371e6eb8c0b4c138dd50259bae3051304e3754002266ebcc008c138dd5181698271baa0340053001304d37546002609a6ea80188c140004c8cc004004c8ccc004004c8c8cc004004090894ccc14400452f5c02660a46ea0c8ccc01c010dd7182a0009bae305430550013050375460a60026600400460a80026eacc140c1440140c48894ccc14000852f5c026464a6660a4006297ae013305333304f337126eb4c15000c00930103d87a80004c0103d879800033300500500130550033054003375a60a400444a66609c00229444c94ccc1314ccc130cdc424000609a6ea8c14400852889998262514a09444cc00c00c0045281828800911192999826182298269baa0011480004dd6982898271baa00132533304c3045304d37540022980103d87a800013233001001375660a4609e6ea8008894ccc144004530103d87a8000132323253330513371e00e6eb8c14800c4c0f4cc154dd4000a5eb804cc014014008dd69829001182a801182980099198008008021129998280008a6103d87a8000132323253330503371e00e6eb8c14400c4c0f0cc150dd3000a5eb804cc014014008dd59828801182a0011829000982600098241baa001304a304737540022c6605c06206a6607e01046eb4004cc0f802c8dd7000981f00a1981e00a91bad0013303b0162303d001303b01916375a608400260840046eb0c100004c100008dd7181f000981f0011bac303c001303c002375a607400260740046eb4c0e0004c0e0008dd6981b000981b00119299981998190008a999818181498188008a5115333030302e303100114a02c2c6ea8c0d0004c0d0008c0c8004c0c8008dd6181800098180011bac302e001302e002375a605800260580046054002604c6ea800458c0a0c094dd50008b181398140011bab3026001302630223754600260446ea8c094c088dd50011181298130008b198040060071bae30223023002375c6042002603a6ea8c080008dd6180f98101810000980d9baa018301d301a37540022c6600200a6eb4c070028c0040048894ccc06c008530103d87a800013232533301a3018003130063301e0024bd70099980280280099b8000348004c07c00cc074008dd2a40006eb0c060c064c064008dd6180b80098099baa007375a602a602c0046eb4c050004c050004c03cdd5003899198008008019129998090008a5013253330103371e6eb8c054008010528899801801800980a8009bae30113012300e37540166eb0c040c044c044c044c044c044c044c044c044c034dd5000980798061baa00114984d958c94ccc024c01c0044c8c8c8c8c8c94ccc048c05400852616375a602600260260046eb4c044004c044008dd6980780098059baa0031533300930020011533300c300b37540062930b0b18049baa002370e90012999802980198031baa0041323232323232533300e301100213232498cc0200088dd680098040028b1bac300f001300f002375c601a002601a0046016002600e6ea80105888c8cc00400400c894ccc02c00452613233003003300f0023003300d00125333004300230053754002264646464a666016601c0042930b1bae300c001300c002375c6014002600c6ea800458dc3a4000ae6955ceaab9e5573eae815d0aba21",
    };
  },
  {
    datum: {
      "title": "RedeemUniformData",
      "anyOf": [{
        "title": "RedeemUniformData",
        "dataType": "constructor",
        "index": 0,
        "fields": [
          {
            "title": "poolNft",
            "anyOf": [{
              "title": "Asset",
              "dataType": "constructor",
              "index": 0,
              "fields": [{ "dataType": "bytes", "title": "policy" }, {
                "dataType": "bytes",
                "title": "name",
              }],
            }],
          },
          { "dataType": "bytes", "title": "redeemer" },
          {
            "dataType": "list",
            "items": { "dataType": "integer" },
            "title": "minExpectedReceivedAssetsBalances",
          },
        ],
      }],
    },
  },
  {
    action: {
      "title": "OrderAction",
      "description": "Order action types.",
      "anyOf": [{
        "title": "ApplyOrder",
        "dataType": "constructor",
        "index": 0,
        "fields": [{ "dataType": "integer", "title": "redeemerInIx" }, {
          "dataType": "integer",
          "title": "redeemerOutIx",
        }, { "dataType": "integer", "title": "poolInIx" }],
      }, {
        "title": "CancelOrder",
        "dataType": "constructor",
        "index": 1,
        "fields": [],
      }],
    },
  },
) as unknown as IRedeemUniformRedeemUniform;
