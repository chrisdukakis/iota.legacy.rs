extern crate iota_kerl as kerl;
extern crate iota_trytes as trytes;

use trytes::*;
use kerl::*;

static TRYTES_HEX: [(&str, &str); 27] = [
    (
        "999999999999999999999999999999999999999999999999999999999999999999999999999999999",
        "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    ),
    (
        "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
        "15c9ac99c46c3b5affda26c63aea1da9fd179e5b5726068a615e109def1c10b6f9e476597ae374feae89f3f0fee0e4a1",
    ),
    (
        "BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB",
        "2b93593388d876b5ffb44d8c75d43b53fa2f3cb6ae4c0d14c2bc213bde38216df3c8ecb2f5c6e9fd5d13e7e1fdc1c942",
    ),
    (
        "CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC",
        "415d05cd4d44b210ff8e7452b0be58fdf746db120572139f241a31d9cd543224edad630c70aa5efc0b9ddbd2fca2ade3",
    ),
    (
        "DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD",
        "5726b26711b0ed6bff689b18eba876a7f45e796d5c981a2985784277bc7042dbe791d965eb8dd3faba27cfc3fb839284",
    ),
    (
        "EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEW",
        "b01c8721851dd107ab3571d3d2a5933b5ffebb5bc07492af45a718679343c2b66bba4e0d3d68a85a2ab0d6e1aec7645c",
    ),
    (
        "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFX",
        "c5e633bb498a0c62ab0f989a0d8fb0e55d1659b7179a9939a7052905825fd36d659ec466b84c1d58d93acad2ada848fd",
    ),
    (
        "GGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGY",
        "dbafe0550df647bdaae9bf604879ce8f5a2df8126ec09fc4086339a3717be4245f833ac0332f925787c4bec3ac892d9e",
    ),
    (
        "HHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHZ",
        "f1798ceed2628318aac3e6268363ec395745966dc5e6a64e69c14a416097f4db5967b119ae130756364eb2b4ab6a123f",
    ),
    (
        "IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII9",
        "0743398896cebe73aa9e0cecbe4e09e3545d34c91d0cacd8cb1f5adf4fb40592534c277328f67c54e4d8a6a5aa4af6e0",
    ),
    (
        "JJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJA",
        "1d0ce6225b3af9ceaa7833b2f938278d5174d3247432b3632c7d6b7d3ed016494d309dcca3d9f15393629a96a92bdb81",
    ),
    (
        "KKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKB",
        "32d692bc1fa73529aa525a79342245374e8c717fcb58b9ed8ddb7c1b2dec2700471514261ebd665241ec8e87a80cc022",
    ),
    (
        "LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLC",
        "48a03f55e4137084aa2c813f6f0c62e14ba40fdb227ec077ef398cb91d0837b740f98a7f99a0db50f0768278a6eda4c3",
    ),
    (
        "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMD",
        "5e69ebefa87fabdfaa06a805a9f6808b48bbae3679a4c70250979d570c24486e3ade00d91484504f9f007669a5ce8964",
    ),
    (
        "NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNW",
        "a19614105780542055f957fa56097f74b74451c9865b38fdaf6862a8f3dbb791c521ff26eb7bafb060ff89965a31769c",
    ),
    (
        "OOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOX",
        "b75fc0aa1bec8f7b55d37ec090f39d1eb45bf024dd813f8810c67346e2f7c848bf067580665f24af0f897d8759125b3d",
    ),
    (
        "PPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPY",
        "cd296d43e058cad655ada586cbddbac8b1738e8034a74612722483e4d213d8ffb8eaebd9e14299adbe13717857f33fde",
    ),
    (
        "QQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQZ",
        "e2f319dda4c506315587cc4d06c7d872ae8b2cdb8bcd4c9cd3829482c12fe9b6b2cf62335c260eac6c9d656956d4247f",
    ),
    (
        "RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR9",
        "f8bcc6776931418c5561f31341b1f61caba2cb36e2f3532734e0a520b04bfa6dacb3d88cd70983ab1b27595a55b50920",
    ),
    (
        "SSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSA",
        "0e8673112d9d7ce7553c19d97c9c13c6a8ba69923a1959b1963eb5be9f680b24a6984ee651ecf8a9c9b14d4b5495edc1",
    ),
    (
        "TTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTB",
        "24501faaf209b8425516409fb7863170a5d207ed913f603bf79cc65c8e841bdba07cc53fccd06da8783b413c5376d262",
    ),
    (
        "UUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUC",
        "3a19cc44b675f39d54f06765f2704f1aa2e9a648e86566c658fad6fa7da02c929a613b9947b3e2a726c5352d5257b703",
    ),
    (
        "VVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVD",
        "4fe378de7ae22ef854ca8e2c2d5a6cc4a00144a43f8b6d50ba58e7986cbc3d499445b1f2c29757a5d54f291e51389ba4",
    ),
    (
        "WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW",
        "a8d94d98ee4f1294009764e7145789580ba18692a367e5d67a87bd88438fbd24186e269a14722c0545d8303c047c6d7c",
    ),
    (
        "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
        "bea2fa32b2bb4def00718bad4f41a70208b924edfa8dec60dbe5ce2632abcddb12529cf38f55a103f462242d035d521d",
    ),
    (
        "YYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY",
        "d46ca6cc7727894a004bb2738a2bc4ac05d0c34951b3f2eb3d43dec421c7de920c37134d0a391602a2ec181e023e36be",
    ),
    (
        "ZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZ",
        "ea3653663b93c4a50025d939c515e25602e861a4a8d9f9759ea1ef6210e3ef49061b89a6851c8b0151760c0f011f1b5f",
    ),
];

#[test]
fn equal_cfb_bigint() {
    for &(trytes, hexstr) in TRYTES_HEX.iter() {
        let trits : Vec<Trit> = trytes.chars().flat_map(char_to_trits).cloned().collect();
        let mut bytes = [0 as u8; 48];
        let mut otrits = [0 as Trit; 243];

        trits_to_bytes(&trits, &mut bytes);
        bytes_to_trits(&mut bytes, &mut otrits);
        trits_to_bytes(&otrits, &mut bytes);

        assert_eq!(trits, otrits.to_vec());

        let chars : Vec<char>= hexstr.chars().collect();
        for (i, v) in chars.chunks(2).enumerate() {
            let s : String = v.iter().collect();
            assert_eq!(bytes[i], u8::from_str_radix(&s, 16).unwrap());
        }
    }
}
