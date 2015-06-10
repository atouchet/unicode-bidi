// NOTE:
// The following code was generated by "tools/generate.py". do not edit directly

#![allow(missing_docs, non_upper_case_globals, non_snake_case)]

/// The version of [Unicode](http://www.unicode.org/)
/// that the `bidi_class` function is based on.
pub const UNICODE_VERSION: (u64, u64, u64) = (7, 0, 0);
pub use self::BidiClass::*;

    #[allow(non_camel_case_types)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum BidiClass {
        AL,
        AN,
        B,
        BN,
        CS,
        EN,
        ES,
        ET,
        FSI,
        L,
        LRE,
        LRI,
        LRO,
        NSM,
        ON,
        PDF,
        PDI,
        R,
        RLE,
        RLI,
        RLO,
        S,
        WS,
    }

    fn bsearch_range_value_table(c: char, r: &'static [(char, char, BidiClass)]) -> BidiClass {
        use ::std::cmp::Ordering::{Equal, Less, Greater};
        match r.binary_search_by(|&(lo, hi, _)| {
            if lo <= c && c <= hi { Equal }
            else if hi < c { Less }
            else { Greater }
        }) {
            Ok(idx) => {
                let (_, _, cat) = r[idx];
                cat
            }
            Err(_) => L // FIXME: Is there a better default behavior?
        }
    }

    /// Find the BidiClass of a single char.
    pub fn bidi_class(c: char) -> BidiClass {
        bsearch_range_value_table(c, bidi_class_table)
    }

    const bidi_class_table: &'static [(char, char, BidiClass)] = &[
        ('\u{0}', '\u{8}', BN), ('\u{9}', '\u{9}', S), ('\u{a}', '\u{a}', B), ('\u{b}', '\u{b}', S),
        ('\u{c}', '\u{c}', WS), ('\u{d}', '\u{d}', B), ('\u{e}', '\u{1b}', BN), ('\u{1c}', '\u{1e}',
        B), ('\u{1f}', '\u{1f}', S), ('\u{20}', '\u{20}', WS), ('\u{21}', '\u{22}', ON), ('\u{23}',
        '\u{25}', ET), ('\u{26}', '\u{2a}', ON), ('\u{2b}', '\u{2b}', ES), ('\u{2c}', '\u{2c}', CS),
        ('\u{2d}', '\u{2d}', ES), ('\u{2e}', '\u{2f}', CS), ('\u{30}', '\u{39}', EN), ('\u{3a}',
        '\u{3a}', CS), ('\u{3b}', '\u{40}', ON), ('\u{41}', '\u{5a}', L), ('\u{5b}', '\u{60}', ON),
        ('\u{61}', '\u{7a}', L), ('\u{7b}', '\u{7e}', ON), ('\u{7f}', '\u{84}', BN), ('\u{85}',
        '\u{85}', B), ('\u{86}', '\u{9f}', BN), ('\u{a0}', '\u{a0}', CS), ('\u{a1}', '\u{a1}', ON),
        ('\u{a2}', '\u{a5}', ET), ('\u{a6}', '\u{a9}', ON), ('\u{aa}', '\u{aa}', L), ('\u{ab}',
        '\u{ac}', ON), ('\u{ad}', '\u{ad}', BN), ('\u{ae}', '\u{af}', ON), ('\u{b0}', '\u{b1}', ET),
        ('\u{b2}', '\u{b3}', EN), ('\u{b4}', '\u{b4}', ON), ('\u{b5}', '\u{b5}', L), ('\u{b6}',
        '\u{b8}', ON), ('\u{b9}', '\u{b9}', EN), ('\u{ba}', '\u{ba}', L), ('\u{bb}', '\u{bf}', ON),
        ('\u{c0}', '\u{d6}', L), ('\u{d7}', '\u{d7}', ON), ('\u{d8}', '\u{f6}', L), ('\u{f7}',
        '\u{f7}', ON), ('\u{f8}', '\u{2b8}', L), ('\u{2b9}', '\u{2ba}', ON), ('\u{2bb}', '\u{2c1}',
        L), ('\u{2c2}', '\u{2cf}', ON), ('\u{2d0}', '\u{2d1}', L), ('\u{2d2}', '\u{2df}', ON),
        ('\u{2e0}', '\u{2e4}', L), ('\u{2e5}', '\u{2ed}', ON), ('\u{2ee}', '\u{2ee}', L),
        ('\u{2ef}', '\u{2ff}', ON), ('\u{300}', '\u{36f}', NSM), ('\u{370}', '\u{373}', L),
        ('\u{374}', '\u{375}', ON), ('\u{376}', '\u{377}', L), ('\u{37a}', '\u{37d}', L),
        ('\u{37e}', '\u{37e}', ON), ('\u{37f}', '\u{37f}', L), ('\u{384}', '\u{385}', ON),
        ('\u{386}', '\u{386}', L), ('\u{387}', '\u{387}', ON), ('\u{388}', '\u{38a}', L),
        ('\u{38c}', '\u{38c}', L), ('\u{38e}', '\u{3a1}', L), ('\u{3a3}', '\u{3f5}', L), ('\u{3f6}',
        '\u{3f6}', ON), ('\u{3f7}', '\u{482}', L), ('\u{483}', '\u{489}', NSM), ('\u{48a}',
        '\u{52f}', L), ('\u{531}', '\u{556}', L), ('\u{559}', '\u{55f}', L), ('\u{561}', '\u{587}',
        L), ('\u{589}', '\u{589}', L), ('\u{58a}', '\u{58a}', ON), ('\u{58d}', '\u{58e}', ON),
        ('\u{58f}', '\u{58f}', ET), ('\u{590}', '\u{590}', R), ('\u{591}', '\u{5bd}', NSM),
        ('\u{5be}', '\u{5be}', R), ('\u{5bf}', '\u{5bf}', NSM), ('\u{5c0}', '\u{5c0}', R),
        ('\u{5c1}', '\u{5c2}', NSM), ('\u{5c3}', '\u{5c3}', R), ('\u{5c4}', '\u{5c5}', NSM),
        ('\u{5c6}', '\u{5c6}', R), ('\u{5c7}', '\u{5c7}', NSM), ('\u{5c8}', '\u{5ff}', R),
        ('\u{600}', '\u{605}', AN), ('\u{606}', '\u{607}', ON), ('\u{608}', '\u{608}', AL),
        ('\u{609}', '\u{60a}', ET), ('\u{60b}', '\u{60b}', AL), ('\u{60c}', '\u{60c}', CS),
        ('\u{60d}', '\u{60d}', AL), ('\u{60e}', '\u{60f}', ON), ('\u{610}', '\u{61a}', NSM),
        ('\u{61b}', '\u{64a}', AL), ('\u{64b}', '\u{65f}', NSM), ('\u{660}', '\u{669}', AN),
        ('\u{66a}', '\u{66a}', ET), ('\u{66b}', '\u{66c}', AN), ('\u{66d}', '\u{66f}', AL),
        ('\u{670}', '\u{670}', NSM), ('\u{671}', '\u{6d5}', AL), ('\u{6d6}', '\u{6dc}', NSM),
        ('\u{6dd}', '\u{6dd}', AN), ('\u{6de}', '\u{6de}', ON), ('\u{6df}', '\u{6e4}', NSM),
        ('\u{6e5}', '\u{6e6}', AL), ('\u{6e7}', '\u{6e8}', NSM), ('\u{6e9}', '\u{6e9}', ON),
        ('\u{6ea}', '\u{6ed}', NSM), ('\u{6ee}', '\u{6ef}', AL), ('\u{6f0}', '\u{6f9}', EN),
        ('\u{6fa}', '\u{710}', AL), ('\u{711}', '\u{711}', NSM), ('\u{712}', '\u{72f}', AL),
        ('\u{730}', '\u{74a}', NSM), ('\u{74b}', '\u{7a5}', AL), ('\u{7a6}', '\u{7b0}', NSM),
        ('\u{7b1}', '\u{7bf}', AL), ('\u{7c0}', '\u{7ea}', R), ('\u{7eb}', '\u{7f3}', NSM),
        ('\u{7f4}', '\u{7f5}', R), ('\u{7f6}', '\u{7f9}', ON), ('\u{7fa}', '\u{815}', R),
        ('\u{816}', '\u{819}', NSM), ('\u{81a}', '\u{81a}', R), ('\u{81b}', '\u{823}', NSM),
        ('\u{824}', '\u{824}', R), ('\u{825}', '\u{827}', NSM), ('\u{828}', '\u{828}', R),
        ('\u{829}', '\u{82d}', NSM), ('\u{82e}', '\u{858}', R), ('\u{859}', '\u{85b}', NSM),
        ('\u{85c}', '\u{89f}', R), ('\u{8a0}', '\u{8e3}', AL), ('\u{8e4}', '\u{902}', NSM),
        ('\u{903}', '\u{939}', L), ('\u{93a}', '\u{93a}', NSM), ('\u{93b}', '\u{93b}', L),
        ('\u{93c}', '\u{93c}', NSM), ('\u{93d}', '\u{940}', L), ('\u{941}', '\u{948}', NSM),
        ('\u{949}', '\u{94c}', L), ('\u{94d}', '\u{94d}', NSM), ('\u{94e}', '\u{950}', L),
        ('\u{951}', '\u{957}', NSM), ('\u{958}', '\u{961}', L), ('\u{962}', '\u{963}', NSM),
        ('\u{964}', '\u{980}', L), ('\u{981}', '\u{981}', NSM), ('\u{982}', '\u{983}', L),
        ('\u{985}', '\u{98c}', L), ('\u{98f}', '\u{990}', L), ('\u{993}', '\u{9a8}', L), ('\u{9aa}',
        '\u{9b0}', L), ('\u{9b2}', '\u{9b2}', L), ('\u{9b6}', '\u{9b9}', L), ('\u{9bc}', '\u{9bc}',
        NSM), ('\u{9bd}', '\u{9c0}', L), ('\u{9c1}', '\u{9c4}', NSM), ('\u{9c7}', '\u{9c8}', L),
        ('\u{9cb}', '\u{9cc}', L), ('\u{9cd}', '\u{9cd}', NSM), ('\u{9ce}', '\u{9ce}', L),
        ('\u{9d7}', '\u{9d7}', L), ('\u{9dc}', '\u{9dd}', L), ('\u{9df}', '\u{9e1}', L), ('\u{9e2}',
        '\u{9e3}', NSM), ('\u{9e6}', '\u{9f1}', L), ('\u{9f2}', '\u{9f3}', ET), ('\u{9f4}',
        '\u{9fa}', L), ('\u{9fb}', '\u{9fb}', ET), ('\u{a01}', '\u{a02}', NSM), ('\u{a03}',
        '\u{a03}', L), ('\u{a05}', '\u{a0a}', L), ('\u{a0f}', '\u{a10}', L), ('\u{a13}', '\u{a28}',
        L), ('\u{a2a}', '\u{a30}', L), ('\u{a32}', '\u{a33}', L), ('\u{a35}', '\u{a36}', L),
        ('\u{a38}', '\u{a39}', L), ('\u{a3c}', '\u{a3c}', NSM), ('\u{a3e}', '\u{a40}', L),
        ('\u{a41}', '\u{a42}', NSM), ('\u{a47}', '\u{a48}', NSM), ('\u{a4b}', '\u{a4d}', NSM),
        ('\u{a51}', '\u{a51}', NSM), ('\u{a59}', '\u{a5c}', L), ('\u{a5e}', '\u{a5e}', L),
        ('\u{a66}', '\u{a6f}', L), ('\u{a70}', '\u{a71}', NSM), ('\u{a72}', '\u{a74}', L),
        ('\u{a75}', '\u{a75}', NSM), ('\u{a81}', '\u{a82}', NSM), ('\u{a83}', '\u{a83}', L),
        ('\u{a85}', '\u{a8d}', L), ('\u{a8f}', '\u{a91}', L), ('\u{a93}', '\u{aa8}', L), ('\u{aaa}',
        '\u{ab0}', L), ('\u{ab2}', '\u{ab3}', L), ('\u{ab5}', '\u{ab9}', L), ('\u{abc}', '\u{abc}',
        NSM), ('\u{abd}', '\u{ac0}', L), ('\u{ac1}', '\u{ac5}', NSM), ('\u{ac7}', '\u{ac8}', NSM),
        ('\u{ac9}', '\u{ac9}', L), ('\u{acb}', '\u{acc}', L), ('\u{acd}', '\u{acd}', NSM),
        ('\u{ad0}', '\u{ad0}', L), ('\u{ae0}', '\u{ae1}', L), ('\u{ae2}', '\u{ae3}', NSM),
        ('\u{ae6}', '\u{af0}', L), ('\u{af1}', '\u{af1}', ET), ('\u{b01}', '\u{b01}', NSM),
        ('\u{b02}', '\u{b03}', L), ('\u{b05}', '\u{b0c}', L), ('\u{b0f}', '\u{b10}', L), ('\u{b13}',
        '\u{b28}', L), ('\u{b2a}', '\u{b30}', L), ('\u{b32}', '\u{b33}', L), ('\u{b35}', '\u{b39}',
        L), ('\u{b3c}', '\u{b3c}', NSM), ('\u{b3d}', '\u{b3e}', L), ('\u{b3f}', '\u{b3f}', NSM),
        ('\u{b40}', '\u{b40}', L), ('\u{b41}', '\u{b44}', NSM), ('\u{b47}', '\u{b48}', L),
        ('\u{b4b}', '\u{b4c}', L), ('\u{b4d}', '\u{b4d}', NSM), ('\u{b56}', '\u{b56}', NSM),
        ('\u{b57}', '\u{b57}', L), ('\u{b5c}', '\u{b5d}', L), ('\u{b5f}', '\u{b61}', L), ('\u{b62}',
        '\u{b63}', NSM), ('\u{b66}', '\u{b77}', L), ('\u{b82}', '\u{b82}', NSM), ('\u{b83}',
        '\u{b83}', L), ('\u{b85}', '\u{b8a}', L), ('\u{b8e}', '\u{b90}', L), ('\u{b92}', '\u{b95}',
        L), ('\u{b99}', '\u{b9a}', L), ('\u{b9c}', '\u{b9c}', L), ('\u{b9e}', '\u{b9f}', L),
        ('\u{ba3}', '\u{ba4}', L), ('\u{ba8}', '\u{baa}', L), ('\u{bae}', '\u{bb9}', L), ('\u{bbe}',
        '\u{bbf}', L), ('\u{bc0}', '\u{bc0}', NSM), ('\u{bc1}', '\u{bc2}', L), ('\u{bc6}',
        '\u{bc8}', L), ('\u{bca}', '\u{bcc}', L), ('\u{bcd}', '\u{bcd}', NSM), ('\u{bd0}',
        '\u{bd0}', L), ('\u{bd7}', '\u{bd7}', L), ('\u{be6}', '\u{bf2}', L), ('\u{bf3}', '\u{bf8}',
        ON), ('\u{bf9}', '\u{bf9}', ET), ('\u{bfa}', '\u{bfa}', ON), ('\u{c00}', '\u{c00}', NSM),
        ('\u{c01}', '\u{c03}', L), ('\u{c05}', '\u{c0c}', L), ('\u{c0e}', '\u{c10}', L), ('\u{c12}',
        '\u{c28}', L), ('\u{c2a}', '\u{c39}', L), ('\u{c3d}', '\u{c3d}', L), ('\u{c3e}', '\u{c40}',
        NSM), ('\u{c41}', '\u{c44}', L), ('\u{c46}', '\u{c48}', NSM), ('\u{c4a}', '\u{c4d}', NSM),
        ('\u{c55}', '\u{c56}', NSM), ('\u{c58}', '\u{c59}', L), ('\u{c60}', '\u{c61}', L),
        ('\u{c62}', '\u{c63}', NSM), ('\u{c66}', '\u{c6f}', L), ('\u{c78}', '\u{c7e}', ON),
        ('\u{c7f}', '\u{c7f}', L), ('\u{c81}', '\u{c81}', NSM), ('\u{c82}', '\u{c83}', L),
        ('\u{c85}', '\u{c8c}', L), ('\u{c8e}', '\u{c90}', L), ('\u{c92}', '\u{ca8}', L), ('\u{caa}',
        '\u{cb3}', L), ('\u{cb5}', '\u{cb9}', L), ('\u{cbc}', '\u{cbc}', NSM), ('\u{cbd}',
        '\u{cc4}', L), ('\u{cc6}', '\u{cc8}', L), ('\u{cca}', '\u{ccb}', L), ('\u{ccc}', '\u{ccd}',
        NSM), ('\u{cd5}', '\u{cd6}', L), ('\u{cde}', '\u{cde}', L), ('\u{ce0}', '\u{ce1}', L),
        ('\u{ce2}', '\u{ce3}', NSM), ('\u{ce6}', '\u{cef}', L), ('\u{cf1}', '\u{cf2}', L),
        ('\u{d01}', '\u{d01}', NSM), ('\u{d02}', '\u{d03}', L), ('\u{d05}', '\u{d0c}', L),
        ('\u{d0e}', '\u{d10}', L), ('\u{d12}', '\u{d3a}', L), ('\u{d3d}', '\u{d40}', L), ('\u{d41}',
        '\u{d44}', NSM), ('\u{d46}', '\u{d48}', L), ('\u{d4a}', '\u{d4c}', L), ('\u{d4d}',
        '\u{d4d}', NSM), ('\u{d4e}', '\u{d4e}', L), ('\u{d57}', '\u{d57}', L), ('\u{d60}',
        '\u{d61}', L), ('\u{d62}', '\u{d63}', NSM), ('\u{d66}', '\u{d75}', L), ('\u{d79}',
        '\u{d7f}', L), ('\u{d82}', '\u{d83}', L), ('\u{d85}', '\u{d96}', L), ('\u{d9a}', '\u{db1}',
        L), ('\u{db3}', '\u{dbb}', L), ('\u{dbd}', '\u{dbd}', L), ('\u{dc0}', '\u{dc6}', L),
        ('\u{dca}', '\u{dca}', NSM), ('\u{dcf}', '\u{dd1}', L), ('\u{dd2}', '\u{dd4}', NSM),
        ('\u{dd6}', '\u{dd6}', NSM), ('\u{dd8}', '\u{ddf}', L), ('\u{de6}', '\u{def}', L),
        ('\u{df2}', '\u{df4}', L), ('\u{e01}', '\u{e30}', L), ('\u{e31}', '\u{e31}', NSM),
        ('\u{e32}', '\u{e33}', L), ('\u{e34}', '\u{e3a}', NSM), ('\u{e3f}', '\u{e3f}', ET),
        ('\u{e40}', '\u{e46}', L), ('\u{e47}', '\u{e4e}', NSM), ('\u{e4f}', '\u{e5b}', L),
        ('\u{e81}', '\u{e82}', L), ('\u{e84}', '\u{e84}', L), ('\u{e87}', '\u{e88}', L), ('\u{e8a}',
        '\u{e8a}', L), ('\u{e8d}', '\u{e8d}', L), ('\u{e94}', '\u{e97}', L), ('\u{e99}', '\u{e9f}',
        L), ('\u{ea1}', '\u{ea3}', L), ('\u{ea5}', '\u{ea5}', L), ('\u{ea7}', '\u{ea7}', L),
        ('\u{eaa}', '\u{eab}', L), ('\u{ead}', '\u{eb0}', L), ('\u{eb1}', '\u{eb1}', NSM),
        ('\u{eb2}', '\u{eb3}', L), ('\u{eb4}', '\u{eb9}', NSM), ('\u{ebb}', '\u{ebc}', NSM),
        ('\u{ebd}', '\u{ebd}', L), ('\u{ec0}', '\u{ec4}', L), ('\u{ec6}', '\u{ec6}', L), ('\u{ec8}',
        '\u{ecd}', NSM), ('\u{ed0}', '\u{ed9}', L), ('\u{edc}', '\u{edf}', L), ('\u{f00}',
        '\u{f17}', L), ('\u{f18}', '\u{f19}', NSM), ('\u{f1a}', '\u{f34}', L), ('\u{f35}',
        '\u{f35}', NSM), ('\u{f36}', '\u{f36}', L), ('\u{f37}', '\u{f37}', NSM), ('\u{f38}',
        '\u{f38}', L), ('\u{f39}', '\u{f39}', NSM), ('\u{f3a}', '\u{f3d}', ON), ('\u{f3e}',
        '\u{f47}', L), ('\u{f49}', '\u{f6c}', L), ('\u{f71}', '\u{f7e}', NSM), ('\u{f7f}',
        '\u{f7f}', L), ('\u{f80}', '\u{f84}', NSM), ('\u{f85}', '\u{f85}', L), ('\u{f86}',
        '\u{f87}', NSM), ('\u{f88}', '\u{f8c}', L), ('\u{f8d}', '\u{f97}', NSM), ('\u{f99}',
        '\u{fbc}', NSM), ('\u{fbe}', '\u{fc5}', L), ('\u{fc6}', '\u{fc6}', NSM), ('\u{fc7}',
        '\u{fcc}', L), ('\u{fce}', '\u{fda}', L), ('\u{1000}', '\u{102c}', L), ('\u{102d}',
        '\u{1030}', NSM), ('\u{1031}', '\u{1031}', L), ('\u{1032}', '\u{1037}', NSM), ('\u{1038}',
        '\u{1038}', L), ('\u{1039}', '\u{103a}', NSM), ('\u{103b}', '\u{103c}', L), ('\u{103d}',
        '\u{103e}', NSM), ('\u{103f}', '\u{1057}', L), ('\u{1058}', '\u{1059}', NSM), ('\u{105a}',
        '\u{105d}', L), ('\u{105e}', '\u{1060}', NSM), ('\u{1061}', '\u{1070}', L), ('\u{1071}',
        '\u{1074}', NSM), ('\u{1075}', '\u{1081}', L), ('\u{1082}', '\u{1082}', NSM), ('\u{1083}',
        '\u{1084}', L), ('\u{1085}', '\u{1086}', NSM), ('\u{1087}', '\u{108c}', L), ('\u{108d}',
        '\u{108d}', NSM), ('\u{108e}', '\u{109c}', L), ('\u{109d}', '\u{109d}', NSM), ('\u{109e}',
        '\u{10c5}', L), ('\u{10c7}', '\u{10c7}', L), ('\u{10cd}', '\u{10cd}', L), ('\u{10d0}',
        '\u{1248}', L), ('\u{124a}', '\u{124d}', L), ('\u{1250}', '\u{1256}', L), ('\u{1258}',
        '\u{1258}', L), ('\u{125a}', '\u{125d}', L), ('\u{1260}', '\u{1288}', L), ('\u{128a}',
        '\u{128d}', L), ('\u{1290}', '\u{12b0}', L), ('\u{12b2}', '\u{12b5}', L), ('\u{12b8}',
        '\u{12be}', L), ('\u{12c0}', '\u{12c0}', L), ('\u{12c2}', '\u{12c5}', L), ('\u{12c8}',
        '\u{12d6}', L), ('\u{12d8}', '\u{1310}', L), ('\u{1312}', '\u{1315}', L), ('\u{1318}',
        '\u{135a}', L), ('\u{135d}', '\u{135f}', NSM), ('\u{1360}', '\u{137c}', L), ('\u{1380}',
        '\u{138f}', L), ('\u{1390}', '\u{1399}', ON), ('\u{13a0}', '\u{13f4}', L), ('\u{1400}',
        '\u{1400}', ON), ('\u{1401}', '\u{167f}', L), ('\u{1680}', '\u{1680}', WS), ('\u{1681}',
        '\u{169a}', L), ('\u{169b}', '\u{169c}', ON), ('\u{16a0}', '\u{16f8}', L), ('\u{1700}',
        '\u{170c}', L), ('\u{170e}', '\u{1711}', L), ('\u{1712}', '\u{1714}', NSM), ('\u{1720}',
        '\u{1731}', L), ('\u{1732}', '\u{1734}', NSM), ('\u{1735}', '\u{1736}', L), ('\u{1740}',
        '\u{1751}', L), ('\u{1752}', '\u{1753}', NSM), ('\u{1760}', '\u{176c}', L), ('\u{176e}',
        '\u{1770}', L), ('\u{1772}', '\u{1773}', NSM), ('\u{1780}', '\u{17b3}', L), ('\u{17b4}',
        '\u{17b5}', NSM), ('\u{17b6}', '\u{17b6}', L), ('\u{17b7}', '\u{17bd}', NSM), ('\u{17be}',
        '\u{17c5}', L), ('\u{17c6}', '\u{17c6}', NSM), ('\u{17c7}', '\u{17c8}', L), ('\u{17c9}',
        '\u{17d3}', NSM), ('\u{17d4}', '\u{17da}', L), ('\u{17db}', '\u{17db}', ET), ('\u{17dc}',
        '\u{17dc}', L), ('\u{17dd}', '\u{17dd}', NSM), ('\u{17e0}', '\u{17e9}', L), ('\u{17f0}',
        '\u{17f9}', ON), ('\u{1800}', '\u{180a}', ON), ('\u{180b}', '\u{180d}', NSM), ('\u{180e}',
        '\u{180e}', BN), ('\u{1810}', '\u{1819}', L), ('\u{1820}', '\u{1877}', L), ('\u{1880}',
        '\u{18a8}', L), ('\u{18a9}', '\u{18a9}', NSM), ('\u{18aa}', '\u{18aa}', L), ('\u{18b0}',
        '\u{18f5}', L), ('\u{1900}', '\u{191e}', L), ('\u{1920}', '\u{1922}', NSM), ('\u{1923}',
        '\u{1926}', L), ('\u{1927}', '\u{1928}', NSM), ('\u{1929}', '\u{192b}', L), ('\u{1930}',
        '\u{1931}', L), ('\u{1932}', '\u{1932}', NSM), ('\u{1933}', '\u{1938}', L), ('\u{1939}',
        '\u{193b}', NSM), ('\u{1940}', '\u{1940}', ON), ('\u{1944}', '\u{1945}', ON), ('\u{1946}',
        '\u{196d}', L), ('\u{1970}', '\u{1974}', L), ('\u{1980}', '\u{19ab}', L), ('\u{19b0}',
        '\u{19c9}', L), ('\u{19d0}', '\u{19da}', L), ('\u{19de}', '\u{19ff}', ON), ('\u{1a00}',
        '\u{1a16}', L), ('\u{1a17}', '\u{1a18}', NSM), ('\u{1a19}', '\u{1a1a}', L), ('\u{1a1b}',
        '\u{1a1b}', NSM), ('\u{1a1e}', '\u{1a55}', L), ('\u{1a56}', '\u{1a56}', NSM), ('\u{1a57}',
        '\u{1a57}', L), ('\u{1a58}', '\u{1a5e}', NSM), ('\u{1a60}', '\u{1a60}', NSM), ('\u{1a61}',
        '\u{1a61}', L), ('\u{1a62}', '\u{1a62}', NSM), ('\u{1a63}', '\u{1a64}', L), ('\u{1a65}',
        '\u{1a6c}', NSM), ('\u{1a6d}', '\u{1a72}', L), ('\u{1a73}', '\u{1a7c}', NSM), ('\u{1a7f}',
        '\u{1a7f}', NSM), ('\u{1a80}', '\u{1a89}', L), ('\u{1a90}', '\u{1a99}', L), ('\u{1aa0}',
        '\u{1aad}', L), ('\u{1ab0}', '\u{1abe}', NSM), ('\u{1b00}', '\u{1b03}', NSM), ('\u{1b04}',
        '\u{1b33}', L), ('\u{1b34}', '\u{1b34}', NSM), ('\u{1b35}', '\u{1b35}', L), ('\u{1b36}',
        '\u{1b3a}', NSM), ('\u{1b3b}', '\u{1b3b}', L), ('\u{1b3c}', '\u{1b3c}', NSM), ('\u{1b3d}',
        '\u{1b41}', L), ('\u{1b42}', '\u{1b42}', NSM), ('\u{1b43}', '\u{1b4b}', L), ('\u{1b50}',
        '\u{1b6a}', L), ('\u{1b6b}', '\u{1b73}', NSM), ('\u{1b74}', '\u{1b7c}', L), ('\u{1b80}',
        '\u{1b81}', NSM), ('\u{1b82}', '\u{1ba1}', L), ('\u{1ba2}', '\u{1ba5}', NSM), ('\u{1ba6}',
        '\u{1ba7}', L), ('\u{1ba8}', '\u{1ba9}', NSM), ('\u{1baa}', '\u{1baa}', L), ('\u{1bab}',
        '\u{1bad}', NSM), ('\u{1bae}', '\u{1be5}', L), ('\u{1be6}', '\u{1be6}', NSM), ('\u{1be7}',
        '\u{1be7}', L), ('\u{1be8}', '\u{1be9}', NSM), ('\u{1bea}', '\u{1bec}', L), ('\u{1bed}',
        '\u{1bed}', NSM), ('\u{1bee}', '\u{1bee}', L), ('\u{1bef}', '\u{1bf1}', NSM), ('\u{1bf2}',
        '\u{1bf3}', L), ('\u{1bfc}', '\u{1c2b}', L), ('\u{1c2c}', '\u{1c33}', NSM), ('\u{1c34}',
        '\u{1c35}', L), ('\u{1c36}', '\u{1c37}', NSM), ('\u{1c3b}', '\u{1c49}', L), ('\u{1c4d}',
        '\u{1c7f}', L), ('\u{1cc0}', '\u{1cc7}', L), ('\u{1cd0}', '\u{1cd2}', NSM), ('\u{1cd3}',
        '\u{1cd3}', L), ('\u{1cd4}', '\u{1ce0}', NSM), ('\u{1ce1}', '\u{1ce1}', L), ('\u{1ce2}',
        '\u{1ce8}', NSM), ('\u{1ce9}', '\u{1cec}', L), ('\u{1ced}', '\u{1ced}', NSM), ('\u{1cee}',
        '\u{1cf3}', L), ('\u{1cf4}', '\u{1cf4}', NSM), ('\u{1cf5}', '\u{1cf6}', L), ('\u{1cf8}',
        '\u{1cf9}', NSM), ('\u{1d00}', '\u{1dbf}', L), ('\u{1dc0}', '\u{1df5}', NSM), ('\u{1dfc}',
        '\u{1dff}', NSM), ('\u{1e00}', '\u{1f15}', L), ('\u{1f18}', '\u{1f1d}', L), ('\u{1f20}',
        '\u{1f45}', L), ('\u{1f48}', '\u{1f4d}', L), ('\u{1f50}', '\u{1f57}', L), ('\u{1f59}',
        '\u{1f59}', L), ('\u{1f5b}', '\u{1f5b}', L), ('\u{1f5d}', '\u{1f5d}', L), ('\u{1f5f}',
        '\u{1f7d}', L), ('\u{1f80}', '\u{1fb4}', L), ('\u{1fb6}', '\u{1fbc}', L), ('\u{1fbd}',
        '\u{1fbd}', ON), ('\u{1fbe}', '\u{1fbe}', L), ('\u{1fbf}', '\u{1fc1}', ON), ('\u{1fc2}',
        '\u{1fc4}', L), ('\u{1fc6}', '\u{1fcc}', L), ('\u{1fcd}', '\u{1fcf}', ON), ('\u{1fd0}',
        '\u{1fd3}', L), ('\u{1fd6}', '\u{1fdb}', L), ('\u{1fdd}', '\u{1fdf}', ON), ('\u{1fe0}',
        '\u{1fec}', L), ('\u{1fed}', '\u{1fef}', ON), ('\u{1ff2}', '\u{1ff4}', L), ('\u{1ff6}',
        '\u{1ffc}', L), ('\u{1ffd}', '\u{1ffe}', ON), ('\u{2000}', '\u{200a}', WS), ('\u{200b}',
        '\u{200d}', BN), ('\u{200e}', '\u{200e}', L), ('\u{200f}', '\u{200f}', R), ('\u{2010}',
        '\u{2027}', ON), ('\u{2028}', '\u{2028}', WS), ('\u{2029}', '\u{2029}', B), ('\u{202a}',
        '\u{202a}', LRE), ('\u{202b}', '\u{202b}', RLE), ('\u{202c}', '\u{202c}', PDF), ('\u{202d}',
        '\u{202d}', LRO), ('\u{202e}', '\u{202e}', RLO), ('\u{202f}', '\u{202f}', CS), ('\u{2030}',
        '\u{2034}', ET), ('\u{2035}', '\u{2043}', ON), ('\u{2044}', '\u{2044}', CS), ('\u{2045}',
        '\u{205e}', ON), ('\u{205f}', '\u{205f}', WS), ('\u{2060}', '\u{2064}', BN), ('\u{2066}',
        '\u{2066}', LRI), ('\u{2067}', '\u{2067}', RLI), ('\u{2068}', '\u{2068}', FSI), ('\u{2069}',
        '\u{2069}', PDI), ('\u{206a}', '\u{206f}', BN), ('\u{2070}', '\u{2070}', EN), ('\u{2071}',
        '\u{2071}', L), ('\u{2074}', '\u{2079}', EN), ('\u{207a}', '\u{207b}', ES), ('\u{207c}',
        '\u{207e}', ON), ('\u{207f}', '\u{207f}', L), ('\u{2080}', '\u{2089}', EN), ('\u{208a}',
        '\u{208b}', ES), ('\u{208c}', '\u{208e}', ON), ('\u{2090}', '\u{209c}', L), ('\u{20a0}',
        '\u{20cf}', ET), ('\u{20d0}', '\u{20f0}', NSM), ('\u{2100}', '\u{2101}', ON), ('\u{2102}',
        '\u{2102}', L), ('\u{2103}', '\u{2106}', ON), ('\u{2107}', '\u{2107}', L), ('\u{2108}',
        '\u{2109}', ON), ('\u{210a}', '\u{2113}', L), ('\u{2114}', '\u{2114}', ON), ('\u{2115}',
        '\u{2115}', L), ('\u{2116}', '\u{2118}', ON), ('\u{2119}', '\u{211d}', L), ('\u{211e}',
        '\u{2123}', ON), ('\u{2124}', '\u{2124}', L), ('\u{2125}', '\u{2125}', ON), ('\u{2126}',
        '\u{2126}', L), ('\u{2127}', '\u{2127}', ON), ('\u{2128}', '\u{2128}', L), ('\u{2129}',
        '\u{2129}', ON), ('\u{212a}', '\u{212d}', L), ('\u{212e}', '\u{212e}', ET), ('\u{212f}',
        '\u{2139}', L), ('\u{213a}', '\u{213b}', ON), ('\u{213c}', '\u{213f}', L), ('\u{2140}',
        '\u{2144}', ON), ('\u{2145}', '\u{2149}', L), ('\u{214a}', '\u{214d}', ON), ('\u{214e}',
        '\u{214f}', L), ('\u{2150}', '\u{215f}', ON), ('\u{2160}', '\u{2188}', L), ('\u{2189}',
        '\u{2189}', ON), ('\u{2190}', '\u{2211}', ON), ('\u{2212}', '\u{2212}', ES), ('\u{2213}',
        '\u{2213}', ET), ('\u{2214}', '\u{2335}', ON), ('\u{2336}', '\u{237a}', L), ('\u{237b}',
        '\u{2394}', ON), ('\u{2395}', '\u{2395}', L), ('\u{2396}', '\u{23fa}', ON), ('\u{2400}',
        '\u{2426}', ON), ('\u{2440}', '\u{244a}', ON), ('\u{2460}', '\u{2487}', ON), ('\u{2488}',
        '\u{249b}', EN), ('\u{249c}', '\u{24e9}', L), ('\u{24ea}', '\u{26ab}', ON), ('\u{26ac}',
        '\u{26ac}', L), ('\u{26ad}', '\u{27ff}', ON), ('\u{2800}', '\u{28ff}', L), ('\u{2900}',
        '\u{2b73}', ON), ('\u{2b76}', '\u{2b95}', ON), ('\u{2b98}', '\u{2bb9}', ON), ('\u{2bbd}',
        '\u{2bc8}', ON), ('\u{2bca}', '\u{2bd1}', ON), ('\u{2c00}', '\u{2c2e}', L), ('\u{2c30}',
        '\u{2c5e}', L), ('\u{2c60}', '\u{2ce4}', L), ('\u{2ce5}', '\u{2cea}', ON), ('\u{2ceb}',
        '\u{2cee}', L), ('\u{2cef}', '\u{2cf1}', NSM), ('\u{2cf2}', '\u{2cf3}', L), ('\u{2cf9}',
        '\u{2cff}', ON), ('\u{2d00}', '\u{2d25}', L), ('\u{2d27}', '\u{2d27}', L), ('\u{2d2d}',
        '\u{2d2d}', L), ('\u{2d30}', '\u{2d67}', L), ('\u{2d6f}', '\u{2d70}', L), ('\u{2d7f}',
        '\u{2d7f}', NSM), ('\u{2d80}', '\u{2d96}', L), ('\u{2da0}', '\u{2da6}', L), ('\u{2da8}',
        '\u{2dae}', L), ('\u{2db0}', '\u{2db6}', L), ('\u{2db8}', '\u{2dbe}', L), ('\u{2dc0}',
        '\u{2dc6}', L), ('\u{2dc8}', '\u{2dce}', L), ('\u{2dd0}', '\u{2dd6}', L), ('\u{2dd8}',
        '\u{2dde}', L), ('\u{2de0}', '\u{2dff}', NSM), ('\u{2e00}', '\u{2e42}', ON), ('\u{2e80}',
        '\u{2e99}', ON), ('\u{2e9b}', '\u{2ef3}', ON), ('\u{2f00}', '\u{2fd5}', ON), ('\u{2ff0}',
        '\u{2ffb}', ON), ('\u{3000}', '\u{3000}', WS), ('\u{3001}', '\u{3004}', ON), ('\u{3005}',
        '\u{3007}', L), ('\u{3008}', '\u{3020}', ON), ('\u{3021}', '\u{3029}', L), ('\u{302a}',
        '\u{302d}', NSM), ('\u{302e}', '\u{302f}', L), ('\u{3030}', '\u{3030}', ON), ('\u{3031}',
        '\u{3035}', L), ('\u{3036}', '\u{3037}', ON), ('\u{3038}', '\u{303c}', L), ('\u{303d}',
        '\u{303f}', ON), ('\u{3041}', '\u{3096}', L), ('\u{3099}', '\u{309a}', NSM), ('\u{309b}',
        '\u{309c}', ON), ('\u{309d}', '\u{309f}', L), ('\u{30a0}', '\u{30a0}', ON), ('\u{30a1}',
        '\u{30fa}', L), ('\u{30fb}', '\u{30fb}', ON), ('\u{30fc}', '\u{30ff}', L), ('\u{3105}',
        '\u{312d}', L), ('\u{3131}', '\u{318e}', L), ('\u{3190}', '\u{31ba}', L), ('\u{31c0}',
        '\u{31e3}', ON), ('\u{31f0}', '\u{321c}', L), ('\u{321d}', '\u{321e}', ON), ('\u{3220}',
        '\u{324f}', L), ('\u{3250}', '\u{325f}', ON), ('\u{3260}', '\u{327b}', L), ('\u{327c}',
        '\u{327e}', ON), ('\u{327f}', '\u{32b0}', L), ('\u{32b1}', '\u{32bf}', ON), ('\u{32c0}',
        '\u{32cb}', L), ('\u{32cc}', '\u{32cf}', ON), ('\u{32d0}', '\u{32fe}', L), ('\u{3300}',
        '\u{3376}', L), ('\u{3377}', '\u{337a}', ON), ('\u{337b}', '\u{33dd}', L), ('\u{33de}',
        '\u{33df}', ON), ('\u{33e0}', '\u{33fe}', L), ('\u{33ff}', '\u{33ff}', ON), ('\u{3400}',
        '\u{4db5}', L), ('\u{4dc0}', '\u{4dff}', ON), ('\u{4e00}', '\u{9fcc}', L), ('\u{a000}',
        '\u{a48c}', L), ('\u{a490}', '\u{a4c6}', ON), ('\u{a4d0}', '\u{a60c}', L), ('\u{a60d}',
        '\u{a60f}', ON), ('\u{a610}', '\u{a62b}', L), ('\u{a640}', '\u{a66e}', L), ('\u{a66f}',
        '\u{a672}', NSM), ('\u{a673}', '\u{a673}', ON), ('\u{a674}', '\u{a67d}', NSM), ('\u{a67e}',
        '\u{a67f}', ON), ('\u{a680}', '\u{a69d}', L), ('\u{a69f}', '\u{a69f}', NSM), ('\u{a6a0}',
        '\u{a6ef}', L), ('\u{a6f0}', '\u{a6f1}', NSM), ('\u{a6f2}', '\u{a6f7}', L), ('\u{a700}',
        '\u{a721}', ON), ('\u{a722}', '\u{a787}', L), ('\u{a788}', '\u{a788}', ON), ('\u{a789}',
        '\u{a78e}', L), ('\u{a790}', '\u{a7ad}', L), ('\u{a7b0}', '\u{a7b1}', L), ('\u{a7f7}',
        '\u{a801}', L), ('\u{a802}', '\u{a802}', NSM), ('\u{a803}', '\u{a805}', L), ('\u{a806}',
        '\u{a806}', NSM), ('\u{a807}', '\u{a80a}', L), ('\u{a80b}', '\u{a80b}', NSM), ('\u{a80c}',
        '\u{a824}', L), ('\u{a825}', '\u{a826}', NSM), ('\u{a827}', '\u{a827}', L), ('\u{a828}',
        '\u{a82b}', ON), ('\u{a830}', '\u{a837}', L), ('\u{a838}', '\u{a839}', ET), ('\u{a840}',
        '\u{a873}', L), ('\u{a874}', '\u{a877}', ON), ('\u{a880}', '\u{a8c3}', L), ('\u{a8c4}',
        '\u{a8c4}', NSM), ('\u{a8ce}', '\u{a8d9}', L), ('\u{a8e0}', '\u{a8f1}', NSM), ('\u{a8f2}',
        '\u{a8fb}', L), ('\u{a900}', '\u{a925}', L), ('\u{a926}', '\u{a92d}', NSM), ('\u{a92e}',
        '\u{a946}', L), ('\u{a947}', '\u{a951}', NSM), ('\u{a952}', '\u{a953}', L), ('\u{a95f}',
        '\u{a97c}', L), ('\u{a980}', '\u{a982}', NSM), ('\u{a983}', '\u{a9b2}', L), ('\u{a9b3}',
        '\u{a9b3}', NSM), ('\u{a9b4}', '\u{a9b5}', L), ('\u{a9b6}', '\u{a9b9}', NSM), ('\u{a9ba}',
        '\u{a9bb}', L), ('\u{a9bc}', '\u{a9bc}', NSM), ('\u{a9bd}', '\u{a9cd}', L), ('\u{a9cf}',
        '\u{a9d9}', L), ('\u{a9de}', '\u{a9e4}', L), ('\u{a9e5}', '\u{a9e5}', NSM), ('\u{a9e6}',
        '\u{a9fe}', L), ('\u{aa00}', '\u{aa28}', L), ('\u{aa29}', '\u{aa2e}', NSM), ('\u{aa2f}',
        '\u{aa30}', L), ('\u{aa31}', '\u{aa32}', NSM), ('\u{aa33}', '\u{aa34}', L), ('\u{aa35}',
        '\u{aa36}', NSM), ('\u{aa40}', '\u{aa42}', L), ('\u{aa43}', '\u{aa43}', NSM), ('\u{aa44}',
        '\u{aa4b}', L), ('\u{aa4c}', '\u{aa4c}', NSM), ('\u{aa4d}', '\u{aa4d}', L), ('\u{aa50}',
        '\u{aa59}', L), ('\u{aa5c}', '\u{aa7b}', L), ('\u{aa7c}', '\u{aa7c}', NSM), ('\u{aa7d}',
        '\u{aaaf}', L), ('\u{aab0}', '\u{aab0}', NSM), ('\u{aab1}', '\u{aab1}', L), ('\u{aab2}',
        '\u{aab4}', NSM), ('\u{aab5}', '\u{aab6}', L), ('\u{aab7}', '\u{aab8}', NSM), ('\u{aab9}',
        '\u{aabd}', L), ('\u{aabe}', '\u{aabf}', NSM), ('\u{aac0}', '\u{aac0}', L), ('\u{aac1}',
        '\u{aac1}', NSM), ('\u{aac2}', '\u{aac2}', L), ('\u{aadb}', '\u{aaeb}', L), ('\u{aaec}',
        '\u{aaed}', NSM), ('\u{aaee}', '\u{aaf5}', L), ('\u{aaf6}', '\u{aaf6}', NSM), ('\u{ab01}',
        '\u{ab06}', L), ('\u{ab09}', '\u{ab0e}', L), ('\u{ab11}', '\u{ab16}', L), ('\u{ab20}',
        '\u{ab26}', L), ('\u{ab28}', '\u{ab2e}', L), ('\u{ab30}', '\u{ab5f}', L), ('\u{ab64}',
        '\u{ab65}', L), ('\u{abc0}', '\u{abe4}', L), ('\u{abe5}', '\u{abe5}', NSM), ('\u{abe6}',
        '\u{abe7}', L), ('\u{abe8}', '\u{abe8}', NSM), ('\u{abe9}', '\u{abec}', L), ('\u{abed}',
        '\u{abed}', NSM), ('\u{abf0}', '\u{abf9}', L), ('\u{ac00}', '\u{d7a3}', L), ('\u{d7b0}',
        '\u{d7c6}', L), ('\u{d7cb}', '\u{d7fb}', L), ('\u{e000}', '\u{fa6d}', L), ('\u{fa70}',
        '\u{fad9}', L), ('\u{fb00}', '\u{fb06}', L), ('\u{fb13}', '\u{fb17}', L), ('\u{fb1d}',
        '\u{fb1d}', R), ('\u{fb1e}', '\u{fb1e}', NSM), ('\u{fb1f}', '\u{fb28}', R), ('\u{fb29}',
        '\u{fb29}', ES), ('\u{fb2a}', '\u{fb4f}', R), ('\u{fb50}', '\u{fd3d}', AL), ('\u{fd3e}',
        '\u{fd3f}', ON), ('\u{fd40}', '\u{fdcf}', AL), ('\u{fdf0}', '\u{fdfc}', AL), ('\u{fdfd}',
        '\u{fdfd}', ON), ('\u{fdfe}', '\u{fdff}', AL), ('\u{fe00}', '\u{fe0f}', NSM), ('\u{fe10}',
        '\u{fe19}', ON), ('\u{fe20}', '\u{fe2d}', NSM), ('\u{fe30}', '\u{fe4f}', ON), ('\u{fe50}',
        '\u{fe50}', CS), ('\u{fe51}', '\u{fe51}', ON), ('\u{fe52}', '\u{fe52}', CS), ('\u{fe54}',
        '\u{fe54}', ON), ('\u{fe55}', '\u{fe55}', CS), ('\u{fe56}', '\u{fe5e}', ON), ('\u{fe5f}',
        '\u{fe5f}', ET), ('\u{fe60}', '\u{fe61}', ON), ('\u{fe62}', '\u{fe63}', ES), ('\u{fe64}',
        '\u{fe66}', ON), ('\u{fe68}', '\u{fe68}', ON), ('\u{fe69}', '\u{fe6a}', ET), ('\u{fe6b}',
        '\u{fe6b}', ON), ('\u{fe70}', '\u{fefe}', AL), ('\u{feff}', '\u{feff}', BN), ('\u{ff01}',
        '\u{ff02}', ON), ('\u{ff03}', '\u{ff05}', ET), ('\u{ff06}', '\u{ff0a}', ON), ('\u{ff0b}',
        '\u{ff0b}', ES), ('\u{ff0c}', '\u{ff0c}', CS), ('\u{ff0d}', '\u{ff0d}', ES), ('\u{ff0e}',
        '\u{ff0f}', CS), ('\u{ff10}', '\u{ff19}', EN), ('\u{ff1a}', '\u{ff1a}', CS), ('\u{ff1b}',
        '\u{ff20}', ON), ('\u{ff21}', '\u{ff3a}', L), ('\u{ff3b}', '\u{ff40}', ON), ('\u{ff41}',
        '\u{ff5a}', L), ('\u{ff5b}', '\u{ff65}', ON), ('\u{ff66}', '\u{ffbe}', L), ('\u{ffc2}',
        '\u{ffc7}', L), ('\u{ffca}', '\u{ffcf}', L), ('\u{ffd2}', '\u{ffd7}', L), ('\u{ffda}',
        '\u{ffdc}', L), ('\u{ffe0}', '\u{ffe1}', ET), ('\u{ffe2}', '\u{ffe4}', ON), ('\u{ffe5}',
        '\u{ffe6}', ET), ('\u{ffe8}', '\u{ffee}', ON), ('\u{fff9}', '\u{fffd}', ON), ('\u{10000}',
        '\u{1000b}', L), ('\u{1000d}', '\u{10026}', L), ('\u{10028}', '\u{1003a}', L), ('\u{1003c}',
        '\u{1003d}', L), ('\u{1003f}', '\u{1004d}', L), ('\u{10050}', '\u{1005d}', L), ('\u{10080}',
        '\u{100fa}', L), ('\u{10100}', '\u{10100}', L), ('\u{10101}', '\u{10101}', ON),
        ('\u{10102}', '\u{10102}', L), ('\u{10107}', '\u{10133}', L), ('\u{10137}', '\u{1013f}', L),
        ('\u{10140}', '\u{1018c}', ON), ('\u{10190}', '\u{1019b}', ON), ('\u{101a0}', '\u{101a0}',
        ON), ('\u{101d0}', '\u{101fc}', L), ('\u{101fd}', '\u{101fd}', NSM), ('\u{10280}',
        '\u{1029c}', L), ('\u{102a0}', '\u{102d0}', L), ('\u{102e0}', '\u{102e0}', NSM),
        ('\u{102e1}', '\u{102fb}', EN), ('\u{10300}', '\u{10323}', L), ('\u{10330}', '\u{1034a}',
        L), ('\u{10350}', '\u{10375}', L), ('\u{10376}', '\u{1037a}', NSM), ('\u{10380}',
        '\u{1039d}', L), ('\u{1039f}', '\u{103c3}', L), ('\u{103c8}', '\u{103d5}', L), ('\u{10400}',
        '\u{1049d}', L), ('\u{104a0}', '\u{104a9}', L), ('\u{10500}', '\u{10527}', L), ('\u{10530}',
        '\u{10563}', L), ('\u{1056f}', '\u{1056f}', L), ('\u{10600}', '\u{10736}', L), ('\u{10740}',
        '\u{10755}', L), ('\u{10760}', '\u{10767}', L), ('\u{10800}', '\u{1091e}', R), ('\u{1091f}',
        '\u{1091f}', ON), ('\u{10920}', '\u{10a00}', R), ('\u{10a01}', '\u{10a03}', NSM),
        ('\u{10a04}', '\u{10a04}', R), ('\u{10a05}', '\u{10a06}', NSM), ('\u{10a07}', '\u{10a0b}',
        R), ('\u{10a0c}', '\u{10a0f}', NSM), ('\u{10a10}', '\u{10a37}', R), ('\u{10a38}',
        '\u{10a3a}', NSM), ('\u{10a3b}', '\u{10a3e}', R), ('\u{10a3f}', '\u{10a3f}', NSM),
        ('\u{10a40}', '\u{10ae4}', R), ('\u{10ae5}', '\u{10ae6}', NSM), ('\u{10ae7}', '\u{10b38}',
        R), ('\u{10b39}', '\u{10b3f}', ON), ('\u{10b40}', '\u{10e5f}', R), ('\u{10e60}',
        '\u{10e7e}', AN), ('\u{10e7f}', '\u{10fff}', R), ('\u{11000}', '\u{11000}', L),
        ('\u{11001}', '\u{11001}', NSM), ('\u{11002}', '\u{11037}', L), ('\u{11038}', '\u{11046}',
        NSM), ('\u{11047}', '\u{1104d}', L), ('\u{11052}', '\u{11065}', ON), ('\u{11066}',
        '\u{1106f}', L), ('\u{1107f}', '\u{11081}', NSM), ('\u{11082}', '\u{110b2}', L),
        ('\u{110b3}', '\u{110b6}', NSM), ('\u{110b7}', '\u{110b8}', L), ('\u{110b9}', '\u{110ba}',
        NSM), ('\u{110bb}', '\u{110c1}', L), ('\u{110d0}', '\u{110e8}', L), ('\u{110f0}',
        '\u{110f9}', L), ('\u{11100}', '\u{11102}', NSM), ('\u{11103}', '\u{11126}', L),
        ('\u{11127}', '\u{1112b}', NSM), ('\u{1112c}', '\u{1112c}', L), ('\u{1112d}', '\u{11134}',
        NSM), ('\u{11136}', '\u{11143}', L), ('\u{11150}', '\u{11172}', L), ('\u{11173}',
        '\u{11173}', NSM), ('\u{11174}', '\u{11176}', L), ('\u{11180}', '\u{11181}', NSM),
        ('\u{11182}', '\u{111b5}', L), ('\u{111b6}', '\u{111be}', NSM), ('\u{111bf}', '\u{111c8}',
        L), ('\u{111cd}', '\u{111cd}', L), ('\u{111d0}', '\u{111da}', L), ('\u{111e1}', '\u{111f4}',
        L), ('\u{11200}', '\u{11211}', L), ('\u{11213}', '\u{1122e}', L), ('\u{1122f}', '\u{11231}',
        NSM), ('\u{11232}', '\u{11233}', L), ('\u{11234}', '\u{11234}', NSM), ('\u{11235}',
        '\u{11235}', L), ('\u{11236}', '\u{11237}', NSM), ('\u{11238}', '\u{1123d}', L),
        ('\u{112b0}', '\u{112de}', L), ('\u{112df}', '\u{112df}', NSM), ('\u{112e0}', '\u{112e2}',
        L), ('\u{112e3}', '\u{112ea}', NSM), ('\u{112f0}', '\u{112f9}', L), ('\u{11301}',
        '\u{11301}', NSM), ('\u{11302}', '\u{11303}', L), ('\u{11305}', '\u{1130c}', L),
        ('\u{1130f}', '\u{11310}', L), ('\u{11313}', '\u{11328}', L), ('\u{1132a}', '\u{11330}', L),
        ('\u{11332}', '\u{11333}', L), ('\u{11335}', '\u{11339}', L), ('\u{1133c}', '\u{1133c}',
        NSM), ('\u{1133d}', '\u{1133f}', L), ('\u{11340}', '\u{11340}', NSM), ('\u{11341}',
        '\u{11344}', L), ('\u{11347}', '\u{11348}', L), ('\u{1134b}', '\u{1134d}', L), ('\u{11357}',
        '\u{11357}', L), ('\u{1135d}', '\u{11363}', L), ('\u{11366}', '\u{1136c}', NSM),
        ('\u{11370}', '\u{11374}', NSM), ('\u{11480}', '\u{114b2}', L), ('\u{114b3}', '\u{114b8}',
        NSM), ('\u{114b9}', '\u{114b9}', L), ('\u{114ba}', '\u{114ba}', NSM), ('\u{114bb}',
        '\u{114be}', L), ('\u{114bf}', '\u{114c0}', NSM), ('\u{114c1}', '\u{114c1}', L),
        ('\u{114c2}', '\u{114c3}', NSM), ('\u{114c4}', '\u{114c7}', L), ('\u{114d0}', '\u{114d9}',
        L), ('\u{11580}', '\u{115b1}', L), ('\u{115b2}', '\u{115b5}', NSM), ('\u{115b8}',
        '\u{115bb}', L), ('\u{115bc}', '\u{115bd}', NSM), ('\u{115be}', '\u{115be}', L),
        ('\u{115bf}', '\u{115c0}', NSM), ('\u{115c1}', '\u{115c9}', L), ('\u{11600}', '\u{11632}',
        L), ('\u{11633}', '\u{1163a}', NSM), ('\u{1163b}', '\u{1163c}', L), ('\u{1163d}',
        '\u{1163d}', NSM), ('\u{1163e}', '\u{1163e}', L), ('\u{1163f}', '\u{11640}', NSM),
        ('\u{11641}', '\u{11644}', L), ('\u{11650}', '\u{11659}', L), ('\u{11680}', '\u{116aa}', L),
        ('\u{116ab}', '\u{116ab}', NSM), ('\u{116ac}', '\u{116ac}', L), ('\u{116ad}', '\u{116ad}',
        NSM), ('\u{116ae}', '\u{116af}', L), ('\u{116b0}', '\u{116b5}', NSM), ('\u{116b6}',
        '\u{116b6}', L), ('\u{116b7}', '\u{116b7}', NSM), ('\u{116c0}', '\u{116c9}', L),
        ('\u{118a0}', '\u{118f2}', L), ('\u{118ff}', '\u{118ff}', L), ('\u{11ac0}', '\u{11af8}', L),
        ('\u{12000}', '\u{12398}', L), ('\u{12400}', '\u{1246e}', L), ('\u{12470}', '\u{12474}', L),
        ('\u{13000}', '\u{1342e}', L), ('\u{16800}', '\u{16a38}', L), ('\u{16a40}', '\u{16a5e}', L),
        ('\u{16a60}', '\u{16a69}', L), ('\u{16a6e}', '\u{16a6f}', L), ('\u{16ad0}', '\u{16aed}', L),
        ('\u{16af0}', '\u{16af4}', NSM), ('\u{16af5}', '\u{16af5}', L), ('\u{16b00}', '\u{16b2f}',
        L), ('\u{16b30}', '\u{16b36}', NSM), ('\u{16b37}', '\u{16b45}', L), ('\u{16b50}',
        '\u{16b59}', L), ('\u{16b5b}', '\u{16b61}', L), ('\u{16b63}', '\u{16b77}', L), ('\u{16b7d}',
        '\u{16b8f}', L), ('\u{16f00}', '\u{16f44}', L), ('\u{16f50}', '\u{16f7e}', L), ('\u{16f8f}',
        '\u{16f92}', NSM), ('\u{16f93}', '\u{16f9f}', L), ('\u{1b000}', '\u{1b001}', L),
        ('\u{1bc00}', '\u{1bc6a}', L), ('\u{1bc70}', '\u{1bc7c}', L), ('\u{1bc80}', '\u{1bc88}', L),
        ('\u{1bc90}', '\u{1bc99}', L), ('\u{1bc9c}', '\u{1bc9c}', L), ('\u{1bc9d}', '\u{1bc9e}',
        NSM), ('\u{1bc9f}', '\u{1bc9f}', L), ('\u{1bca0}', '\u{1bca3}', BN), ('\u{1d000}',
        '\u{1d0f5}', L), ('\u{1d100}', '\u{1d126}', L), ('\u{1d129}', '\u{1d166}', L), ('\u{1d167}',
        '\u{1d169}', NSM), ('\u{1d16a}', '\u{1d172}', L), ('\u{1d173}', '\u{1d17a}', BN),
        ('\u{1d17b}', '\u{1d182}', NSM), ('\u{1d183}', '\u{1d184}', L), ('\u{1d185}', '\u{1d18b}',
        NSM), ('\u{1d18c}', '\u{1d1a9}', L), ('\u{1d1aa}', '\u{1d1ad}', NSM), ('\u{1d1ae}',
        '\u{1d1dd}', L), ('\u{1d200}', '\u{1d241}', ON), ('\u{1d242}', '\u{1d244}', NSM),
        ('\u{1d245}', '\u{1d245}', ON), ('\u{1d300}', '\u{1d356}', ON), ('\u{1d360}', '\u{1d371}',
        L), ('\u{1d400}', '\u{1d454}', L), ('\u{1d456}', '\u{1d49c}', L), ('\u{1d49e}', '\u{1d49f}',
        L), ('\u{1d4a2}', '\u{1d4a2}', L), ('\u{1d4a5}', '\u{1d4a6}', L), ('\u{1d4a9}', '\u{1d4ac}',
        L), ('\u{1d4ae}', '\u{1d4b9}', L), ('\u{1d4bb}', '\u{1d4bb}', L), ('\u{1d4bd}', '\u{1d4c3}',
        L), ('\u{1d4c5}', '\u{1d505}', L), ('\u{1d507}', '\u{1d50a}', L), ('\u{1d50d}', '\u{1d514}',
        L), ('\u{1d516}', '\u{1d51c}', L), ('\u{1d51e}', '\u{1d539}', L), ('\u{1d53b}', '\u{1d53e}',
        L), ('\u{1d540}', '\u{1d544}', L), ('\u{1d546}', '\u{1d546}', L), ('\u{1d54a}', '\u{1d550}',
        L), ('\u{1d552}', '\u{1d6a5}', L), ('\u{1d6a8}', '\u{1d6da}', L), ('\u{1d6db}', '\u{1d6db}',
        ON), ('\u{1d6dc}', '\u{1d714}', L), ('\u{1d715}', '\u{1d715}', ON), ('\u{1d716}',
        '\u{1d74e}', L), ('\u{1d74f}', '\u{1d74f}', ON), ('\u{1d750}', '\u{1d788}', L),
        ('\u{1d789}', '\u{1d789}', ON), ('\u{1d78a}', '\u{1d7c2}', L), ('\u{1d7c3}', '\u{1d7c3}',
        ON), ('\u{1d7c4}', '\u{1d7cb}', L), ('\u{1d7ce}', '\u{1d7ff}', EN), ('\u{1e800}',
        '\u{1e8cf}', R), ('\u{1e8d0}', '\u{1e8d6}', NSM), ('\u{1e8d7}', '\u{1edff}', R),
        ('\u{1ee00}', '\u{1eeef}', AL), ('\u{1eef0}', '\u{1eef1}', ON), ('\u{1eef2}', '\u{1eeff}',
        AL), ('\u{1ef00}', '\u{1efff}', R), ('\u{1f000}', '\u{1f02b}', ON), ('\u{1f030}',
        '\u{1f093}', ON), ('\u{1f0a0}', '\u{1f0ae}', ON), ('\u{1f0b1}', '\u{1f0bf}', ON),
        ('\u{1f0c1}', '\u{1f0cf}', ON), ('\u{1f0d1}', '\u{1f0f5}', ON), ('\u{1f100}', '\u{1f10a}',
        EN), ('\u{1f10b}', '\u{1f10c}', ON), ('\u{1f110}', '\u{1f12e}', L), ('\u{1f130}',
        '\u{1f169}', L), ('\u{1f16a}', '\u{1f16b}', ON), ('\u{1f170}', '\u{1f19a}', L),
        ('\u{1f1e6}', '\u{1f202}', L), ('\u{1f210}', '\u{1f23a}', L), ('\u{1f240}', '\u{1f248}', L),
        ('\u{1f250}', '\u{1f251}', L), ('\u{1f300}', '\u{1f32c}', ON), ('\u{1f330}', '\u{1f37d}',
        ON), ('\u{1f380}', '\u{1f3ce}', ON), ('\u{1f3d4}', '\u{1f3f7}', ON), ('\u{1f400}',
        '\u{1f4fe}', ON), ('\u{1f500}', '\u{1f54a}', ON), ('\u{1f550}', '\u{1f579}', ON),
        ('\u{1f57b}', '\u{1f5a3}', ON), ('\u{1f5a5}', '\u{1f642}', ON), ('\u{1f645}', '\u{1f6cf}',
        ON), ('\u{1f6e0}', '\u{1f6ec}', ON), ('\u{1f6f0}', '\u{1f6f3}', ON), ('\u{1f700}',
        '\u{1f773}', ON), ('\u{1f780}', '\u{1f7d4}', ON), ('\u{1f800}', '\u{1f80b}', ON),
        ('\u{1f810}', '\u{1f847}', ON), ('\u{1f850}', '\u{1f859}', ON), ('\u{1f860}', '\u{1f887}',
        ON), ('\u{1f890}', '\u{1f8ad}', ON), ('\u{20000}', '\u{2a6d6}', L), ('\u{2a700}',
        '\u{2b734}', L), ('\u{2b740}', '\u{2b81d}', L), ('\u{2f800}', '\u{2fa1d}', L), ('\u{e0001}',
        '\u{e0001}', BN), ('\u{e0020}', '\u{e007f}', BN), ('\u{e0100}', '\u{e01ef}', NSM),
        ('\u{f0000}', '\u{ffffd}', L), ('\u{100000}', '\u{10fffd}', L)
    ];

