// no implementation necessary

#[cfg(test)]
mod tests {
    use openssl;

    #[test]
    fn provided_example() {
        let ciphertext = include_str!("data/challenge7.txt");
        let decoded = openssl::base64::decode_block(ciphertext).unwrap();

        let key = b"YELLOW SUBMARINE";

        let cipher = openssl::symm::Cipher::aes_128_ecb();
        let plaintext = openssl::symm::decrypt(cipher, key, None, &decoded).unwrap();

        let expected = include_str!("data/challenge7_solution.txt");
        assert_eq!(openssl::base64::encode_block(&plaintext), expected);
    }
}
