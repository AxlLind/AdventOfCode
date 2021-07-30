import Aoc
import Data.Function

input = "{{{{{<>},{{<u!>},<!!!!!!!><!!u!!\"!>!!!!!>!>},<,!!o>},{}},{<!>},<{!!a!a!!!!,!{i!!!>>,{<!!!!i!!!>>}}},{{{{{<<{!>,<!>,<o!>},<!!<!,i!!<,!>,<!>}e>}}},{{},<!!}',!>},<!>},<e,!>,<!!!><>},{}},{{{{{<}i>,{{}}},{<!>},<oe!>},<<,!!a'}{\"!!!!!>\"'u!>,<!!!>'>,{{<!!u!!!!!>!>!!!>,}'u,!{!>,<}!>},<}u>}}}},{<{!\"}!>!!u!>},<!>},<eoi!!}o\"\"i!!u}e>},{<\"{>,{{<>}}}},{{<{!,>},{<e!'\"!,u}>,{<>}},{{},{{<!>,<!<e!>i!!!>o!!!>>}}}},{{},{{{},{<!>!>u<\"'o\"'!}!>,<!>,<\"ea<i{!!a>,{<a!!!!!>!>,<a!!!>\"ei!!<iu!!!!,>}}},{<a!oe!!!>eauu'>}}},{{{<!u!>ou!>,<eeau!\"eae\"'!!!>u>,{<!>!!{>}},{<!!!>}}}!!!>}>,<\"!!'<u!!<>}},{{},{}}}},{},{{{<>},<!>}<!i>},{{<!>,<!!!!!<}!au!>!!!!}i\"!>e}!{!>},<!!}!>e>},{{},{}}},{}}},{{{{{}},{<!,}!!!!}a!!!>u>}},{{},{{<!!u!>},<!u{,ea!!\"a'{!!ueoo>}}},{{{<{o!!!>},<!!!>},<ea<\"!>!>},<!{e!!>,{}},{{<oeo!<!!!>!a!!!>e\"o}!!!>,<oi<,!>>}}},{{{<'!>!\"!!!>'a<}i\"!>!!>}}},{<!!!!}!!!io!io!!!>'>,{{<u}!!!!{<}!!!u!!!>!!,<>}}}}},{{<}!>,<}u!>,<,!!!>{'!!!>!!>,<o!!!>!>,<!>},<}!>,<!!ea!>},<{ia}!!a<!}>},{<<>,<>},{{{<<!!{u!!u!!!><!>},<}'!!'<i!oe\"!>>}},{{<'!>},<{!!i,u{!>},<}'!>!!!,}>},{<u!!!>\"e!>,!>,<!!e!!o\"!>i!>,<'>}},{{<!!>,<<!>},<!\"{\"<\"!>,<u!!!>!>!o<a>}}}},{{{<,!!!!!>!>,<!ie!!!>\"\"!>!{'o!!\"'!!!>'>,{<}!!!>},<!>},<a!>,<!>},<{!!<!!ue{!!!<a!,>}}}},{{<oaao>,{<aoa!!!>!!,<>}},{{<!!a!!!!!!!!!!<uu,<!>,,'!i!>},<u!>,<>,<!!!>},<iii!!>},{<ei>,{}},{{{<!!!>,<i!!!<{!!!!!>!>,<'u!!i!}!>},<>,<!>,<!!a!!!>!!!>i!>}\"!>,<!>},<<o!!},o}a>},{<a'!>,<!>!>!!!\"!>,<!!!>,'!>,<!!!>\"!>!!!\"e>}}}},{<!!}u{i!!!!!>,u\"<'!>!!!>,<e,!!!>'>,{{<!!!!!>a!!!>\"!<>}}}}}},{{{<>}}},{{{{<!!'!!!>!>},<!!!>},<!!a{u!!!!!,au\"\"!!!>>,<!>,<io!>{!>,<'!o>}},{{{{<<!>},<!!!>a!!u>}},{{{{},{<!!<<',{'e!!!>e>,{<!!!!<<!>,<}!!!!\"oo>}}},{{<{!>,<'!!!>,uo!!!!}!!ia!!!!!!!>',o{>}}},{{}}},{{<,'!!!ie!!!!\"ea!>},<\"a!>{<!!a!!!>},<>},{<o{,}!!!>>}}},{{{<u!>},<!>},<!!!!<>,{{}}},{{},{}},{{{},{<i!!!>!!i!><'\"!!ia!a>}},{<!!>,{<{!!eue{>}}}},{},{{<!{u!!!>!i<aoio,<u>},<!!!!<!!!>'!!o!!'a>}},{{{{{<!!<,iu!!!>}i!>,<!!o!!',!>,<u>},{{{<ia!}!>},<!>,<,'}e\"!>!>i!a<a>}}}},{}},{{{<!>,<!>},<'}!!!>e!!!!!>eo>}},<!!e>}}}},{{{{<o'>,{{<!!au!>,<e\"!!,!>,<!>!>!>u<}!\"!!!>!>>}}},{{<!!<{!!!!aooo}>},<,i{!!!}{!!!>},<u\"!!!!oiu!>},<!>!!!>>}},{<{\"!!,,{o!{>,{<!!!>,u'!!o!>},<>}},{}},{{{<!!!!i,}i!!!>'!>e,!!a{'!>!>!!!>!>},<>,{{<!i}o!!!>'<u!i,e!!<>}}},{<!{!>,<!!!a'>,<<!!i\"a!>},<e!!!!!!,\"}!>},<!>,<!>},<<}>}},{{<!!!>!>,<!>},<{!>},<!,!!<{},\"{!!e!!<i>}},{{<i\">,{<}>,<'<>}}}},{{{},<u!!!><aoa!>},<\">},{{<i!>},<\"a!!!!!{e!!>}}}},{{{{}},{{{<}!iu!!!!!u<u!o,<!>,<\"a!!!>ea}!>},<!>,<}>},<!!}}{'i}!!!>},<!>,!!!>ue>},{{}}}},{{{<!!!>,!>'!!e}!<!!!>!!!>!>,<!>i'\"!!!>{>},<u!>!!!'!!!>>}}}},{{{<\"'>}}},{{<ie!!{!!!>!!!>,<i{>,{<}!!!>!>,<',{>}},{{{{<u{!o!!!>},<!>},<,e}!!\"!!!>!>},<uu>},<a}e!!!>!!!!,!>!>},<{!!!>},<!>},<!>e!!o>},<o!!!{}oeui}o\"!!!>!!oe!!!>!!!>,<>}},{{},{{<u!!!>!!uiiae'!!'!>},<!>>},{<!!u!>,<i!>,<!!uaau}\"{<!>},<i!!u!>>}}}}}},{{{{{<}>}}},{}},{{{{{<{!uu'<!!'uo{e!>,<>}},<!>!>'}!>a<,!>},<ea!><!>,<,a>}},{{<a!>,<e!>},<!}'!!!>,<!!!!!>!!i!oa!!!!!>,!!!>,<!>,<!>>},{{{<,ee>}}}}},{{<ea!!iiu{o!!u\">},{}}}},{{{{{{<,u!!!>!!ai'<!o>},{}},{{{},<'!!a!,!>},<!!!>!!!>},<!aea>},{{<!>,<>,<!!!>},<!!!>}!!!!!>a!!!,!>{!!u!!e}>},<!>!'!!!>u'{',>},{{<u},,!>,<u>}}}},{{{{<'},{\"\"!>,<}\"!!!>'}>,{{<\"!,!!!!<!!!!!>!o,i!!!>>},<!!!>e!!o!!!>,<ui!!o{'<!>!!o\"''>}}},{}},{{<>},<}\"}!!!>e!>,<!!<!!!>}!>,<\"!!!>io>},{{{},<!'i!>},<!<!>},<!!!>!ao!>,<a>}}},{{{}}},{{},{<\"o!>},<!>\"uo!o}!>i!>,<{>,<u!>\">},{{},{{<\"!!a!>,<o!!!!ae!!!!<aee<'o!u!!!!{\"u>,{{<<!e'<!>,<!,!!!>{,!!!>>}}},<!>}u\"u'!>},<}!!!!!>!>,<!!!>>},{{}}}}},{{{<!>,<{{{<a!e!>uo\"!>,<!>e>},<>}},{{{}},{<a}!i{o!!o!>!>!i!>!!!>,<{!>!!ao>}},{{{<}}\"!>,<,!u!{u'!<>},{<e!>},<o\"i!'!a!}!!!>},<>}},{<!>,<{\"}!!!>e!!!>}ea!!!>'uia,{'o>,{}},{{{<'!!\"!!i}!{!!{,>}},{{}}}}}},{{{{{{<<e!<'}u<!!o>,<ouo>},{{<e!!\"!!\"a<<!>ii!>,<a!>},<,'u!!u>},{{<e>,<oui>},{<\"!>,<aui!!!!!>}!!!>!!}e!!i<iu<\">}}},{{{<{!>}u!!,!>},<o<ao,!!e\"a!!a!>,<!!o!>,<>},{{<!!!><a}{!!{e!!!>!!!>>}}},<a>}},{{<!!>},<}u}{i\"!!o{,<<!'i!!!>'a>},{}},{{{<>},{{{<i{!a{{}!>u!e<>},<i,'!!!>i!>!>},<>},<i!{a!>ai!>,<>}}},{{{{<io''u\"!!{\"!!!>e!}}!>\"!!!!!>!!'<!>,<>},{<<!!!!!e}}>}},{{{},{{},{<!>},<!'!>,<{'!!!!!!!>ioa'\">}}},<!!o!>},<a!>},<<!>,<}u!>},<,!>,<!!!>!<i!!!>''>}},{{{<>}},{{<'\"!!!>},<!!!!<'!>!,!!a\",!>!>,<!!!!!>,<!,'>},<\"!a<'!>,<u\"{!>,<\"e!!!>!>o>}},{{<!>},<<o{u\"!>}<i}a',\"!!u!>,<!!!>>,{}}}},{{<ueu!!!>'e>},{{},{{}},{{<o!>!!!!!>},<>},{<>}}}}},{{{{{},<}u!>,<\",!!u!>,<e!{>},<{{!>},<a!>,<!>,<{!>,<!>,<!>!>>}},{<!>},<e!!!>!>,<!!o>,{}}},{{{<\">,<\"!o,i,!,>},{<!!!>ie!>,<\"uu!!!>,<!!!>},<!!}'!!!>i>,<!,,!!a<!!>},{<!>!>},<!>!>},<!!!>{!{!>},<o!!}i!!<a>}},{{<!!!>!!!>,<'!>},<!>!>,<<!!!!o!>},<oo>},{{{}},<u{>},{{{<!>},<!a,!!!>u>},{<,{'!!!>,<!!!<u'!!{,!!!!!>>}}}},{{{<'!!'<!!!!!>,<{!>,<!>,<!!<!!!>!>},<>,{<,{!!!>!>},<>}},{}},{{{{{<!>,{!>,<,}!,}u{!>},<!!!>,<!{,>}},<>},<,{!!\"o!>,<!'!\"{,>},{{<{!o,<o!>},<,io!!i!!!!!>e{!!!>>},<>},{<i,'ee!!!ea!i!>},<>,{<ai}!>,<!!!!!>},<!!e!!!>''!{!a!!!}!>!!>}}}},{}}},{{{{<,!!!>\"ei\"'}o!>!!!>o{!!!>e>},{{}}}},{{{}},{{{{{}},<{{!>},<!>,<,!!ii!>,<!!!>},<>},<,}o\"!!>},{<'!!,!!!!!><!!}}{i\"!!!>u>},{{{},<\"\"\"{!!!>'\">},<i>}},{{{{{<!!!>ii>,{{<!!!oo\"a}!!!>u!\"uu!!!>!>}!\"!!!>!>>},<!>!>,<>}}},{{{},<>},{}}},{{{{{<'!!!>\"!!!!\"}!>,<{!!!><a!>>},<!>ea\"a!!!!oua<!>},<'e!>},<>},<\"u{o<e!>},<i!'!oa\"'!!!!!>!!{>}}},{{{<o'u!!}io!!!>u<e{\">}}}},{{{{{<''!!!>!>},<>},<!!!>ae>}},{{{{{{}}}}},{<u!!!><\"!!i,>,{{<!!!>!>},<{!>},<\"!!!>>},<,}!>,<{!eia}!>},<!!!>!>!>},<\"{a!!>}},{<}!!\"!!!>uu'!>,<>,{<i!>,<>}}},{{},{<>},{<!!a!!}},!,u!!!>!>,<>,<!!!>!!!>},<!>,<ue>}}},{{<o>,<!>\"!>,<a!\"e!!!!!!!><a!>,<\"o!!i,,>},{{<\"}\"{'oi<!!!uiue!!!>},<!>>}}},{{{{{<!!u>},{<!>,<!!!!u\"!>},<{e!>},<!>,<!a>}},{<u'!!i{!!!>>}},{{<!!i!!a!,i!!!>,<i!!!>,!!!>>,<'e\"!!a!!{!>!!,\"e!!!!!!!>,<',!!!>!!!'>},{{},<o!!!>!!ui!!!>,<,<ia>},{<}>,{{<{!>,<!}!>,<ai,!!<!>},<{!!!>!!a!!!>,!!!>>}}}},{{<!!i,!!!>!>{<>,{<!>,<i!<!>!>,<>}},{{},{{},<<!!<ai!,\"!!!>},<u!>a\"!!!>u>}},{<!>},<!!!>!!!>}e!>,<o}>,{{}}}}},{{{{{<!!!!!>!>},<!!!>}!!aa{!>,<oi!>>},<ee!o,!!a{,!>,<'i}!!!>!>,<>}}},{{{},<'<!>},<!{!i!>},<a!!!><<u<u\"!!!>!!!'e!!>}},{{{{<!<!>},<u{!>,<'!>,!><{!uu!!a<!!\"!!,,>},<!ae{ao\"!>,<!u!>uu!>,<>},{}},{{<!!o!>},<!!!!\"'!!!>a!!ei!{!!>},{{<\"!>!>},<!oa!>},<}>}},{{<}<eu!>,<!>},<>,{{{<!o!>},<!!!>!!,e!>},<{!>,<!!ue!>!!e<>,{<,{!!,i!aoi!>,<!!!>!>},<,e!!!!u!!!>'e!>>}}},<!\"\"'<<!!!!!>,e{!!!>!{!!!>}ou<e>}},{{<a!>},<!>e!!e!>!>,<,!!ua!!e!!!!!>},<a>}}}}},{{{<,!>,<!!,o{'aa'u>},<!>,<i,!>,<!!,i}<e<o}uu!>},<'!!!>u!>,<>},{{<oio!>a'!>},<,>},{}},{{<!i!!',!>,!>,<,!>},<o!>,<i\"{!>},<!!!'{o>},<!!!!!>},<ii>}}},{{{<i!!!>,},!!u'!>,<,!>,<!!u>}},{}},{{{{<!!!!!!!>!>o{!>\"!{!>,<o!>\"!!}o!!i!>,<u,!!!>>}},{{{<}{!a!>},<o{o!>},<i,a}!>},<!'e>}}},{{<!>,<a!!!{}!!!>!!!>!!!>!!u!}!o>},{<uiao!>,<ee!!{}!!>}}},{{{},{<!!o!!!>},<u!}{!!a!!!>},<!>},<u\"\"oo!,!>>}}}}},{{<!!!!!>!u!!!>!>},<o>,<oi<!{'!>>},{{<!>,<!!!>},<'!!!!!>ea!!!>u!>,<!>e!!!>!oai<>}},{<eu!!{,!\"!!<}}>,<'\"}!!!!!''iiu!!o'<,!!o'!!u!!!>>}}},{{{},{<o!>!}!!!>},<oe!>{e<!>,<!!}o,!>},<!!i>}},{<i>,<u\"i!>u!!!>!>},<!>{!ou>},{{<u,a!!!!e!>},<>},<}ao}>}}}},{{{{<!>\"!>,<\"o!>},<,}u>},{}},{<}{!>},<!!<o!!!>\"!'!>},<!>},<!!!>,<}{o!!<!>e>}},{{{{<>,<!>'!!!>},<!<,!>},<i!>,<!>},<!!<!>,<!>},<u!!\"!!aee{>}},{<!!!>>,{<!>\",'>}},{{},<!!,oa!!,\",>}}},{{<!>,<!>},<\"{}a!<!!!>{ea,'o!>,<!>},<o>},{{<!!!>o!>,<iu>},{<!!!>!>,<!!e!!!>!>},<{!!\"!!}!!},!><'<!,!!>}},{<!o{\"e>}}},{}},{{{{},{<!>},<!!o<u!!'!!!!!>!'}a>}}},{{{<\"!!{>},{<'!!!>!>,<!!!>!!!>!!,e!!\"'<!!u'!!<!,!>,o\">,<oe<!!!ie!>a,<!>,<!<>}},{{},{<{!!!!!>!!!!!>,!>{!,!>,<>,<!oe>},{{{{}},{},{{<>}}},{{}},{{{}},{{}},{{<iiaa>}}}}},{{{{{<!>},<i,!>oae!!!>!>,<o,\"'!!>}},{{},{<ieei!!u{!}>}}}},{},{{{{<!>,<'{{!>,<ou!!!>!!!!}!>,<!>e,i<!!'}>,<\"!!{}!>,<o!!e!>},<<a>}}},{{<i,!!uiu{a!!!>!>e!>,<{ao\">},{}},{{<ue!>},<!{>},{<o{!>},<u!>,<\"'u'!!!>u}!>,<>}}}},{{{<u!!i>,<!!u!!!>},<!,u!>},<!>},<!}!>,<!>o}>},{<!!>},{{},<ua!>'}a,!!!!!>},<o!,!!<,!!!>{u,!>>}},{{{{<!!!!<!>{!!!!!!o<u!,!>,<!}>},<!>!!!>!>o!>,<!!!>'!>,<>},{}},{<e!>!!!>!u,!!!>!>},<\">},{<}}!!!>'u!!!!!>}!!!u{\"!!o<<>,<>}},{{{},<>}},{{{<i}!!'!!,!!!>\"!>,<!>,<!>,<!ou}{u!!<>,<e>},{{<euoa,!<o{a!>,<!oe{!!!>!!'>,<!>,<>}},{<io!<a}!>},<u!!!!'<i!>\"!e!>!>,<u!>},<>}},{{<!>,<a!!!!!!!!ua!>,<oa,u!'e!>!>,<>,{}},<!!{!!{,>},{{<e!!!>ei{}!!'i!!{{{o!e!><>}}}}},{{{{<e'!!!>!>!!!!>},{{{{<!{o!>},<a!>,!>!>},<,!!{eu'}>}},<!>},<{o!>}!>,<\"!!!!!><!o{a!!e,!!}!>>},{<>,{<uauii!>},<}>}}}},{<!>a}u!>},<!>},<<\"o!>,<}!ue!>o<!,>,<!!!>!<>},{{<{,{!!{a!>},<'!!!!!>!!e!>!>},<!>,<>},{<iaeo<{,i!>,<!!!>'{!!!>!!!>!>!>,<}>}}},{{{<u}!>},<o}>}},{<!>,<!!!>!!,{!!!>!!a}!>,<'u!\"{{u>},{{{{<!!eu!<>}},{{{},{<\"ou!>},<'!!!>},<e!!\"!!!>!!!>\"!>},<>}}}}}},{{},<}!!!>o!!,<e\"!!,>}}}},{{{{},{<e!!eo{!>,<{>,<u!>,<{!!!\"e!!!!!!},!a\"!eii>},{<<u!!!>!>},<!!!>uu!\"'>}},{{{{<ou!>},<!!!{o!!}!!!>i'!>,<e<!!!>}i}>,{<!!i!!a>}},{{{<i!>a!a<<!!\">},{<!!'io>}},{<!>'!>},<!!!>!!{a{!>},<!\"'!!!!!>>}},{}}},{{{<u\"!!!>uo!>!>'iua!!!<<>,{{},{}}},{{<!,,!!!>},<>},{<!!!>!!'!!!>,<!>,<{!o,au!!ooa>}}},{},{{{{<{ia!!''!!i,>},<,!!!>!!!>!e!!!!u!!}\",e}aaa!!!>}'a>},{{}},{}},{}}},{{{<!\"i!!a!>!>},<a!>,<e!>>,{<!>},<}o!>,<!>},<!>},<!!u!>},<i}!>,<!!!>!!},>}},{{<e!u!!!>!>},<i!!!>},<!>,<i>,{<o{a\"a!!!>iaaa<}u!>}!!!!!!>}},<\"!>},<!>,<o!<}!>},<!'u{u!!!>!!!>!>iau\">},{{{}},{<,}{iaue,>,{<!i!!{!!!>}e!>,<\"!!!!!\"!>},<!}'<!!,>}}}},{{{<{{}e!!!>!!!>!!>,{<!!!>!!!>,<<{!{ioai!>,<!!i<',ie<>}},{{<!>},<<<,!!i<>},{<!!!!!>,<'u!>,<o>}}},{<}eo\"!!o'\">,{<,'!>!!!>i!<e!!a!!!>!!!>i!!!>!!!>},<!>>}},{{<!!!>eee!}i!>'\"e''!!oa!!!!}!u>}}},{{<o'!,{!!{!>!>,<!!!>{!!!>},<!!!>},<!!!>},<io!!o!>!o>},{{<!!!!!>!>,<!!!>!!a!!!>},<>}}}}},{{{{}}},{{<!>},<\"!!ie!>!>'!!eae{i<,i,!>},<>},<!>,<!>},<u<}\"!><>}}},{},{{{{<}>},{{{<o!>u!\"'au{!!e\"!!!>!!!>!!!><'>}}}},{{{},<!>},<>},{{}}},{{<{!e\"!!!>{>},{{{<{!>e>}}}}},{{{{{{<,,!>},<'i!>},<!>,<!>},<!>},<{e!>e!u,>},{<oi!>},<!!!>,<'!o,!,\"}i!!!>},<>}},{<!>},<u{!!!>!!<<!i!>},<e!>},<oi!>,<{{!!!!!o>}},{{<{!!<!!!><}aiu!!e!!{<!!>}}},{{},{<u!!o!!!>{o!!i},>,{{{<<{!>!>!!!!<!uu!>e!'o{!!!>!!'!o\"e>},{<!>},<<!!i<!!!>,<!>'a{i'!!!!>}}}}},{{<!>,<!'!!!>!!<,!>},<ii!!!o{{u!\">,<'oi\"!!!>\"o<!>,<i\"{!!!>,>},{<\"<u!a!!a!!ouo!>!,!>},<>}}},{{{{<u'!>,<,u!!<>}}},{<a!>!!<!>a!!!>},<!!!>!!!!!!!!o>}},{{{{<}i}\"a!!!>>},{}},{{{<ia!!!>!>},<i<!!!!!!!>o'ie>,<,a,!!!!!>!!!!{o!!,!!!>,{}!!!>,<i!!!>>}}},{{<,ue!!!>'<u},<u!!!>\"o!!\"!>,<!!<}>},{<!uo!>},<!!!!!!<!!i!!!>},<}!!!>!>,<a!!}i<u!!!>a!>},<>}}},{{<,o!!>},{{<'o!>},<!!!!!>!!,}>}},{{<ao!!!!{{!!a,!!!>,!!!!>,{}},{<aa>}}},{{{<!!a}ii}o>,<!>},<}!!,{}e,!o!!!>},<}ea{>}},{{{},{<ai'i}u!!<u!>,<i,!>,<!!!>!!oa{!!}'>}},{{}},{{},{{{{},{<!>,<!!i,{,<!>!>},<u'i!!!>>}},<<e!>!!'u<,ie!>!!!>{<<\">}}}},{{{{{<!>},<>}},{<!>,<!>,<}'<oa,!!!>{e!!u'''!>{\"}>,{<{o\"!!!>}!>},<,!}{!>!>!!a<\">}}},{}},{{<'!!}<>},{<!>,<{\">,<>}},{<!'}!>},<!>},<{{e!!!>e!>!!!!!>'!>,<'>,{{{<u!!}o\">,{<!!!!,!>},<!>,<'ei!>,<<ao!!!>,<!>,<<!>,<!>!>,<,>}},{<!!!!!>!>!,ui}ae!>o<,i>}},<!>,<'<e}!!{!>!!!!e!ii>}}}}},{{<'!!!>},<!!\"\",oe!!!,}!!>,<!!ue!!ae!>},<i>},{}}},{{{{{}},{}},{{{<a!!,!!!>'!!<!!!!u,e,!>,<!!!,i!>},<{>,<,!>,<!>,<}e!!!>!!!!o'}!>,<!{\"!!!>u>}},{{<a!!!>!>e!>},<eae!>,<}a'>},<{e\",!!i!>},<'}>}}},{{{{{{<u!>,<!!!>},<,ai>}}},{{<ai<\"iia}!!,!!!><>,{}}},{{{{}},{{<!>},<e!!!!<!>},<!!!>},<!e,>},{<!!!!'{!!{\"a'}{}!>}a!>!!i>}},{{<'\"o'!!uo!>},<\"<>,{<!>u!!!>!'!>!!<ouaa!>!>},<!>e!>},<{}}i>,<!!!>\">}},{{<<!>!!\"!!!>},<{!!!!}u>,<>},{{{{<i!>},<!!!>!!!>!!,!>\"e!!!>!>!!!>i',!!!!>,<!>,<o<!!!>},<!i<,!!}u{o!>},<!>,!>},<{!}!!!u>},{},{<<'e!>,<!>!!a\"{>}}}},{<!>,<u!!ea\"a>}}}}}},{{{}},{{{<e!!!>!!!>!>},<}!!{!>a,u!>,<o>}},{{},{{}}}},{{<!!\"i,>,<>},{{<{!>u!!<e{!>e!au!>!>,<!!!><e!!!>a!!u>,<}!>!>,<a,\"{e!!i}}>}}}},{{{<!>i!!!>!>},<>,<!!'!>,<}<'!>},<!!!>,o'!!e!!e>},{}}},{{{<\"!o!!,e\"!{u!>},<u!>},<'o<>},{<!!!!!>!!\"aa>}},{{{<\">,{<u'>}},{{<'}!!!>'!>},<<e!>!!\"!><ue!>,<e'>,{<!>,<i!>!!{!!u<!!},<<!>,<!!!>!!!>>}},{<!>},<ae<o',i>}},{{{<!>!>!!!>\",!!!>},<}}!!!>},<u>},<!!!>},<!>},<ea<!!!!!>,<!!!>!!!!\"!!!>uo!>i<}>}}}}}},{{{<!!}>,<o!>},<<>},{},{}},{{{},{{{},{{<!!!i!o!i'u!>,<!>,<!>u>},{{<\"}}!!!>,<!>},<i!>!!!!!>},<!!ei!>a!>,<!!!>\"<!>a!!>},{<!>>}}},{{<!>},<<u>,<i!>,<{,eo!!{,<>},{{<>},{{<''a!!!>\"'\"a!!!>}!>\">,<ou!>!\"'!!!>!!ei',!!!>>},{{<{!>,<\"!!'!!>}}},{<!>,<!!u,!!euia!!!>a!!!>},<!>,<ie!>,<e>,<{!!a!!,ie,!!\">}},{{<eu!>!!!>},<!>},<!,e!!!>!!!!!>!!{}a<,'!!!>!!!{>,<ao!{'e!!!>!e>}}}},{{{<ie!!a!!\">},{<{u,!>\"o}!>,<<!>\"ue!!a,!>!>,<>}}},{{{<\"''i'''!!{u<'!!\"u{!!a}>},<!>},<!u!>},<a'!!!!e!!!>},<!>},<\"!!!!!>u}o>},{<!!'!<ioi!!<!!'{>},{{}}},{{{{<!'>},{<!<aa<\">}}},{<!>'!!!>!!>,<e!>,<,'!!>},{<!i!}!!!>i,\">,{<{}!>,<e{}}!!!>{>}}}},{}},{{<!>},<>,<eu'!>\"}!>},<<'a!>,<>},{},{}}},{{{}},{{<!!<}}!!!>}!>,<!>},<i!>},<!>,<'!>},<>}},{<!!{a!!!i!>,<>,{{<<!,!>,<eo!>},<!!!>{a{>}}}}},{{{{<\">,<<a,!!!>},<!>,<\"!>a}!!!>uo!{!!!>!!u!!o!i>},{<!!ioi!>},<!!oe!!!\">}},{{{<!>},<!!o!!u}>},{{<'!},!>},<<'\"!>!<o!,o!>{!i<o>}}},{{<}}ei!!i,{<!!<!>u>}},{{<!>!!!!!>oi\"}'e>,<,!!{i>},{}}},{{},{{<>}},{{<i!>,<{!>au>},<!!!>!!a!!<ouo''>}},{{{{<!\"!!<!>'!>i>}},{<!!!>!}!!'!<>}}}},{{{<<!!!>,<!!,>},{{<a!!!,{>,{<>,{}}}}}},{{{{<'{o!>!!!{u<e<>,{<!>,<''a,!>,>}},{{}}},{{{{<!!!>\"!!!>,<!!!>!>,<u!!o!!i!>}!>i!o\"!!!!!!!>,<>}}}},{{{<a\"!!i!!!>!!e!>},<!!<>},<,!>,<!\"!>},<{e'>},{},{{<!!<o!!{{,!!!>u!iii!{!!<o>}}}},{{{{<!!'!{a!>>},{{},<{{,i<>}},{{<!>,<!!<!!o!>,<!u!!!>!>,<<{!>e}>},<e'!!''o!u>}},{{<>,{{<!!,}!>!!iu!!}'!>!!!!!>e!>,<!!u'!>,<\"\">}}},{}},{{{<{,e!!!e!!!!!>ei!>'<!!!>!>},<>,{<o!{!!i,!>!\"!!!>i!!!>o!>},<!!a!>,<<!!o!>'>}},{{{<!!!i<!!!!{<'!{!!o!!!>!\">,{}},{}},{<!>},<i>,<,\"i}\"<!>,<!!!>>},{{{<!\"a}>},{<'!!}!!!!!>}!!!!i<,!>,<eo>}}}},{{{<>,{}},{{},{<!!u<>,<!>},<a!!!!uu>},{}},{{{<!>!>,i!!!>},<i!<!>>},{}},{{<!>},<oeu\"!!!!!>}i{!>},<!!'!!ie'}>}}}},{{<}!'}u!>},<o!>},<\"\",!>,<a!!}!>,<!!a}u!!!>>,{{<!!!>,<i'i\"!!\"<!>,<!!a!!!>!>},<i\"}!u>},{<'!!a!\"a}o!!!>>}}},{<!,\">,{}},{}},{<o<!!!!a>,{}}}},{{}},{{<!!!>},<!!a!>o\"!!!!!!!>ai!!!>,<}\">},{<o!!{a!a\"<,!!!>!>},<!>},<!>,<!>i>}}}},{{{<'!!!!a<<!>,!!!>,<!!!>!u!!!><i!!u!>},<,>,<aea}!>},<e,,>},{<\"a,'!!\">},{{<u!!!!,i<{!!}}!<!>{!!<>},{{},{}}}},{{{<e!!!><\"<ui!!!>u\"!>},<!!<!!oe>},{<<!>},<ee<u{!>!!!!e<}'!!'!!\"\"!>'>,<!>{!!!>}}!!!!!>!!\"o\">}},{{<,!>,<!!!\"!,'i!<!!!!'\"!!!>!{\"!>!!!e'u!>>},{<,a<}\"}{!>!!u!>},<>}}},{{{}},{{<!e{oeu!!!>u>}}},{}},{{{{<!>,!!u'\"<<>,{<ie!!'!>},<!>,<ue\"!!\"!>},<!>},<>,{<ooeu\"}!!!>a!!uaua,!>},<!>iu>}}},{{},{{{{<!i>}},{}},{<!!!>!>,<\",a\"!>},<!>,<!!!!\"!>,<!!!>'>}},{{<a\"ae\"!u!!!>{!>!>!!!!!>\"!>,<>},{<o<>}}}},{{},<'!!!>o!<o!!!!!>},<,eao,>},{{<uau<!>},<!>!!a\"\"!!!!!!!>,<<!>},<!!!!!>>},{{<!!!>e!!!!!!!!!!!>>}}}},{{{{{},{{<!>u!!!>},<o!!!>,<!!!!ou',}'!!}!!\">},{<!\">}}},{{{}}}},{{{{},{<}!!!!i!!!>!>}!>},<!>!}>}}},{<!}!>,<!!\"}!>},<\"u\">,{<}eu\">,{<ai<!!i>}}}}},{{},{{{},<o}>},<}!>!!>},{<i'!!\"!>>}},{{<>,{<i\"ieo{'>,<<!>},<i!>,<!!!!!>},<''>}},{{{<!!\"<!>},<}!>},<ai!!!>a!!!>},<!!a!!!>i!!!>>}},{{{{}},{<!>i!!{au!>,<>}},{{<e!!!>,<i!>,<!>,<{,}>}}},{<!>,<i'},>,{{<!!!>,<\"{!!!>!!a>}}}},{<!!!>!'!\"\"!>,<!>o\"e>,{<\"!!>}}},{{{{<<!!<'!>},<u!!!>o!!!>!>},<\"!!!>>},{<o}o>}}},{{{}},{{<<!>,\"!!!!!>>}}}}},{{}},{{{<!>,<!!\"!!!>,!!!!}'a!!}!>io!!!>!>!>},<!!\"!!e>,<>},{{<},u>},{<!!!><!i',!{!u!>!!!>{,>}},{<o!,!>},<<>,<'!>!i!>''!!!o,>}},{{<a,!i\"!!ie!o{{!>{!!!>!>!!!>>}}}},{{{},{{<u!>!>},<'i!!!>i>,{<i{,,e<<!!\"!!!>o!>},<!>},<!!!!>,<!>,<!>,<{o!>},<u}!{!{{>}},{{<<\"e\">},{<!!!>\"a!!!>!!o!>,<,u!>},<!>,<o>}}}}}},{{{{{<!>!!u<!!!>!!!!!!!>,<a\">},{<!e!>,<!>!>,<i{,!>,'!!'!>},<'u'ia>}}}},{{{{<!!!>,i!>!>},<!!!>!!!>e,!!!\"!>>}}},{{<'!>aa',!!!>i\",a>},{<},>}}},{{{<{}e}!'\"a!!<'}<,!!!>!>e>,{{<>,{}}}},{<,!!!>!a!!}!>,!''!\"i'{>,{<!!,!>o!!aeoi>}},{{{{<a,!!}!>,<!i<u}ee{!!!!!>e!!\"}<ei>}},{{<!>,<<!!'{!!a!e}o,!!!>!>}e>,{}},{<a>}},{<!<o!a!>,<!>}i{e>}}}},{},{{{<!!!>,<i'<\"!!!><!>,<'!!{e!!!!!>,!!i{'!<>},{<u\"!!{>}},{<>,<!!aeu!!!!!>\"'}}>}}},{{},{{<io!>!>,<!>,<a>},<!!!>,<!>},<!>!>ae}!>,<!!!><,,!o>},{{<ee<uai!>>}}}}},{{{<u{!>,<'e!!}!>,!>e!>},<!!!!\"uu!}>,{<<!>},<'ai!!!>!>},<!!!!!>oe!>,<e!o!>},<\">}}},{{<!>{'a!!!><!>},>},{{},{{<oi'!>,<a!>},<!!!>,<!'<i!!!!<{}!>},<!>},<ie'>},<o'!!!>!\"'!>,<e\"!u!>,<a\"!!!>!>>}},{<<>,<!!!>,i}>}},{{{},{{<i'e<!>},<!!!>o}>},{{}}}},{<ia}oa!>,<!>},<'!'}!!!>>,<<\",ua!!}uu{!>!!!>!!!>!>},<>},{{{{<!!!>!!!>a{'!!}i<'a<>},<!!!>},<!>,<!>,<!>,<!!\",<\"e{!>,<!>,<a>},{<!!!>!>,<{!>},<}>}},{{{{<}!!u<!<<!!!>>},{<!>},<!!!>!'!>},<!>!!e!\"{!!!i>}},<!>!!!>!>},<!>,<o!!ei>},{{<!!!!!>},<\"i!!,!!!>a'!!a!!a<\"{>},{<a,<}\"!>,'}u>}},{}}}},{{{}},{}}}},{{{{<!!<!!e!e}!>a!!!>!}>}}}},{}},{},{{{{{{<{'>},{<\"{}!>},<!!!>{,!>,<!>},<!!}e!\"!!!>>}},{<a>,<!>},<}i!>,<!,,!>,<<!!!!a!!!!!>>}},{{{<o!>,<u''u!!!><<!!!>!>,<!!i>},{}},{<!!,!>,<u,o!>{>,<i!!!>>}},{<!>,<\"e!>!!!>},<e<>}},{<e'!>},<!!!>ii>,<'<!>,<!>},<,o}!\"!>!>,<<{!>,<}!!!>>},{}},{{{<,!'!>\">,<!!<'!>},<!!!>!!!>!\"!!!!!>}u,!{!!!>}>}},{<<'!>},<!!{>,{{<u'!>!}ui!!!ee!>},i!!!>}!!!>!!!>'{>},{}}}},{{{{{{<u>}},{}},{{<'{{<!!!!u{!><,!a!>>,{}},{{<i>}}},{{}}},{{{},<!>!>,<!{,a!!<>},{{<e!!!>},<<u!!!!!>},<ui>},<!>},<>},{{<!!!!!>},<!!!>}ou!!!>,<,\"a'!>},<!>\"!>,<a!!!>}>}}}},{{{{<>},{{<i!!!>,<!>,<\"!!!>!!{!e!>},<\"e!!{!>,<!>},<!!!>!!!>}>},{{},{<}>}}}},{{},{{},{}},{{<!!!>!!!>!!!>!'i<!!}!!!>,<!>},<>,{<}ua,i\"ai>}},{{<!>},<!>,!!!>,<,>},{{},<<!>,<>},{<!>,<>}}}}},{{{<!>i!>,<!!!>},<e!>!>{!>!!!>o!>}}e!>i>,{<>}},{<!>!>>},{<!!}'!!a!>},<i!}ee!'>}},{{{<!!!!!>!\"u!!}u,}i>,<ii{}o<!>\"!!!><!>},<!!u'uua'>},{<!!!>!!!>o\"!>o!!\"!!e{}<}!>},<>,<!>},<,{!><a>},{{<!>\"o!>,<i!>,<a>,{}},{<!o!!!}!!!>{>,{<u<,<!!i!!!>e!'}e!!!>},<!!!>},<!o,>}},{<ao}!!!>a!>!a}!<<u,!!!o>,{}}}}},{}},{{{{{<e!!\"!!!>i!><!>},<i!>},<i!>>}},{<!!}!,!>{''!!!>!>},<{,!\"e,!!<!>,<!{!!!>>,{<\"i,!!!>},<a,'io!>},<!,!!!>'!!!>,!!}!!!>,>}},{{<!!!>!>'\"{!!!>,,<!><}}\"u!!!!!>!!>,<\"!!e'!>o!>{e!!!>,<!>},<{>}}},{},{}},{{{{<'!!,{!!!>},<!!,ai!!e\">},{<i,\"i<>}},{<{,oi!!!>o}}!>>}},{{<u!!!>!>,<>}},{{{{<u'!>!>},<a!!<!!!><!!!>e!!!!!!!!\"u!e>,{{<!>,<a<oie!>,<u{u<iiuu!>,<!!o>},<o!{uu>}}}}}},{{},{{<!>,<o!!u!!e!!!>,<>},{}}},{{{{},<!>,<!u<o!>,<o'!>>}},{{<!>>},{<\"!e!>},<!>,<!!!>i!!!\"'!,!>},<i>}}}},{{{<!!!>!!ui!!''!>},<!!!>!{!!!>o{>},{{<!>},<!!!>!!!>e!>!!,{>}}},{<i!>},<a!!a!>},<!>{!!!!e'{>,<!>\"!>u\"<a}!u!!!!!>},!!}u!>,<ui}>}}},{{{<{a\"u!>!\"io!!!>e!>},<}u!>},<}!>},<e>},{<'!!\"}!oo!{!!!>!!!>!!{!o!>},<e{>}},{{{},<\"u!>!!>},{{<!!<!!!>!!!>e!><!>},<!>},<{!!u{!!!>!o!>},<>,<>}}}},{{{{}}},{{<{!!!>!>},<!>!>ii!>!>,<{\"}\"!!>,{<!>,<u!!,!>},<\"\"u{!>!>},<!>},<{!>,<<!>,<!!{>}},<a',\",!>,<!>!!e,>}}},{{{{},{}}},{{{{{<!>,<!!!>eu!!o\"}o!,u!>,<{!,{'o'e!!{>}}}},{{{<!!!!i\"{!>>},<{<}e!>>},{{<!>,<,}}!!{{>},<}!!i'!>},<''i!!!>},!!!!ui!!!>,<,!!!>!>,<>},{{},{<i\"u\"a!!!>u!>},<!!!>>}}},{{},{{},{<i!!!>e!>,<a\"u\"!!!>!>,<{oi!!>}},{<o!>,<{!!!>!>,!>!>},<>}}},{{{<\"!!!>o!i!>!u!!<<!oo!>},<u!!o\"<!>!e>}},{<!!!>{!>,<!>},<u<{<!>,<<e!!i!>,>,{<,\"!!,!!,}{o}eo!>,<>}}}}}},{{{<i!>,<!!!>,!!!>'<!>u!ie!>!>},<!>,<\"!!>,{<}!!eu<<!a!>},<,}>}},{{<!>},<u}'!!a!<!!!!!!!!'\"e!>,<i!>,<'e'iei>}},{{<!>>},{<\"!!!>,{!!'!!!!!>e!>,<i,!!!>,!!{,>,<,}{i!>},<e!!{'{!>},<!!!>{>},{{<}!!!>!!'i!>,<!>},<!\">}}}},{{<''!>,<i,<!!>},{<!!!>,<\"!>,<}!>,<<\"!>,<!>!!!!!!'oo!!!!!>a>,{<ue>}},{{{<oou!!!>!>,<a,>}}}},{{}},{{{{{{},<!!!>!>},<!!!>},<<!>},<}<>},{<\">,{<!>iu,e\"u!!ao'>}}},{<o!o!>,<}!!u<a!!!!!>!!}>,<>}},{{{{<u}!>>}},{<<u!>!>},<!!!>,<!>\"!!!!!>},<<!>!!>}}}},{{{{{},<i{}a>},{{<io!!!,i!>},<!>!!!>},<{,!>},<!!'>}},{{<!>},<ia!>},<iuo!>},<u{e!!!>!!!>,<!!!>!>!>>}}},{{},<!!!!!>,e<!!}o>}},{<!!!!u}!!ia>,<!!u<\"},e,!>},<e!!'!!!!!>{!>,<!a{!!\",u'>},{{{},<>},{{<{\"}!!<!>}{,>,<>},{<oa!>i!>,,,ae!>},<!!!u!!ia!!!>e'>,<!>!>>},{<a>,{<!>ai!!!>!\"!>},<!>,<!\"\">}}},{{<\"!>o'u!>},<!{!!!>!!!!!>!>,<!>},<\">,{}},{<{,!!!>!!!!a!!!>!!!>i!>},<>}}}},{{{<>}},{{<!>>},<!!!>},<u!>},<ae!!iai!!!!!!i\"aia!i\"!>,<>}},{{<!>},<!>},<\"<{,!!!>,<!}{!>}>,<!>},<o!>},<a!!!>!!!>!\"!\"!>,<>},{{}}}}}},{{{{{<!'!>},<'u<'{!>i>}},{{<u}!!!}'!!!>!!!>,<>},<a>},{{<!auia\"!>},<'<<'!!!>\"}!!!!o}<o>},{{<!e!>},<!!!>,<!!}!>},<!!!!o!>,<!a!>a>},<!!!!!>!!aa!!,!!!>!>!!'!,,'!>},<!>},<{ea>}}},{{{{<!>!}}!>!!!!!>a\"o!>,<!>},<uo!!!>!>!!,o>},{{<'{',\"{!!!>e!>,<u\"!e!>!>},<>},{<!!\"ui!u!ao,!!e''i!>!>a!!'!o>}}},{}},{{<e>,<{>},{{{{<!>,<'a}!!'!!!e'<!!!>ue!>!>},<,!!!>{>}},<!!!'e\"{}i'o{!!,a'>},{<!>},<<a!!'{e>}},{{<>},<o!!a!!!!!>!!}e<'!!!>!!!>u,!}!>},<>}}},{{{<!>!!!><a!>a<}<>},{<'{>}},{},{{{<!>!!e!>'>}}}}},{{{<o!!!>{!!!>!!!>'>,<!!\"!!\"<e\"!>'i!!}a\"<a>},{{<!!!!!!}!}>,{}},{<o>}}}}}}"

countGarbage :: Bool -> String -> Int
countGarbage inGarbage s = case (s, inGarbage) of
  ('!':_:cs, True) -> countGarbage True cs
  ('>':cs,   True) -> countGarbage False cs
  ('<':cs,  False) -> countGarbage True cs
  (c:cs,     True) -> countGarbage True cs + 1
  (c:cs,    False) -> countGarbage False cs
  ([],          _) -> 0

removeGarbage :: Bool -> String -> String
removeGarbage inGarbage s = case (s, inGarbage) of
  ('!':_:cs, True) -> removeGarbage True cs
  ('>':cs,   True) -> removeGarbage False cs
  ('<':cs,  False) -> removeGarbage True cs
  (c:cs,     True) -> removeGarbage True cs
  (c:cs,    False) -> c:removeGarbage False cs
  ([],          _) -> []

scoreGroups :: Int -> Int -> String -> (Int,String)
scoreGroups score _ [] = (score, [])
scoreGroups score level (c:s) = case c of
  '{' ->
    let (newScore, rest) = scoreGroups (score + level) (level+1) s in
    scoreGroups newScore level rest
  '}' -> (score, s)
  _ -> scoreGroups score level s

solveParts :: Int -> (Int,Int)
solveParts _ = (part1, part2)
  where
    part1 = input & removeGarbage False & scoreGroups 0 1 & fst
    part2 = countGarbage False input

main = Aoc.timer solveParts
