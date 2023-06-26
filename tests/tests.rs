use hfile::hash;
use std::path::Path;

#[test]
fn hash_md5() {
    let result = hash::md5(Path::new("tests/test-file"));
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), "37c4b87edffc5d198ff5a185cee7ee09")
}

#[test]
fn hash_sha1() {
    let result = hash::sha1(Path::new("tests/test-file"));
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), "be417768b5c3c5c1d9bcb2e7c119196dd76b5570")
}

#[test]
fn hash_sha256() {
    let result = hash::sha256(Path::new("tests/test-file"));
    assert_eq!(result.is_ok(), true);
    assert_eq!(
        result.unwrap(),
        "c03905fcdab297513a620ec81ed46ca44ddb62d41cbbd83eb4a5a3592be26a69"
    )
}

#[test]
fn hash_sha384() {
    let result = hash::sha384(Path::new("tests/test-file"));
    assert_eq!(result.is_ok(), true);
    assert_eq!(
        result.unwrap(),
        "f565ad8f9c76cf8c4a2e145e712df740702e066a5908f6285eafa1a83a623e882207643ce5ec29628ff0186150275ef3")
}

#[test]
fn hash_sha512() {
    let result = hash::sha512(Path::new("tests/test-file"));
    assert_eq!(result.is_ok(), true);
    assert_eq!(
        result.unwrap(),
        "a12ac6bdd854ac30c5cc5b576e1ee2c060c0d8c2bec8797423d7119aa2b962f7f30ce2e39879cbff0109c8f0a3fd9389a369daae45df7d7b286d7d98272dc5b1")
}

#[test]
fn hash_blake() {
    let result = hash::blake3(Path::new("tests/test-file"));
    assert_eq!(result.is_ok(), true);
    assert_eq!(
        result.unwrap(),
        "9a689455c65ca329fbcae5a1ae8725d88c7a6fbc82fd25bbcd9370ad9c272c50"
    )
}
