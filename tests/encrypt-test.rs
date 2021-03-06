extern crate rust_salsa20;
use rust_salsa20::{Salsa20, Key::Key32};

#[test]
fn encrypt_test() {
    test(
        0x00000000,
        vec![
            42, 129, 33, 161, 137, 35, 27, 149, 115, 154, 192, 232, 160, 76,
            105, 227, 191, 104, 48, 89, 162, 249, 242, 123, 10, 123, 90, 68,
            27, 16, 219, 59, 219, 128, 111, 75, 245, 26, 231, 52, 107, 67, 13,
            79, 81, 158, 1, 86, 11, 50, 35, 238, 166, 23, 112, 201, 114, 197,
            52, 38, 201, 77, 251, 219, 14, 173, 179, 196, 215, 54, 65, 125,
            181, 65, 162, 116, 44, 201, 92, 43, 79, 101, 68, 64, 95, 30, 97,
            108, 254, 161, 159, 254, 10, 59, 177, 76, 76, 79, 41, 89, 234, 74,
            4, 36, 224, 209, 83, 138, 51, 66, 234, 157, 74, 181, 53, 104, 19,
            218, 204, 171, 233, 151, 205, 236, 61, 141, 76, 209, 103, 246, 230,
            85, 129, 125, 34, 84, 68, 39, 240, 59, 9, 244, 168, 11, 134, 89,
            74, 182, 63, 77, 49, 247, 62, 19, 175, 123, 83, 170, 160, 78, 218,
            196, 145, 7, 47, 80, 196, 212, 154, 165, 193, 230, 247, 247, 56,
            132, 231, 146, 154, 132, 177, 83, 67, 251, 90, 71, 4, 52, 246, 25,
            1, 212, 80, 138, 143, 91, 91, 93, 86, 169
        ]
    );

    test(
        0xffffffff,
        vec![
            203, 77, 83, 178, 103, 107, 194, 132, 26, 81, 220, 133, 193, 160,
            108, 82, 236, 56, 88, 206, 169, 188, 55, 39, 180, 2, 107, 47, 246,
            166, 247, 46, 129, 157, 123, 227, 229, 195, 18, 157, 190, 174, 115,
            4, 219, 51, 119, 183, 70, 207, 163, 18, 83, 184, 81, 137, 253, 39,
            215, 80, 15, 19, 106, 144, 188, 107, 158, 236, 61, 112, 250, 187,
            171, 209, 187, 61, 28, 206, 237, 0, 205, 63, 210, 115, 58, 109, 1,
            9, 187, 19, 112, 100, 57, 104, 8, 199, 255, 106, 186, 201, 103, 87,
            26, 28, 12, 87, 232, 39, 103, 193, 215, 58, 147, 20, 31, 86, 210,
            148, 73, 121, 62, 30, 177, 76, 65, 98, 81, 215, 137, 155, 89, 96,
            58, 8, 226, 97, 225, 101, 10, 107, 103, 129, 160, 120, 69, 38, 55,
            168, 23, 111, 143, 209, 52, 58, 168, 181, 101, 146, 102, 181, 137,
            51, 130, 186, 145, 179, 15, 242, 226, 176, 192, 37, 33, 242, 102,
            46, 247, 164, 143, 76, 135, 97, 236, 209, 128, 113, 8, 146, 10,
            167, 139, 179, 44, 123, 60, 56, 133, 88, 133, 116
        ]
    );

    fn test(counter: u64, expected_data: Vec<u8>) {
        let key = Key32([
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
            17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
        ]);
        let nonce = [1, 2, 3, 4, 5, 6, 7, 8];
        let mut salsa = Salsa20::new(key, nonce, counter);
        let mut buffer = vec![7; 200];

        salsa.encrypt(&mut buffer[..7]);
        salsa.encrypt(&mut buffer[7..13]);
        salsa.encrypt(&mut buffer[13..197]);
        salsa.encrypt(&mut buffer[197..200]);

        assert_eq!(buffer, expected_data);
    }
}
