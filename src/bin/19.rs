use std::collections::HashSet;
use itertools::Itertools;

static INPUT: &str = "--- scanner 0 ---\n-817,-765,856\n443,-709,-511\n-658,753,-745\n378,506,-625\n557,-593,616\n-622,-827,819\n-611,-838,856\n-433,650,563\n-586,-856,-622\n398,565,499\n229,541,474\n585,-710,-578\n-584,611,490\n-796,-861,-671\n528,-778,-656\n-448,738,509\n702,-600,648\n-635,590,-725\n368,455,500\n339,605,-490\n288,624,-682\n-687,-819,-750\n-646,726,-814\n-134,-69,143\n-4,-120,3\n632,-644,504\n\n--- scanner 1 ---\n-576,-655,-870\n83,71,-65\n455,510,-438\n-496,-588,-822\n-601,-396,364\n-752,-444,373\n601,-737,495\n125,-92,-181\n402,514,256\n505,551,-412\n-407,683,546\n-700,501,-622\n603,-857,372\n-717,-310,415\n-409,-628,-813\n545,-770,-395\n363,354,318\n-538,654,501\n395,344,337\n466,407,-474\n-784,586,-567\n634,-809,-409\n687,-686,-444\n606,-670,479\n-593,685,436\n-752,662,-653\n\n--- scanner 2 ---\n-351,-447,608\n-522,-602,-725\n652,-529,515\n839,-608,-778\n116,-62,-109\n606,504,639\n-288,499,737\n-219,487,710\n-283,528,-835\n-455,-744,-726\n612,637,-413\n646,-520,584\n-408,-537,490\n-543,-498,589\n-411,427,-872\n530,514,796\n805,-675,-742\n-308,476,574\n872,621,-400\n-362,637,-877\n496,468,745\n740,489,-416\n964,-658,-827\n-377,-687,-794\n471,-547,543\n\n--- scanner 3 ---\n-600,-676,-662\n690,-716,598\n317,-749,-574\n716,563,-791\n444,358,756\n523,-729,495\n47,106,1\n-456,-623,-710\n-634,789,-520\n-625,659,492\n-649,745,633\n-514,796,-415\n496,487,728\n340,-665,-745\n-548,-726,-667\n762,490,-879\n-64,40,-123\n-576,-661,725\n867,555,-818\n346,499,705\n559,-583,601\n305,-766,-626\n-584,685,482\n-652,-454,737\n-455,845,-563\n-612,-636,677\n\n--- scanner 4 ---\n555,657,454\n-681,394,490\n-492,305,-497\n-707,-820,611\n720,-949,377\n818,447,-591\n-631,473,537\n770,492,-727\n193,7,-89\n-246,-812,-593\n-466,321,-586\n696,-780,402\n637,-890,-779\n-659,-811,762\n-793,-769,727\n467,577,391\n755,-800,-722\n-366,-689,-591\n774,355,-587\n656,-839,-587\n-433,-705,-596\n806,-810,403\n417,625,515\n109,-153,6\n-671,504,536\n-457,253,-428\n\n--- scanner 5 ---\n492,549,-889\n676,-597,352\n-552,-915,660\n685,-637,439\n-521,633,-789\n-2,-104,3\n-431,515,-846\n-102,40,-105\n515,386,-909\n503,413,-978\n290,489,399\n733,-570,519\n-541,574,335\n441,528,326\n-511,684,336\n272,-416,-691\n305,401,345\n-501,-829,544\n307,-539,-744\n258,-455,-602\n-482,534,-654\n-510,-781,695\n-632,-488,-658\n-616,-555,-663\n-509,-368,-660\n-372,598,310\n\n--- scanner 6 ---\n514,-446,-382\n414,762,-372\n-307,640,368\n-763,741,-861\n-499,-612,393\n415,-376,-468\n-304,643,449\n-518,-643,425\n381,743,-590\n795,-343,698\n-592,-701,-409\n-824,855,-903\n838,-421,652\n-429,629,495\n430,-425,-453\n858,-439,598\n584,691,680\n448,619,744\n-608,-645,-578\n-416,-656,379\n52,-36,-100\n553,722,676\n-37,110,83\n-842,751,-763\n371,655,-417\n-529,-773,-524\n\n--- scanner 7 ---\n-687,452,941\n-356,789,-401\n811,-554,-481\n897,-543,-349\n594,-328,503\n624,844,676\n-580,503,809\n638,-375,558\n-25,138,155\n832,839,-318\n748,890,-467\n-609,402,922\n-430,859,-504\n802,-497,-426\n-521,-476,669\n-360,-391,667\n-604,-522,-809\n-505,-560,-728\n-381,795,-329\n98,-36,82\n590,706,799\n-740,-513,-735\n-434,-514,604\n608,919,748\n861,804,-541\n689,-335,673\n\n--- scanner 8 ---\n-396,443,652\n-446,501,582\n883,-477,-273\n893,965,-608\n823,-389,-359\n938,922,-715\n42,-10,-17\n-669,-725,483\n-701,-787,591\n-373,-699,-453\n-744,549,-479\n928,822,493\n-472,490,571\n688,-496,-328\n618,-742,726\n152,149,23\n-556,-654,-489\n684,-778,544\n-650,-739,456\n720,955,-696\n-392,-555,-479\n674,-732,650\n842,921,423\n-665,709,-497\n-676,599,-652\n857,906,472\n\n--- scanner 9 ---\n-74,11,-72\n-84,-133,86\n668,503,-564\n539,815,778\n522,656,693\n-667,720,-632\n453,-604,677\n677,497,-633\n-711,-836,795\n-493,463,605\n-581,710,-619\n-806,-655,798\n699,-692,-638\n-495,-692,-520\n750,-500,-657\n589,690,729\n510,-526,745\n650,322,-592\n-671,712,-682\n-488,524,641\n595,-671,676\n660,-557,-539\n-587,-688,-476\n-455,323,626\n-632,-552,-524\n-695,-710,809\n\n--- scanner 10 ---\n-414,391,538\n-875,-448,-650\n373,308,473\n-949,-584,-641\n654,-692,573\n-426,393,596\n529,-667,-687\n-446,273,587\n527,-828,-772\n-79,-99,-74\n640,-697,443\n5,43,50\n709,558,-396\n505,-743,531\n-625,-713,705\n-830,575,-716\n434,346,377\n-636,-849,854\n-536,-746,797\n445,383,382\n-830,494,-503\n-875,519,-578\n464,-754,-649\n-868,-683,-603\n748,531,-351\n801,499,-520\n\n--- scanner 11 ---\n421,-382,-811\n454,-471,-719\n-823,-412,652\n413,618,635\n-615,517,821\n475,808,-587\n-659,572,774\n-599,-532,-347\n583,-795,627\n500,683,673\n-918,-335,606\n484,-714,589\n-832,-467,595\n424,659,-545\n-710,687,814\n-641,664,-871\n-583,665,-782\n-535,632,-812\n719,-721,636\n318,-482,-868\n-461,-512,-436\n9,143,51\n470,794,-457\n-122,11,-24\n-384,-464,-361\n339,681,758\n\n--- scanner 12 ---\n677,-807,-475\n-514,-719,780\n-723,-601,-559\n840,468,-281\n-721,-664,-469\n682,477,-349\n-720,670,653\n100,12,-28\n541,-759,-520\n-448,443,-736\n-758,682,618\n-333,-691,762\n33,-147,98\n489,-626,862\n677,377,661\n-508,-650,789\n546,-744,772\n675,257,784\n-757,803,548\n-731,-651,-555\n-262,475,-694\n681,-645,811\n681,-763,-383\n672,444,677\n771,503,-333\n-426,499,-674\n\n--- scanner 13 ---\n-473,-566,-801\n520,720,876\n-523,489,329\n-725,-624,671\n-687,593,327\n830,-456,-396\n-423,-636,-859\n705,-676,642\n901,-477,-480\n-499,-555,-812\n-474,559,369\n-703,668,-703\n380,824,858\n693,712,-347\n157,-9,-102\n-771,495,-753\n911,731,-380\n422,710,797\n-764,-781,582\n759,-804,563\n-791,-790,654\n-692,513,-593\n815,-407,-381\n866,624,-337\n874,-732,626\n-4,-74,33\n\n--- scanner 14 ---\n-534,368,659\n-788,-761,-491\n-878,-818,-609\n-464,512,615\n25,-4,112\n367,-626,-472\n-723,-647,724\n-581,-714,725\n589,352,-779\n387,739,618\n567,-608,857\n-609,480,608\n678,-663,846\n-638,-792,793\n583,-482,843\n420,790,648\n-158,-135,147\n-768,363,-550\n669,415,-691\n-113,-39,-22\n-798,364,-440\n-827,-626,-590\n504,-766,-476\n300,748,687\n-937,338,-526\n637,476,-829\n459,-630,-428\n\n--- scanner 15 ---\n-601,-717,591\n471,744,-416\n633,-619,-476\n-856,501,412\n-6,160,45\n-762,-665,560\n628,-610,-709\n-753,490,376\n-712,540,350\n-146,23,18\n-580,-372,-608\n406,-672,715\n-609,-503,-563\n-919,430,-399\n620,-737,707\n-578,-558,-498\n511,-717,723\n471,610,828\n-677,-749,505\n-749,437,-318\n445,842,-377\n443,406,874\n655,-563,-506\n-909,398,-380\n497,717,-299\n384,443,802\n\n--- scanner 16 ---\n-801,655,-363\n482,-522,-398\n602,-354,555\n-594,-657,-859\n-511,864,386\n54,93,4\n705,-446,598\n-789,690,-401\n-866,-353,702\n600,817,-607\n655,-498,603\n-104,15,104\n-810,-532,745\n728,419,691\n532,-518,-390\n-922,585,-364\n531,826,-613\n572,799,-788\n545,-432,-400\n-584,818,474\n644,442,520\n-618,-637,-695\n-817,-412,850\n-576,969,447\n-562,-699,-673\n745,407,639\n\n--- scanner 17 ---\n733,-307,-527\n-583,868,-693\n667,797,-426\n70,174,-81\n-450,-610,-736\n610,-626,389\n816,881,-477\n-517,909,-589\n-346,864,714\n593,910,-512\n805,-332,-521\n816,705,594\n819,479,574\n-716,-406,591\n-641,-619,-639\n-568,-679,-796\n704,-660,367\n740,-636,400\n-398,830,724\n-554,879,-786\n-745,-392,488\n-508,808,760\n787,639,555\n630,-381,-457\n-699,-394,397\n-23,29,4\n\n--- scanner 18 ---\n-609,-497,-861\n-749,-667,561\n-719,-467,-860\n-524,334,-797\n-563,485,-870\n562,720,-844\n-436,349,-851\n-508,414,513\n535,777,-677\n399,582,254\n368,-800,319\n-698,-880,543\n476,536,349\n531,593,340\n-626,-805,527\n591,-765,290\n-74,79,4\n794,-457,-652\n-623,374,462\n517,-851,403\n-453,367,557\n880,-508,-536\n-633,-328,-886\n746,-466,-501\n602,729,-757\n75,-21,-160\n\n--- scanner 19 ---\n-817,-615,-560\n-807,587,607\n-639,-366,727\n549,-811,-382\n-663,635,554\n22,94,40\n610,495,-385\n849,-559,685\n-717,866,-533\n-855,-755,-581\n470,-784,-333\n-715,630,504\n686,-483,658\n-939,821,-534\n436,-724,-339\n-729,-368,664\n360,479,606\n491,471,758\n-118,21,-56\n-643,-358,677\n744,531,-309\n-814,950,-496\n-822,-554,-596\n723,423,-403\n797,-516,689\n367,603,722\n\n--- scanner 20 ---\n-707,534,-290\n-488,377,466\n-491,574,510\n859,-343,-465\n615,-893,839\n-664,-864,-409\n-454,-687,819\n-560,-651,673\n447,597,-654\n526,580,-481\n920,-448,-400\n-762,583,-427\n480,415,695\n597,557,-563\n466,474,537\n-786,524,-307\n34,-31,22\n-527,495,571\n-557,-520,806\n-687,-714,-397\n786,-870,755\n-715,-766,-268\n832,-494,-473\n667,-905,856\n482,423,421\n\n--- scanner 21 ---\n438,-512,-856\n562,-627,-808\n516,577,-789\n-549,-597,868\n43,5,13\n646,636,558\n-535,-644,-343\n-681,-729,884\n184,-95,123\n474,-636,449\n713,494,559\n-477,-560,-368\n-677,576,-484\n-614,567,542\n-730,555,-555\n602,-535,412\n502,-709,-825\n-421,-745,871\n-648,500,506\n-540,-518,-418\n553,-745,407\n567,594,-761\n568,579,490\n-773,550,457\n-738,684,-506\n559,500,-844\n\n--- scanner 22 ---\n139,7,36\n597,513,782\n846,-774,604\n729,-705,-721\n-554,-568,688\n-265,406,-834\n-274,-414,-697\n-521,706,469\n-576,717,580\n488,292,-471\n658,549,787\n-702,703,564\n-418,-616,643\n-409,-404,-797\n-7,-172,97\n105,-155,-83\n401,335,-584\n696,-736,-832\n428,286,-505\n653,-732,-884\n844,-711,694\n854,-724,608\n457,534,820\n-558,-612,780\n-388,520,-774\n-413,-528,-688\n-473,413,-807\n\n--- scanner 23 ---\n-470,462,-472\n397,-623,373\n874,565,-809\n-577,597,-441\n825,428,359\n807,-769,-685\n-558,402,-386\n-859,601,431\n-401,-666,-586\n702,-701,-788\n840,443,310\n-813,611,537\n-333,-687,-584\n-68,22,-17\n-428,-543,-525\n84,-16,-148\n460,-723,277\n-606,-747,663\n-823,667,613\n-610,-678,801\n671,-705,-757\n841,347,-828\n-581,-728,769\n741,357,380\n417,-677,355\n884,392,-721\n\n--- scanner 24 ---\n-583,-471,-740\n695,458,-641\n-464,566,-502\n363,-388,460\n437,-318,563\n682,528,583\n-601,-258,-689\n587,-674,-750\n732,500,-773\n616,-775,-863\n-854,-373,600\n388,-382,388\n-378,582,-453\n-741,-287,526\n-394,482,-547\n-617,-333,-617\n-825,-338,377\n47,144,1\n-583,694,406\n-423,713,374\n771,572,-697\n-649,733,377\n694,-725,-882\n-119,51,-90\n692,539,453\n716,506,489\n\n--- scanner 25 ---\n825,-589,-478\n-5,-111,11\n-756,-672,559\n-481,-363,-407\n-41,69,169\n752,-804,598\n-686,849,-763\n791,651,-479\n-463,-418,-541\n-622,-677,458\n642,-794,644\n654,647,-619\n-786,-662,525\n841,-615,-389\n-459,735,657\n-496,796,553\n489,432,562\n-786,859,-628\n-444,859,710\n-677,735,-624\n893,-580,-320\n673,-754,645\n569,344,499\n699,695,-483\n-463,-351,-577\n521,294,623\n\n--- scanner 26 ---\n-795,-443,-559\n-804,-231,881\n447,-796,-426\n787,552,772\n323,-746,-474\n-939,-241,781\n-692,879,-385\n-800,-355,772\n534,-231,539\n761,681,-643\n-740,586,891\n-655,-439,-696\n-948,579,918\n719,720,-763\n481,-355,554\n-109,74,78\n725,489,700\n386,-658,-394\n377,-294,591\n710,651,-676\n-637,890,-496\n-659,-395,-604\n-511,876,-480\n-825,693,858\n736,637,788\n\n--- scanner 27 ---\n649,-535,687\n-453,717,-503\n-656,483,676\n-709,-760,-314\n-416,655,-698\n30,-136,63\n550,-661,-354\n-96,6,-70\n-798,-651,348\n-773,-675,517\n615,-658,825\n-624,490,821\n645,-831,-347\n-768,534,751\n631,-586,900\n-782,-721,-465\n-416,728,-492\n601,-776,-394\n-772,-565,487\n625,541,-560\n758,567,725\n-763,-848,-329\n810,450,844\n796,555,858\n543,479,-582\n565,396,-625\n\n--- scanner 28 ---\n-730,770,-606\n-308,-494,617\n716,721,-503\n938,683,335\n-735,560,-557\n550,-782,-727\n-802,-262,-939\n-477,-478,561\n494,-587,528\n-693,669,-478\n852,710,-414\n135,108,-167\n887,729,301\n-662,-244,-835\n-423,-557,689\n480,-670,572\n879,698,-483\n-760,-323,-825\n-271,560,417\n596,-657,-713\n781,693,411\n469,-779,-724\n526,-568,559\n-258,576,677\n33,-21,-16\n-274,708,545\n\n--- scanner 29 ---\n-730,-682,812\n582,-864,-690\n436,772,-425\n644,-964,-631\n-719,588,-416\n486,-851,510\n-871,-700,686\n-766,-590,650\n-803,643,-501\n-487,-549,-382\n526,815,855\n-68,-180,-10\n705,-954,-635\n-515,388,673\n615,801,894\n-428,397,713\n668,766,771\n73,-15,108\n477,-750,560\n447,736,-322\n-490,-581,-412\n-595,340,740\n461,-900,431\n-771,557,-577\n-429,-484,-442\n486,798,-466\n\n--- scanner 30 ---\n-676,-823,-346\n-635,-849,-396\n-685,-639,809\n520,-325,-262\n-411,813,-492\n483,-484,816\n-524,-608,861\n-470,913,-554\n-719,498,577\n548,-371,-466\n-466,903,-615\n697,-492,881\n-600,-694,893\n407,740,-578\n-779,-787,-357\n569,774,-569\n88,28,-12\n588,-476,861\n520,692,592\n461,-397,-299\n-659,433,519\n-689,530,580\n572,819,705\n611,788,-568\n472,733,727\n-64,-55,137\n\n--- scanner 31 ---\n-723,642,402\n-674,-706,617\n600,459,590\n737,-568,299\n585,426,791\n-550,618,-599\n-561,-322,-516\n688,-618,424\n-603,474,-551\n-633,-679,792\n361,-481,-822\n-477,-299,-687\n327,-643,-921\n537,564,-830\n538,342,-785\n-500,566,-594\n-62,44,-15\n652,-551,458\n-460,-287,-635\n-660,552,404\n500,467,-914\n-605,-697,611\n-673,638,394\n271,-657,-843\n564,464,651\n\n--- scanner 32 ---\n677,-408,-808\n-395,369,546\n627,-359,-933\n622,653,-890\n583,859,407\n601,582,-837\n-623,-342,718\n610,-444,-774\n-607,-328,592\n-648,771,-940\n-718,-339,537\n541,-460,486\n-490,657,-922\n677,942,399\n-401,501,531\n-405,-532,-581\n-400,-496,-642\n683,-461,553\n600,887,356\n-367,-511,-441\n598,-392,408\n493,604,-898\n-432,436,684\n-18,89,12\n-580,632,-900";

fn rotate([x,y,z]: [i32;3], way: usize) -> [i32;3] {
  match way {
    0  => [ x,  y,  z],
    1  => [ y, -x,  z],
    2  => [-x, -y,  z],
    3  => [-y,  x,  z],
    4  => [ z,  y, -x],
    5  => [ y, -z, -x],
    6  => [-z, -y, -x],
    7  => [-y,  z, -x],
    8  => [ z, -x, -y],
    9  => [-x, -z, -y],
    10 => [-z,  x, -y],
    11 => [ x,  z, -y],
    12 => [ z, -y,  x],
    13 => [-y, -z,  x],
    14 => [-z,  y,  x],
    15 => [ y,  z,  x],
    16 => [ z,  x,  y],
    17 => [ x, -z,  y],
    18 => [-z, -x,  y],
    19 => [-x,  z,  y],
    20 => [-x,  y, -z],
    21 => [ y,  x, -z],
    22 => [ x, -y, -z],
    23 => [-y, -x, -z],
    _ => unreachable!()
  }
}

fn merge_scans(total_scan: &mut HashSet<[i32;3]>, b: &[[i32;3]], rot: usize) -> Option<[i32;3]> {
  let b = b.iter().map(|&v| rotate(v, rot)).collect::<Vec<_>>();
  let distances = total_scan.iter()
    .cartesian_product(&b)
    .map(|([x1,y1,z1], [x2,y2,z2])| [x1-x2, y1-y2, z1-z2]);
  for [dx,dy,dz] in distances {
    let translated = b.iter().map(|[x3,y3,z3]| [x3+dx, y3+dy, z3+dz]);
    if translated.clone().filter(|v| total_scan.contains(v)).count() >= 12 {
      total_scan.extend(translated);
      return Some([dx,dy,dz]);
    }
  }
  None
}

fn find_scan_merge(scans: &[Vec<[i32;3]>], total_scan: &mut HashSet<[i32;3]>) -> Option<([i32;3], usize)> {
  (0..scans.len()).find_map(|i| (0..24)
    .find_map(|rot| merge_scans(total_scan, &scans[i], rot))
    .map(|d| (d, i))
  )
}

aoc2021::main! {
  let mut scans = INPUT.split("\n\n")
    .map(|s| s.lines()
      .skip(1)
      .map(|l| {
        let (a,tmp) = l.split_once(',').unwrap();
        let (b,c) = tmp.split_once(',').unwrap();
        [a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap(), c.parse::<i32>().unwrap()]
      })
      .collect::<Vec<_>>()
    )
    .collect::<Vec<_>>();
  let mut total_scan = scans.pop()
    .unwrap()
    .into_iter()
    .collect::<HashSet<_>>();
  let mut dists = vec![[0; 3]];
  while let Some((d,i)) = find_scan_merge(&scans, &mut total_scan) {
    dists.push(d);
    scans.swap_remove(i);
  }
  let p1 = total_scan.len();
  let p2 = dists.iter()
    .tuple_combinations()
    .map(|([x1,y1,z1], [x2,y2,z2])| (x1-x2).abs() + (y1-y2).abs() + (z1-z2).abs())
    .max()
    .unwrap();
  (p1,p2)
}
