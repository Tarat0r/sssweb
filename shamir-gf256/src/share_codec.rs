/*
Hex codec for a single Shamir `Share` with Reed–Solomon ECC.
Serializes as `SHR1`: stores x, y_len, and RS codeword (y || ecc_len parity).
`ecc_len` >= 2; corrects up to floor(ecc_len/2) random byte errors.

Layout:
  magic:  b"SHR1"     // 4 bytes
  x:      u8          // 1 byte
  ecc:    u8          // number of RS parity bytes
  y_len:  u32 LE      // original y length (without parity)
  code:   [u8; y_len + ecc]
*/

use crate::{GF256, Share};
use thiserror::Error;
use reed_solomon::{Encoder, Decoder};
use zeroize::{Zeroizing};

const MAGIC: &[u8; 4] = b"SHR1";

#[derive(Debug, Error)]
pub enum ShareCodecError {
    #[error("invalid hex: {0}")]
    Hex(#[from] hex::FromHexError),
    #[error("invalid header (bad magic)")]
    BadMagic,
    #[error("truncated or malformed input")]
    Truncated,
    #[error("too many items or lengths overflow")]
    Overflow,
    #[error("reed-solomon decode failed")]
    EccDecode,
}

pub fn share_to_hex(share: &Share, ecc_len: usize) -> String {
    assert!((2..=255).contains(&ecc_len), "ecc_len must be in 2..=255");

    let enc = Encoder::new(ecc_len);
    let code = enc.encode(&share.y);

    let mut out = Zeroizing::new(Vec::with_capacity(4 + 1 + 1 + 4 + code.len()));
    out.extend_from_slice(MAGIC);
    out.push(share.x.0);
    out.push(ecc_len as u8);

    let y_len: u32 = share.y.len().try_into().expect("y too long");
    out.extend_from_slice(&y_len.to_le_bytes());
    out.extend_from_slice(&code[..]);  // data + parity

    hex::encode(out)
}

pub fn share_from_hex(s: &str) -> Result<Share, ShareCodecError> {
    let bytes = Zeroizing::new(hex::decode(s)?);
    let mut i = 0usize;

    // magic
    if bytes.len() < i + 4 { return Err(ShareCodecError::Truncated); }
    if &bytes[i..i + 4] != MAGIC { return Err(ShareCodecError::BadMagic); }
    i += 4;

    // x
    if bytes.len() < i + 1 { return Err(ShareCodecError::Truncated); }
    let x = GF256(bytes[i]); i += 1;

    // ecc_len
    if bytes.len() < i + 1 { return Err(ShareCodecError::Truncated); }
    let ecc_len = bytes[i] as usize; i += 1;
    if ecc_len < 2 { return Err(ShareCodecError::Truncated); }

    // y_len (original, without parity)
    if bytes.len() < i + 4 { return Err(ShareCodecError::Truncated); }
    let y_len = u32::from_le_bytes(bytes[i..i + 4].try_into().unwrap()) as usize;
    i += 4;

    let code_len = y_len.checked_add(ecc_len).ok_or(ShareCodecError::Overflow)?;
    if bytes.len() < i + code_len { return Err(ShareCodecError::Truncated); }

    // Correct using RS
    let code = Zeroizing::new(bytes[i..i + code_len].to_vec());
    let dec = Decoder::new(ecc_len);
    let recovered = dec.correct(&code, None).map_err(|_| ShareCodecError::EccDecode)?;
    let y = recovered.data().to_vec();
    if y.len() != y_len { return Err(ShareCodecError::EccDecode); }
    i += code_len;

    if i != bytes.len() { return Err(ShareCodecError::Truncated); }
    Ok(Share { x, y })
}

#[cfg(test)]
mod ecc_tests {
    use super::*;
    use crate::{split, Share, GF256};

    #[test]
    fn shr2_roundtrip_and_correction() {
        let sh = Share { x: GF256(0x2A), y: b"hello, world!".to_vec() };
        let ecc = 16;
        let hex_str = share_to_hex(&sh, ecc);

        // Corrupt some bytes inside the stored codeword
        let mut bytes = hex::decode(&hex_str).unwrap();
        // flip a few bytes within the codeword region
        let base = 4 + 1 + 1 + 4; // header size for SHR2
        bytes[base + 0] ^= 0x55;
        bytes[base + 5] ^= 0xAA;
        let corrupted = hex::encode(bytes);

        // Should still decode to original y
        let decoded = share_from_hex(&corrupted).unwrap();
        assert_eq!(decoded, sh);
    }

    #[test]
    fn integrates_with_split_reconstruct() {
        let secret = b"supersecret";
        let shares = split(secret, 3, 5);

        // encode each share with ECC
        let blobs: Vec<String> = shares.iter().map(|s| share_to_hex(s, 16)).collect();

        // decode & recover the same shares
        let decoded: Vec<Share> = blobs.iter().map(|h| share_from_hex(h).unwrap()).collect();
        assert_eq!(shares, decoded);
    }
}