use crc_any::CRCu16;

pub fn digest_crc<T: AsRef<[u8]>>(data: T) -> u16 {
    let mut digest = CRCu16::crc16xmodem();
    digest.digest(data.as_ref());
    digest.get_crc()
}

#[cfg(test)]
mod test {
    use crate::crc::digest_crc;

    #[test]
    fn test_digest_crc() {
        assert_eq!(digest_crc(b"QMOD"), 0x49C1);
        assert_eq!(digest_crc(b"QPIGS"), 0xB7A9);
        assert_eq!(digest_crc(b"QPIRI"), 0xF854);
        assert_eq!(digest_crc(b"QPIWS"), 0xB4DA);
    }
}