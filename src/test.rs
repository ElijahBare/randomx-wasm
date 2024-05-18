use blake2::{Blake2b512, Blake2s256, Digest};
// use hex_literal::hex;

#[test]
// Test the keys have been generated as described in the specification.
// At the same time, we check that the version of Blake2b512 we use is
// consistent.
fn test_aesgenerator_1r_key_consistency() {
    let mut hasher = Blake2b512::new();

    hasher.update(b"RandomX AesGenerator1R keys");

    let exp_k0: [u8; 16] = [
        0x53, 0xa5, 0xac, 0x6d, 0x09, 0x66, 0x71, 0x62, 0x2b, 0x55, 0xb5, 0xdb, 0x17, 0x49, 0xf4,
        0xb4,
    ];

    let exp_k1: [u8; 16] = [
        0x07, 0xaf, 0x7c, 0x6d, 0x0d, 0x71, 0x6a, 0x84, 0x78, 0xd3, 0x25, 0x17, 0x4e, 0xdc, 0xa1,
        0x0d,
    ];

    let exp_k2: [u8; 16] = [
        0xf1, 0x62, 0x12, 0x3f, 0xc6, 0x7e, 0x94, 0x9f, 0x4f, 0x79, 0xc0, 0xf4, 0x45, 0xe3, 0x20,
        0x3e,
    ];

    let exp_k3: [u8; 16] = [
        0x35, 0x81, 0xef, 0x6a, 0x7c, 0x31, 0xba, 0xb1, 0x88, 0x4c, 0x31, 0x16, 0x54, 0x91, 0x16,
        0x49,
    ];
    let res = hasher.finalize();

    assert_eq!(exp_k0, res[0..16]);
    assert_eq!(exp_k1, res[16..32]);
    assert_eq!(exp_k2, res[32..48]);
    assert_eq!(exp_k3, res[48..64]);
}
