use super::*;

#[test]
fn simple() {
    // FIXTURE
    let st = vec!["three4fournineone"];
    // ACTION
    let aws = p2(st);

    // CHECK
    assert_eq!(31, aws.unwrap());
}

#[test]
fn bulk_13_30() {
    // FIXTURE
    let st = vec![
        "qfbp2tpgvsxm",
        "kvbhsseven2rkbllhrhvsevenfour2vf",
        "bqbtzbtt3",
        "ninek6",
        "3fivebvqhvhlvz",
        "two68jxfnqlzfkninenine8",
        "vhckhhhb2sixtftjtdjf2nine",
        "4jc",
        "3sevenxlxfnpk",
        "5sjfive",
        "c8bvlkdq283rqfftj",
        "8eight1threehhvmnlft",
        "4sixmfcronelzcpnfourlgdbkgt",
        "hntcjmppp9fmntwokx2",
        "24mkgr6mx9n4",
        "cksfkrdmdcmnbmkgk3two4",
        "twosixthreefivelq3nine",
        "njmrtr6fmldninevcbtxr42",
    ];

    // ACTION
    let x = p2(st).unwrap();
    // rows 1 to 12
    //let aws: u64 = vec![21, 38, 92, 37, 57, 22, 69, 67, 31, 66, 12, 22] .into_iter() .sum();

    let aws: u64 = vec![
        22, 72, 33, 96, 35, 28, 29, 44, 37, 55, 83, 83, 44, 92, 24, 34, 29, 62,
    ]
    .into_iter()
    .sum();
}

#[test]
fn bulk_31_64() {
    // FIXTURE
    let st = vec![
        "eightfrdrczdxrcrfxh6",
        "8sevenonefmfqhtx8lk",
        "lnoneight8twodg",
        "tszrcb7tvvhfive4",
        "nknphprtv67fivehqlfournine2jk",
        "9eightszgdhftggrktkzbsmnhtwonekh",
        "1onefzkgf2two417",
        "77fivepsmmpp",
        "bnine7vmttgmdskc",
        "6tnsixclbvkhmsp",
        "ggreightwo1mnsbzmxsdcdrhhg5one",
        "ninetwo6qtbklgb2",
        "four8fmfxmr",
        "threetxfrsevenvfdgrtsixtwofsq7",
        "hqxlseven53",
        "vxqq24eight6",
        "7psxpgxmkpzkzeight3",
        "9four7",
        "stwoone4eightwoj",
        "one38",
        "9cjlrgrgpfjeighteightdcpjhzbjqcdthree",
        "rqfone2",
        "dvkxhvkgxcfivescjcnqqnnine2tthreecptkgxmzjsbzdrhk",
        "5lhprmg8two7two3",
        "5lvtg5sevennine2zrrmghf4seven",
        "fourfive97sevensevensixcsffnlcbtjk",
        "onesixoneninesixnbkf5xpsgsmpvdsdsqdvpjjzp",
        "nineb321mgkpqjqjqh",
        "seven2cstttkvkmpssflkxjmbnvthreefour",
        "six1bmcjbhdgqm6kgzsxbgdqnjqf3four6",
        "zchkfjxsllkncz175",
        "vsveightwo8",
        "fourbtrnfdp2four5",
        "13dtf8769",
    ];

    // ACTION
    let x = p2(st).unwrap();
    let aws: u64 = vec![
        86, 88, 12, 74, 62, 91, 17, 75, 97, 66, 81, 92, 48, 37, 73, 26, 73, 97, 22, 18, 93, 12, 53,
        53, 57, 46, 15, 91, 74, 66, 1588, 45, 19,
    ]
    .into_iter()
    .sum();
}
#[test]
fn bulk_65_87() {
    // FIXTURE
    let st = vec![
        "6qfvvdqdrtstwothree4seven8vszpseven",          // 67
        "7rgzpxfbczk1fivetwo4two9nine",                 // 79
        "3sksmfxn",                                     // 33
        "16ninetwonqqmptq",                             // 12
        "9five4fckzssxsvpzbvlktjzcninethree",           // 93
        "3threel889bvbcgvl",                            // 39
        "threetzzjntxlsd2klrjfnbg87",                   // 37
        "3two3qrrdbkbk",                                // 33
        "29oneightt",                                   // 28
        "7mkpfchstjt7mshqht4fivesix",                   // 76
        "tvmqndvsix875nine8",                           // 68
        "vg7477ninecpnrvnine7",                         // 77
        "seveneightmknlqjlstx4zjjrpbhjhnnc8",           // 78
        "8qklthreetwo7tlmrfzvtwo7zvccc",                // 87
        "6one1dpstqpmfbhst",                            // 61
        "sixfourfourseven6rzdkfour",                    // 64
        "6eighttbltmntb8",                              // 68
        "seven72cqslvzpgj",                             // 72
        "jpvrsgfhtwo5nfc1cgxdrdrbfnseven",              // 27
        "four1ninezlhqxtsgfzsevenhspvmxrtztzgtmseven5", // 45
        "foursixzjvgjgsevensvkd5",                      // 45
        "seven121fivefgxhdfive",                        // 75
        "twoh8",                                        // 28
    ];

    // ACTION
    let x = p2(st).unwrap();
}
#[test]
fn bulk_88_123() {
    // FIXTURE
    let st = vec![
        "ghhtttxqgr95", // 95
        "zkpklcjbjlr2", // 22
        "nine2mtrdcbrbntrdqninevbkbfg35lnpx", // 95
        "nineninesix68pcvsdnns", // 98
        "rf16one7", // 17
        "seven276", // 76
        "sixseven1", // 61
        "qjxseven9six", // 76
        "7c9xvhgmpf41", // 71
        "dlzpvdpxseven9nine", // 79
        "mlmlqrpn4twofivesbmhdbcsixtwo", // 42
        "ld74txfzksr5qconenine", // 79
        "six7djcdrtk7chktrh88", // 68
        "2seven7jtrbhznt", // 27
        "1eighthnhchsevenfive", // 15
        "qdhscpeight8", //88
        "12gnxzzm", // 12
        "k7fourmhcfkggt8919np", // 79
        "8zljspvnmlx6four", // 84
        "36twofivethreeptbdrfqzbz7qbjm", // 37
        "bxkbjbzgone3ldhlnhxfcf7btkktspxrtqkxqfdmlqgqvgclb", // 17
        "sevenfourmdrxseven2z2fiveone", // 71
        "8bbprzqrqn", // 88
        "twofivethree95mpqclhfkzlsix", // 26
        "gpfjflrkc96tmsix6dxjnfive", // 95
        "qzeight6", // 86
        "five5zlthgqjntwotqmnnsd", // 52
        "eight6nbpvfour4kzkvbzrxcmkh", // 84
        "fdkrxdckqbpsklz64fourones8", // 68
        "5qpldkhltl", // 55
        "27xldkbqbnmrrbqkhmksvk", // 27
        "pjoneightknpcgkkv7cbknrhfdmtdm8", // 18
        "432xsixvvktflzxone", // 41
        "35fouronemzzszfqppllgchsjjnine", // 39
        "qgcnzcbvsxbtn6lthreenineone", // 61
        "two2fourkdgbfb", // 24
    ];

    // ACTION
    let x = p2(st).unwrap();
}

#[test]
fn spec() {
    // FIXTURE
    let x = p2(vec!["lnoneight8twodg"]);

    // ACTION
    let aws = 37;
}

#[test]
fn calibration() {
    // FIXTURE
    let st = vec![
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen",
    ];

    // ACTION
    let x = p2(st).unwrap();

    let aws: u64 = vec![29, 83, 13, 24, 42, 14, 76].into_iter().sum();

    // CHECK
    assert_eq!(x, aws);
}

#[test]
fn extreme_cases() {
    // FIXTURE

    println!("\nsmall changes");
    let st = vec![
        "four", // control case
        "fiur", "fivr", "siven", "fix",
    ];
    let x = p2(st).unwrap();
    println!("fused numbers: ");
    let st = vec![
        // fused
        "eighthree",
        "twone",
    ];
    let x = p2(st).unwrap();
}


#[test]
fn base() {
    // FIXTURE
    let st = vec![
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "twone",
        "eightwo",
        "nineight",
        "eighthree",
        "eeeight",
        "oooneoo",
        "ttwonee",
    ];

    let x = p2(st).unwrap();
}
