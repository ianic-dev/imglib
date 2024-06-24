#[cfg(test)]
mod tests {
    use crate::packbits;

    #[test]
    fn testbitpack() {
        let bytes: Vec<u8> = vec![1, 1, 1, 1, 1, 1, 1, 1];
        let byte: u8 = 255;
        assert_eq!(packbits(&bytes[..]), byte);

        let bytes: Vec<u8> = vec![1, 0, 1, 0, 1, 0, 1, 0];
        let byte: u8 = 170;
        assert_eq!(packbits(&bytes[..]), byte);

        let bytes: Vec<u8> = vec![1, 1, 1, 1, 1, 0, 0, 1];
        let byte: u8 = 249;
        assert_eq!(packbits(&bytes[..]), byte);

        let bytes: Vec<u8> = vec![1, 0, 0, 0, 0, 0, 0, 1];
        let byte: u8 = 129;
        assert_eq!(packbits(&bytes[..]), byte);

        let bytes: Vec<u8> = vec![1, 1, 1, 1, 1];
        let byte: u8 = 248;
        assert_eq!(packbits(&bytes[..]), byte);
    }
}
