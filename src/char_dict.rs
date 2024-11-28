use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static CHAR_DICT: Lazy<HashMap<char, Vec<u8>>> = Lazy::new(|| {
    let mut dict = HashMap::new();
    
    // Numbers
    dict.insert('0', vec![0b00011111, 0b00010001, 0b00011111]);
    dict.insert('1', vec![0b00010001, 0b00011111, 0b00010000]);
    dict.insert('2', vec![0b00011001, 0b00010101, 0b00010011]);
    dict.insert('3', vec![0b00010001, 0b00010101, 0b00011111]);
    dict.insert('4', vec![0b00000111, 0b00000100, 0b00011111]);
    dict.insert('5', vec![0b00010111, 0b00010101, 0b00011101]);
    dict.insert('6', vec![0b00011111, 0b00010101, 0b00011101]);
    dict.insert('7', vec![0b00000001, 0b00000001, 0b00011111]);
    dict.insert('8', vec![0b00011111, 0b00010101, 0b00011111]);
    dict.insert('9', vec![0b00010111, 0b00010101, 0b00011111]);
    
    // Letters
    dict.insert('A', vec![0b00011110, 0b00000101, 0b00011110]);
    dict.insert('B', vec![0b00011111, 0b00010101, 0b00001010]);
    dict.insert('C', vec![0b00001110, 0b00010001, 0b00010001]);
    dict.insert('D', vec![0b00011111, 0b00010001, 0b00001110]);
    dict.insert('E', vec![0b00011111, 0b00010101, 0b00010101]);
    dict.insert('F', vec![0b00011111, 0b00000101, 0b00000101]);
    dict.insert('G', vec![0b00001110, 0b00010001, 0b00011101]);
    dict.insert('H', vec![0b00011111, 0b00000100, 0b00011111]);
    dict.insert('I', vec![0b00010001, 0b00011111, 0b00010001]);
    dict.insert('J', vec![0b00011000, 0b00010000, 0b00001111]);
    dict.insert('K', vec![0b00011111, 0b00000100, 0b00011011]);
    dict.insert('L', vec![0b00011111, 0b00010000, 0b00010000]);
    dict.insert('M', vec![0b00011111, 0b00000010, 0b00011111]);
    dict.insert('N', vec![0b00011111, 0b00000011, 0b00011100]);
    dict.insert('O', vec![0b00001110, 0b00010001, 0b00001110]);
    dict.insert('P', vec![0b00011111, 0b00000101, 0b00000010]);
    dict.insert('Q', vec![0b00001110, 0b00010001, 0b00011110]);
    dict.insert('R', vec![0b00011111, 0b00000101, 0b00011010]);
    dict.insert('S', vec![0b00010010, 0b00010101, 0b00001001]);
    dict.insert('T', vec![0b00000001, 0b00011111, 0b00000001]);
    dict.insert('U', vec![0b00001111, 0b00010000, 0b00001111]);
    dict.insert('V', vec![0b00001111, 0b00010000, 0b00001111]);
    dict.insert('W', vec![0b00011111, 0b00001000, 0b00011111]);
    dict.insert('X', vec![0b00011011, 0b00000100, 0b00011011]);
    dict.insert('Y', vec![0b00000011, 0b00011100, 0b00000011]);
    dict.insert('Z', vec![0b00011001, 0b00010101, 0b00010011]);
    
    // Special characters
    dict.insert(' ', vec![0b00000000]);
    dict.insert('-', vec![0b00000100, 0b00000100, 0b00000100]);
    dict.insert('_', vec![0b00010000, 0b00010000, 0b00010000]);
    dict.insert('=', vec![0b00001010, 0b00001010, 0b00001010]);
    dict.insert('+', vec![0b00000100, 0b00001110, 0b00000100]);
    dict.insert('*', vec![0b00001010, 0b00000100, 0b00001010]);
    dict.insert('/', vec![0b00011000, 0b00000110, 0b00000001]);
    dict.insert('\\', vec![0b00000001, 0b00000110, 0b00011000]);
    dict.insert('.', vec![0b00010000]);
    dict.insert(':', vec![0b00000000, 0b00001010, 0b00000000]);
    dict.insert('℃', vec![0b00000110, 0b00001001, 0b00001001, 0b00000110]);
    
    dict
});
