pub const SPLASH_NAME: &str = "SPLASH";

pub const GT_NAME: u8 = 244;

pub const LQ_NAME: &str = "SPLASH/ADA LQ*";

pub const DEFAULT_AUTH_TOKEN_NAME: u8 = 164;

/// Length of one emission epoch in milliseconds.
pub const EPOCH_LEN: u64 = 604_800_000;

/// Length of the emission reduction period in epochs.
pub const EMISSION_REDUCTION_PERIOD_LEN: u32 = 13;

/// MAX supply of Gov token (== MAX supply of SPLASH/ADA LQ* == MAX supply of ADA lovelace).
pub const MAX_GT_SUPPLY: u64 = 45_000_000_000_000_000;

pub const TOTAL_EMISSION: u64 = 100_000_000_000_000;

pub const TOTAL_COMMUNITY_EMISSION: u64 = 32_000_000_000_000;

/// Epochly inflation rate in micro-SPLASH.
pub const RATE_INITIAL: u64 = 224_000_000_000;

pub const RATE_AFTER_FIRST_REDUCTION: u64 = 156_800_000_000;

/// Reduction rate applied after first reduction.
pub const TAIL_REDUCTION_RATE_NUM: u64 = 94_524;

pub const TAIL_REDUCTION_RATE_DEN: u64 = 100_000;

/// Maximum tolerable time inaccuracy. (12 hours)
pub const MAX_TIME_DRIFT_MILLIS: u64 = 43_200_000;

/// Max lock timespan in seconds. (1 year)
pub const MAX_LOCK_TIME_SECONDS: u64 = 31_536_000;

pub const MAX_LOCK_TIME_MILLIS: u64 = 31_536_000_000;

/// Max length of voting on proposal. (30 days)
pub const MAX_VOTING_TIME_MILLIS: u64 = 2_592_000_000;

/// Min length of voting on proposal. (7 days)
pub const MIN_VOTING_TIME_MILLIS: u64 = 604_800_000;

pub const MIN_PROPOSAL_OPTIONS: usize = 2;

pub const MILLIS_IN_SECOND: u64 = 1000;

/// Constant index of a proposal (GovProposal or WeightingPoll) output.
pub const PROPOSAL_OUT_INDEX: usize = 0;

/// Constant index of Voting Escrow output in gov action.
pub const VE_OUT_INDEX_NON_GOV: usize = 0;

/// Constant index of Voting Escrow output in non-gov actions.
pub const VE_OUT_INDEX_GOV: usize = 1;

/// Constant index of Smart Farm output.
pub const FARM_OUT_INDEX: usize = 1;

/// Constant index of the weighting poll.
pub const WP_OUT_IX: usize = 1;

pub const MINT_WEIGHTING_POWER_SCRIPT: &str = "5907bb0100003232323232323232322322322322232533300c3232323253330103370e900018078008991919191919299980b19b87480080084c8c8c8c8c8cc004004008894ccc07c00452889919299980f19b88002480004cc01001000452818118011bad302100130020013232533301a3370e90010008a5eb7bdb1804dd5980f980c001180c00099801802803980080091299980d8008a5eb804cc070c068c074004cc008008c0780044c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c94ccc0a4cdc3a400060500022646464a66605866e1d2004302b001132323232323253330323370e900018188008991919299981a99b8748010c0d00044c8c8c8c8c8c8c8c8c8c8c8c94ccc1054ccc1054ccc1054ccc1054ccc1054ccc104078407052808088a50100914a0200a2940400852808008a503253330413370e900018200008991919191919299982399b8748000c1180044c8c8c8c8c8c8c94ccc1394ccc138024400852808008a5032533304e3370e90021826800899191919b8f375c60ac60ae60ae60ae609e00408064a6660a266e1d200000113232323232323232533305c305f002132498c8cc004004020894ccc1780045261323300300330620023232533305d3370e9000000899191919299983218338010a4c2c6eb4c194004c194008dd71831800982d8010b182d80098300008b1bae305d001305d002375a60b600260b60046eb4c164004c164008dd6182b80098278010b1827800982a00098260008b1817982580519b87001011337020020046660206eacc0a0c1200080e4044ccc03c0140e0040c134004c11400458cc07c09d20003370e66601600207e01890011bab30223042001302130410013047001303f00116330190230253371200800266e0ccdc100099b83323253330403370e9000000899b81375a608a607c00403826eb4c114c0f8008c0f8004c088c0f004520d00f482033a2478ccc00cdd5980d981d980d981d80981aa4501f4003370e664600200244a66608200229000099b8048008cc008008c110004c8cc0040040a8894ccc10400452f5c0264666444646600200200644a66608e0022006264660926e9ccc124dd4803198249ba9375c608c002660926ea0dd69823800a5eb80cc00c00cc12c008c124004dd718200009bab3041001330030033045002304300148008ccc0040a00a8008888c8c8c94ccc100cdc3a40040022900009bad3045303e002303e00132533303f3370e90010008a60103d87a8000132323300100100222533304500114c103d87a800013232323253330463371e014004266e9520003304a375000297ae0133006006003375a608e0066eb8c114008c124008c11c004dd59822181e801181e800998140018011b9437666ea0088cdc39bad302630360020213009001303b0013033001163016303200130380013030001163300a01248008cdc41bad301e302e003019300100223253330303370e90000008991919191919191919191919299981f982100109924c64a66607a66e1d20000011323253330423045002149858dd69821800981d8060a99981e99b87480080044c8c94ccc108c11400852616375a608600260760182c60760162c6eb4c100004c100008dd6981f000981f0011bad303c001303c002375a607400260740046eb8c0e0004c0e0008c0d8004c0b800858c0b8004c0c8004c0a800458c034c0a4c024c0a4004c0bc004c09c00458cc00402c03cc0040048894ccc0b00085300103d87a800013232533302b3370e0069000099ba548000cc0bc0092f5c0266600a00a00266e0400d20023030003302e002323253330273370e900100089919299981499b8748008c0a8dd51803981398069813804099b880090011337120120026eb4c0b4004c0940085281812800980498119804981180219191919299981419b8748008c09c00c4c8c94ccc0a800c400458dd6981700098130018b19b8748008c0a0dd518160009816001181500098111804181100199191919299981399b8748008c09800c4c8c94ccc0a400c4c8c8c8c94ccc0b4cdc3a4004605800626464a66605e006266e24cdc0803800a41017132522c6eb4c0cc004c0ac00c58cdc3a4004605a6ea8c0c4004c0c4008c0bc004c09cc034c09c02058dd6981680098128018b19b8748008c09cdd518158009815801181480098109800981080111814181480098131813981398139813981398139813980f80a19b8001c337040129040487260021bac3001301d0122302430253025001375860026036020460440026eb4c080004c080008dd6980f000980f0011bad301c001301400d22323300100100322533301c00114c0103d87a8000132323232533301d3371e00e004266e95200033021374c00297ae01330060060033756603c0066eb8c070008c080008c078004c04c02cc8c8cc004004008894ccc06400452f5bded8c0264646464a66603466e3d220100002100313301e337606ea4008dd3000998030030019bab301b003375c6032004603a00460360026eacc004c0440188c060c064c064c064c064004dd7180b00098070008b180a000980a001180900098050010a4c26cac64a66601866e1d2000001132323232323253330153018002149858dd6980b000980b0011bad30140013014002375a602400260140062a66601866e1d20020011533300f300a00314985858c028008dd70009bae001375a0024600a6ea80048c00cdd5000ab9a5573aaae7955cfaba05742ae881";

pub const INFLATION_SCRIPT: &str = "5904e301000032323232323232323223223223223222232533300f3232323253330133370e90011809000899191919299980b99b8748000c0580044c8c8c8c8c8c8c8c8c94ccc080cdc3a4000603e00226464646464646464646464646464a66605c66e1d2000302d001132323232323253330343370e90021819800899191919299981c299981c299981c299981c299981c00a88080a50100c14a0200e2940401852808008a503371e6eb8c0f0c0f4c0f4c0f4c0d40080bcc94ccc0dccdc3a400000226464646464646464a666084608a0042649319198008008041129998220008a4c2646600600660900046464a66608666e1d2000001132323232533304a304d002149858dd6982580098258011bae30490013041002163041001304600116375c608600260860046eb4c104004c104008dd6981f800981f8011bac303d0013035002163035001303a001303200116301830310043370e66601c00405c91010653504c4153480000b3370e66601a0020566e50dd99ba802348008dd5980b9817000981a00098160008b19807808a4004a666058002294454ccc0b000c400852819b8700548000cdd79807981400499ba548010cc0b8dd419b8001c480092f5c066ebcc050c09c020c050c09c03ccdc380099299981499b8800400110041001323232533302b3370e00290000a4101010def081a2a66605666e1c005200214820202468240244c8cdc199b82482020246824024cc00d20f8c40b00133003483026830004cdc0800a400466e0c071201a3001001222533302b3371000290000a40002a66605666e1c005200014800854ccc0accdc399b8600148011200013330030033370400400466e0c005200413370400466600600666e08008008cdc199b81001480092004337020040026660046eacc034c0900140892210653504c4153480033300137566018604601604291010653504c41534800222323232533302a3370e90010008a400026eb4c0bcc0a0008c0a0004c94ccc0a4cdc3a4004002298103d87a8000132323300100100222533302f00114c103d87a800013232323253330303371e014004266e95200033034375000297ae0133006006003375a60620066eb8c0bc008c0cc008c0c4004dd598171813801181380099198008008021129998160008a6103d87a8000132323232533302d3371e010004266e95200033031374c00297ae01330060060033756605c0066eb8c0b0008c0c0008c0b8004c8c8c94ccc098cdc3a400400226464a66605066e1d200230293754601e604c6026604c00a266e200180044cdc48030009bad302c001302400214a06048002601e6044601e604400260506052605260526052605260526052604202666e00060cdc100a2410121c98008604c002603c0022c660020069000180080091129998118010a60103d87a80001323253330223370e0069000099ba548000cc0980092f5c0266600a00a00266e0400d2002302700330250023758600260340184604260446044002600260300044603e6040002603a002602a0022c64646600200200444a666038002298103d87a800013232533301b3375e600c603200400e266e9520003301f0024bd700998020020009810001180f0009bac300130140062301b0013019001301100116301700130170023015001300d00214984d958dd68019bad001375c0026eb8004dd7000918029baa001230033754002ae6955ceaab9e5573eae815d0aba201";

pub const WP_FACTORY_SCRIPT: &str = "590487010000323232323232323232232232222323232533300d3232323253330113370e90011808000899191919191919299980c19b8748000c05c0044c8c8c8c8c8c94ccc078cdc3a4000603a002264646464a66604466e1d20043021001132323232323253330285333028003100214a020022940c8c8c8c8c94ccc0b0cdc3a400000826464646464a666062a66606200820062940400452819b873232323253330343370e90010008a400026eb4c0e4c0c8008c0c8004c94ccc0cccdc3a4004002298103d87a8000132323300100100222533303900114c103d87a8000132323232533303a3371e016004266e9520003303e375000297ae0133006006003375a60760066eb8c0e4008c0f4008c0ec004dd5981c181880118188009980300081698031bab3007302e02248008dca1bb3375000666ebcc050c0b009cc050c0b002ccdc39bad301a302b00a001337006eb4c064c0a809520021323232533302f533302f003100214a020022940cc88c8cc00400400c894ccc0d40045280991919299981a99baf00600114a226600a00a004606a0046072004606e0026eacc0ccc0d0c0d0c0d0c0d0c0d0c0d0c0b0080cdd2a40006606466e95200233032375205097ae04bd7019b87375a603460560146eb4c068c0ac098cdd79ba6323232533302f3370e90010008a5eb7bdb1804dd5981a181680118168009980100081498011bab3003302a01e4c101a00022323300100100322533303200114c0103d87a800013232323253330333371e00e004266e95200033037374c00297ae0133006006003375660680066eb8c0c8008c0d8008c0d00048c8cc004004008894ccc0c000452f5bded8c0264646464a66606266e3d2201000021003133035337606ea4008dd3000998030030019bab3032003375c6060004606800460640024605e6060606060606060002604e002601a604a03e66ebcdd30051ba60063375e602460460146024604600c6034002605000260400022c6018603e0046eacc018c078004c090004c07000458ccc8c0040048894ccc08c008530103d87a80001323253330223370e0069000099ba548000cc0980092f5c0266600a00a00266e0400d200230270033025002007375a6014603602a6eacc008c068004c004c0640088c080c084004c078004c05800458c8cc004004010894ccc070004530103d87a800013232533301b3375e60106032004012266e9520003301f0024bd700998020020009810001180f0009bac300130140082301b301c301c00137586002602400c46032002602e002601e0022c602a002602a0046026002601600829309b2b19299980699b87480000044c8c8c8c94ccc050c05c0084c9263253330123370e90000008a99980a98080010a4c2c2a66602466e1d200200115333015301000214985858c04000458c054004c054008dd6980980098058028b18058021800802119299980619b87480000044c8c8c8c94ccc04cc0580084c926323300100100222533301500114984c8cc00c00cc064008dd7180b8008b1bac30140013014002375a602400260140042c60140026eb8004dd7000918029baa001230033754002ae6955ceaab9e5573eae815d0aba201";

pub const MINT_WP_AUTH_TOKEN_SCRIPT: &str = "590f3801000032323232323232323223223223223222533300d3370e90001806000899299980719191919299980919b8748000c0440044c8c8c8c8c8c8c94ccc064cdc3a40000062646464646464a66603e66e1d2000301e0011323232323232323253330273370e90021813000899191919191919191919191919299981a19b8748000c0cc0044c8c8c8c8c8c8c94ccc0eccdc3a40086074002264646464a66607ea66607ea66607ea66607e036201a2940402452808040a50100114a0a66607ca66607c66ebcc08cc0f0008dd3803099b87375a603a607800402429404cdc39bad3019303c00200514a064a66607c66e1d2000001132323232323232325333049304c002132498cc07401c8c94ccc120cdc3a4000002264646464a66609e60a40042930b1bad30500013050002375c609c002608c0042c608c0022c6eb8c128004c128008dd6982400098240011bad304600130460023758608800260780042c6078002608200260720022c602a607000a646464a66607866e1c0052000148202021bde103454ccc0f0cdc3800a4004290404048d0480489919b833370490404048d0480499801a41f188160026600690604d0600099b8100148008cdc1808240346002002444a66607866e20005200014800054ccc0f0cdc3800a4000290010a99981e19b873370c002900224000266600600666e08008008cdc1800a4008266e08008ccc00c00ccdc100100119b833370200290012400864646600200200444a66607c002297ae013303f3374a90001981f98200009981fa610100004bd701980100118208009bac301730360103375e6038606a00466e9520003303b3374a90011981d9ba90244bd701981da60103d87a80004bd7019baf374c660446eacc054c0d0005220100374ca66606c66e1d20024800052f5bded8c0264646600200297adef6c6022533303c00113303d337606ea4098dd3001a5eb7bdb1804c8c8c8c94ccc0f4cdd79980701500126103d8798000133041337606ea40a8dd30038028a99981e99b8f02a002133041337606ea40a8dd300380189982099bb037520046e98004cc01801800cdd5981f0019bae303c0023040002303e00133330064bd6f7b630004a400400e607400260640022c6602c6eb0c038c0c409120023375e6e98010dd30009999800a5eb7bdb18001120020022222323300100100522533303900113303a337606ea4014dd400225eb7bdb1804c8c8c8c94ccc0e8cdd79980380480126103d879800013303e337606ea4024dd40040028a99981d19b8f00900213303e337606ea4024dd400400189981f19bb037520046ea0004cc01801800cdd6981d8019bae3039002303d002303b00122533303133720004002298103d8798000153330313371e0040022980103d87a800014c103d87b80003301601901b37286eccdd400119b800223370466e0000520024820243930010cdc01bad3010302900348008c94ccc0accdc3a4000002264646464a666064606a004264931980300091bae001163758606600260660046eb4c0c4004c0a400c58c0a400888c8cc00400400c894ccc0c000452613233003003303400230033032001302d00130250011630013024300530240062302b302c302c0013370e646464a66604e66e1d20020011480004dd698161812801181280099299981319b87480080045300103d87a8000132323300100100222533302c00114c103d87a8000132323232533302d3371e91101a40000213374a9000198189ba80014bd700998030030019bad302e003375c60580046060004605c0026eacc0acc090008c090004cc0340040712002375660046042002600260400044604e6050002604a002603a0022c660026eb0c00cc07003c010c0040048894ccc088008530103d87a80001323253330213370e0069000099ba548000cc0940092f5c0266600a00a00266e0400d20023026003302400223021001375a603e002602e01c264646464646600200200444a66604400229444c8c94ccc084cdc4001240002660080080022940c098008dd69812000980100099801803004180080091299980f0008a5eb804cc07cc074c080004cc008008c08400488c8c94ccc070cdc3a4004002297adef6c60137566042603400460340026600600400244646600200200644a66603c0022980103d87a8000132323232533301f3371e00e004266e95200033023374c00297ae0133006006003375660400066eb8c078008c088008c080004c05402ccc004dd5980d180d980d980d980d980980324410022323300100100322533301b00114bd6f7b630099191919299980e19b8f0070021003133020337606ea4008dd3000998030030019bab301d003375c6036004603e004603a0026eb8c060004c04000458c058004c058008c050004c0300085261365632533300e3370e9000000899192999809980b0010a4c2c6eb4c050004c03000c54ccc038cdc3a40040022a66602260180062930b0b18060010991191919299980919191919299980b19b8748008c0540044c8c8c8c8c8c8c94ccc074cdc3a40006038002264646464646464646464646464a66605466e1d200400313232533302c002100114a0660046006605203066e000152080a0f6a7133371200a9001099299981599b8748008c0a80044c8c8c8c8c8c8c8c94ccc0cccdc3a40006064002264646464646464646464a66607a66e1d2004303c001132323232323232323232325333048533304853330485333048006100514a020082940400852808008a50323253330493370e90000010991919192999826a99982680208018a50100114a066e1cc8cc004004008894ccc144004520001337006eb4c0bcc130c14c004cc008008c15000404cccc00c02c0292819b894800003cc8c8c94ccc130cdc3a400400226464a66609c66e1d2002304f3754605e6098605e609800a266e200040a04cdc48008141bad3052001304a00214a0609400260646090605660900026042608e06c2a66609266e1d200200213232323232323253330503370e900018278008991919191919191919299982c99b8748000c1600044c8c8c8c8c8c94ccc17ccdc3a400060bc00226464646464a6660c8a6660c8a6660c8a6660c8a6660c802e202c2940404052808068a50100814a020022940cdc381419b8148000004cdc080080299981e9bab3042305f00105d4890653504c415348003065001305d001163302d04348008ccc0e40081652210653504c415348003370e6660700020ac01490011bab303c3059303c3059001305f0013057001163302703f00c3375e6e9c008dd38009982d19ba548000cc168dd48021982d1ba83370290001bad303730540054bd7025eb80ccc0340540512899b8700133702900000c99b83337046eb4c0e4c144120dd6981a182880100e1bae303a30500013056001304e001163301e00f0053370e02a9000198121812982581d0139bad30510013051002375a609e002608e0782c44464666002002008006444a6660a20042a6660a2002297ae016132323232323253330545333054533305400a14a2266e25200000213371e0026eb8c0f0c1480105280a99982a19982a19b8700248001282511330583374a90001982c1ba900133058375000497ae0333009009005003133300900900500316375c607660a200a66e04dd6981998280011bad303330500043057004305500330550033053002304603a533304653330463370e00290030a99982319b8848000030528899b88480000385280a511533304653330463370e0029004099b8848000030528099b884800003852819198008008089129998250008a400026466644466e00004c8cc00400400c894ccc140004520001323322337006600a00a60ac00890011bae304f001375a60a000260a40026eb8c124004dd59825000998018019827001182600099b8733302000f01801648008cdd781098159820807a99982119b87375a60466080008038266e1cdd6981418200020098a5037586052607e0066eb0c0a0c0f80d4c0c0004c10c004c0ec00458c088c0e8020cdc080080b19980b00281b24410653504c415348003370200200466602800600401466602602c0020126eb8c0ecc0f0c0f0c0f0c0d00acdd5980b1819800981c80098188008b1980080ba40006002002444a66606c0042980103d87a80001323253330353370e0069000099ba548000cc0e40092f5c0266600a00a00266e0400d2002303a0033038002375a602a605a0486eb8c0cc004c8c8c004c8c94ccc0c4cdc3a4004002297adef6c6013756606c605e004605e0026601801c006600200244a666064002297ae013303330303034001330020023035001375c606200260520022c60246050010446464a66605a66e1d200200113232533302f3370e900118181baa3010302d3017302d00613371000a002266e24014004dd6981980098158010a50302b00130133029301330290022302e302f302f302f302f302f302f302f001302601a375a600e60480366660020080429110653504c41534800222323232533302a3370e90010008a400026eb4c0bcc0a0008c0a0004c94ccc0a4cdc3a4004002298103d87a8000132323300100100222533302f00114c103d87a800013232323253330303371e014004266e95200033034375000297ae0133006006003375a60620066eb8c0bc008c0cc008c0c4004dd5981718138011813800998020018011119198008008019129998150008a60103d87a8000132323232533302b3371e00e004266e9520003302f374c00297ae0133006006003375660580066eb8c0a8008c0b8008c0b0004c028c080008dd59801180f8009800980f0011181298130009811800980d8008b19198008008021129998108008a60103d87a80001323253330203375e6010603c004012266e952000330240024bd70099802002000981280118118009bac30013019008230203021302100137586002602e00c4603c002603800260280022c603400260340046030002602000829309b2b19299980919b874800000454ccc054c04001452616153330123370e9001000899191919299980c980e0010a4c2c6eb4c068004c068008dd6980c00098080028a99980919b874801000454ccc054c04001452616163010004300100523253330113370e900000089919191919191919299980e180f80109924c646600200201044a66603c0022930991980180198110011919299980e99b87480000044c8c8c8c94ccc090c09c00852616375a604a002604a0046eb8c08c004c06c00858c06c004c08000458dd7180e800980e8011bad301b001301b002375a603200260320046eb0c05c004c03c00858c03c004c048c02c004dd68009bae001375c0026eb80048c014dd5000918019baa0015734aae7555cf2ab9f5740ae855d101";