extern crate rust_salsa20;
use rust_salsa20::{Salsa20, Key::Key32};

#[test]
fn generate_test() {
    test(
        0x00000000,
        vec![
            45, 134, 38, 166, 142, 36, 28, 146, 116, 157, 199, 239, 167, 75,
            110, 228, 184, 111, 55, 94, 165, 254, 245, 124, 13, 124, 93, 67,
            28, 23, 220, 60, 220, 135, 104, 76, 242, 29, 224, 51, 108, 68, 10,
            72, 86, 153, 6, 81, 12, 53, 36, 233, 161, 16, 119, 206, 117, 194,
            51, 33, 206, 74, 252, 220, 9, 170, 180, 195, 208, 49, 70, 122, 178,
            70, 165, 115, 43, 206, 91, 44, 72, 98, 67, 71, 88, 25, 102, 107,
            249, 166, 152, 249, 13, 60, 182, 75, 75, 72, 46, 94, 237, 77, 3,
            35, 231, 214, 84, 141, 52, 69, 237, 154, 77, 178, 50, 111, 20, 221,
            203, 172, 238, 144, 202, 235, 58, 138, 75, 214, 96, 241, 225, 82,
            134, 122, 37, 83, 67, 32, 247, 60, 14, 243, 175, 12, 129, 94, 77,
            177, 56, 74, 54, 240, 57, 20, 168, 124, 84, 173, 167, 73, 221, 195,
            150, 0, 40, 87, 195, 211, 157, 162, 198, 225, 240, 240, 63, 131,
            224, 149, 157, 131, 182, 84, 68, 252, 93, 64, 3, 51, 241, 30, 6,
            211, 87, 141, 136, 92, 92, 90, 81, 174
        ]
    );

    test(
        0xffffffff,
        vec![
            204, 74, 84, 181, 96, 108, 197, 131, 29, 86, 219, 130, 198, 167,
            107, 85, 235, 63, 95, 201, 174, 187, 48, 32, 179, 5, 108, 40, 241,
            161, 240, 41, 134, 154, 124, 228, 226, 196, 21, 154, 185, 169, 116,
            3, 220, 52, 112, 176, 65, 200, 164, 21, 84, 191, 86, 142, 250, 32,
            208, 87, 8, 20, 109, 151, 187, 108, 153, 235, 58, 119, 253, 188,
            172, 214, 188, 58, 27, 201, 234, 7, 202, 56, 213, 116, 61, 106, 6,
            14, 188, 20, 119, 99, 62, 111, 15, 192, 248, 109, 189, 206, 96, 80,
            29, 27, 11, 80, 239, 32, 96, 198, 208, 61, 148, 19, 24, 81, 213,
            147, 78, 126, 57, 25, 182, 75, 70, 101, 86, 208, 142, 156, 94, 103,
            61, 15, 229, 102, 230, 98, 13, 108, 96, 134, 167, 127, 66, 33, 48,
            175, 16, 104, 136, 214, 51, 61, 175, 178, 98, 149, 97, 178, 142,
            52, 133, 189, 150, 180, 8, 245, 229, 183, 199, 34, 38, 245, 97, 41,
            240, 163, 136, 75, 128, 102, 235, 214, 135, 118, 15, 149, 13, 160,
            140, 180, 43, 124, 59, 63, 130, 95, 130, 115
        ]
    );

    fn test(counter: u64, expected_data: Vec<u8>) {
        let key = Key32([
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
            17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
        ]);
        let nonce = [1, 2, 3, 4, 5, 6, 7, 8];
        let mut salsa = Salsa20::new(key, nonce, counter);
        let mut buffer = vec![0; 200];

        salsa.generate(&mut buffer[..7]);
        salsa.generate(&mut buffer[7..13]);
        salsa.generate(&mut buffer[13..197]);
        salsa.generate(&mut buffer[197..200]);

        assert_eq!(buffer, expected_data);
    }
}
