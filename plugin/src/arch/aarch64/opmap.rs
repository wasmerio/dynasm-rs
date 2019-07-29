// This file was generated bo tools/aarch64_gen_opmap.py
Ops!(map ;

"abs" = [
    0b01011110_11100000_10111000_00000000 = [D, D] => [R(0), R(5)];
    0b00001110_00100000_10111000_00000000 = [VSized(BYTE), VSized(BYTE)] => [R(0), R(5), Rwidth(30)];
    0b00001110_01100000_10111000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_10100000_10111000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_11100000_10111000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
]
"adc" = [
    0b00011010_00000000_00000000_00000000 = [W, W, W] => [R(0), R(5), R(16)];
    0b10011010_00000000_00000000_00000000 = [X, X, X] => [R(0), R(5), R(16)];
]
"adcs" = [
    0b00111010_00000000_00000000_00000000 = [W, W, W] => [R(0), R(5), R(16)];
    0b10111010_00000000_00000000_00000000 = [X, X, X] => [R(0), R(5), R(16)];
]
"add" = [
    // ADD (shifted register)
    0b00001011_00000000_00000000_00000000 = [W, W, W, End, Mod(SHIFTS)] => [R(0), R(5), R(16), Rotates(22), Ubits(10, 5)];
    0b10001011_00000000_00000000_00000000 = [X, X, X, End, Mod(SHIFTS)] => [R(0), R(5), R(16), Rotates(22), Ubits(10, 6)];
    // ADD (extended register)
    0b00001011_00100000_00000000_00000000 = [WSP, WSP, W, End, Mod(EXTENDS)] => [R(0), R(5), R(16), ExtendsX(13), Urange(10, 0, 4)];
    0b10001011_00100000_00000000_00000000 = [XSP, XSP, W, End, Mod(EXTENDS_W)] => [R(0), R(5), R(16), ExtendsX(13), Urange(10, 0, 4)];
    0b10001011_00100000_00000000_00000000 = [XSP, XSP, X, End, Mod(EXTENDS_X)] => [R(0), R(5), R(16), ExtendsX(13), Urange(10, 0, 4)];
    // ADD (immediate)
    0b00010001_00000000_00000000_00000000 = [WSP, WSP, Imm, End, Mod(&[LSL])] => [R(0), R(5), Ubits(10, 12), A, Ulist(22, &[0, 12])];
    0b10010001_00000000_00000000_00000000 = [XSP, XSP, Imm, End, Mod(&[LSL])] => [R(0), R(5), Ubits(10, 12), A, Ulist(22, &[0, 12])];
    // ADD (vector)
    0b01011110_11100000_10000100_00000000 = [D, D, D] => [R(0), R(5), R(16)];
    0b00001110_00100000_10000100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01100000_10000100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10100000_10000100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_11100000_10000100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"addhn" = [
    0b00001110_00100000_01000000_00000000 = [VSizedStatic(BYTE, 8), VSizedStatic(WORD, 8), VSizedStatic(WORD, 8)] => [R(0), R(5), R(16)];
    0b00001110_01100000_01000000_00000000 = [VSizedStatic(WORD, 4), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
    0b00001110_10100000_01000000_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16)];
]
"addhn2" = [
    0b01001110_00100000_01000000_00000000 = [VSizedStatic(BYTE, 16), VSizedStatic(WORD, 8), VSizedStatic(WORD, 8)] => [R(0), R(5), R(16)];
    0b01001110_01100000_01000000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
    0b01001110_10100000_01000000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16)];
]
"addp" = [
    // ADDP (scalar)
    0b01011110_11110001_10111000_00000000 = [D, VSizedStatic(QWORD, 2)] => [R(0), R(5)];
    // ADDP (vector)
    0b00001110_00100000_10111100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01100000_10111100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10100000_10111100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_11100000_10111100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"adds" = [
    // ADDS (shifted register)
    0b00101011_00000000_00000000_00000000 = [W, W, W, End, Mod(SHIFTS)] => [R(0), R(5), R(16), Rotates(22), Ubits(10, 5)];
    0b10101011_00000000_00000000_00000000 = [X, X, X, End, Mod(SHIFTS)] => [R(0), R(5), R(16), Rotates(22), Ubits(10, 6)];
    // ADDS (extended register)
    0b00101011_00100000_00000000_00000000 = [W, WSP, W, End, Mod(EXTENDS)] => [R(0), R(5), R(16), ExtendsW(13), Urange(10, 0, 4)];
    0b10101011_00100000_00000000_00000000 = [X, XSP, W, End, Mod(EXTENDS_W)] => [R(0), R(5), R(16), ExtendsX(13), Urange(10, 0, 4)];
    0b10101011_00100000_00000000_00000000 = [X, XSP, X, End, Mod(EXTENDS_X)] => [R(0), R(5), R(16), ExtendsX(13), Urange(10, 0, 4)];
    // ADDS (immediate)
    0b00110001_00000000_00000000_00000000 = [W, WSP, Imm, End, Mod(&[LSL])] => [R(0), R(5), Ubits(10, 12), A, Ulist(22, &[0, 12])];
    0b10110001_00000000_00000000_00000000 = [X, XSP, Imm, End, Mod(&[LSL])] => [R(0), R(5), Ubits(10, 12), A, Ulist(22, &[0, 12])];
]
"addv" = [
    0b00001110_00110001_10111000_00000000 = [B, VSized(BYTE)] => [R(0), R(5), Rwidth(30)];
    0b00001110_01110001_10111000_00000000 = [H, VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_10110001_10111000_00000000 = [S, VSizedStatic(DWORD, 4)] => [R(0), R(5), Rwidth(30)];
]
"adr" = [
    0b00010000_00000000_00000000_00000000 = [X, Offset] => [R(0), BSbits(21), BSslice(29, 2, 0), BSslice(5, 19, 2), A];
]
"adrp" = [
    0b10010000_00000000_00000000_00000000 = [X, Offset] => [R(0), BSscaled(21, 12), BSslice(29, 2, 12), BSslice(5, 19, 14), A];
]
"aesd" = [
    0b01001110_00101000_01011000_00000000 = [VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16)] => [R(0), R(5)];
]
"aese" = [
    0b01001110_00101000_01001000_00000000 = [VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16)] => [R(0), R(5)];
]
"aesimc" = [
    0b01001110_00101000_01111000_00000000 = [VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16)] => [R(0), R(5)];
]
"aesmc" = [
    0b01001110_00101000_01101000_00000000 = [VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16)] => [R(0), R(5)];
]
"and" = [
    // AND (vector)
    0b00001110_00100000_00011100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    // AND (immediate)
    0b00010010_00000000_00000000_00000000 = [WSP, W, Imm] => [R(0), R(5), UlogicalW(10)];
    0b10010010_00000000_00000000_00000000 = [XSP, X, Imm] => [R(0), R(5), UlogicalX(10)];
    // AND (shifted register)
    0b00001010_00000000_00000000_00000000 = [W, W, W, End, Mod(ROTATES)] => [R(0), R(5), R(16), Rotates(22), Ubits(10, 5)];
    0b10001010_00000000_00000000_00000000 = [X, X, X, End, Mod(ROTATES)] => [R(0), R(5), R(16), Rotates(22), Ubits(10, 6)];
]
"ands" = [
    // ANDS (immediate)
    0b01110010_00000000_00000000_00000000 = [W, W, Imm] => [R(0), R(5), UlogicalW(5)];
    0b11110010_00000000_00000000_00000000 = [X, X, Imm] => [R(0), R(5), UlogicalX(5)];
    // ANDS (shifted register)
    0b01101010_00000000_00000000_00000000 = [W, W, W, End, Mod(ROTATES)] => [R(0), R(5), R(16), Rotates(22), Ubits(10, 5)];
    0b11101010_00000000_00000000_00000000 = [X, X, X, End, Mod(ROTATES)] => [R(0), R(5), R(16), Rotates(22), Ubits(10, 6)];
]
"asr" = [
    // ASR (register)
    0b00011010_11000000_00101000_00000000 = [W, W, W] => [R(0), R(5), R(16)];
    0b10011010_11000000_00101000_00000000 = [X, X, X] => [R(0), R(5), R(16)];
    // ASR (immediate)
    0b00010011_00000000_01111100_00000000 = [W, W, Imm] => [R(0), R(5), Ubits(16, 5)];
    0b10010011_01000000_11111100_00000000 = [X, X, Imm] => [R(0), R(5), Ubits(16, 6)];
]
"asrv" = [
    0b00011010_11000000_00101000_00000000 = [W, W, W] => [R(0), R(5), R(16)];
    0b10011010_11000000_00101000_00000000 = [X, X, X] => [R(0), R(5), R(16)];
]
"at" = [
    0b11010101_00001000_01111000_00000000 = [Ident, X] => [LitList(5, "AT_OPS"), R(0)];
]
"autda" = [
    0b11011010_11000001_00011000_00000000 = [X, XSP] => [R(0), R(5)];
]
"autdb" = [
    0b11011010_11000001_00011100_00000000 = [X, XSP] => [R(0), R(5)];
]
"autdza" = [
    0b11011010_11000001_00111011_11100000 = [X] => [R(0)];
]
"autdzb" = [
    0b11011010_11000001_00111111_11100000 = [X] => [R(0)];
]
"autia" = [
    0b11011010_11000001_00010000_00000000 = [X, XSP] => [R(0), R(5)];
]
"autia1716" = [
    0b11010101_00000011_00100001_10011111 = [] => [];
]
"autiasp" = [
    0b11010101_00000011_00100011_10111111 = [] => [];
]
"autiaz" = [
    0b11010101_00000011_00100011_10011111 = [] => [];
]
"autib" = [
    0b11011010_11000001_00010100_00000000 = [X, XSP] => [R(0), R(5)];
]
"autib1716" = [
    0b11010101_00000011_00100001_11011111 = [] => [];
]
"autibsp" = [
    0b11010101_00000011_00100011_11111111 = [] => [];
]
"autibz" = [
    0b11010101_00000011_00100011_11011111 = [] => [];
]
"autiza" = [
    0b11011010_11000001_00110011_11100000 = [X] => [R(0)];
]
"autizb" = [
    0b11011010_11000001_00110111_11100000 = [X] => [R(0)];
]
"b" = [
    // B.cond
    0b01010100_00000000_00000000_00000000 = [Dot, Cond, Offset] => [Cond(0), Sscaled(5, 19, 2)];
    // B
    0b00010100_00000000_00000000_00000000 = [Offset] => [Sscaled(0, 26, 2)];
]
"bcax" = [
    0b11001110_00100000_00000000_00000000 = [VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16)] => [R(0), R(5), R(16), R(10)];
]
"bfc" = [
    0b00110011_00000000_00000011_11100000 = [W, Imm, Imm] => [R(0), Usub(16, 5, 32), Urange(10, 1, 32)];
    0b10110011_01000000_00000011_11100000 = [X, Imm, Imm] => [R(0), Usub(16, 6, 64), Urange(10, 1, 64)];
]
"bfi" = [
    0b00110011_00000000_00000000_00000000 = [W, W, Imm, Imm] => [R(0), R(5), Usub(16, 5, 32), Urange(10, 1, 32)];
    0b10110011_01000000_00000000_00000000 = [X, X, Imm, Imm] => [R(0), R(5), Usub(16, 6, 64), Urange(10, 1, 64)];
]
"bfm" = [
    0b00110011_00000000_00000000_00000000 = [W, W, Imm, Imm] => [R(0), R(5), Ubits(16, 5), Ubits(10, 5)];
    0b10110011_01000000_00000000_00000000 = [X, X, Imm, Imm] => [R(0), R(5), Ubits(16, 6), Ubits(10, 6)];
]
"bfxil" = [
    0b00110011_00000000_00000000_00000000 = [W, W, Imm, Imm] => [R(0), R(5), Ubits(16, 5), Usumdec(10, 5)];
    0b10110011_01000000_00000000_00000000 = [X, X, Imm, Imm] => [R(0), R(5), Ubits(16, 6), Usumdec(10, 6)];
]
"bic" = [
    // BIC (vector, immediate)
    0b00101111_00000000_10010100_00000000 = [VSized(WORD), Imm, End, Mod(&[LSL])] => [R(0), BUbits(8), BUslice(5, 5, 0), BUslice(16, 3, 5), A, A, Ulist(13, &[0, 8])];
    0b00101111_00000000_00010100_00000000 = [VSized(DWORD), Imm, End, Mod(&[LSL])] => [R(0), BUbits(8), BUslice(5, 5, 0), BUslice(16, 3, 5), A, A, Ulist(13, &[0, 8, 16, 24])];
    // BIC (vector, register)
    0b00001110_01100000_00011100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    // BIC (shifted register)
    0b00001010_00100000_00000000_00000000 = [W, W, W, End, Mod(ROTATES)] => [R(0), R(5), R(16), Rotates(22), Ubits(10, 5)];
    0b10001010_00100000_00000000_00000000 = [X, X, X, End, Mod(ROTATES)] => [R(0), R(5), R(16), Rotates(22), Ubits(10, 6)];
]
"bics" = [
    0b01101010_00100000_00000000_00000000 = [W, W, W, End, Mod(ROTATES)] => [R(0), R(5), R(16), Rotates(22), Ubits(10, 5)];
    0b11101010_00100000_00000000_00000000 = [X, X, X, End, Mod(ROTATES)] => [R(0), R(5), R(16), Rotates(22), Ubits(10, 6)];
]
"bif" = [
    0b00101110_11100000_00011100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
]
"bit" = [
    0b00101110_10100000_00011100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
]
"bl" = [
    0b10010100_00000000_00000000_00000000 = [Offset] => [Sscaled(0, 26, 2)];
]
"blr" = [
    0b11010110_00111111_00000000_00000000 = [X] => [R(5)];
]
"blraa" = [
    0b11010111_00111111_00001000_00000000 = [X, XSP] => [R(5), R(0)];
]
"blraaz" = [
    0b11010110_00111111_00001000_00011111 = [X] => [R(5)];
]
"blrab" = [
    0b11010111_00111111_00001100_00000000 = [X, XSP] => [R(5), R(0)];
]
"blrabz" = [
    0b11010110_00111111_00001100_00011111 = [X] => [R(5)];
]
"br" = [
    0b11010110_00011111_00000000_00000000 = [X] => [R(5)];
]
"braa" = [
    0b11010111_00011111_00001000_00000000 = [X, XSP] => [R(5), R(0)];
]
"braaz" = [
    0b11010110_00011111_00001000_00011111 = [X] => [R(5)];
]
"brab" = [
    0b11010111_00011111_00001100_00000000 = [X, XSP] => [R(5), R(0)];
]
"brabz" = [
    0b11010110_00011111_00001100_00011111 = [X] => [R(5)];
]
"brk" = [
    0b11010100_00100000_00000000_00000000 = [Imm] => [Ubits(5, 16)];
]
"bsl" = [
    0b00101110_01100000_00011100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
]
"cas" = [
    0b10001000_10100000_01111100_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11001000_10100000_01111100_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"casa" = [
    0b10001000_11100000_01111100_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11001000_11100000_01111100_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"casab" = [
    0b00001000_11100000_01111100_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"casah" = [
    0b01001000_11100000_01111100_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"casal" = [
    0b10001000_11100000_11111100_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11001000_11100000_11111100_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"casalb" = [
    0b00001000_11100000_11111100_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"casalh" = [
    0b01001000_11100000_11111100_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"casb" = [
    0b00001000_10100000_01111100_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"cash" = [
    0b01001000_10100000_01111100_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"casl" = [
    0b10001000_10100000_11111100_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11001000_10100000_11111100_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"caslb" = [
    0b00001000_10100000_11111100_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"caslh" = [
    0b01001000_10100000_11111100_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"casp" = [
    0b00001000_00100000_01111100_00000000 = [W, W, W, W, RefBase] => [R(16), Special(0, "previous reg incremented"), R(0), Special(0, "previous reg incremented"), R(5)];
    0b01001000_00100000_01111100_00000000 = [X, X, X, X, RefBase] => [R(16), Special(0, "previous reg incremented"), R(0), Special(0, "previous reg incremented"), R(5)];
]
"caspa" = [
    0b00001000_01100000_01111100_00000000 = [W, W, W, W, RefBase] => [R(16), Special(0, "previous reg incremented"), R(0), Special(0, "previous reg incremented"), R(5)];
    0b01001000_01100000_01111100_00000000 = [X, X, X, X, RefBase] => [R(16), Special(0, "previous reg incremented"), R(0), Special(0, "previous reg incremented"), R(5)];
]
"caspal" = [
    0b00001000_01100000_11111100_00000000 = [W, W, W, W, RefBase] => [R(16), Special(0, "previous reg incremented"), R(0), Special(0, "previous reg incremented"), R(5)];
    0b01001000_01100000_11111100_00000000 = [X, X, X, X, RefBase] => [R(16), Special(0, "previous reg incremented"), R(0), Special(0, "previous reg incremented"), R(5)];
]
"caspl" = [
    0b00001000_00100000_11111100_00000000 = [W, W, W, W, RefBase] => [R(16), Special(0, "previous reg incremented"), R(0), Special(0, "previous reg incremented"), R(5)];
    0b01001000_00100000_11111100_00000000 = [X, X, X, X, RefBase] => [R(16), Special(0, "previous reg incremented"), R(0), Special(0, "previous reg incremented"), R(5)];
]
"cbnz" = [
    0b00110101_00000000_00000000_00000000 = [W, Offset] => [R(0), Sscaled(5, 19, 2)];
    0b10110101_00000000_00000000_00000000 = [X, Offset] => [R(0), Sscaled(5, 19, 2)];
]
"cbz" = [
    0b00110100_00000000_00000000_00000000 = [W, Offset] => [R(0), Sscaled(5, 19, 2)];
    0b10110100_00000000_00000000_00000000 = [X, Offset] => [R(0), Sscaled(5, 19, 2)];
]
"ccmn" = [
    // CCMN (immediate)
    0b00111010_01000000_00001000_00000000 = [W, Imm, Imm, Cond] => [R(5), Ubits(16, 5), Ubits(0, 4), Cond(12)];
    0b10111010_01000000_00001000_00000000 = [X, Imm, Imm, Cond] => [R(5), Ubits(16, 5), Ubits(0, 4), Cond(12)];
    // CCMN (register)
    0b00111010_01000000_00000000_00000000 = [W, W, Imm, Cond] => [R(5), R(16), Ubits(0, 4), Cond(12)];
    0b10111010_01000000_00000000_00000000 = [X, X, Imm, Cond] => [R(5), R(16), Ubits(0, 4), Cond(12)];
]
"ccmp" = [
    // CCMP (immediate)
    0b01111010_01000000_00001000_00000000 = [W, Imm, Imm, Cond] => [R(5), Ubits(16, 5), Ubits(0, 4), Cond(12)];
    0b11111010_01000000_00001000_00000000 = [X, Imm, Imm, Cond] => [R(5), Ubits(16, 5), Ubits(0, 4), Cond(12)];
    // CCMP (register)
    0b01111010_01000000_00000000_00000000 = [W, W, Imm, Cond] => [R(5), R(16), Ubits(0, 4), Cond(12)];
    0b11111010_01000000_00000000_00000000 = [X, X, Imm, Cond] => [R(5), R(16), Ubits(0, 4), Cond(12)];
]
"cfinv" = [
    0b11010101_00000000_01000000_00011111 = [] => [];
]
"cfp" = [
    0b11010101_00001011_01110011_10000000 = [Lit("RCTX"), X] => [R(0)];
]
"cinc" = [
    0b00011010_10000000_00000100_00000000 = [W, W, Cond] => [R(0), R(5), C, R(16), CondInv(12)];
    0b10011010_10000000_00000100_00000000 = [X, X, Cond] => [R(0), R(5), C, R(16), CondInv(12)];
]
"cinv" = [
    0b01011010_10000000_00000000_00000000 = [W, W, Cond] => [R(0), R(5), C, R(16), CondInv(12)];
    0b11011010_10000000_00000000_00000000 = [X, X, Cond] => [R(0), R(5), C, R(16), CondInv(12)];
]
"clrex" = [
    0b11010101_00000011_00110000_01011111 = [End, Imm] => [Ubits(8, 4)];
]
"cls" = [
    // CLS (vector)
    0b00001110_00100000_01001000_00000000 = [VSized(BYTE), VSized(BYTE)] => [R(0), R(5), Rwidth(30)];
    0b00001110_01100000_01001000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_10100000_01001000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    // CLS
    0b01011010_11000000_00010100_00000000 = [W, W] => [R(0), R(5)];
    0b11011010_11000000_00010100_00000000 = [X, X] => [R(0), R(5)];
]
"clz" = [
    // CLZ (vector)
    0b00101110_00100000_01001000_00000000 = [VSized(BYTE), VSized(BYTE)] => [R(0), R(5), Rwidth(30)];
    0b00101110_01100000_01001000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_10100000_01001000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    // CLZ
    0b01011010_11000000_00010000_00000000 = [W, W] => [R(0), R(5)];
    0b11011010_11000000_00010000_00000000 = [X, X] => [R(0), R(5)];
]
"cmeq" = [
    // CMEQ (register)
    0b01111110_11100000_10001100_00000000 = [D, D, D] => [R(0), R(5), R(16)];
    0b00101110_00100000_10001100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_01100000_10001100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_10100000_10001100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_11100000_10001100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
    // CMEQ (zero)
    0b01011110_11100000_10011000_00000000 = [D, D, LitInt(0)] => [R(0), R(5)];
    0b00001110_00100000_10011000_00000000 = [VSized(BYTE), VSized(BYTE), LitInt(0)] => [R(0), R(5), Rwidth(30)];
    0b00001110_01100000_10011000_00000000 = [VSized(WORD), VSized(WORD), LitInt(0)] => [R(0), R(5), Rwidth(30)];
    0b00001110_10100000_10011000_00000000 = [VSized(DWORD), VSized(DWORD), LitInt(0)] => [R(0), R(5), Rwidth(30)];
    0b00001110_11100000_10011000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), LitInt(0)] => [R(0), R(5), Rwidth(30)];
]
"cmge" = [
    // CMGE (register)
    0b01011110_11100000_00111100_00000000 = [D, D, D] => [R(0), R(5), R(16)];
    0b00001110_00100000_00111100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01100000_00111100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10100000_00111100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_11100000_00111100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
    // CMGE (zero)
    0b01111110_11100000_10001000_00000000 = [D, D, LitInt(0)] => [R(0), R(5)];
    0b00101110_00100000_10001000_00000000 = [VSized(BYTE), VSized(BYTE), LitInt(0)] => [R(0), R(5), Rwidth(30)];
    0b00101110_01100000_10001000_00000000 = [VSized(WORD), VSized(WORD), LitInt(0)] => [R(0), R(5), Rwidth(30)];
    0b00101110_10100000_10001000_00000000 = [VSized(DWORD), VSized(DWORD), LitInt(0)] => [R(0), R(5), Rwidth(30)];
    0b00101110_11100000_10001000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), LitInt(0)] => [R(0), R(5), Rwidth(30)];
]
"cmgt" = [
    // CMGT (register)
    0b01011110_11100000_00110100_00000000 = [D, D, D] => [R(0), R(5), R(16)];
    0b00001110_00100000_00110100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01100000_00110100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10100000_00110100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_11100000_00110100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
    // CMGT (zero)
    0b01011110_11100000_10001000_00000000 = [D, D, LitInt(0)] => [R(0), R(5)];
    0b00001110_00100000_10001000_00000000 = [VSized(BYTE), VSized(BYTE), LitInt(0)] => [R(0), R(5), Rwidth(30)];
    0b00001110_01100000_10001000_00000000 = [VSized(WORD), VSized(WORD), LitInt(0)] => [R(0), R(5), Rwidth(30)];
    0b00001110_10100000_10001000_00000000 = [VSized(DWORD), VSized(DWORD), LitInt(0)] => [R(0), R(5), Rwidth(30)];
    0b00001110_11100000_10001000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), LitInt(0)] => [R(0), R(5), Rwidth(30)];
]
"cmhi" = [
    0b01111110_11100000_00110100_00000000 = [D, D, D] => [R(0), R(5), R(16)];
    0b00101110_00100000_00110100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_01100000_00110100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_10100000_00110100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_11100000_00110100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"cmhs" = [
    0b01111110_11100000_00111100_00000000 = [D, D, D] => [R(0), R(5), R(16)];
    0b00101110_00100000_00111100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_01100000_00111100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_10100000_00111100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_11100000_00111100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"cmle" = [
    0b01111110_11100000_10011000_00000000 = [D, D, LitInt(0)] => [R(0), R(5)];
    0b00101110_00100000_10011000_00000000 = [VSized(BYTE), VSized(BYTE), LitInt(0)] => [R(0), R(5), Rwidth(30)];
    0b00101110_01100000_10011000_00000000 = [VSized(WORD), VSized(WORD), LitInt(0)] => [R(0), R(5), Rwidth(30)];
    0b00101110_10100000_10011000_00000000 = [VSized(DWORD), VSized(DWORD), LitInt(0)] => [R(0), R(5), Rwidth(30)];
    0b00101110_11100000_10011000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), LitInt(0)] => [R(0), R(5), Rwidth(30)];
]
"cmlt" = [
    0b01011110_11100000_10101000_00000000 = [D, D, LitInt(0)] => [R(0), R(5)];
    0b00001110_00100000_10101000_00000000 = [VSized(BYTE), VSized(BYTE), LitInt(0)] => [R(0), R(5), Rwidth(30)];
    0b00001110_01100000_10101000_00000000 = [VSized(WORD), VSized(WORD), LitInt(0)] => [R(0), R(5), Rwidth(30)];
    0b00001110_10100000_10101000_00000000 = [VSized(DWORD), VSized(DWORD), LitInt(0)] => [R(0), R(5), Rwidth(30)];
    0b00001110_11100000_10101000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), LitInt(0)] => [R(0), R(5), Rwidth(30)];
]
"cmn" = [
    // CMN (shifted register)
    0b00101011_00000000_00000000_00011111 = [W, W, End, Mod(SHIFTS)] => [R(5), R(16), Rotates(22), Ubits(10, 5)];
    0b10101011_00000000_00000000_00011111 = [X, X, End, Mod(SHIFTS)] => [R(5), R(16), Rotates(22), Ubits(10, 6)];
    // CMN (extended register)
    0b00101011_00100000_00000000_00011111 = [WSP, W, End, Mod(EXTENDS)] => [R(5), R(16), ExtendsX(13), Urange(10, 0, 4)];
    0b10101011_00100000_00000000_00011111 = [XSP, W, End, Mod(EXTENDS_W)] => [R(0), R(5), R(16), ExtendsW(13), Urange(10, 0, 4)];
    0b10101011_00100000_00000000_00011111 = [XSP, X, End, Mod(EXTENDS_X)] => [R(0), R(5), R(16), ExtendsX(13), Urange(10, 0, 4)];
    // CMN (immediate)
    0b00110001_00000000_00000000_00011111 = [WSP, Imm, End, Mod(&[LSL]), Imm] => [R(5), Ubits(10, 12), A, Ulist(22, &[0, 12])];
    0b10110001_00000000_00000000_00011111 = [XSP, Imm, End, Mod(&[LSL])] => [R(5), Ubits(10, 12), A, Ulist(22, &[0, 12])];
]
"cmp" = [
    // CMP (shifted register)
    0b01101011_00000000_00000000_00011111 = [W, W, End, Mod(SHIFTS)] => [R(5), R(16), Rotates(22), Ubits(10, 5)];
    0b11101011_00000000_00000000_00011111 = [X, X, End, Mod(SHIFTS)] => [R(5), R(16), Rotates(22), Ubits(10, 6)];
    // CMP (extended register)
    0b01101011_00100000_00000000_00011111 = [WSP, W, End, Mod(EXTENDS)] => [R(5), R(16), ExtendsX(13), Urange(10, 0, 4)];
    0b11101011_00100000_00000000_00011111 = [XSP, W, End, Mod(EXTENDS_W)] => [R(0), R(5), R(16), ExtendsW(13), Urange(10, 0, 4)];
    0b11101011_00100000_00000000_00011111 = [XSP, X, End, Mod(EXTENDS_X)] => [R(0), R(5), R(16), ExtendsX(13), Urange(10, 0, 4)];
    // CMP (immediate)
    0b01110001_00000000_00000000_00011111 = [WSP, Imm, End, Mod(&[LSL]), Imm] => [R(5), Ubits(10, 12), A, Ulist(22, &[0, 12])];
    0b11110001_00000000_00000000_00011111 = [XSP, Imm, End, Mod(&[LSL])] => [R(5), Ubits(10, 12), A, Ulist(22, &[0, 12])];
]
"cmtst" = [
    0b01011110_11100000_10001100_00000000 = [D, D, D] => [R(0), R(5), R(16)];
    0b00001110_00100000_10001100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01100000_10001100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10100000_10001100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_11100000_10001100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"cneg" = [
    0b01011010_10000000_00000100_00000000 = [W, W, Cond] => [R(0), R(5), C, R(16), CondInv(12)];
    0b11011010_10000000_00000100_00000000 = [X, X, Cond] => [R(0), R(5), C, R(16), CondInv(12)];
]
"cnt" = [
    0b00001110_00100000_01011000_00000000 = [VSized(BYTE), VSized(BYTE)] => [R(0), R(5), Rwidth(30)];
]
"cpp" = [
    0b11010101_00001011_01110011_11100000 = [Lit("RCTX"), X] => [R(0)];
]
"crc32b" = [
    0b00011010_11000000_01000000_00000000 = [W, W, W] => [R(0), R(5), R(16)];
]
"crc32cb" = [
    0b00011010_11000000_01010000_00000000 = [W, W, W] => [R(0), R(5), R(16)];
]
"crc32ch" = [
    0b00011010_11000000_01010100_00000000 = [W, W, W] => [R(0), R(5), R(16)];
]
"crc32cw" = [
    0b00011010_11000000_01011000_00000000 = [W, W, W] => [R(0), R(5), R(16)];
]
"crc32cx" = [
    0b10011010_11000000_01011100_00000000 = [W, W, X] => [R(0), R(5), R(16)];
]
"crc32h" = [
    0b00011010_11000000_01000100_00000000 = [W, W, W] => [R(0), R(5), R(16)];
]
"crc32w" = [
    0b00011010_11000000_01001000_00000000 = [W, W, W] => [R(0), R(5), R(16)];
]
"crc32x" = [
    0b10011010_11000000_01001100_00000000 = [W, W, X] => [R(0), R(5), R(16)];
]
"csdb" = [
    0b11010101_00000011_00100010_10011111 = [] => [];
]
"csel" = [
    0b00011010_10000000_00000000_00000000 = [W, W, W, Cond] => [R(0), R(5), R(16), Cond(12)];
    0b10011010_10000000_00000000_00000000 = [X, X, X, Cond] => [R(0), R(5), R(16), Cond(12)];
]
"cset" = [
    0b00011010_10011111_00000111_11100000 = [W, Cond] => [R(0), Cond(12)];
    0b10011010_10011111_00000111_11100000 = [X, Cond] => [R(0), Cond(12)];
]
"csetm" = [
    0b01011010_10011111_00000011_11100000 = [W, Cond] => [R(0), Cond(12)];
    0b11011010_10011111_00000011_11100000 = [X, Cond] => [R(0), Cond(12)];
]
"csinc" = [
    0b00011010_10000000_00000100_00000000 = [W, W, W, Cond] => [R(0), R(5), R(16), Cond(12)];
    0b10011010_10000000_00000100_00000000 = [X, X, X, Cond] => [R(0), R(5), R(16), Cond(12)];
]
"csinv" = [
    0b01011010_10000000_00000000_00000000 = [W, W, W, Cond] => [R(0), R(5), R(16), Cond(12)];
    0b11011010_10000000_00000000_00000000 = [X, X, X, Cond] => [R(0), R(5), R(16), Cond(12)];
]
"csneg" = [
    0b01011010_10000000_00000100_00000000 = [W, W, W, Cond] => [R(0), R(5), R(16), Cond(12)];
    0b11011010_10000000_00000100_00000000 = [X, X, X, Cond] => [R(0), R(5), R(16), Cond(12)];
]
"dc" = [
    0b11010101_00001000_01110000_00000000 = [Ident, X] => [LitList(5, "DC_OPS"), R(0)];
]
"dcps1" = [
    0b11010100_10100000_00000000_00000001 = [End, Imm] => [Ubits(5, 16)];
]
"dcps2" = [
    0b11010100_10100000_00000000_00000010 = [End, Imm] => [Ubits(5, 16)];
]
"dcps3" = [
    0b11010100_10100000_00000000_00000011 = [End, Imm] => [Ubits(5, 16)];
]
"dmb" = [
    0b11010101_00000011_00110000_10111111 = [Ident] => [LitList(8, "BARRIER_OPS")];
    0b11010101_00000011_00110000_10111111 = [Imm] => [Ubits(8, 4)];
]
"drps" = [
    0b11010110_10111111_00000011_11100000 = [] => [];
]
"dsb" = [
    0b11010101_00000011_00110000_10011111 = [Ident] => [LitList(8, "BARRIER_OPS")];
    0b11010101_00000011_00110000_10011111 = [Imm] => [Ubits(8, 4)];
]
"dup" = [
    // DUP (element)
    0b01011110_00000001_00000100_00000000 = [B, VLanes(BYTE)] => [R(0), R(5), Ubits(17, 4)];
    0b01011110_00000010_00000100_00000000 = [H, VLanes(WORD)] => [R(0), R(5), Ubits(18, 3)];
    0b01011110_00000100_00000100_00000000 = [S, VLanes(DWORD)] => [R(0), R(5), Ubits(19, 2)];
    0b01011110_00001000_00000100_00000000 = [D, VLanes(QWORD)] => [R(0), R(5), Ubits(20, 1)];
    0b00001110_00000001_00000100_00000000 = [VSized(BYTE), VLanes(BYTE)] => [R(0), R(5), Ubits(17, 4), Rwidth(30)];
    0b00001110_00000010_00000100_00000000 = [VSized(WORD), VLanes(BYTE)] => [R(0), R(5), Ubits(18, 3), Rwidth(30)];
    0b00001110_00000100_00000100_00000000 = [VSized(DWORD), VLanes(WORD)] => [R(0), R(5), Ubits(19, 2), Rwidth(30)];
    0b00001110_00001000_00000100_00000000 = [VSizedStatic(QWORD, 2), VLanes(DWORD)] => [R(0), R(5), Ubits(20, 1), Rwidth(30)];
    // DUP (general)
    0b00001110_00000001_00001100_00000000 = [VSized(BYTE), W] => [R(0), Rwidth(30), R(5)];
    0b00001110_00000010_00001100_00000000 = [VSized(WORD), W] => [R(0), Rwidth(30), R(5)];
    0b00001110_00000100_00001100_00000000 = [VSized(DWORD), W] => [R(0), Rwidth(30), R(5)];
    0b00001110_00001000_00001100_00000000 = [VSizedStatic(QWORD, 2), X] => [R(0), Rwidth(30), R(5)];
]
"dvp" = [
    0b11010101_00001011_01110011_10100000 = [Lit("RCTX"), X] => [R(0)];
]
"eon" = [
    0b01001010_00100000_00000000_00000000 = [W, W, W, End, Mod(ROTATES)] => [R(0), R(5), R(16), Rotates(22), Ubits(10, 5)];
    0b11001010_00100000_00000000_00000000 = [X, X, X, End, Mod(ROTATES)] => [R(0), R(5), R(16), Rotates(22), Ubits(10, 6)];
]
"eor" = [
    // EOR (vector)
    0b00101110_00100000_00011100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    // EOR (immediate)
    0b01010010_00000000_00000000_00000000 = [WSP, W, Imm] => [R(0), R(5), UlogicalW(10)];
    0b11010010_00000000_00000000_00000000 = [XSP, X, Imm] => [R(0), R(5), UlogicalX(10)];
    // EOR (shifted register)
    0b01001010_00000000_00000000_00000000 = [W, W, W, End, Mod(ROTATES)] => [R(0), R(5), R(16), Rotates(22), Ubits(10, 5)];
    0b11001010_00000000_00000000_00000000 = [X, X, X, End, Mod(ROTATES)] => [R(0), R(5), R(16), Rotates(22), Ubits(10, 6)];
]
"eor3" = [
    0b11001110_00000000_00000000_00000000 = [VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16)] => [R(0), R(5), R(16), R(10)];
]
"eret" = [
    0b11010110_10011111_00000011_11100000 = [] => [];
]
"eretaa" = [
    0b11010110_10011111_00001011_11111111 = [] => [];
]
"eretab" = [
    0b11010110_10011111_00001111_11111111 = [] => [];
]
"esb" = [
    0b11010101_00000011_00100010_00011111 = [] => [];
]
"ext" = [
    0b00101110_00000000_00000000_00000000 = [VSizedStatic(BYTE, 8), VSizedStatic(BYTE, 8), VSizedStatic(BYTE, 8), Imm] => [R(0), R(5), R(16), Ubits(11, 3)];
    0b01101110_00000000_00000000_00000000 = [VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16), Imm] => [R(0), R(5), R(16), Ubits(11, 4)];
]
"extr" = [
    0b00010011_10000000_00000000_00000000 = [W, W, W, Imm] => [R(0), R(5), R(16), Ubits(10, 5)];
    0b10010011_11000000_00000000_00000000 = [X, X, X, Imm] => [R(0), R(5), R(16), Ubits(10, 6)];
]
"fabd" = [
    0b01111110_11000000_00010100_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b01111110_10100000_11010100_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b01111110_11100000_11010100_00000000 = [D, D, D] => [R(0), R(5), R(16)];
    0b00101110_11000000_00010100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_10100000_11010100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_11100000_11010100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"fabs" = [
    // FABS (vector)
    0b00001110_11111000_11111000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_10100000_11111000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_11100000_11111000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
    // FABS (scalar)
    0b00011110_11100000_11000000_00000000 = [H, H] => [R(0), R(5)];
    0b00011110_00100000_11000000_00000000 = [S, S] => [R(0), R(5)];
    0b00011110_01100000_11000000_00000000 = [D, D] => [R(0), R(5)];
]
"facge" = [
    0b01111110_01000000_00101100_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b01111110_00100000_11101100_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b01111110_01100000_11101100_00000000 = [D, D, D] => [R(0), R(5), R(16)];
    0b00101110_01000000_00101100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_00100000_11101100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_01100000_11101100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"facgt" = [
    0b01111110_11000000_00101100_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b01111110_10100000_11101100_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b01111110_11100000_11101100_00000000 = [D, D, D] => [R(0), R(5), R(16)];
    0b00101110_11000000_00101100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_10100000_11101100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_11100000_11101100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"fadd" = [
    // FADD (vector)
    0b00001110_01000000_00010100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_00100000_11010100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01100000_11010100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
    // FADD (scalar)
    0b00011110_11100000_00101000_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b00011110_00100000_00101000_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b00011110_01100000_00101000_00000000 = [D, D, D] => [R(0), R(5), R(16)];
]
"faddp" = [
    // FADDP (scalar)
    0b01011110_00110000_11011000_00000000 = [H, VSizedStatic(WORD, 2)] => [R(0), R(5)];
    0b01111110_00110000_11011000_00000000 = [S, VSizedStatic(DWORD, 2)] => [R(0), R(5)];
    0b01111110_01110000_11011000_00000000 = [D, VSizedStatic(QWORD, 2)] => [R(0), R(5)];
    // FADDP (vector)
    0b00101110_01000000_00010100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_00100000_11010100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_01100000_11010100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"fcadd" = [
    0b00101110_01000000_11100100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD), Imm] => [R(0), R(5), R(16), Ulist(12, &[90, 270]), Rwidth(30)];
    0b00101110_10000000_11100100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD), Imm] => [R(0), R(5), R(16), Ulist(12, &[90, 270]), Rwidth(30)];
    0b00101110_11000000_11100100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSized(QWORD), Imm] => [R(0), R(5), R(16), Ulist(12, &[90, 270]), Rwidth(30)];
]
"fccmp" = [
    0b00011110_11100000_00000100_00000000 = [H, H, Imm, Cond] => [R(5), R(16), Ubits(0, 4), Cond(12)];
    0b00011110_00100000_00000100_00000000 = [S, S, Imm, Cond] => [R(5), R(16), Ubits(0, 4), Cond(12)];
    0b00011110_01100000_00000100_00000000 = [D, D, Imm, Cond] => [R(5), R(16), Ubits(0, 4), Cond(12)];
]
"fccmpe" = [
    0b00011110_11100000_00000100_00010000 = [H, H, Imm, Cond] => [R(5), R(16), Ubits(0, 4), Cond(12)];
    0b00011110_00100000_00000100_00010000 = [S, S, Imm, Cond] => [R(5), R(16), Ubits(0, 4), Cond(12)];
    0b00011110_01100000_00000100_00010000 = [D, D, Imm, Cond] => [R(5), R(16), Ubits(0, 4), Cond(12)];
]
"fcmeq" = [
    // FCMEQ (register)
    0b01011110_01000000_00100100_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b01011110_00100000_11100100_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b01011110_01100000_11100100_00000000 = [D, D, D] => [R(0), R(5), R(16)];
    0b00001110_01000000_00100100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_00100000_11100100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01100000_11100100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
    // FCMEQ (zero)
    0b01011110_11111000_11011000_00000000 = [H, H, LitFloat(0.0)] => [R(0), R(5)];
    0b01011110_10100000_11011000_00000000 = [S, S, LitFloat(0.0)] => [R(0), R(5)];
    0b01011110_11100000_11011000_00000000 = [D, D, LitFloat(0.0)] => [R(0), R(5)];
    0b00001110_11111000_11011000_00000000 = [VSized(WORD), VSized(WORD), LitFloat(0.0)] => [R(0), R(5), Rwidth(30)];
    0b00001110_10100000_11011000_00000000 = [VSized(DWORD), VSized(DWORD), LitFloat(0.0)] => [R(0), R(5), Rwidth(30)];
    0b00001110_11100000_11011000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), LitFloat(0.0)] => [R(0), R(5), Rwidth(30)];
]
"fcmge" = [
    // FCMGE (register)
    0b01111110_01000000_00100100_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b01111110_00100000_11100100_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b01111110_01100000_11100100_00000000 = [D, D, D] => [R(0), R(5), R(16)];
    0b00101110_01000000_00100100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_00100000_11100100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_01100000_11100100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
    // FCMGE (zero)
    0b01111110_11111000_11001000_00000000 = [H, H, LitFloat(0.0)] => [R(0), R(5)];
    0b01111110_10100000_11001000_00000000 = [S, S, LitFloat(0.0)] => [R(0), R(5)];
    0b01111110_11100000_11001000_00000000 = [D, D, LitFloat(0.0)] => [R(0), R(5)];
    0b00101110_11111000_11001000_00000000 = [VSized(WORD), VSized(WORD), LitFloat(0.0)] => [R(0), R(5), Rwidth(30)];
    0b00101110_10100000_11001000_00000000 = [VSized(DWORD), VSized(DWORD), LitFloat(0.0)] => [R(0), R(5), Rwidth(30)];
    0b00101110_11100000_11001000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), LitFloat(0.0)] => [R(0), R(5), Rwidth(30)];
]
"fcmgt" = [
    // FCMGT (register)
    0b01111110_11000000_00100100_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b01111110_10100000_11100100_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b01111110_11100000_11100100_00000000 = [D, D, D] => [R(0), R(5), R(16)];
    0b00101110_11000000_00100100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_10100000_11100100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_11100000_11100100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
    // FCMGT (zero)
    0b01011110_11111000_11001000_00000000 = [H, H, LitFloat(0.0)] => [R(0), R(5)];
    0b01011110_10100000_11001000_00000000 = [S, S, LitFloat(0.0)] => [R(0), R(5)];
    0b01011110_11100000_11001000_00000000 = [D, D, LitFloat(0.0)] => [R(0), R(5)];
    0b00001110_11111000_11001000_00000000 = [VSized(WORD), VSized(WORD), LitFloat(0.0)] => [R(0), R(5), Rwidth(30)];
    0b00001110_10100000_11001000_00000000 = [VSized(DWORD), VSized(DWORD), LitFloat(0.0)] => [R(0), R(5), Rwidth(30)];
    0b00001110_11100000_11001000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), LitFloat(0.0)] => [R(0), R(5), Rwidth(30)];
]
"fcmla" = [
    // FCMLA (by element)
    0b00101111_01000000_00010000_00000000 = [VSized(WORD), VSized(WORD), VLanes(WORD), Imm] => [R(0), R(5), R4(16), Ufields(&[11, 21]), Ulist(13, &[0, 90, 180, 270]), Rwidth(30)];
    0b00101111_10000000_00010000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4), VLanes(DWORD), Imm] => [R(0), R(5), R(16), Ufields(&[11]), Ulist(13, &[0, 90, 180, 270]), Rwidth(30)];
    // FCMLA
    0b00101110_01000000_11000100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD), Imm] => [R(0), R(5), R(16), Ulist(11, &[0, 90, 180, 270]), Rwidth(30)];
    0b00101110_10000000_11000100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD), Imm] => [R(0), R(5), R(16), Ulist(11, &[0, 90, 180, 270]), Rwidth(30)];
    0b00101110_11000000_11000100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSized(QWORD), Imm] => [R(0), R(5), R(16), Ulist(11, &[0, 90, 180, 270]), Rwidth(30)];
]
"fcmle" = [
    0b01111110_11111000_11011000_00000000 = [H, H, LitFloat(0.0)] => [R(0), R(5)];
    0b01111110_10100000_11011000_00000000 = [S, S, LitFloat(0.0)] => [R(0), R(5)];
    0b01111110_11100000_11011000_00000000 = [D, D, LitFloat(0.0)] => [R(0), R(5)];
    0b00101110_11111000_11011000_00000000 = [VSized(WORD), VSized(WORD), LitFloat(0.0)] => [R(0), R(5), Rwidth(30)];
    0b00101110_10100000_11011000_00000000 = [VSized(DWORD), VSized(DWORD), LitFloat(0.0)] => [R(0), R(5), Rwidth(30)];
    0b00101110_11100000_11011000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), LitFloat(0.0)] => [R(0), R(5), Rwidth(30)];
]
"fcmlt" = [
    0b01011110_11111000_11101000_00000000 = [H, H, LitFloat(0.0)] => [R(0), R(5)];
    0b01011110_10100000_11101000_00000000 = [S, S, LitFloat(0.0)] => [R(0), R(5)];
    0b01011110_11100000_11101000_00000000 = [D, D, LitFloat(0.0)] => [R(0), R(5)];
    0b00001110_11111000_11101000_00000000 = [VSized(WORD), VSized(WORD), LitFloat(0.0)] => [R(0), R(5), Rwidth(30)];
    0b00001110_10100000_11101000_00000000 = [VSized(DWORD), VSized(DWORD), LitFloat(0.0)] => [R(0), R(5), Rwidth(30)];
    0b00001110_11100000_11101000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), LitFloat(0.0)] => [R(0), R(5), Rwidth(30)];
]
"fcmp" = [
    0b00011110_11100000_00100000_00000000 = [H, H] => [R(5), R(16)];
    0b00011110_11100000_00100000_00001000 = [H, LitFloat(0.0)] => [R(5)];
    0b00011110_00100000_00100000_00000000 = [S, S] => [R(5), R(16)];
    0b00011110_00100000_00100000_00001000 = [S, LitFloat(0.0)] => [R(5)];
    0b00011110_01100000_00100000_00000000 = [D, D] => [R(5), R(16)];
    0b00011110_01100000_00100000_00001000 = [D, LitFloat(0.0)] => [R(5)];
]
"fcmpe" = [
    0b00011110_11100000_00100000_00010000 = [H, H] => [R(5), R(16)];
    0b00011110_11100000_00100000_00011000 = [H, LitFloat(0.0)] => [R(5)];
    0b00011110_00100000_00100000_00010000 = [S, S] => [R(5), R(16)];
    0b00011110_00100000_00100000_00011000 = [S, LitFloat(0.0)] => [R(5)];
    0b00011110_01100000_00100000_00010000 = [D, D] => [R(5), R(16)];
    0b00011110_01100000_00100000_00011000 = [D, LitFloat(0.0)] => [R(5)];
]
"fcsel" = [
    0b00011110_11100000_00001100_00000000 = [H, H, H, Cond] => [R(0), R(5), R(16), Cond(12)];
    0b00011110_00100000_00001100_00000000 = [S, S, S, Cond] => [R(0), R(5), R(16), Cond(12)];
    0b00011110_01100000_00001100_00000000 = [D, D, D, Cond] => [R(0), R(5), R(16), Cond(12)];
]
"fcvt" = [
    0b00011110_11100010_01000000_00000000 = [S, H] => [R(0), R(5)];
    0b00011110_11100010_11000000_00000000 = [D, H] => [R(0), R(5)];
    0b00011110_00100011_11000000_00000000 = [H, S] => [R(0), R(5)];
    0b00011110_00100010_11000000_00000000 = [D, S] => [R(0), R(5)];
    0b00011110_01100011_11000000_00000000 = [H, D] => [R(0), R(5)];
    0b00011110_01100010_01000000_00000000 = [S, D] => [R(0), R(5)];
]
"fcvtas" = [
    // FCVTAS (vector)
    0b01011110_01111001_11001000_00000000 = [H, H] => [R(0), R(5)];
    0b01011110_00100001_11001000_00000000 = [S, S] => [R(0), R(5)];
    0b01011110_01100001_11001000_00000000 = [D, D] => [R(0), R(5)];
    0b00001110_01111001_11001000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_00100001_11001000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_01100001_11001000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
    // FCVTAS (scalar)
    0b00011110_11100100_00000000_00000000 = [W, H] => [R(0), R(5)];
    0b10011110_11100100_00000000_00000000 = [X, H] => [R(0), R(5)];
    0b00011110_00100100_00000000_00000000 = [W, S] => [R(0), R(5)];
    0b10011110_00100100_00000000_00000000 = [X, S] => [R(0), R(5)];
    0b00011110_01100100_00000000_00000000 = [W, D] => [R(0), R(5)];
    0b10011110_01100100_00000000_00000000 = [X, D] => [R(0), R(5)];
]
"fcvtau" = [
    // FCVTAU (vector)
    0b01111110_01111001_11001000_00000000 = [H, H] => [R(0), R(5)];
    0b01111110_00100001_11001000_00000000 = [S, S] => [R(0), R(5)];
    0b01111110_01100001_11001000_00000000 = [D, D] => [R(0), R(5)];
    0b00101110_01111001_11001000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_00100001_11001000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_01100001_11001000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
    // FCVTAU (scalar)
    0b00011110_11100101_00000000_00000000 = [W, H] => [R(0), R(5)];
    0b10011110_11100101_00000000_00000000 = [X, H] => [R(0), R(5)];
    0b00011110_00100101_00000000_00000000 = [W, S] => [R(0), R(5)];
    0b10011110_00100101_00000000_00000000 = [X, S] => [R(0), R(5)];
    0b00011110_01100101_00000000_00000000 = [W, D] => [R(0), R(5)];
    0b10011110_01100101_00000000_00000000 = [X, D] => [R(0), R(5)];
]
"fcvtl" = [
    0b00001110_00100001_01111000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4)] => [R(0), R(5)];
    0b00001110_01100001_01111000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2)] => [R(0), R(5)];
]
"fcvtl2" = [
    0b01001110_00100001_01111000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8)] => [R(0), R(5)];
    0b01001110_01100001_01111000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4)] => [R(0), R(5)];
]
"fcvtms" = [
    // FCVTMS (vector)
    0b01011110_01111001_10111000_00000000 = [H, H] => [R(0), R(5)];
    0b01011110_00100001_10111000_00000000 = [S, S] => [R(0), R(5)];
    0b01011110_01100001_10111000_00000000 = [D, D] => [R(0), R(5)];
    0b00001110_01111001_10111000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_00100001_10111000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_01100001_10111000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
    // FCVTMS (scalar)
    0b00011110_11110000_00000000_00000000 = [W, H] => [R(0), R(5)];
    0b10011110_11110000_00000000_00000000 = [X, H] => [R(0), R(5)];
    0b00011110_00110000_00000000_00000000 = [W, S] => [R(0), R(5)];
    0b10011110_00110000_00000000_00000000 = [X, S] => [R(0), R(5)];
    0b00011110_01110000_00000000_00000000 = [W, D] => [R(0), R(5)];
    0b10011110_01110000_00000000_00000000 = [X, D] => [R(0), R(5)];
]
"fcvtmu" = [
    // FCVTMU (vector)
    0b01111110_01111001_10111000_00000000 = [H, H] => [R(0), R(5)];
    0b01111110_00100001_10111000_00000000 = [S, S] => [R(0), R(5)];
    0b01111110_01100001_10111000_00000000 = [D, D] => [R(0), R(5)];
    0b00101110_01111001_10111000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_00100001_10111000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_01100001_10111000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
    // FCVTMU (scalar)
    0b00011110_11110001_00000000_00000000 = [W, H] => [R(0), R(5)];
    0b10011110_11110001_00000000_00000000 = [X, H] => [R(0), R(5)];
    0b00011110_00110001_00000000_00000000 = [W, S] => [R(0), R(5)];
    0b10011110_00110001_00000000_00000000 = [X, S] => [R(0), R(5)];
    0b00011110_01110001_00000000_00000000 = [W, D] => [R(0), R(5)];
    0b10011110_01110001_00000000_00000000 = [X, D] => [R(0), R(5)];
]
"fcvtn" = [
    0b00001110_00100001_01101000_00000000 = [VSizedStatic(WORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5)];
    0b00001110_01100001_01101000_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5)];
]
"fcvtn2" = [
    0b01001110_00100001_01101000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(DWORD, 4)] => [R(0), R(5)];
    0b01001110_01100001_01101000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(QWORD, 2)] => [R(0), R(5)];
]
"fcvtns" = [
    // FCVTNS (vector)
    0b01011110_01111001_10101000_00000000 = [H, H] => [R(0), R(5)];
    0b01011110_00100001_10101000_00000000 = [S, S] => [R(0), R(5)];
    0b01011110_01100001_10101000_00000000 = [D, D] => [R(0), R(5)];
    0b00001110_01111001_10101000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_00100001_10101000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_01100001_10101000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
    // FCVTNS (scalar)
    0b00011110_11100000_00000000_00000000 = [W, H] => [R(0), R(5)];
    0b10011110_11100000_00000000_00000000 = [X, H] => [R(0), R(5)];
    0b00011110_00100000_00000000_00000000 = [W, S] => [R(0), R(5)];
    0b10011110_00100000_00000000_00000000 = [X, S] => [R(0), R(5)];
    0b00011110_01100000_00000000_00000000 = [W, D] => [R(0), R(5)];
    0b10011110_01100000_00000000_00000000 = [X, D] => [R(0), R(5)];
]
"fcvtnu" = [
    // FCVTNU (vector)
    0b01111110_01111001_10101000_00000000 = [H, H] => [R(0), R(5)];
    0b01111110_00100001_10101000_00000000 = [S, S] => [R(0), R(5)];
    0b01111110_01100001_10101000_00000000 = [D, D] => [R(0), R(5)];
    0b00101110_01111001_10101000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_00100001_10101000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_01100001_10101000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
    // FCVTNU (scalar)
    0b00011110_11100001_00000000_00000000 = [W, H] => [R(0), R(5)];
    0b10011110_11100001_00000000_00000000 = [X, H] => [R(0), R(5)];
    0b00011110_00100001_00000000_00000000 = [W, S] => [R(0), R(5)];
    0b10011110_00100001_00000000_00000000 = [X, S] => [R(0), R(5)];
    0b00011110_01100001_00000000_00000000 = [W, D] => [R(0), R(5)];
    0b10011110_01100001_00000000_00000000 = [X, D] => [R(0), R(5)];
]
"fcvtps" = [
    // FCVTPS (vector)
    0b01011110_11111001_10101000_00000000 = [H, H] => [R(0), R(5)];
    0b01011110_10100001_10101000_00000000 = [S, S] => [R(0), R(5)];
    0b01011110_11100001_10101000_00000000 = [D, D] => [R(0), R(5)];
    0b00001110_11111001_10101000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_10100001_10101000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_11100001_10101000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
    // FCVTPS (scalar)
    0b00011110_11101000_00000000_00000000 = [W, H] => [R(0), R(5)];
    0b10011110_11101000_00000000_00000000 = [X, H] => [R(0), R(5)];
    0b00011110_00101000_00000000_00000000 = [W, S] => [R(0), R(5)];
    0b10011110_00101000_00000000_00000000 = [X, S] => [R(0), R(5)];
    0b00011110_01101000_00000000_00000000 = [W, D] => [R(0), R(5)];
    0b10011110_01101000_00000000_00000000 = [X, D] => [R(0), R(5)];
]
"fcvtpu" = [
    // FCVTPU (vector)
    0b01111110_11111001_10101000_00000000 = [H, H] => [R(0), R(5)];
    0b01111110_10100001_10101000_00000000 = [S, S] => [R(0), R(5)];
    0b01111110_11100001_10101000_00000000 = [D, D] => [R(0), R(5)];
    0b00101110_11111001_10101000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_10100001_10101000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_11100001_10101000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
    // FCVTPU (scalar)
    0b00011110_11101001_00000000_00000000 = [W, H] => [R(0), R(5)];
    0b10011110_11101001_00000000_00000000 = [X, H] => [R(0), R(5)];
    0b00011110_00101001_00000000_00000000 = [W, S] => [R(0), R(5)];
    0b10011110_00101001_00000000_00000000 = [X, S] => [R(0), R(5)];
    0b00011110_01101001_00000000_00000000 = [W, D] => [R(0), R(5)];
    0b10011110_01101001_00000000_00000000 = [X, D] => [R(0), R(5)];
]
"fcvtxn" = [
    0b01111110_01100001_01101000_00000000 = [S, D] => [R(0), R(5)];
    0b00101110_00100001_01101000_00000000 = [VSizedStatic(WORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5)];
    0b00101110_01100001_01101000_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5)];
]
"fcvtxn2" = [
    0b01101110_00100001_01101000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(DWORD, 4)] => [R(0), R(5)];
    0b01101110_01100001_01101000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(QWORD, 2)] => [R(0), R(5)];
]
"fcvtzs" = [
    // FCVTZS (vector, fixed-point)
    0b01011111_00000000_11111100_00000000 = [H, H, Imm] => [R(0), R(5), BUrange(1, 16), Usub(16, 5, 32)];
    0b01011111_00000000_11111100_00000000 = [S, S, Imm] => [R(0), R(5), BUrange(1, 32), Usub(16, 6, 64)];
    0b01011111_00000000_11111100_00000000 = [D, D, Imm] => [R(0), R(5), BUrange(1, 64), Usub(16, 7, 128)];
    0b00001111_00010000_11111100_00000000 = [VSized(WORD), VSized(WORD), Imm] => [R(0), R(5), R(16), Usub(16, 4, 16), Rwidth(30)];
    0b00001111_00100000_11111100_00000000 = [VSized(DWORD), VSized(DWORD), Imm] => [R(0), R(5), R(16), Usub(16, 5, 32), Rwidth(30)];
    0b00001111_01000000_11111100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), R(16), Usub(16, 6, 64), Rwidth(30)];
    // FCVTZS (vector, integer)
    0b01011110_11111001_10111000_00000000 = [H, H] => [R(0), R(5)];
    0b01011110_10100001_10111000_00000000 = [S, S] => [R(0), R(5)];
    0b01011110_11100001_10111000_00000000 = [D, D] => [R(0), R(5)];
    0b00001110_11111001_10111000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_10100001_10111000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_11100001_10111000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
    // FCVTZS (scalar, fixed-point)
    0b00011110_11011000_00000000_00000000 = [W, H, Imm] => [R(0), R(5), BUrange(1, 32), Usub(10, 6, 64)];
    0b10011110_11011000_00000000_00000000 = [X, H, Imm] => [R(0), R(5), Usub(10, 6, 64)];
    0b00011110_00011000_00000000_00000000 = [W, S, Imm] => [R(0), R(5), BUrange(1, 32), Usub(10, 6, 64)];
    0b10011110_00011000_00000000_00000000 = [X, S, Imm] => [R(0), R(5), Usub(10, 6, 64)];
    0b00011110_01011000_00000000_00000000 = [W, D, Imm] => [R(0), R(5), BUrange(1, 32), Usub(10, 6, 64)];
    0b10011110_01011000_00000000_00000000 = [X, D, Imm] => [R(0), R(5), Usub(10, 6, 64)];
    // FCVTZS (scalar, integer)
    0b00011110_11111000_00000000_00000000 = [W, H] => [R(0), R(5)];
    0b10011110_11111000_00000000_00000000 = [X, H] => [R(0), R(5)];
    0b00011110_00111000_00000000_00000000 = [W, S] => [R(0), R(5)];
    0b10011110_00111000_00000000_00000000 = [X, S] => [R(0), R(5)];
    0b00011110_01111000_00000000_00000000 = [W, D] => [R(0), R(5)];
    0b10011110_01111000_00000000_00000000 = [X, D] => [R(0), R(5)];
]
"fcvtzu" = [
    // FCVTZU (vector, fixed-point)
    0b01111111_00000000_11111100_00000000 = [H, H, Imm] => [R(0), R(5), BUrange(1, 16), Usub(16, 5, 32)];
    0b01111111_00000000_11111100_00000000 = [S, S, Imm] => [R(0), R(5), BUrange(1, 32), Usub(16, 6, 64)];
    0b01111111_00000000_11111100_00000000 = [D, D, Imm] => [R(0), R(5), BUrange(1, 64), Usub(16, 7, 128)];
    0b00101111_00010000_11111100_00000000 = [VSized(WORD), VSized(WORD), Imm] => [R(0), R(5), R(16), Usub(16, 4, 16), Rwidth(30)];
    0b00101111_00100000_11111100_00000000 = [VSized(DWORD), VSized(DWORD), Imm] => [R(0), R(5), R(16), Usub(16, 5, 32), Rwidth(30)];
    0b00101111_01000000_11111100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), R(16), Usub(16, 6, 64), Rwidth(30)];
    // FCVTZU (vector, integer)
    0b01111110_11111001_10111000_00000000 = [H, H] => [R(0), R(5)];
    0b01111110_10100001_10111000_00000000 = [S, S] => [R(0), R(5)];
    0b01111110_11100001_10111000_00000000 = [D, D] => [R(0), R(5)];
    0b00101110_11111001_10111000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_10100001_10111000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_11100001_10111000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
    // FCVTZU (scalar, fixed-point)
    0b00011110_11011001_00000000_00000000 = [W, H, Imm] => [R(0), R(5), BUrange(1, 32), Usub(10, 6, 64)];
    0b10011110_11011001_00000000_00000000 = [X, H, Imm] => [R(0), R(5), Usub(10, 6, 64)];
    0b00011110_00011001_00000000_00000000 = [W, S, Imm] => [R(0), R(5), BUrange(1, 32), Usub(10, 6, 64)];
    0b10011110_00011001_00000000_00000000 = [X, S, Imm] => [R(0), R(5), Usub(10, 6, 64)];
    0b00011110_01011001_00000000_00000000 = [W, D, Imm] => [R(0), R(5), BUrange(1, 32), Usub(10, 6, 64)];
    0b10011110_01011001_00000000_00000000 = [X, D, Imm] => [R(0), R(5), Usub(10, 6, 64)];
    // FCVTZU (scalar, integer)
    0b00011110_11111001_00000000_00000000 = [W, H] => [R(0), R(5)];
    0b10011110_11111001_00000000_00000000 = [X, H] => [R(0), R(5)];
    0b00011110_00111001_00000000_00000000 = [W, S] => [R(0), R(5)];
    0b10011110_00111001_00000000_00000000 = [X, S] => [R(0), R(5)];
    0b00011110_01111001_00000000_00000000 = [W, D] => [R(0), R(5)];
    0b10011110_01111001_00000000_00000000 = [X, D] => [R(0), R(5)];
]
"fdiv" = [
    // FDIV (vector)
    0b00101110_01000000_00111100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_00100000_11111100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_01100000_11111100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
    // FDIV (scalar)
    0b00011110_11100000_00011000_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b00011110_00100000_00011000_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b00011110_01100000_00011000_00000000 = [D, D, D] => [R(0), R(5), R(16)];
]
"fjcvtzs" = [
    0b00011110_01111110_00000000_00000000 = [W, D] => [R(0), R(5)];
]
"fmadd" = [
    0b00011111_11000000_00000000_00000000 = [H, H, H, H] => [R(0), R(5), R(16), R(10)];
    0b00011111_00000000_00000000_00000000 = [S, S, S, S] => [R(0), R(5), R(16), R(10)];
    0b00011111_01000000_00000000_00000000 = [D, D, D, D] => [R(0), R(5), R(16), R(10)];
]
"fmax" = [
    // FMAX (vector)
    0b00001110_01000000_00110100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_00100000_11110100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01100000_11110100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
    // FMAX (scalar)
    0b00011110_11100000_01001000_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b00011110_00100000_01001000_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b00011110_01100000_01001000_00000000 = [D, D, D] => [R(0), R(5), R(16)];
]
"fmaxnm" = [
    // FMAXNM (vector)
    0b00001110_01000000_00000100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_00100000_11000100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01100000_11000100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
    // FMAXNM (scalar)
    0b00011110_11100000_01101000_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b00011110_00100000_01101000_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b00011110_01100000_01101000_00000000 = [D, D, D] => [R(0), R(5), R(16)];
]
"fmaxnmp" = [
    // FMAXNMP (scalar)
    0b01011110_00110000_11001000_00000000 = [H, VSizedStatic(WORD, 2)] => [R(0), R(5)];
    0b01111110_00110000_11001000_00000000 = [S, VSizedStatic(DWORD, 2)] => [R(0), R(5)];
    0b01111110_01110000_11001000_00000000 = [D, VSizedStatic(QWORD, 2)] => [R(0), R(5)];
    // FMAXNMP (vector)
    0b00101110_01000000_00000100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_00100000_11000100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_01100000_11000100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"fmaxnmv" = [
    0b00001110_00110000_11001000_00000000 = [H, VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_00110000_11001000_00000000 = [S, VSizedStatic(DWORD, 4)] => [R(0), R(5), Rwidth(30)];
]
"fmaxp" = [
    // FMAXP (scalar)
    0b01011110_00110000_11111000_00000000 = [H, VSizedStatic(WORD, 2)] => [R(0), R(5)];
    0b01111110_00110000_11111000_00000000 = [S, VSizedStatic(DWORD, 2)] => [R(0), R(5)];
    0b01111110_01110000_11111000_00000000 = [D, VSizedStatic(QWORD, 2)] => [R(0), R(5)];
    // FMAXP (vector)
    0b00101110_01000000_00110100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_00100000_11110100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_01100000_11110100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"fmaxv" = [
    0b00001110_00110000_11111000_00000000 = [H, VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_00110000_11111000_00000000 = [S, VSizedStatic(DWORD, 4)] => [R(0), R(5), Rwidth(30)];
]
"fmin" = [
    // FMIN (vector)
    0b00001110_11000000_00110100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10100000_11110100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_11100000_11110100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
    // FMIN (scalar)
    0b00011110_11100000_01011000_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b00011110_00100000_01011000_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b00011110_01100000_01011000_00000000 = [D, D, D] => [R(0), R(5), R(16)];
]
"fminnm" = [
    // FMINNM (vector)
    0b00001110_11000000_00000100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10100000_11000100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_11100000_11000100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
    // FMINNM (scalar)
    0b00011110_11100000_01111000_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b00011110_00100000_01111000_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b00011110_01100000_01111000_00000000 = [D, D, D] => [R(0), R(5), R(16)];
]
"fminnmp" = [
    // FMINNMP (scalar)
    0b01011110_10110000_11001000_00000000 = [H, VSizedStatic(WORD, 2)] => [R(0), R(5)];
    0b01111110_10110000_11001000_00000000 = [S, VSizedStatic(DWORD, 2)] => [R(0), R(5)];
    0b01111110_11110000_11001000_00000000 = [D, VSizedStatic(QWORD, 2)] => [R(0), R(5)];
    // FMINNMP (vector)
    0b00101110_11000000_00000100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_10100000_11000100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_11100000_11000100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"fminnmv" = [
    0b00001110_10110000_11001000_00000000 = [H, VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_10110000_11001000_00000000 = [S, VSizedStatic(DWORD, 4)] => [R(0), R(5), Rwidth(30)];
]
"fminp" = [
    // FMINP (scalar)
    0b01011110_10110000_11111000_00000000 = [H, VSizedStatic(WORD, 2)] => [R(0), R(5)];
    0b01111110_10110000_11111000_00000000 = [S, VSizedStatic(DWORD, 2)] => [R(0), R(5)];
    0b01111110_11110000_11111000_00000000 = [D, VSizedStatic(QWORD, 2)] => [R(0), R(5)];
    // FMINP (vector)
    0b00101110_11000000_00110100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_10100000_11110100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_11100000_11110100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"fminv" = [
    0b00001110_10110000_11111000_00000000 = [H, VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_10110000_11111000_00000000 = [S, VSizedStatic(DWORD, 4)] => [R(0), R(5), Rwidth(30)];
]
"fmla" = [
    // FMLA (by element)
    0b01011111_00000000_00010000_00000000 = [H, H, VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20])];
    0b01011111_10000000_00010000_00000000 = [S, S, VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    0b01011111_11000000_00010000_00000000 = [D, D, VLanes(QWORD)] => [R(0), R(5), R(16), Ufields(&[11])];
    0b00001111_00000000_00010000_00000000 = [VSized(WORD), VSized(WORD), VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20]), Rwidth(30)];
    0b00001111_10000000_00010000_00000000 = [VSized(DWORD), VSized(DWORD), VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21]), Rwidth(30)];
    0b00001111_11000000_00010000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VLanes(QWORD)] => [R(0), R(5), R(16), Ufields(&[11]), Rwidth(30)];
    // FMLA (vector)
    0b00001110_01000000_00001100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_00100000_11001100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01100000_11001100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"fmlal" = [
    // FMLAL, FMLAL2 (by element)
    0b00001111_10000000_00000000_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(WORD, 2), VLanes(WORD)] => [R(0), R(5), R(16), Ufields(&[11, 21, 20])];
    0b01001111_10000000_00000000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VLanes(WORD)] => [R(0), R(5), R(16), Ufields(&[11, 21, 20])];
    // FMLAL, FMLAL2 (vector)
    0b00001110_00100000_11101100_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(WORD, 2), VSizedStatic(WORD, 2)] => [R(0), R(5), R(16)];
    0b01001110_00100000_11101100_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VSizedStatic(WORD, 4)] => [R(0), R(5), R(16)];
]
"fmlal2" = [
    // FMLAL, FMLAL2 (by element)
    0b00101111_10000000_10000000_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(WORD, 2), VLanes(WORD)] => [R(0), R(5), R(16), Ufields(&[11, 21, 20])];
    0b01101111_10000000_10000000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VLanes(WORD)] => [R(0), R(5), R(16), Ufields(&[11, 21, 20])];
    // FMLAL, FMLAL2 (vector)
    0b00101110_00100000_11001100_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(WORD, 2), VSizedStatic(WORD, 2)] => [R(0), R(5), R(16)];
    0b01101110_00100000_11001100_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VSizedStatic(WORD, 4)] => [R(0), R(5), R(16)];
]
"fmls" = [
    // FMLS (by element)
    0b01011111_00000000_01010000_00000000 = [H, H, VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20])];
    0b01011111_10000000_01010000_00000000 = [S, S, VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    0b01011111_11000000_01010000_00000000 = [D, D, VLanes(QWORD)] => [R(0), R(5), R(16), Ufields(&[11])];
    0b00001111_00000000_01010000_00000000 = [VSized(WORD), VSized(WORD), VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20]), Rwidth(30)];
    0b00001111_10000000_01010000_00000000 = [VSized(DWORD), VSized(DWORD), VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21]), Rwidth(30)];
    0b00001111_11000000_01010000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VLanes(QWORD)] => [R(0), R(5), R(16), Ufields(&[11]), Rwidth(30)];
    // FMLS (vector)
    0b00001110_11000000_00001100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10100000_11001100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_11100000_11001100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"fmlsl" = [
    // FMLSL, FMLSL2 (by element)
    0b00001111_10000000_01000000_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(WORD, 2), VLanes(WORD)] => [R(0), R(5), R(16), Ufields(&[11, 21, 20])];
    0b01001111_10000000_01000000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VLanes(WORD)] => [R(0), R(5), R(16), Ufields(&[11, 21, 20])];
    // FMLSL, FMLSL2 (vector)
    0b00001110_10100000_11101100_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(WORD, 2), VSizedStatic(WORD, 2)] => [R(0), R(5), R(16)];
    0b01001110_10100000_11101100_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VSizedStatic(WORD, 4)] => [R(0), R(5), R(16)];
]
"fmlsl2" = [
    // FMLSL, FMLSL2 (by element)
    0b00101111_10000000_11000000_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(WORD, 2), VLanes(WORD)] => [R(0), R(5), R(16), Ufields(&[11, 21, 20])];
    0b01101111_10000000_11000000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VLanes(WORD)] => [R(0), R(5), R(16), Ufields(&[11, 21, 20])];
    // FMLSL, FMLSL2 (vector)
    0b00101110_10100000_11001100_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(WORD, 2), VSizedStatic(WORD, 2)] => [R(0), R(5), R(16)];
    0b01101110_10100000_11001100_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VSizedStatic(WORD, 4)] => [R(0), R(5), R(16)];
]
"fmov" = [
    // FMOV (vector, immediate)
    0b00001111_00000000_11111100_00000000 = [VSized(WORD), Imm] => [R(0), Special(5, "split float"), Rwidth(30)];
    0b00001111_00000000_11110100_00000000 = [VSized(DWORD), Imm] => [R(0), Special(5, "split float"), Rwidth(30)];
    0b01101111_00000000_11110100_00000000 = [VSizedStatic(QWORD, 2), Imm] => [R(0), Special(5, "split float")];
    // FMOV (register)
    0b00011110_11100000_01000000_00000000 = [H, H] => [R(0), R(5)];
    0b00011110_00100000_01000000_00000000 = [S, S] => [R(0), R(5)];
    0b00011110_01100000_01000000_00000000 = [D, D] => [R(0), R(5)];
    // FMOV (general)
    0b00011110_11100110_00000000_00000000 = [W, H] => [R(0), R(5)];
    0b10011110_11100110_00000000_00000000 = [X, H] => [R(0), R(5)];
    0b00011110_11100111_00000000_00000000 = [H, W] => [R(0), R(5)];
    0b00011110_00100111_00000000_00000000 = [S, W] => [R(0), R(5)];
    0b00011110_00100110_00000000_00000000 = [W, S] => [R(0), R(5)];
    0b10011110_11100111_00000000_00000000 = [H, X] => [R(0), R(5)];
    0b10011110_01100111_00000000_00000000 = [D, X] => [R(0), R(5)];
    0b10011110_10101111_00000000_00000000 = [VLanesStatic(QWORD, 1), X] => [R(0), R(5)];
    0b10011110_01100110_00000000_00000000 = [X, D] => [R(0), R(5)];
    0b10011110_10101110_00000000_00000000 = [X, VLanesStatic(QWORD, 1)] => [R(0), R(5)];
    // FMOV (scalar, immediate)
    0b00011110_11100000_00010000_00000000 = [H, Imm] => [R(0), Sfloat(13)];
    0b00011110_00100000_00010000_00000000 = [S, Imm] => [R(0), Sfloat(13)];
    0b00011110_01100000_00010000_00000000 = [D, Imm] => [R(0), Sfloat(13)];
]
"fmsub" = [
    0b00011111_11000000_10000000_00000000 = [H, H, H, H] => [R(0), R(5), R(16), R(10)];
    0b00011111_00000000_10000000_00000000 = [S, S, S, S] => [R(0), R(5), R(16), R(10)];
    0b00011111_01000000_10000000_00000000 = [D, D, D, D] => [R(0), R(5), R(16), R(10)];
]
"fmul" = [
    // FMUL (by element)
    0b01011111_00000000_10010000_00000000 = [H, H, VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20])];
    0b01011111_10000000_10010000_00000000 = [S, S, VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    0b01011111_11000000_10010000_00000000 = [D, D, VLanes(QWORD)] => [R(0), R(5), R(16), Ufields(&[11])];
    0b00001111_00000000_10010000_00000000 = [VSized(WORD), VSized(WORD), VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20]), Rwidth(30)];
    0b00001111_10000000_10010000_00000000 = [VSized(DWORD), VSized(DWORD), VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21]), Rwidth(30)];
    0b00001111_11000000_10010000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VLanes(QWORD)] => [R(0), R(5), R(16), Ufields(&[11]), Rwidth(30)];
    // FMUL (vector)
    0b00101110_01000000_00011100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_00100000_11011100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_01100000_11011100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
    // FMUL (scalar)
    0b00011110_11100000_00001000_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b00011110_00100000_00001000_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b00011110_01100000_00001000_00000000 = [D, D, D] => [R(0), R(5), R(16)];
]
"fmulx" = [
    // FMULX (by element)
    0b01111111_00000000_10010000_00000000 = [H, H, VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20])];
    0b01111111_10000000_10010000_00000000 = [S, S, VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    0b01111111_11000000_10010000_00000000 = [D, D, VLanes(QWORD)] => [R(0), R(5), R(16), Ufields(&[11])];
    0b00101111_00000000_10010000_00000000 = [VSized(WORD), VSized(WORD), VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20]), Rwidth(30)];
    0b00101111_10000000_10010000_00000000 = [VSized(DWORD), VSized(DWORD), VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21]), Rwidth(30)];
    0b00101111_11000000_10010000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VLanes(QWORD)] => [R(0), R(5), R(16), Ufields(&[11]), Rwidth(30)];
    // FMULX
    0b01011110_01000000_00011100_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b01011110_00100000_11011100_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b01011110_01100000_11011100_00000000 = [D, D, D] => [R(0), R(5), R(16)];
    0b00001110_01000000_00011100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_00100000_11011100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01100000_11011100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"fneg" = [
    // FNEG (vector)
    0b00101110_11111000_11111000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_10100000_11111000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_11100000_11111000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
    // FNEG (scalar)
    0b00011110_11100001_01000000_00000000 = [H, H] => [R(0), R(5)];
    0b00011110_00100001_01000000_00000000 = [S, S] => [R(0), R(5)];
    0b00011110_01100001_01000000_00000000 = [D, D] => [R(0), R(5)];
]
"fnmadd" = [
    0b00011111_11100000_00000000_00000000 = [H, H, H, H] => [R(0), R(5), R(16), R(10)];
    0b00011111_00100000_00000000_00000000 = [S, S, S, S] => [R(0), R(5), R(16), R(10)];
    0b00011111_01100000_00000000_00000000 = [D, D, D, D] => [R(0), R(5), R(16), R(10)];
]
"fnmsub" = [
    0b00011111_11100000_10000000_00000000 = [H, H, H, H] => [R(0), R(5), R(16), R(10)];
    0b00011111_00100000_10000000_00000000 = [S, S, S, S] => [R(0), R(5), R(16), R(10)];
    0b00011111_01100000_10000000_00000000 = [D, D, D, D] => [R(0), R(5), R(16), R(10)];
]
"fnmul" = [
    0b00011110_11100000_10001000_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b00011110_00100000_10001000_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b00011110_01100000_10001000_00000000 = [D, D, D] => [R(0), R(5), R(16)];
]
"frecpe" = [
    0b01011110_11111001_11011000_00000000 = [H, H] => [R(0), R(5)];
    0b01011110_10100001_11011000_00000000 = [S, S] => [R(0), R(5)];
    0b01011110_11100001_11011000_00000000 = [D, D] => [R(0), R(5)];
    0b00001110_11111001_11011000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_10100001_11011000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_11100001_11011000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
]
"frecps" = [
    0b01011110_01000000_00111100_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b01011110_00100000_11111100_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b01011110_01100000_11111100_00000000 = [D, D, D] => [R(0), R(5), R(16)];
    0b00001110_01000000_00111100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_00100000_11111100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01100000_11111100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"frecpx" = [
    0b01011110_11111001_11111000_00000000 = [H, H] => [R(0), R(5)];
    0b01011110_10100001_11111000_00000000 = [S, S] => [R(0), R(5)];
    0b01011110_11100001_11111000_00000000 = [D, D] => [R(0), R(5)];
]
"frinta" = [
    // FRINTA (vector)
    0b00101110_01111001_10001000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_00100001_10001000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_01100001_10001000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
    // FRINTA (scalar)
    0b00011110_11100110_01000000_00000000 = [H, H] => [R(0), R(5)];
    0b00011110_00100110_01000000_00000000 = [S, S] => [R(0), R(5)];
    0b00011110_01100110_01000000_00000000 = [D, D] => [R(0), R(5)];
]
"frinti" = [
    // FRINTI (vector)
    0b00101110_11111001_10011000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_10100001_10011000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_11100001_10011000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
    // FRINTI (scalar)
    0b00011110_11100111_11000000_00000000 = [H, H] => [R(0), R(5)];
    0b00011110_00100111_11000000_00000000 = [S, S] => [R(0), R(5)];
    0b00011110_01100111_11000000_00000000 = [D, D] => [R(0), R(5)];
]
"frintm" = [
    // FRINTM (vector)
    0b00001110_01111001_10011000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_00100001_10011000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_01100001_10011000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
    // FRINTM (scalar)
    0b00011110_11100101_01000000_00000000 = [H, H] => [R(0), R(5)];
    0b00011110_00100101_01000000_00000000 = [S, S] => [R(0), R(5)];
    0b00011110_01100101_01000000_00000000 = [D, D] => [R(0), R(5)];
]
"frintn" = [
    // FRINTN (vector)
    0b00001110_01111001_10001000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_00100001_10001000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_01100001_10001000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
    // FRINTN (scalar)
    0b00011110_11100100_01000000_00000000 = [H, H] => [R(0), R(5)];
    0b00011110_00100100_01000000_00000000 = [S, S] => [R(0), R(5)];
    0b00011110_01100100_01000000_00000000 = [D, D] => [R(0), R(5)];
]
"frintp" = [
    // FRINTP (vector)
    0b00001110_11111001_10001000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_10100001_10001000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_11100001_10001000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
    // FRINTP (scalar)
    0b00011110_11100100_11000000_00000000 = [H, H] => [R(0), R(5)];
    0b00011110_00100100_11000000_00000000 = [S, S] => [R(0), R(5)];
    0b00011110_01100100_11000000_00000000 = [D, D] => [R(0), R(5)];
]
"frintx" = [
    // FRINTX (vector)
    0b00101110_01111001_10011000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_00100001_10011000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_01100001_10011000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
    // FRINTX (scalar)
    0b00011110_11100111_01000000_00000000 = [H, H] => [R(0), R(5)];
    0b00011110_00100111_01000000_00000000 = [S, S] => [R(0), R(5)];
    0b00011110_01100111_01000000_00000000 = [D, D] => [R(0), R(5)];
]
"frintz" = [
    // FRINTZ (vector)
    0b00001110_11111001_10011000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_10100001_10011000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_11100001_10011000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
    // FRINTZ (scalar)
    0b00011110_11100101_11000000_00000000 = [H, H] => [R(0), R(5)];
    0b00011110_00100101_11000000_00000000 = [S, S] => [R(0), R(5)];
    0b00011110_01100101_11000000_00000000 = [D, D] => [R(0), R(5)];
]
"frsqrte" = [
    0b01111110_11111001_11011000_00000000 = [H, H] => [R(0), R(5)];
    0b01111110_10100001_11011000_00000000 = [S, S] => [R(0), R(5)];
    0b01111110_11100001_11011000_00000000 = [D, D] => [R(0), R(5)];
    0b00101110_11111001_11011000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_10100001_11011000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_11100001_11011000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
]
"frsqrts" = [
    0b01011110_11000000_00111100_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b01011110_10100000_11111100_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b01011110_11100000_11111100_00000000 = [D, D, D] => [R(0), R(5), R(16)];
    0b00001110_11000000_00111100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10100000_11111100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_11100000_11111100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"fsqrt" = [
    // FSQRT (vector)
    0b00101110_11111001_11111000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_10100001_11111000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_11100001_11111000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
    // FSQRT (scalar)
    0b00011110_11100001_11000000_00000000 = [H, H] => [R(0), R(5)];
    0b00011110_00100001_11000000_00000000 = [S, S] => [R(0), R(5)];
    0b00011110_01100001_11000000_00000000 = [D, D] => [R(0), R(5)];
]
"fsub" = [
    // FSUB (vector)
    0b00001110_11000000_00010100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10100000_11010100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_11100000_11010100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
    // FSUB (scalar)
    0b00011110_11100000_00111000_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b00011110_00100000_00111000_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b00011110_01100000_00111000_00000000 = [D, D, D] => [R(0), R(5), R(16)];
]
"hint" = [
    0b11010101_00000011_00100000_00011111 = [Imm] => [Ubits(5, 7)];
]
"hlt" = [
    0b11010100_01000000_00000000_00000000 = [Imm] => [Ubits(5, 16)];
]
"hvc" = [
    0b11010100_00000000_00000000_00000010 = [Imm] => [Ubits(5, 16)];
]
"ic" = [
    0b11010101_00001000_01110000_00000000 = [Ident, End, X] => [LitList(5, "DC_OPS"), R(0)];
]
"ins" = [
    // INS (element)
    0b01101110_00000001_00000100_00000000 = [VLanes(BYTE), VLanes(BYTE)] => [R(0), Ubits(17, 4), R(5), Ubits(11, 4)];
    0b01101110_00000010_00000100_00000000 = [VLanes(WORD), VLanes(WORD)] => [R(0), Ubits(18, 3), R(5), Ubits(12, 3)];
    0b01101110_00000100_00000100_00000000 = [VLanes(DWORD), VLanes(DWORD)] => [R(0), Ubits(19, 2), R(5), Ubits(13, 2)];
    0b01101110_00001000_00000100_00000000 = [VLanes(QWORD), VLanes(QWORD)] => [R(0), Ubits(20, 1), R(5), Ubits(14, 1)];
    // INS (general)
    0b01001110_00000001_00011100_00000000 = [VLanes(BYTE), W] => [R(0), Ubits(17, 4), R(5)];
    0b01001110_00000010_00011100_00000000 = [VLanes(WORD), W] => [R(0), Ubits(18, 3), R(5)];
    0b01001110_00000100_00011100_00000000 = [VLanes(DWORD), W] => [R(0), Ubits(19, 2), R(5)];
    0b01001110_00001000_00011100_00000000 = [VLanes(QWORD), X] => [R(0), Ubits(20, 1), R(5)];
]
"isb" = [
    0b11010101_00000011_00111111_11011111 = [Lit("SY")] => [];
    0b11010101_00000011_00110000_11011111 = [Imm] => [Ubits(8, 4)];
    0b11010101_00000011_00111111_11011111 = [] => [];
]
"ld1" = [
    // LD1 (multiple structures)
    0b00001100_01000000_01110000_00000000 = [RegList(1, BYTE), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_01000000_01110100_00000000 = [RegList(1, WORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_01000000_01111000_00000000 = [RegList(1, DWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_01000000_01111100_00000000 = [RegList(1, QWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_01000000_10100000_00000000 = [RegList(2, BYTE), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_01000000_10100100_00000000 = [RegList(2, WORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_01000000_10101000_00000000 = [RegList(2, DWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_01000000_10101100_00000000 = [RegList(2, QWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_01000000_01100000_00000000 = [RegList(3, BYTE), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_01000000_01100100_00000000 = [RegList(3, WORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_01000000_01101000_00000000 = [RegList(3, DWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_01000000_01101100_00000000 = [RegList(3, QWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_01000000_00100000_00000000 = [RegList(4, BYTE), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_01000000_00100100_00000000 = [RegList(4, WORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_01000000_00101000_00000000 = [RegList(4, DWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_01000000_00101100_00000000 = [RegList(4, QWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_11011111_01110000_00000000 = [RegListSized(1, BYTE, 8), RefBase, LitInt(8)] => [R(0), R(5)];
    0b00001100_11011111_01110100_00000000 = [RegListSized(1, WORD, 4), RefBase, LitInt(8)] => [R(0), R(5)];
    0b00001100_11011111_01111000_00000000 = [RegListSized(1, DWORD, 2), RefBase, LitInt(8)] => [R(0), R(5)];
    0b00001100_11011111_01111100_00000000 = [RegListSized(1, QWORD, 1), RefBase, LitInt(8)] => [R(0), R(5)];
    0b01001100_11011111_01110000_00000000 = [RegListSized(1, BYTE, 16), RefBase, LitInt(16)] => [R(0), R(5)];
    0b01001100_11011111_01110100_00000000 = [RegListSized(1, WORD, 8), RefBase, LitInt(16)] => [R(0), R(5)];
    0b01001100_11011111_01111000_00000000 = [RegListSized(1, DWORD, 4), RefBase, LitInt(16)] => [R(0), R(5)];
    0b01001100_11011111_01111100_00000000 = [RegListSized(1, QWORD, 2), RefBase, LitInt(16)] => [R(0), R(5)];
    0b00001100_11000000_01110000_00000000 = [RegList(1, BYTE), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_11000000_01110100_00000000 = [RegList(1, WORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_11000000_01111000_00000000 = [RegList(1, DWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_11000000_01111100_00000000 = [RegList(1, QWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_11011111_10100000_00000000 = [RegListSized(2, BYTE, 8), RefBase, LitInt(16)] => [R(0), R(5)];
    0b00001100_11011111_10100100_00000000 = [RegListSized(2, WORD, 4), RefBase, LitInt(16)] => [R(0), R(5)];
    0b00001100_11011111_10101000_00000000 = [RegListSized(2, DWORD, 2), RefBase, LitInt(16)] => [R(0), R(5)];
    0b00001100_11011111_10101100_00000000 = [RegListSized(2, QWORD, 1), RefBase, LitInt(16)] => [R(0), R(5)];
    0b01001100_11011111_10100000_00000000 = [RegListSized(2, BYTE, 16), RefBase, LitInt(32)] => [R(0), R(5)];
    0b01001100_11011111_10100100_00000000 = [RegListSized(2, WORD, 8), RefBase, LitInt(32)] => [R(0), R(5)];
    0b01001100_11011111_10101000_00000000 = [RegListSized(2, DWORD, 4), RefBase, LitInt(32)] => [R(0), R(5)];
    0b01001100_11011111_10101100_00000000 = [RegListSized(2, QWORD, 2), RefBase, LitInt(32)] => [R(0), R(5)];
    0b00001100_11000000_10100000_00000000 = [RegList(2, BYTE), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_11000000_10100100_00000000 = [RegList(2, WORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_11000000_10101000_00000000 = [RegList(2, DWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_11000000_10101100_00000000 = [RegList(2, QWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_11011111_01100000_00000000 = [RegListSized(3, BYTE, 8), RefBase, LitInt(24)] => [R(0), R(5)];
    0b00001100_11011111_01100100_00000000 = [RegListSized(3, WORD, 4), RefBase, LitInt(24)] => [R(0), R(5)];
    0b00001100_11011111_01101000_00000000 = [RegListSized(3, DWORD, 2), RefBase, LitInt(24)] => [R(0), R(5)];
    0b00001100_11011111_01101100_00000000 = [RegListSized(3, QWORD, 1), RefBase, LitInt(24)] => [R(0), R(5)];
    0b01001100_11011111_01100000_00000000 = [RegListSized(3, BYTE, 16), RefBase, LitInt(48)] => [R(0), R(5)];
    0b01001100_11011111_01100100_00000000 = [RegListSized(3, WORD, 8), RefBase, LitInt(48)] => [R(0), R(5)];
    0b01001100_11011111_01101000_00000000 = [RegListSized(3, DWORD, 4), RefBase, LitInt(48)] => [R(0), R(5)];
    0b01001100_11011111_01101100_00000000 = [RegListSized(3, QWORD, 2), RefBase, LitInt(48)] => [R(0), R(5)];
    0b00001100_11000000_01100000_00000000 = [RegList(3, BYTE), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_11000000_01100100_00000000 = [RegList(3, WORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_11000000_01101000_00000000 = [RegList(3, DWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_11000000_01101100_00000000 = [RegList(3, QWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_11011111_00100000_00000000 = [RegListSized(4, BYTE, 8), RefBase, LitInt(32)] => [R(0), R(5)];
    0b00001100_11011111_00100100_00000000 = [RegListSized(4, WORD, 4), RefBase, LitInt(32)] => [R(0), R(5)];
    0b00001100_11011111_00101000_00000000 = [RegListSized(4, DWORD, 2), RefBase, LitInt(32)] => [R(0), R(5)];
    0b00001100_11011111_00101100_00000000 = [RegListSized(4, QWORD, 1), RefBase, LitInt(32)] => [R(0), R(5)];
    0b01001100_11011111_00100000_00000000 = [RegListSized(4, BYTE, 16), RefBase, LitInt(64)] => [R(0), R(5)];
    0b01001100_11011111_00100100_00000000 = [RegListSized(4, WORD, 8), RefBase, LitInt(64)] => [R(0), R(5)];
    0b01001100_11011111_00101000_00000000 = [RegListSized(4, DWORD, 4), RefBase, LitInt(64)] => [R(0), R(5)];
    0b01001100_11011111_00101100_00000000 = [RegListSized(4, QWORD, 2), RefBase, LitInt(64)] => [R(0), R(5)];
    0b00001100_11000000_00100000_00000000 = [RegList(4, BYTE), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_11000000_00100100_00000000 = [RegList(4, WORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_11000000_00101000_00000000 = [RegList(4, DWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_11000000_00101100_00000000 = [RegList(4, QWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    // LD1 (single structure)
    0b00001101_01000000_00000000_00000000 = [RegListLanes(1, BYTE), RefBase] => [R(0), Ufields(&[30, 12, 11, 10]), R(5)];
    0b00001101_01000000_01000000_00000000 = [RegListLanes(1, WORD), RefBase] => [R(0), Ufields(&[30, 12, 11]), R(5)];
    0b00001101_01000000_10000000_00000000 = [RegListLanes(1, DWORD), RefBase] => [R(0), Ufields(&[30, 12]), R(5)];
    0b00001101_01000000_10000100_00000000 = [RegListLanes(1, QWORD), RefBase] => [R(0), Ufields(&[30]), R(5)];
    0b00001101_11011111_00000000_00000000 = [RegListLanes(1, BYTE), RefBase, LitInt(1)] => [R(0), Ufields(&[30, 12, 11, 10]), R(5)];
    0b00001101_11000000_00000000_00000000 = [RegListLanes(1, BYTE), RefBase, X] => [R(0), Ufields(&[30, 12, 11, 10]), R(5), R(16)];
    0b00001101_11011111_01000000_00000000 = [RegListLanes(1, WORD), RefBase, LitInt(2)] => [R(0), Ufields(&[30, 12, 11]), R(5)];
    0b00001101_11000000_01000000_00000000 = [RegListLanes(1, WORD), RefBase, X] => [R(0), Ufields(&[30, 12, 11]), R(5), R(16)];
    0b00001101_11011111_10000000_00000000 = [RegListLanes(1, DWORD), RefBase, LitInt(4)] => [R(0), Ufields(&[30, 12]), R(5)];
    0b00001101_11000000_10000000_00000000 = [RegListLanes(1, DWORD), RefBase, X] => [R(0), Ufields(&[30, 12]), R(5), R(16)];
    0b00001101_11011111_10000100_00000000 = [RegListLanes(1, QWORD), RefBase, LitInt(8)] => [R(0), Ufields(&[30]), R(5)];
    0b00001101_11000000_10000100_00000000 = [RegListLanes(1, QWORD), RefBase, X] => [R(0), Ufields(&[30]), R(5), R(16)];
]
"ld1r" = [
    0b00001101_01000000_11000000_00000000 = [RegList(1, BYTE), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001101_01000000_11000100_00000000 = [RegList(1, WORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001101_01000000_11001000_00000000 = [RegList(1, DWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001101_01000000_11001100_00000000 = [RegList(1, QWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001101_11011111_11000000_00000000 = [RegList(1, BYTE), RefBase, LitInt(1)] => [R(0), R(5), Rwidth(30)];
    0b00001101_11011111_11000100_00000000 = [RegList(1, WORD), RefBase, LitInt(2)] => [R(0), R(5), Rwidth(30)];
    0b00001101_11011111_11001000_00000000 = [RegList(1, DWORD), RefBase, LitInt(4)] => [R(0), R(5), Rwidth(30)];
    0b00001101_11011111_11001100_00000000 = [RegList(1, QWORD), RefBase, LitInt(8)] => [R(0), R(5), Rwidth(30)];
    0b00001101_11000000_11000000_00000000 = [RegList(1, BYTE), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001101_11000000_11000100_00000000 = [RegList(1, WORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001101_11000000_11001000_00000000 = [RegList(1, DWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001101_11000000_11001100_00000000 = [RegList(1, QWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
]
"ld2" = [
    // LD2 (multiple structures)
    0b00001100_01000000_10000000_00000000 = [RegList(2, BYTE), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_01000000_10000100_00000000 = [RegList(2, WORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_01000000_10001000_00000000 = [RegList(2, DWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_01000000_10001100_00000000 = [RegListSized(2, QWORD, 2), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_11011111_10000000_00000000 = [RegListSized(2, BYTE, 8), RefBase, LitInt(16)] => [R(0), R(5)];
    0b00001100_11011111_10000100_00000000 = [RegListSized(2, WORD, 4), RefBase, LitInt(16)] => [R(0), R(5)];
    0b00001100_11011111_10001000_00000000 = [RegListSized(2, DWORD, 2), RefBase, LitInt(16)] => [R(0), R(5)];
    0b01001100_11011111_10000000_00000000 = [RegListSized(2, BYTE, 16), RefBase, LitInt(32)] => [R(0), R(5)];
    0b01001100_11011111_10000100_00000000 = [RegListSized(2, WORD, 8), RefBase, LitInt(32)] => [R(0), R(5)];
    0b01001100_11011111_10001000_00000000 = [RegListSized(2, DWORD, 4), RefBase, LitInt(32)] => [R(0), R(5)];
    0b01001100_11011111_10001100_00000000 = [RegListSized(2, QWORD, 2), RefBase, LitInt(32)] => [R(0), R(5)];
    0b00001100_11000000_10000000_00000000 = [RegList(2, BYTE), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_11000000_10000100_00000000 = [RegList(2, WORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_11000000_10001000_00000000 = [RegList(2, DWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_11000000_10001100_00000000 = [RegListSized(2, QWORD, 2), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    // LD2 (single structure)
    0b00001101_01100000_00000000_00000000 = [RegListLanes(2, BYTE), RefBase] => [R(0), Ufields(&[30, 12, 11, 10]), R(5)];
    0b00001101_01100000_01000000_00000000 = [RegListLanes(2, WORD), RefBase] => [R(0), Ufields(&[30, 12, 11]), R(5)];
    0b00001101_01100000_10000000_00000000 = [RegListLanes(2, DWORD), RefBase] => [R(0), Ufields(&[30, 12]), R(5)];
    0b00001101_01100000_10000100_00000000 = [RegListLanes(2, QWORD), RefBase] => [R(0), Ufields(&[30]), R(5)];
    0b00001101_11111111_00000000_00000000 = [RegListLanes(2, BYTE), RefBase, LitInt(2)] => [R(0), Ufields(&[30, 12, 11, 10]), R(5)];
    0b00001101_11100000_00000000_00000000 = [RegListLanes(2, BYTE), RefBase, X] => [R(0), Ufields(&[30, 12, 11, 10]), R(5), R(16)];
    0b00001101_11111111_01000000_00000000 = [RegListLanes(2, WORD), RefBase, LitInt(4)] => [R(0), Ufields(&[30, 12, 11]), R(5)];
    0b00001101_11100000_01000000_00000000 = [RegListLanes(2, WORD), RefBase, X] => [R(0), Ufields(&[30, 12, 11]), R(5), R(16)];
    0b00001101_11111111_10000000_00000000 = [RegListLanes(2, DWORD), RefBase, LitInt(8)] => [R(0), Ufields(&[30, 12]), R(5)];
    0b00001101_11100000_10000000_00000000 = [RegListLanes(2, DWORD), RefBase, X] => [R(0), Ufields(&[30, 12]), R(5), R(16)];
    0b00001101_11111111_10000100_00000000 = [RegListLanes(2, QWORD), RefBase, LitInt(16)] => [R(0), Ufields(&[30]), R(5)];
    0b00001101_11100000_10000100_00000000 = [RegListLanes(2, QWORD), RefBase, X] => [R(0), Ufields(&[30]), R(5), R(16)];
]
"ld2r" = [
    0b00001101_01100000_11000000_00000000 = [RegList(2, BYTE), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001101_01100000_11000100_00000000 = [RegList(2, WORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001101_01100000_11001000_00000000 = [RegList(2, DWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001101_01100000_11001100_00000000 = [RegList(2, QWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001101_11111111_11000000_00000000 = [RegList(2, BYTE), RefBase, LitInt(2)] => [R(0), R(5), Rwidth(30)];
    0b00001101_11111111_11000100_00000000 = [RegList(2, WORD), RefBase, LitInt(4)] => [R(0), R(5), Rwidth(30)];
    0b00001101_11111111_11001000_00000000 = [RegList(2, DWORD), RefBase, LitInt(8)] => [R(0), R(5), Rwidth(30)];
    0b00001101_11111111_11001100_00000000 = [RegList(2, QWORD), RefBase, LitInt(16)] => [R(0), R(5), Rwidth(30)];
    0b00001101_11100000_11000000_00000000 = [RegList(2, BYTE), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001101_11100000_11000100_00000000 = [RegList(2, WORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001101_11100000_11001000_00000000 = [RegList(2, DWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001101_11100000_11001100_00000000 = [RegList(2, QWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
]
"ld3" = [
    // LD3 (multiple structures)
    0b00001100_01000000_01000000_00000000 = [RegList(3, BYTE), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_01000000_01000100_00000000 = [RegList(3, WORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_01000000_01001000_00000000 = [RegList(3, DWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_01000000_01001100_00000000 = [RegListSized(3, QWORD, 2), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_11011111_01000000_00000000 = [RegListSized(3, BYTE, 8), RefBase, LitInt(24)] => [R(0), R(5)];
    0b00001100_11011111_01000100_00000000 = [RegListSized(3, WORD, 4), RefBase, LitInt(24)] => [R(0), R(5)];
    0b00001100_11011111_01001000_00000000 = [RegListSized(3, DWORD, 2), RefBase, LitInt(24)] => [R(0), R(5)];
    0b01001100_11011111_01000000_00000000 = [RegListSized(3, BYTE, 16), RefBase, LitInt(48)] => [R(0), R(5)];
    0b01001100_11011111_01000100_00000000 = [RegListSized(3, WORD, 8), RefBase, LitInt(48)] => [R(0), R(5)];
    0b01001100_11011111_01001000_00000000 = [RegListSized(3, DWORD, 4), RefBase, LitInt(48)] => [R(0), R(5)];
    0b01001100_11011111_01001100_00000000 = [RegListSized(3, QWORD, 2), RefBase, LitInt(48)] => [R(0), R(5)];
    0b00001100_11000000_01000000_00000000 = [RegList(3, BYTE), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_11000000_01000100_00000000 = [RegList(3, WORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_11000000_01001000_00000000 = [RegList(3, DWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_11000000_01001100_00000000 = [RegListSized(3, QWORD, 2), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    // LD3 (single structure)
    0b00001101_01000000_00100000_00000000 = [RegListLanes(3, BYTE), RefBase] => [R(0), Ufields(&[30, 12, 11, 10]), R(5)];
    0b00001101_01000000_01100000_00000000 = [RegListLanes(3, WORD), RefBase] => [R(0), Ufields(&[30, 12, 11]), R(5)];
    0b00001101_01000000_10100000_00000000 = [RegListLanes(3, DWORD), RefBase] => [R(0), Ufields(&[30, 12]), R(5)];
    0b00001101_01000000_10100100_00000000 = [RegListLanes(3, QWORD), RefBase] => [R(0), Ufields(&[30]), R(5)];
    0b00001101_11011111_00100000_00000000 = [RegListLanes(3, BYTE), RefBase, LitInt(3)] => [R(0), Ufields(&[30, 12, 11, 10]), R(5)];
    0b00001101_11000000_00100000_00000000 = [RegListLanes(3, BYTE), RefBase, X] => [R(0), Ufields(&[30, 12, 11, 10]), R(5), R(16)];
    0b00001101_11011111_01100000_00000000 = [RegListLanes(3, WORD), RefBase, LitInt(6)] => [R(0), Ufields(&[30, 12, 11]), R(5)];
    0b00001101_11000000_01100000_00000000 = [RegListLanes(3, WORD), RefBase, X] => [R(0), Ufields(&[30, 12, 11]), R(5), R(16)];
    0b00001101_11011111_10100000_00000000 = [RegListLanes(3, DWORD), RefBase, LitInt(12)] => [R(0), Ufields(&[30, 12]), R(5)];
    0b00001101_11000000_10100000_00000000 = [RegListLanes(3, DWORD), RefBase, X] => [R(0), Ufields(&[30, 12]), R(5), R(16)];
    0b00001101_11011111_10100100_00000000 = [RegListLanes(3, QWORD), RefBase, LitInt(24)] => [R(0), Ufields(&[30]), R(5)];
    0b00001101_11000000_10100100_00000000 = [RegListLanes(3, QWORD), RefBase, X] => [R(0), Ufields(&[30]), R(5), R(16)];
]
"ld3r" = [
    0b00001101_01000000_11100000_00000000 = [RegList(3, BYTE), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001101_01000000_11100100_00000000 = [RegList(3, WORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001101_01000000_11101000_00000000 = [RegList(3, DWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001101_01000000_11101100_00000000 = [RegList(3, QWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001101_11011111_11100000_00000000 = [RegList(3, BYTE), RefBase, LitInt(3)] => [R(0), R(5), Rwidth(30)];
    0b00001101_11011111_11100100_00000000 = [RegList(3, WORD), RefBase, LitInt(6)] => [R(0), R(5), Rwidth(30)];
    0b00001101_11011111_11101000_00000000 = [RegList(3, DWORD), RefBase, LitInt(12)] => [R(0), R(5), Rwidth(30)];
    0b00001101_11011111_11101100_00000000 = [RegList(3, QWORD), RefBase, LitInt(24)] => [R(0), R(5), Rwidth(30)];
    0b00001101_11000000_11100000_00000000 = [RegList(3, BYTE), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001101_11000000_11100100_00000000 = [RegList(3, WORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001101_11000000_11101000_00000000 = [RegList(3, DWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001101_11000000_11101100_00000000 = [RegList(3, QWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
]
"ld4" = [
    // LD4 (multiple structures)
    0b00001100_01000000_00000000_00000000 = [RegList(4, BYTE), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_01000000_00000100_00000000 = [RegList(4, WORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_01000000_00001000_00000000 = [RegList(4, DWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_01000000_00001100_00000000 = [RegListSized(4, QWORD, 2), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_11011111_00000000_00000000 = [RegListSized(4, BYTE, 8), RefBase, LitInt(32)] => [R(0), R(5)];
    0b00001100_11011111_00000100_00000000 = [RegListSized(4, WORD, 4), RefBase, LitInt(32)] => [R(0), R(5)];
    0b00001100_11011111_00001000_00000000 = [RegListSized(4, DWORD, 2), RefBase, LitInt(32)] => [R(0), R(5)];
    0b01001100_11011111_00000000_00000000 = [RegListSized(4, BYTE, 16), RefBase, LitInt(64)] => [R(0), R(5)];
    0b01001100_11011111_00000100_00000000 = [RegListSized(4, WORD, 8), RefBase, LitInt(64)] => [R(0), R(5)];
    0b01001100_11011111_00001000_00000000 = [RegListSized(4, DWORD, 4), RefBase, LitInt(64)] => [R(0), R(5)];
    0b01001100_11011111_00001100_00000000 = [RegListSized(4, QWORD, 2), RefBase, LitInt(64)] => [R(0), R(5)];
    0b00001100_11000000_00000000_00000000 = [RegList(4, BYTE), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_11000000_00000100_00000000 = [RegList(4, WORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_11000000_00001000_00000000 = [RegList(4, DWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_11000000_00001100_00000000 = [RegListSized(4, QWORD, 2), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    // LD4 (single structure)
    0b00001101_01100000_00100000_00000000 = [RegListLanes(4, BYTE), RefBase] => [R(0), Ufields(&[30, 12, 11, 10]), R(5)];
    0b00001101_01100000_01100000_00000000 = [RegListLanes(4, WORD), RefBase] => [R(0), Ufields(&[30, 12, 11]), R(5)];
    0b00001101_01100000_10100000_00000000 = [RegListLanes(4, DWORD), RefBase] => [R(0), Ufields(&[30, 12]), R(5)];
    0b00001101_01100000_10100100_00000000 = [RegListLanes(4, QWORD), RefBase] => [R(0), Ufields(&[30]), R(5)];
    0b00001101_11111111_00100000_00000000 = [RegListLanes(4, BYTE), RefBase, LitInt(4)] => [R(0), Ufields(&[30, 12, 11, 10]), R(5)];
    0b00001101_11100000_00100000_00000000 = [RegListLanes(4, BYTE), RefBase, X] => [R(0), Ufields(&[30, 12, 11, 10]), R(5), R(16)];
    0b00001101_11111111_01100000_00000000 = [RegListLanes(4, WORD), RefBase, LitInt(8)] => [R(0), Ufields(&[30, 12, 11]), R(5)];
    0b00001101_11100000_01100000_00000000 = [RegListLanes(4, WORD), RefBase, X] => [R(0), Ufields(&[30, 12, 11]), R(5), R(16)];
    0b00001101_11111111_10100000_00000000 = [RegListLanes(4, DWORD), RefBase, LitInt(16)] => [R(0), Ufields(&[30, 12]), R(5)];
    0b00001101_11100000_10100000_00000000 = [RegListLanes(4, DWORD), RefBase, X] => [R(0), Ufields(&[30, 12]), R(5), R(16)];
    0b00001101_11111111_10100100_00000000 = [RegListLanes(4, QWORD), RefBase, LitInt(32)] => [R(0), Ufields(&[30]), R(5)];
    0b00001101_11100000_10100100_00000000 = [RegListLanes(4, QWORD), RefBase, X] => [R(0), Ufields(&[30]), R(5), R(16)];
]
"ld4r" = [
    0b00001101_01100000_11100000_00000000 = [RegList(4, BYTE), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001101_01100000_11100100_00000000 = [RegList(4, WORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001101_01100000_11101000_00000000 = [RegList(4, DWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001101_01100000_11101100_00000000 = [RegList(4, QWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001101_11111111_11100000_00000000 = [RegList(4, BYTE), RefBase, LitInt(4)] => [R(0), R(5), Rwidth(30)];
    0b00001101_11111111_11100100_00000000 = [RegList(4, WORD), RefBase, LitInt(8)] => [R(0), R(5), Rwidth(30)];
    0b00001101_11111111_11101000_00000000 = [RegList(4, DWORD), RefBase, LitInt(16)] => [R(0), R(5), Rwidth(30)];
    0b00001101_11111111_11101100_00000000 = [RegList(4, QWORD), RefBase, LitInt(32)] => [R(0), R(5), Rwidth(30)];
    0b00001101_11100000_11100000_00000000 = [RegList(4, BYTE), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001101_11100000_11100100_00000000 = [RegList(4, WORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001101_11100000_11101000_00000000 = [RegList(4, DWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001101_11100000_11101100_00000000 = [RegList(4, QWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
]
"ldadd" = [
    0b10111000_00100000_00000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_00100000_00000000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"ldadda" = [
    0b10111000_10100000_00000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_10100000_00000000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"ldaddab" = [
    0b00111000_10100000_00000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldaddah" = [
    0b01111000_10100000_00000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldaddal" = [
    0b10111000_11100000_00000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_11100000_00000000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"ldaddalb" = [
    0b00111000_11100000_00000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldaddalh" = [
    0b01111000_11100000_00000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldaddb" = [
    0b00111000_00100000_00000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldaddh" = [
    0b01111000_00100000_00000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldaddl" = [
    0b10111000_01100000_00000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_01100000_00000000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"ldaddlb" = [
    0b00111000_01100000_00000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldaddlh" = [
    0b01111000_01100000_00000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldapr" = [
    0b10111000_10111111_11000000_00000000 = [W, RefBase] => [R(0), R(5)];
    0b11111000_10111111_11000000_00000000 = [X, RefBase] => [R(0), R(5)];
]
"ldaprb" = [
    0b00111000_10111111_11000000_00000000 = [W, RefBase] => [R(0), R(5)];
]
"ldaprh" = [
    0b01111000_10111111_11000000_00000000 = [W, RefBase] => [R(0), R(5)];
]
"ldapur" = [
    0b10011001_01000000_00000000_00000000 = [W, RefOffset] => [R(0), R(5), Sbits(12, 9)];
    0b11011001_01000000_00000000_00000000 = [X, RefOffset] => [R(0), R(5), Sbits(12, 9)];
]
"ldapurb" = [
    0b00011001_01000000_00000000_00000000 = [W, RefOffset] => [R(0), R(5), Sbits(12, 9)];
]
"ldapurh" = [
    0b01011001_01000000_00000000_00000000 = [W, RefOffset] => [R(0), R(5), Sbits(12, 9)];
]
"ldapursb" = [
    0b00011001_11000000_00000000_00000000 = [W, RefOffset] => [R(0), R(5), Sbits(12, 9)];
    0b00011001_10000000_00000000_00000000 = [X, RefOffset] => [R(0), R(5), Sbits(12, 9)];
]
"ldapursh" = [
    0b01011001_11000000_00000000_00000000 = [W, RefOffset] => [R(0), R(5), Sbits(12, 9)];
    0b01011001_10000000_00000000_00000000 = [X, RefOffset] => [R(0), R(5), Sbits(12, 9)];
]
"ldapursw" = [
    0b10011001_10000000_00000000_00000000 = [X, RefOffset] => [R(0), R(5), Sbits(12, 9)];
]
"ldar" = [
    0b10001000_11011111_11111100_00000000 = [W, RefBase] => [R(0), R(5)];
    0b11001000_11011111_11111100_00000000 = [X, RefBase] => [R(0), R(5)];
]
"ldarb" = [
    0b00001000_11011111_11111100_00000000 = [W, RefBase] => [R(0), R(5)];
]
"ldarh" = [
    0b01001000_11011111_11111100_00000000 = [W, RefBase] => [R(0), R(5)];
]
"ldaxp" = [
    0b10001000_01111111_10000000_00000000 = [W, W, RefBase] => [R(0), R(10), R(5)];
    0b11001000_01111111_10000000_00000000 = [X, X, RefBase] => [R(0), R(10), R(5)];
]
"ldaxr" = [
    0b10001000_01011111_11111100_00000000 = [W, RefBase] => [R(0), R(5)];
    0b11001000_01011111_11111100_00000000 = [X, RefBase] => [R(0), R(5)];
]
"ldaxrb" = [
    0b00001000_01011111_11111100_00000000 = [W, RefBase] => [R(0), R(5)];
]
"ldaxrh" = [
    0b01001000_01011111_11111100_00000000 = [W, RefBase] => [R(0), R(5)];
]
"ldclr" = [
    0b10111000_00100000_00010000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_00100000_00010000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"ldclra" = [
    0b10111000_10100000_00010000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_10100000_00010000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"ldclrab" = [
    0b00111000_10100000_00010000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldclrah" = [
    0b01111000_10100000_00010000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldclral" = [
    0b10111000_11100000_00010000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_11100000_00010000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"ldclralb" = [
    0b00111000_11100000_00010000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldclralh" = [
    0b01111000_11100000_00010000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldclrb" = [
    0b00111000_00100000_00010000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldclrh" = [
    0b01111000_00100000_00010000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldclrl" = [
    0b10111000_01100000_00010000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_01100000_00010000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"ldclrlb" = [
    0b00111000_01100000_00010000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldclrlh" = [
    0b01111000_01100000_00010000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldeor" = [
    0b10111000_00100000_00100000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_00100000_00100000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"ldeora" = [
    0b10111000_10100000_00100000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_10100000_00100000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"ldeorab" = [
    0b00111000_10100000_00100000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldeorah" = [
    0b01111000_10100000_00100000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldeoral" = [
    0b10111000_11100000_00100000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_11100000_00100000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"ldeoralb" = [
    0b00111000_11100000_00100000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldeoralh" = [
    0b01111000_11100000_00100000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldeorb" = [
    0b00111000_00100000_00100000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldeorh" = [
    0b01111000_00100000_00100000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldeorl" = [
    0b10111000_01100000_00100000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_01100000_00100000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"ldeorlb" = [
    0b00111000_01100000_00100000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldeorlh" = [
    0b01111000_01100000_00100000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldlar" = [
    0b10001000_11011111_01111100_00000000 = [W, RefBase] => [R(0), R(5)];
    0b11001000_11011111_01111100_00000000 = [X, RefBase] => [R(0), R(5)];
]
"ldlarb" = [
    0b00001000_11011111_01111100_00000000 = [W, RefBase] => [R(0), R(5)];
]
"ldlarh" = [
    0b01001000_11011111_01111100_00000000 = [W, RefBase] => [R(0), R(5)];
]
"ldnp" = [
    // LDNP (SIMD&FP)
    0b00101100_01000000_00000000_00000000 = [S, S, RefOffset] => [R(0), R(10), R(5), Sscaled(15, 7, 2)];
    0b01101100_01000000_00000000_00000000 = [D, D, RefOffset] => [R(0), R(10), R(5), Sscaled(15, 7, 3)];
    0b10101100_01000000_00000000_00000000 = [Q, Q, RefOffset] => [R(0), R(10), R(5), Sscaled(15, 7, 4)];
    // LDNP
    0b00101000_01000000_00000000_00000000 = [W, W, RefOffset] => [R(0), R(10), R(5), Sscaled(15, 7, 2)];
    0b10101000_01000000_00000000_00000000 = [X, X, RefOffset] => [R(0), R(10), R(5), Sscaled(15, 7, 3)];
]
"ldp" = [
    // LDP (SIMD&FP)
    0b00101100_11000000_00000000_00000000 = [S, S, RefBase, Imm] => [R(0), R(10), R(5), Sscaled(15, 7, 2)];
    0b01101100_11000000_00000000_00000000 = [D, D, RefBase, Imm] => [R(0), R(10), R(5), Sscaled(15, 7, 3)];
    0b10101100_11000000_00000000_00000000 = [Q, Q, RefBase, Imm] => [R(0), R(10), R(5), Sscaled(15, 7, 4)];
    0b00101101_11000000_00000000_00000000 = [S, S, RefPre] => [R(0), R(10), R(5), Sscaled(15, 7, 2)];
    0b01101101_11000000_00000000_00000000 = [D, D, RefPre] => [R(0), R(10), R(5), Sscaled(15, 7, 3)];
    0b10101101_11000000_00000000_00000000 = [Q, Q, RefPre] => [R(0), R(10), R(5), Sscaled(15, 7, 4)];
    0b00101101_01000000_00000000_00000000 = [S, S, RefOffset] => [R(0), R(10), R(5), Sscaled(15, 7, 2)];
    0b01101101_01000000_00000000_00000000 = [D, D, RefOffset] => [R(0), R(10), R(5), Sscaled(15, 7, 3)];
    0b10101101_01000000_00000000_00000000 = [Q, Q, RefOffset] => [R(0), R(10), R(5), Sscaled(15, 7, 4)];
    // LDP
    0b00101000_11000000_00000000_00000000 = [W, W, RefBase, Imm] => [R(0), R(10), R(5), Sscaled(15, 7, 2)];
    0b10101000_11000000_00000000_00000000 = [X, X, RefBase, Imm] => [R(0), R(10), R(5), Sscaled(15, 7, 3)];
    0b00101001_11000000_00000000_00000000 = [W, W, RefPre] => [R(0), R(10), R(5), Sscaled(15, 7, 2)];
    0b10101001_11000000_00000000_00000000 = [X, X, RefPre] => [R(0), R(10), R(5), Sscaled(15, 7, 3)];
    0b00101001_01000000_00000000_00000000 = [W, W, RefOffset] => [R(0), R(10), R(5), Sscaled(15, 7, 2)];
    0b10101001_01000000_00000000_00000000 = [X, X, RefOffset] => [R(0), R(10), R(5), Sscaled(15, 7, 3)];
]
"ldpsw" = [
    0b01101000_11000000_00000000_00000000 = [X, X, RefBase, Imm] => [R(0), R(10), R(5), Sscaled(15, 7, 3)];
    0b01101001_11000000_00000000_00000000 = [X, X, RefPre] => [R(0), R(10), R(5), Sscaled(15, 7, 3)];
    0b01101001_01000000_00000000_00000000 = [X, X, RefOffset] => [R(0), R(10), R(5), Sscaled(15, 7, 3)];
]
"ldr" = [
    // LDR (immediate, SIMD&FP)
    0b00111100_01000000_00000100_00000000 = [B, RefBase, Imm] => [R(0), R(5), Sbits(12, 9)];
    0b01111100_01000000_00000100_00000000 = [H, RefBase, Imm] => [R(0), R(5), Sbits(12, 9)];
    0b10111100_01000000_00000100_00000000 = [S, RefBase, Imm] => [R(0), R(5), Sbits(12, 9)];
    0b11111100_01000000_00000100_00000000 = [B, RefBase, Imm] => [R(0), R(5), Sbits(12, 9)];
    0b00111100_11000000_00000100_00000000 = [Q, RefBase, Imm] => [R(0), R(5), Sbits(12, 9)];
    0b00111100_01000000_00001100_00000000 = [B, RefPre] => [R(0), R(5), Sbits(12, 9)];
    0b01111100_01000000_00001100_00000000 = [H, RefPre] => [R(0), R(5), Sbits(12, 9)];
    0b10111100_01000000_00001100_00000000 = [S, RefPre] => [R(0), R(5), Sbits(12, 9)];
    0b11111100_01000000_00001100_00000000 = [D, RefPre] => [R(0), R(5), Sbits(12, 9)];
    0b00111100_11000000_00001100_00000000 = [Q, RefPre] => [R(0), R(5), Sbits(12, 9)];
    0b00111101_01000000_00000000_00000000 = [B, RefOffset] => [R(0), R(5), Ubits(10, 12)];
    0b01111101_01000000_00000000_00000000 = [H, RefOffset] => [R(0), R(5), Uscaled(10, 12, 1)];
    0b10111101_01000000_00000000_00000000 = [Q, RefOffset] => [R(0), R(5), Uscaled(10, 12, 2)];
    0b11111101_01000000_00000000_00000000 = [B, RefOffset] => [R(0), R(5), Uscaled(10, 12, 3)];
    0b00111101_11000000_00000000_00000000 = [Q, RefOffset] => [R(0), R(5), Uscaled(10, 12, 4)];
    // LDR (immediate)
    0b10111000_01000000_00000100_00000000 = [W, RefBase, Imm] => [R(0), R(5), Sbits(12, 9)];
    0b11111000_01000000_00000100_00000000 = [X, RefBase, Imm] => [R(0), R(5), Sbits(12, 9)];
    0b10111000_01000000_00001100_00000000 = [W, RefPre] => [R(0), R(5), Sbits(12, 9)];
    0b11111000_01000000_00001100_00000000 = [X, RefPre] => [R(0), R(5), Sbits(12, 9)];
    0b10111001_01000000_00000000_00000000 = [W, RefOffset] => [R(0), R(5), Uscaled(10, 12, 2)];
    0b11111001_01000000_00000000_00000000 = [X, RefOffset] => [R(0), R(5), Uscaled(10, 12, 3)];
    // LDR (literal, SIMD&FP)
    0b00011100_00000000_00000000_00000000 = [S, Offset] => [R(0), Sscaled(5, 19, 2)];
    0b01011100_00000000_00000000_00000000 = [D, Offset] => [R(0), Sscaled(5, 19, 2)];
    0b10011100_00000000_00000000_00000000 = [Q, Offset] => [R(0), Sscaled(5, 19, 2)];
    // LDR (literal)
    0b00011000_00000000_00000000_00000000 = [W, Offset] => [R(0), Sscaled(5, 19, 2)];
    0b01011000_00000000_00000000_00000000 = [X, Offset] => [R(0), Sscaled(5, 19, 2)];
    // LDR (register, SIMD&FP)
    0b00111100_01100000_00001000_00000000 = [B, RefIndex] => [R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 0])];
    0b01111100_01100000_00001000_00000000 = [H, RefIndex] => [R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 1])];
    0b10111100_01100000_00001000_00000000 = [S, RefIndex] => [R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 2])];
    0b11111100_01100000_00001000_00000000 = [D, RefIndex] => [R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 3])];
    0b00111100_11100000_00001000_00000000 = [Q, RefIndex] => [R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 4])];
    // LDR (register)
    0b10111000_01100000_00001000_00000000 = [W, RefIndex] => [R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 2])];
    0b11111000_01100000_00001000_00000000 = [X, RefIndex] => [R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 3])];
]
"ldraa" = [
    0b11111000_00100000_00000100_00000000 = [X, RefOffset] => [R(0), R(5), BSbits(10), BSslice(12, 9, 0), BSslice(22, 1, 9), A];
    0b11111000_00100000_00001100_00000000 = [X, RefPre] => [R(0), R(5), BSbits(10), BSslice(12, 9, 0), BSslice(22, 1, 9), A];
]
"ldrab" = [
    0b11111000_10100000_00000100_00000000 = [X, RefOffset] => [R(0), R(5), BSbits(10), BSslice(12, 9, 0), BSslice(22, 1, 9), A];
    0b11111000_10100000_00001100_00000000 = [X, RefPre] => [R(0), R(5), BSbits(10), BSslice(12, 9, 0), BSslice(22, 1, 9), A];
]
"ldrb" = [
    // LDRB (immediate)
    0b00111000_01000000_00000100_00000000 = [W, RefBase, Imm] => [R(0), R(5), Sbits(12, 9)];
    0b00111000_01000000_00001100_00000000 = [W, RefPre] => [R(0), R(5), Sbits(12, 9)];
    0b00111001_01000000_00000000_00000000 = [W, RefOffset] => [R(0), R(5), Ubits(10, 12)];
    // LDRB (register)
    0b00111000_01100000_00001000_00000000 = [W, RefIndex] => [R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 0])];
]
"ldrh" = [
    // LDRH (immediate)
    0b01111000_01000000_00000100_00000000 = [W, RefBase, Imm] => [R(0), R(5), Sbits(12, 9)];
    0b01111000_01000000_00001100_00000000 = [W, RefPre] => [R(0), R(5), Sbits(12, 9)];
    0b01111001_01000000_00000000_00000000 = [W, RefOffset] => [R(0), R(5), Uscaled(10, 12, 1)];
    // LDRH (register)
    0b01111000_01100000_00001000_00000000 = [W, RefIndex] => [R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 1])];
]
"ldrsb" = [
    // LDRSB (immediate)
    0b00111000_11000000_00000100_00000000 = [W, RefBase, Imm] => [R(0), R(5), Sbits(12, 9)];
    0b00111000_10000000_00000100_00000000 = [X, RefBase, Imm] => [R(0), R(5), Sbits(12, 9)];
    0b00111000_11000000_00001100_00000000 = [W, RefPre] => [R(0), R(5), Sbits(12, 9)];
    0b00111000_10000000_00001100_00000000 = [X, RefPre] => [R(0), R(5), Sbits(12, 9)];
    0b00111001_11000000_00000000_00000000 = [W, RefOffset] => [R(0), R(5), Ubits(10, 12)];
    0b00111001_10000000_00000000_00000000 = [X, RefOffset] => [R(0), R(5), Ubits(10, 12)];
    // LDRSB (register)
    0b00111000_11100000_00001000_00000000 = [W, RefIndex] => [R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 0])];
    0b00111000_10100000_00001000_00000000 = [X, RefIndex] => [R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 0])];
]
"ldrsh" = [
    // LDRSH (immediate)
    0b01111000_11000000_00000100_00000000 = [W, RefBase, Imm] => [R(0), R(5), Sbits(12, 9)];
    0b01111000_10000000_00000100_00000000 = [X, RefBase, Imm] => [R(0), R(5), Sbits(12, 9)];
    0b01111000_11000000_00001100_00000000 = [W, RefPre] => [R(0), R(5), Sbits(12, 9)];
    0b01111000_10000000_00001100_00000000 = [X, RefPre] => [R(0), R(5), Sbits(12, 9)];
    0b01111001_11000000_00000000_00000000 = [W, RefOffset] => [R(0), R(5), Uscaled(10, 12, 1)];
    0b01111001_10000000_00000000_00000000 = [X, RefOffset] => [R(0), R(5), Uscaled(10, 12, 1)];
    // LDRSH (register)
    0b01111000_11100000_00001000_00000000 = [W, RefIndex] => [R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 1])];
    0b01111000_10100000_00001000_00000000 = [X, RefIndex] => [R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 1])];
]
"ldrsw" = [
    // LDRSW (immediate)
    0b10111000_10000000_00000100_00000000 = [X, RefBase, Imm] => [R(0), R(5), Sbits(12, 9)];
    0b10111000_10000000_00001100_00000000 = [X, RefPre] => [R(0), R(5), Sbits(12, 9)];
    0b10111001_10000000_00000000_00000000 = [X, RefOffset] => [R(0), R(5), Uscaled(10, 12, 2)];
    // LDRSW (literal)
    0b10011000_00000000_00000000_00000000 = [X, Offset] => [R(0), Sscaled(5, 19, 2)];
    // LDRSW (register)
    0b10111000_10100000_00001000_00000000 = [X, RefIndex] => [R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 2])];
]
"ldset" = [
    0b10111000_00100000_00110000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_00100000_00110000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"ldseta" = [
    0b10111000_10100000_00110000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_10100000_00110000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"ldsetab" = [
    0b00111000_10100000_00110000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldsetah" = [
    0b01111000_10100000_00110000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldsetal" = [
    0b10111000_11100000_00110000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_11100000_00110000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"ldsetalb" = [
    0b00111000_11100000_00110000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldsetalh" = [
    0b01111000_11100000_00110000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldsetb" = [
    0b00111000_00100000_00110000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldseth" = [
    0b01111000_00100000_00110000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldsetl" = [
    0b10111000_01100000_00110000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_01100000_00110000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"ldsetlb" = [
    0b00111000_01100000_00110000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldsetlh" = [
    0b01111000_01100000_00110000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldsmax" = [
    0b10111000_00100000_01000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_00100000_01000000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"ldsmaxa" = [
    0b10111000_10100000_01000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_10100000_01000000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"ldsmaxab" = [
    0b00111000_10100000_01000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldsmaxah" = [
    0b01111000_10100000_01000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldsmaxal" = [
    0b10111000_11100000_01000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_11100000_01000000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"ldsmaxalb" = [
    0b00111000_11100000_01000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldsmaxalh" = [
    0b01111000_11100000_01000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldsmaxb" = [
    0b00111000_00100000_01000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldsmaxh" = [
    0b01111000_00100000_01000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldsmaxl" = [
    0b10111000_01100000_01000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_01100000_01000000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"ldsmaxlb" = [
    0b00111000_01100000_01000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldsmaxlh" = [
    0b01111000_01100000_01000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldsmin" = [
    0b10111000_00100000_01010000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_00100000_01010000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"ldsmina" = [
    0b10111000_10100000_01010000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_10100000_01010000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"ldsminab" = [
    0b00111000_10100000_01010000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldsminah" = [
    0b01111000_10100000_01010000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldsminal" = [
    0b10111000_11100000_01010000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_11100000_01010000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"ldsminalb" = [
    0b00111000_11100000_01010000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldsminalh" = [
    0b01111000_11100000_01010000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldsminb" = [
    0b00111000_00100000_01010000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldsminh" = [
    0b01111000_00100000_01010000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldsminl" = [
    0b10111000_01100000_01010000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_01100000_01010000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"ldsminlb" = [
    0b00111000_01100000_01010000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldsminlh" = [
    0b01111000_01100000_01010000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldtr" = [
    0b10111000_01000000_00001000_00000000 = [W, RefOffset] => [R(0), R(5), Sbits(12, 9)];
    0b11111000_01000000_00001000_00000000 = [X, RefOffset] => [R(0), R(5), Sbits(12, 9)];
]
"ldtrb" = [
    0b00111000_01000000_00001000_00000000 = [W, RefOffset] => [R(0), R(5), Sbits(12, 9)];
]
"ldtrh" = [
    0b01111000_01000000_00001000_00000000 = [W, RefOffset] => [R(0), R(5), Sbits(12, 9)];
]
"ldtrsb" = [
    0b00111000_11000000_00001000_00000000 = [W, RefOffset] => [R(0), R(5), Sbits(12, 9)];
    0b00111000_10000000_00001000_00000000 = [X, RefOffset] => [R(0), R(5), Sbits(12, 9)];
]
"ldtrsh" = [
    0b01111000_11000000_00001000_00000000 = [W, RefOffset] => [R(0), R(5), Sbits(12, 9)];
    0b01111000_10000000_00001000_00000000 = [X, RefOffset] => [R(0), R(5), Sbits(12, 9)];
]
"ldtrsw" = [
    0b10111000_10000000_00001000_00000000 = [X, RefOffset] => [R(0), R(5), Sbits(12, 9)];
]
"ldumax" = [
    0b10111000_00100000_01100000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_00100000_01100000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"ldumaxa" = [
    0b10111000_10100000_01100000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_10100000_01100000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"ldumaxab" = [
    0b00111000_10100000_01100000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldumaxah" = [
    0b01111000_10100000_01100000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldumaxal" = [
    0b10111000_11100000_01100000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_11100000_01100000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"ldumaxalb" = [
    0b00111000_11100000_01100000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldumaxalh" = [
    0b01111000_11100000_01100000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldumaxb" = [
    0b00111000_00100000_01100000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldumaxh" = [
    0b01111000_00100000_01100000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldumaxl" = [
    0b10111000_01100000_01100000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_01100000_01100000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"ldumaxlb" = [
    0b00111000_01100000_01100000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldumaxlh" = [
    0b01111000_01100000_01100000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldumin" = [
    0b10111000_00100000_01110000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_00100000_01110000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"ldumina" = [
    0b10111000_10100000_01110000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_10100000_01110000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"lduminab" = [
    0b00111000_10100000_01110000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"lduminah" = [
    0b01111000_10100000_01110000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"lduminal" = [
    0b10111000_11100000_01110000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_11100000_01110000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"lduminalb" = [
    0b00111000_11100000_01110000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"lduminalh" = [
    0b01111000_11100000_01110000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"lduminb" = [
    0b00111000_00100000_01110000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"lduminh" = [
    0b01111000_00100000_01110000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"lduminl" = [
    0b10111000_01100000_01110000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_01100000_01110000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"lduminlb" = [
    0b00111000_01100000_01110000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"lduminlh" = [
    0b01111000_01100000_01110000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"ldur" = [
    // LDUR (SIMD&FP)
    0b00111100_01000000_00000000_00000000 = [B, RefOffset] => [R(0), R(5), Sbits(12, 9)];
    0b01111100_01000000_00000000_00000000 = [H, RefOffset] => [R(0), R(5), Sbits(12, 9)];
    0b10111100_01000000_00000000_00000000 = [S, RefOffset] => [R(0), R(5), Sbits(12, 9)];
    0b11111100_01000000_00000000_00000000 = [D, RefOffset] => [R(0), R(5), Sbits(12, 9)];
    0b00111100_11000000_00000000_00000000 = [Q, RefOffset] => [R(0), R(5), Sbits(12, 9)];
    // LDUR
    0b10111000_01000000_00000000_00000000 = [W, RefOffset] => [R(0), R(5), Sbits(12, 9)];
    0b11111000_01000000_00000000_00000000 = [X, RefOffset] => [R(0), R(5), Sbits(12, 9)];
]
"ldurb" = [
    0b00111000_01000000_00000000_00000000 = [W, RefOffset] => [R(0), R(5), Sbits(12, 9)];
]
"ldurh" = [
    0b01111000_01000000_00000000_00000000 = [W, RefOffset] => [R(0), R(5), Sbits(12, 9)];
]
"ldursb" = [
    0b00111000_11000000_00000000_00000000 = [W, RefOffset] => [R(0), R(5), Sbits(12, 9)];
    0b00111000_10000000_00000000_00000000 = [X, RefOffset] => [R(0), R(5), Sbits(12, 9)];
]
"ldursh" = [
    0b01111000_11000000_00000000_00000000 = [W, RefOffset] => [R(0), R(5), Sbits(12, 9)];
    0b01111000_10000000_00000000_00000000 = [X, RefOffset] => [R(0), R(5), Sbits(12, 9)];
]
"ldursw" = [
    0b10111000_10000000_00000000_00000000 = [X, RefOffset] => [R(0), R(5), Sbits(12, 9)];
]
"ldxp" = [
    0b10001000_01111111_00000000_00000000 = [W, W, RefBase] => [R(0), R(10), R(5)];
    0b11001000_01111111_00000000_00000000 = [X, X, RefBase] => [R(0), R(10), R(5)];
]
"ldxr" = [
    0b10001000_01011111_01111100_00000000 = [W, RefBase] => [R(0), R(5)];
    0b11001000_01011111_01111100_00000000 = [X, RefBase] => [R(0), R(5)];
]
"ldxrb" = [
    0b00001000_01011111_01111100_00000000 = [W, RefBase] => [R(0), R(5)];
]
"ldxrh" = [
    0b01001000_01011111_01111100_00000000 = [W, RefBase] => [R(0), R(5)];
]
"lsl" = [
    // LSL (register)
    0b00011010_11000000_00100000_00000000 = [W, W, W] => [R(0), R(5), R(16)];
    0b10011010_11000000_00100000_00000000 = [X, X, X] => [R(0), R(5), R(16)];
    // LSL (immediate)
    0b01010011_00000000_00000000_00000000 = [W, W, Imm] => [R(0), R(5), Usub(16, 5, 32), C, Usub(16, 5, 31)];
    0b11010011_01000000_00000000_00000000 = [X, X, Imm] => [R(0), R(5), Usub(16, 6, 64), C, Usub(16, 6, 63)];
]
"lslv" = [
    0b00011010_11000000_00100000_00000000 = [W, W, W] => [R(0), R(5), R(16)];
    0b10011010_11000000_00100000_00000000 = [X, X, X] => [R(0), R(5), R(16)];
]
"lsr" = [
    // LSR (register)
    0b00011010_11000000_00100100_00000000 = [W, W, W] => [R(0), R(5), R(16)];
    0b10011010_11000000_00100100_00000000 = [X, X, X] => [R(0), R(5), R(16)];
    // LSR (immediate)
    0b01010011_00000000_01111100_00000000 = [W, W, Imm] => [R(0), R(5), Ubits(16, 5)];
    0b11010011_01000000_11111100_00000000 = [X, X, Imm] => [R(0), R(5), Ubits(16, 6)];
]
"lsrv" = [
    0b00011010_11000000_00100100_00000000 = [W, W, W] => [R(0), R(5), R(16)];
    0b10011010_11000000_00100100_00000000 = [X, X, X] => [R(0), R(5), R(16)];
]
"madd" = [
    0b00011011_00000000_00000000_00000000 = [W, W, W, W] => [R(0), R(5), R(16), R(10)];
    0b10011011_00000000_00000000_00000000 = [X, X, X, X] => [R(0), R(5), R(16), R(10)];
]
"mla" = [
    // MLA (by element)
    0b00101111_01000000_00000000_00000000 = [VSized(WORD), VSized(WORD), VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20]), Rwidth(30)];
    0b00101111_10000000_00000000_00000000 = [VSized(DWORD), VSized(DWORD), VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21]), Rwidth(30)];
    // MLA (vector)
    0b00001110_00100000_10010100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01100000_10010100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10100000_10010100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
]
"mls" = [
    // MLS (by element)
    0b00101111_01000000_01000000_00000000 = [VSized(WORD), VSized(WORD), VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20]), Rwidth(30)];
    0b00101111_10000000_01000000_00000000 = [VSized(DWORD), VSized(DWORD), VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21]), Rwidth(30)];
    // MLS (vector)
    0b00101110_00100000_10010100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_01100000_10010100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_10100000_10010100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
]
"mneg" = [
    0b00011011_00000000_11111100_00000000 = [W, W, W] => [R(0), R(5), R(16)];
    0b10011011_00000000_11111100_00000000 = [X, X, X] => [R(0), R(5), R(16)];
]
"mov" = [
    // MOV (to/from SP)
    0b00010001_00000000_00000000_00000000 = [WSP, WSP] => [R(0), R(5)];
    0b10010001_00000000_00000000_00000000 = [XSP, XSP] => [R(0), R(5)];
    // MOV (scalar)
    0b01011110_00000001_00000100_00000000 = [B, VLanes(BYTE)] => [R(0), R(5), Ubits(17, 4)];
    0b01011110_00000010_00000100_00000000 = [H, VLanes(WORD)] => [R(0), R(5), Ubits(18, 3)];
    0b01011110_00000100_00000100_00000000 = [S, VLanes(DWORD)] => [R(0), R(5), Ubits(19, 2)];
    0b01011110_00001000_00000100_00000000 = [D, VLanes(QWORD)] => [R(0), R(5), Ubits(20, 1)];
    // MOV (element)
    0b01101110_00000001_00000100_00000000 = [VLanes(BYTE), VLanes(BYTE)] => [R(0), Ubits(17, 4), R(5), Ubits(11, 4)];
    0b01101110_00000010_00000100_00000000 = [VLanes(WORD), VLanes(WORD)] => [R(0), Ubits(18, 3), R(5), Ubits(12, 3)];
    0b01101110_00000100_00000100_00000000 = [VLanes(DWORD), VLanes(DWORD)] => [R(0), Ubits(19, 2), R(5), Ubits(13, 2)];
    0b01101110_00001000_00000100_00000000 = [VLanes(QWORD), VLanes(QWORD)] => [R(0), Ubits(20, 1), R(5), Ubits(14, 1)];
    // MOV (from general)
    0b01001110_00000001_00011100_00000000 = [VLanes(BYTE), W] => [R(0), Ubits(17, 4), R(5)];
    0b01001110_00000010_00011100_00000000 = [VLanes(WORD), W] => [R(0), Ubits(18, 3), R(5)];
    0b01001110_00000100_00011100_00000000 = [VLanes(DWORD), W] => [R(0), Ubits(19, 2), R(5)];
    0b01001110_00001000_00011100_00000000 = [VLanes(QWORD), X] => [R(0), Ubits(20, 1), R(5)];
    // MOV (inverted wide immediate)
    0b00010010_10000000_00000000_00000000 = [Dot, Lit("inverted"), W, Imm] => [R(0), Special(5, "inverted wide imm")];
    0b10010010_10000000_00000000_00000000 = [Dot, Lit("inverted"), X, Imm] => [R(0), Special(5, "inverted wide imm")];
    // MOV (wide immediate)
    0b01010010_10000000_00000000_00000000 = [W, Imm] => [R(0), Special(5, "wide imm")];
    0b11010010_10000000_00000000_00000000 = [X, Imm] => [R(0), Special(5, "wide imm")];
    // MOV (vector)
    0b00001110_10100000_00011100_00000000 = [VSized(BYTE), VSized(BYTE)] => [R(0), R(5), C, R(16), Rwidth(30)];
    // MOV (bitmask immediate)
    0b00110010_00000000_00000011_11100000 = [Dot, Lit("Logical"), WSP, Imm] => [R(0), UlogicalW(10)];
    0b10110010_00000000_00000011_11100000 = [Dot, Lit("Logical"), XSP, Imm] => [R(0), UlogicalX(10)];
    // MOV (register)
    0b00101010_00000000_00000011_11100000 = [W, W] => [R(0), R(16)];
    0b10101010_00000000_00000011_11100000 = [X, X] => [R(0), R(16)];
    // MOV (to general)
    0b00001110_00000100_00111100_00000000 = [W, VLanes(DWORD)] => [R(0), R(5), Ubits(19, 2)];
    0b01001110_00001000_00111100_00000000 = [X, VLanes(QWORD)] => [R(0), R(5), Ubits(20, 1)];
]
"movi" = [
    0b00001111_00000000_11100100_00000000 = [VSized(BYTE), Imm, End, Mod(&[LSL])] => [R(0), BUbits(8), BUslice(5, 5, 0), BUslice(16, 3, 5), A, A, BUbits(0), A, Rwidth(30)];
    0b00001111_00000000_10000100_00000000 = [VSized(WORD), Imm, End, Mod(&[LSL])] => [R(0), BUbits(8), BUslice(5, 5, 0), BUslice(16, 3, 5), A, A, Ulist(13, &[0, 8]), Rwidth(30)];
    0b00001111_00000000_00000100_00000000 = [VSized(DWORD), Imm, End, Mod(&[LSL])] => [R(0), BUbits(8), BUslice(5, 5, 0), BUslice(16, 3, 5), A, A, Ulist(13, &[0, 8, 16, 24]), Rwidth(30)];
    0b00001111_00000000_11000100_00000000 = [VSized(DWORD), Imm, Mod(&[MSL])] => [R(0), BUbits(8), BUslice(5, 5, 0), BUslice(16, 3, 5), A, A, Ulist(12, &[8, 16]), Rwidth(30)];
    0b00101111_00000000_11100100_00000000 = [D, Imm] => [R(0), Special(5, "stretched imm")];
    0b01101111_00000000_11100100_00000000 = [VSizedStatic(QWORD, 2), Imm] => [R(0), Special(5, "stretched imm")];
]
"movk" = [
    0b01110010_10000000_00000000_00000000 = [W, Imm, End, Mod(&[LSL])] => [R(0), Ubits(5, 16), A, Ulist(21, &[0, 16])];
    0b11110010_10000000_00000000_00000000 = [X, Imm, End, Mod(&[LSL])] => [R(0), Ubits(5, 16), A, Ulist(21, &[0, 16, 32, 48])];
]
"movn" = [
    0b00010010_10000000_00000000_00000000 = [W, Imm, End, Mod(&[LSL])] => [R(0), Ubits(5, 16), A, Ulist(21, &[0, 16])];
    0b10010010_10000000_00000000_00000000 = [X, Imm, End, Mod(&[LSL])] => [R(0), Ubits(5, 16), A, Ulist(21, &[0, 16, 32, 48])];
]
"movz" = [
    0b01010010_10000000_00000000_00000000 = [W, Imm, End, Mod(&[LSL])] => [R(0), Ubits(5, 16), A, Ulist(21, &[0, 16])];
    0b11010010_10000000_00000000_00000000 = [X, Imm, End, Mod(&[LSL])] => [R(0), Ubits(5, 16), A, Ulist(21, &[0, 16, 32, 48])];
]
"mrs" = [
    0b11010101_00110000_00000000_00000000 = [X, Imm] => [R(0), Ubits(5, 15)];
]
"msr" = [
    // MSR (immediate)
    0b11010101_00000000_01000000_00011111 = [Ident, Imm] => [LitList(5, "MSR_IMM_OPS"), Ubits(8, 4)];
    // MSR (register)
    0b11010101_00010000_00000000_00000000 = [Imm, X] => [Ubits(5, 15), R(0)];
]
"msub" = [
    0b00011011_00000000_10000000_00000000 = [W, W, W, W] => [R(0), R(5), R(16), R(10)];
    0b10011011_00000000_10000000_00000000 = [X, X, X, X] => [R(0), R(5), R(16), R(10)];
]
"mul" = [
    // MUL (by element)
    0b00001111_01000000_10000000_00000000 = [VSized(WORD), VSized(WORD), VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20]), Rwidth(30)];
    0b00001111_10000000_10000000_00000000 = [VSized(DWORD), VSized(DWORD), VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21]), Rwidth(30)];
    // MUL (vector)
    0b00001110_00100000_10011100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01100000_10011100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10100000_10011100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    // MUL
    0b00011011_00000000_01111100_00000000 = [W, W, W] => [R(0), R(5), R(16)];
    0b10011011_00000000_01111100_00000000 = [X, X, X] => [R(0), R(5), R(16)];
]
"mvn" = [
    0b00101010_00100000_00000011_11100000 = [W, W, End, Mod(SHIFTS)] => [R(0), R(16), Rotates(22), Ubits(10, 5)];
    0b10101010_00100000_00000011_11100000 = [X, X, End, Mod(SHIFTS)] => [R(0), R(16), Rotates(22), Ubits(10, 6)];
    0b00101110_00100000_01011000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
]
"mvni" = [
    0b00101111_00000000_10000100_00000000 = [VSized(WORD), Imm, End, Mod(&[LSL])] => [R(0), BUbits(8), BUslice(5, 5, 0), BUslice(16, 3, 5), A, A, Ulist(13, &[0, 8]), Rwidth(30)];
    0b00101111_00000000_00000100_00000000 = [VSized(DWORD), Imm, End, Mod(&[LSL])] => [R(0), BUbits(8), BUslice(5, 5, 0), BUslice(16, 3, 5), A, A, Ulist(13, &[0, 8, 16, 24]), Rwidth(30)];
    0b00101111_00000000_11000100_00000000 = [VSized(DWORD), Imm, Mod(&[MSL])] => [R(0), BUbits(8), BUslice(5, 5, 0), BUslice(16, 3, 5), A, A, Ulist(12, &[8, 16]), Rwidth(30)];
]
"neg" = [
    // NEG (shifted register)
    0b01001011_00000000_00000011_11100000 = [W, W, End, Mod(SHIFTS)] => [R(0), R(16), Rotates(22), Ubits(10, 5)];
    0b11001011_00000000_00000011_11100000 = [X, X, End, Mod(SHIFTS)] => [R(0), R(16), Rotates(22), Ubits(10, 6)];
    // NEG (vector)
    0b01111110_11100000_10111000_00000000 = [D, D] => [R(0), R(5)];
    0b00101110_00100000_10111000_00000000 = [VSized(BYTE), VSized(BYTE)] => [R(0), R(5), Rwidth(30)];
    0b00101110_01100000_10111000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_10100000_10111000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_11100000_10111000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
]
"negs" = [
    0b01101011_00000000_00000011_11100000 = [W, W, End, Mod(SHIFTS)] => [R(0), R(16), Rotates(22), Ubits(10, 5)];
    0b11101011_00000000_00000011_11100000 = [X, X, End, Mod(SHIFTS)] => [R(0), R(16), Rotates(22), Ubits(10, 6)];
]
"ngc" = [
    0b01011010_00000000_00000011_11100000 = [W, W] => [R(0), R(16)];
    0b11011010_00000000_00000011_11100000 = [X, X] => [R(0), R(16)];
]
"ngcs" = [
    0b01111010_00000000_00000011_11100000 = [W, W] => [R(0), R(16)];
    0b11111010_00000000_00000011_11100000 = [X, X] => [R(0), R(16)];
]
"nop" = [
    0b11010101_00000011_00100000_00011111 = [] => [];
]
"not" = [
    0b00101110_00100000_01011000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
]
"orn" = [
    // ORN (vector)
    0b00001110_11100000_00011100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    // ORN (shifted register)
    0b00101010_00100000_00000000_00000000 = [W, W, W, End, Mod(ROTATES)] => [R(0), R(5), R(16), Rotates(22), Ubits(10, 5)];
    0b10101010_00100000_00000000_00000000 = [X, X, X, End, Mod(ROTATES)] => [R(0), R(5), R(16), Rotates(22), Ubits(10, 6)];
]
"orr" = [
    // ORR (vector, immediate)
    0b00001111_00000000_10010100_00000000 = [VSized(WORD), Imm, End, Mod(&[LSL])] => [R(0), BUbits(8), BUslice(5, 5, 0), BUslice(16, 3, 5), A, A, Ulist(13, &[0, 8])];
    0b00001111_00000000_00010100_00000000 = [VSized(DWORD), Imm, End, Mod(&[LSL])] => [R(0), BUbits(8), BUslice(5, 5, 0), BUslice(16, 3, 5), A, A, Ulist(13, &[0, 8, 16, 24])];
    // ORR (vector, register)
    0b00001110_10100000_00011100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    // ORR (immediate)
    0b00110010_00000000_00000000_00000000 = [WSP, W, Imm] => [R(0), R(5), UlogicalW(10)];
    0b10110010_00000000_00000000_00000000 = [XSP, X, Imm] => [R(0), R(5), UlogicalX(10)];
    // ORR (shifted register)
    0b00101010_00000000_00000000_00000000 = [W, W, W, End, Mod(ROTATES)] => [R(0), R(5), R(16), Rotates(22), Ubits(10, 5)];
    0b10101010_00000000_00000000_00000000 = [X, X, X, End, Mod(ROTATES)] => [R(0), R(5), R(16), Rotates(22), Ubits(10, 6)];
]
"pacda" = [
    0b11011010_11000001_00001000_00000000 = [X, XSP] => [R(0), R(5)];
]
"pacdb" = [
    0b11011010_11000001_00001100_00000000 = [X, XSP] => [R(0), R(5)];
]
"pacdza" = [
    0b11011010_11000001_00101011_11100000 = [X] => [R(0)];
]
"pacdzb" = [
    0b11011010_11000001_00101111_11100000 = [X] => [R(0)];
]
"pacga" = [
    0b10011010_11000000_00110000_00000000 = [X, X, XSP] => [R(0), R(5), R(16)];
]
"pacia" = [
    0b11011010_11000001_00000000_00000000 = [X, XSP] => [R(0), R(5)];
]
"pacia1716" = [
    0b11010101_00000011_00100001_00011111 = [] => [];
]
"paciasp" = [
    0b11010101_00000011_00100011_00111111 = [] => [];
]
"paciaz" = [
    0b11010101_00000011_00100011_00011111 = [] => [];
]
"pacib" = [
    0b11011010_11000001_00000100_00000000 = [X, XSP] => [R(0), R(5)];
]
"pacib1716" = [
    0b11010101_00000011_00100001_01011111 = [] => [];
]
"pacibsp" = [
    0b11010101_00000011_00100011_01111111 = [] => [];
]
"pacibz" = [
    0b11010101_00000011_00100011_01011111 = [] => [];
]
"paciza" = [
    0b11011010_11000001_00100011_11100000 = [X] => [R(0)];
]
"pacizb" = [
    0b11011010_11000001_00100111_11100000 = [X] => [R(0)];
]
"pmul" = [
    0b00101110_00100000_10011100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
]
"pmull" = [
    0b00001110_00100000_11100000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 8), VSizedStatic(BYTE, 8)] => [R(0), R(5), R(16)];
    0b00001110_11100000_11100000_00000000 = [VSizedStatic(OWORD, 1), VSizedStatic(QWORD, 1), VSizedStatic(QWORD, 1)] => [R(0), R(5), R(16)];
]
"pmull2" = [
    0b01001110_00100000_11100000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16)] => [R(0), R(5), R(16)];
    0b01001110_11100000_11100000_00000000 = [VSizedStatic(OWORD, 1), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16)];
]
"prfm" = [
    // PRFM (literal)
    0b11011000_00000000_00000000_00000000 = [Imm, Offset] => [Ubits(0, 5), Sscaled(5, 19, 2)];
    // PRFM (register)
    0b11111000_10100000_00001000_00000000 = [Imm, RefIndex] => [Ubits(0, 5), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 3])];
]
"prfum" = [
    0b11111000_10000000_00000000_00000000 = [Imm, RefOffset] => [Ubits(0, 5), R(5), Sbits(12, 9)];
]
"psb" = [
    0b11010101_00000011_00100010_00111111 = [Lit("CSYNC")] => [];
]
"pssbb" = [
    0b11010101_00000011_00110100_10011111 = [] => [];
]
"raddhn" = [
    0b00101110_00100000_01000000_00000000 = [VSizedStatic(BYTE, 8), VSizedStatic(WORD, 8), VSizedStatic(WORD, 8)] => [R(0), R(5), R(16)];
    0b00101110_01100000_01000000_00000000 = [VSizedStatic(WORD, 4), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
    0b00101110_10100000_01000000_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16)];
]
"raddhn2" = [
    0b01101110_00100000_01000000_00000000 = [VSizedStatic(BYTE, 16), VSizedStatic(WORD, 8), VSizedStatic(WORD, 8)] => [R(0), R(5), R(16)];
    0b01101110_01100000_01000000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
    0b01101110_10100000_01000000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16)];
]
"rax1" = [
    0b11001110_01100000_10001100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16)];
]
"rbit" = [
    // RBIT (vector)
    0b00101110_01100000_01011000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    // RBIT
    0b01011010_11000000_00000000_00000000 = [W, W] => [R(0), R(5)];
    0b11011010_11000000_00000000_00000000 = [X, X] => [R(0), R(5)];
]
"ret" = [
    0b11010110_01011111_00000000_00000000 = [X] => [R(5)];
    0b11010110_01011111_00000011_11000000 = [] => [];
]
"retaa" = [
    0b11010110_01011111_00001011_11111111 = [] => [];
]
"retab" = [
    0b11010110_01011111_00001111_11111111 = [] => [];
]
"rev" = [
    0b01011010_11000000_00001000_00000000 = [W, W] => [R(0), R(5)];
    0b11011010_11000000_00001100_00000000 = [X, X] => [R(0), R(5)];
]
"rev16" = [
    // REV16 (vector)
    0b00001110_00100000_00011000_00000000 = [VSized(BYTE), VSized(BYTE)] => [R(0), R(5), Rwidth(30)];
    // REV16
    0b01011010_11000000_00000100_00000000 = [W, W] => [R(0), R(5)];
    0b11011010_11000000_00000100_00000000 = [X, X] => [R(0), R(5)];
]
"rev32" = [
    // REV32 (vector)
    0b00101110_00100000_00001000_00000000 = [VSized(BYTE), VSized(BYTE)] => [R(0), R(5), Rwidth(30)];
    0b00101110_01100000_00001000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    // REV32
    0b11011010_11000000_00001000_00000000 = [X, X] => [R(0), R(5)];
]
"rev64" = [
    0b00001110_00100000_00001000_00000000 = [VSized(BYTE), VSized(BYTE)] => [R(0), R(5), Rwidth(30)];
    0b00001110_01100000_00001000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_10100000_00001000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b11011010_11000000_00001100_00000000 = [X, X] => [R(0), R(5)];
]
"rmif" = [
    0b10111010_00000000_00000100_00000000 = [X, Imm, Imm] => [R(5), Ubits(15, 6), Ubits(0, 4)];
]
"ror" = [
    // ROR (immediate)
    0b00010011_10000000_00000000_00000000 = [W, W, Imm] => [R(0), R(5), C, R(16), Ubits(10, 5)];
    0b10010011_11000000_00000000_00000000 = [X, X, Imm] => [R(0), R(5), C, R(16), Ubits(10, 6)];
    // ROR (register)
    0b00011010_11000000_00101100_00000000 = [W, W, W] => [R(0), R(5), R(16)];
    0b10011010_11000000_00101100_00000000 = [X, X, X] => [R(0), R(5), R(16)];
]
"rorv" = [
    0b00011010_11000000_00101100_00000000 = [W, W, W] => [R(0), R(5), R(16)];
    0b10011010_11000000_00101100_00000000 = [X, X, X] => [R(0), R(5), R(16)];
]
"rshrn" = [
    0b00001111_00001000_10001100_00000000 = [VSizedStatic(BYTE, 8), VSizedStatic(WORD, 8), Imm] => [R(0), R(5), Usub(16, 3, 8)];
    0b00001111_00010000_10001100_00000000 = [VSizedStatic(WORD, 4), VSizedStatic(DWORD, 4), Imm] => [R(0), R(5), Usub(16, 4, 16)];
    0b00001111_00100000_10001100_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Usub(16, 5, 32)];
]
"rshrn2" = [
    0b01001111_00001000_10001100_00000000 = [VSizedStatic(BYTE, 16), VSizedStatic(WORD, 8), Imm] => [R(0), R(5), Usub(16, 3, 8)];
    0b01001111_00010000_10001100_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(DWORD, 4), Imm] => [R(0), R(5), Usub(16, 4, 16)];
    0b01001111_00100000_10001100_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Usub(16, 5, 32)];
]
"rsubhn" = [
    0b00101110_00100000_01100000_00000000 = [VSizedStatic(BYTE, 8), VSizedStatic(WORD, 8), VSizedStatic(WORD, 8)] => [R(0), R(5), R(16)];
    0b00101110_01100000_01100000_00000000 = [VSizedStatic(WORD, 4), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
    0b00101110_10100000_01100000_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16)];
]
"rsubhn2" = [
    0b01101110_00100000_01100000_00000000 = [VSizedStatic(BYTE, 16), VSizedStatic(WORD, 8), VSizedStatic(WORD, 8)] => [R(0), R(5), R(16)];
    0b01101110_01100000_01100000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
    0b01101110_10100000_01100000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16)];
]
"saba" = [
    0b00001110_00100000_01111100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01100000_01111100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10100000_01111100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
]
"sabal" = [
    0b00001110_00100000_01010000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 8), VSizedStatic(BYTE, 8)] => [R(0), R(5), R(16)];
    0b00001110_01100000_01010000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VSizedStatic(WORD, 4)] => [R(0), R(5), R(16)];
    0b00001110_10100000_01010000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2), VSizedStatic(DWORD, 2)] => [R(0), R(5), R(16)];
]
"sabal2" = [
    0b01001110_00100000_01010000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16)] => [R(0), R(5), R(16)];
    0b01001110_01100000_01010000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8), VSizedStatic(WORD, 8)] => [R(0), R(5), R(16)];
    0b01001110_10100000_01010000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"sabd" = [
    0b00001110_00100000_01110100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01100000_01110100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10100000_01110100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
]
"sabdl" = [
    0b00001110_00100000_01110000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 8), VSizedStatic(BYTE, 8)] => [R(0), R(5), R(16)];
    0b00001110_01100000_01110000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VSizedStatic(WORD, 4)] => [R(0), R(5), R(16)];
    0b00001110_10100000_01110000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2), VSizedStatic(DWORD, 2)] => [R(0), R(5), R(16)];
]
"sabdl2" = [
    0b01001110_00100000_01110000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16)] => [R(0), R(5), R(16)];
    0b01001110_01100000_01110000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8), VSizedStatic(WORD, 8)] => [R(0), R(5), R(16)];
    0b01001110_10100000_01110000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"sadalp" = [
    0b00001110_00100000_01101000_00000000 = [VSized(WORD), VSized(BYTE)] => [R(0), R(5), Rwidth(30)];
    0b00001110_01100000_01101000_00000000 = [VSized(DWORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_10100000_01101000_00000000 = [VSized(QWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
]
"saddl" = [
    0b00001110_00100000_00000000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 8), VSizedStatic(BYTE, 8)] => [R(0), R(5), R(16)];
    0b00001110_01100000_00000000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VSizedStatic(WORD, 4)] => [R(0), R(5), R(16)];
    0b00001110_10100000_00000000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2), VSizedStatic(DWORD, 2)] => [R(0), R(5), R(16)];
]
"saddl2" = [
    0b01001110_00100000_00000000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16)] => [R(0), R(5), R(16)];
    0b01001110_01100000_00000000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8), VSizedStatic(WORD, 8)] => [R(0), R(5), R(16)];
    0b01001110_10100000_00000000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"saddlp" = [
    0b00001110_00100000_00101000_00000000 = [VSized(WORD), VSized(BYTE)] => [R(0), R(5), Rwidth(30)];
    0b00001110_01100000_00101000_00000000 = [VSized(DWORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_10100000_00101000_00000000 = [VSized(QWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
]
"saddlv" = [
    0b00001110_00110000_00111000_00000000 = [B, VSized(BYTE)] => [R(0), R(5), Rwidth(30)];
    0b00001110_01110000_00111000_00000000 = [H, VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_10110000_00111000_00000000 = [S, VSizedStatic(DWORD, 4)] => [R(0), R(5), Rwidth(30)];
]
"saddw" = [
    0b00001110_00100000_00010000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(WORD, 8), VSizedStatic(BYTE, 8)] => [R(0), R(5), R(16)];
    0b00001110_01100000_00010000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4)] => [R(0), R(5), R(16)];
    0b00001110_10100000_00010000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2)] => [R(0), R(5), R(16)];
]
"saddw2" = [
    0b01001110_00100000_00010000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(WORD, 8), VSizedStatic(BYTE, 16)] => [R(0), R(5), R(16)];
    0b01001110_01100000_00010000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8)] => [R(0), R(5), R(16)];
    0b01001110_10100000_00010000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"sb" = [
    0b11010101_00000011_00110000_11111111 = [] => [];
]
"sbc" = [
    0b01011010_00000000_00000000_00000000 = [W, W, W] => [R(0), R(5), R(16)];
    0b11011010_00000000_00000000_00000000 = [X, X, X] => [R(0), R(5), R(16)];
]
"sbcs" = [
    0b01111010_00000000_00000000_00000000 = [W, W, W] => [R(0), R(5), R(16)];
    0b11111010_00000000_00000000_00000000 = [X, X, X] => [R(0), R(5), R(16)];
]
"sbfiz" = [
    0b00010011_00000000_00000000_00000000 = [W, W, Imm, Imm] => [R(0), R(5), Usub(16, 5, 32), Urange(10, 1, 32)];
    0b10010011_01000000_00000000_00000000 = [X, X, Imm, Imm] => [R(0), R(5), Usub(16, 6, 64), Urange(10, 1, 64)];
]
"sbfm" = [
    0b00010011_00000000_00000000_00000000 = [W, W, Imm, Imm] => [R(0), R(5), Ubits(16, 5), Ubits(10, 5)];
    0b10010011_01000000_00000000_00000000 = [X, X, Imm, Imm] => [R(0), R(5), Ubits(16, 6), Ubits(10, 6)];
]
"sbfx" = [
    0b00010011_00000000_00000000_00000000 = [W, W, Imm, Imm] => [R(0), R(5), Ubits(16, 5), Usumdec(10, 5)];
    0b10010011_01000000_00000000_00000000 = [X, X, Imm, Imm] => [R(0), R(5), Ubits(16, 6), Usumdec(10, 6)];
]
"scvtf" = [
    // SCVTF (vector, fixed-point)
    0b01011111_00000000_11100100_00000000 = [H, H, Imm] => [R(0), R(5), BUrange(1, 16), Usub(16, 5, 32)];
    0b01011111_00000000_11100100_00000000 = [S, S, Imm] => [R(0), R(5), BUrange(1, 32), Usub(16, 6, 64)];
    0b01011111_00000000_11100100_00000000 = [D, D, Imm] => [R(0), R(5), BUrange(1, 64), Usub(16, 7, 128)];
    0b00001111_00010000_11100100_00000000 = [VSized(WORD), VSized(WORD), Imm] => [R(0), R(5), R(16), Usub(16, 4, 16), Rwidth(30)];
    0b00001111_00100000_11100100_00000000 = [VSized(DWORD), VSized(DWORD), Imm] => [R(0), R(5), R(16), Usub(16, 5, 32), Rwidth(30)];
    0b00001111_01000000_11100100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), R(16), Usub(16, 6, 64), Rwidth(30)];
    // SCVTF (vector, integer)
    0b01011110_01111001_11011000_00000000 = [H, H] => [R(0), R(5)];
    0b01011110_00100001_11011000_00000000 = [S, S] => [R(0), R(5)];
    0b01011110_01100001_11011000_00000000 = [D, D] => [R(0), R(5)];
    0b00001110_01111001_11011000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_00100001_11011000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_01100001_11011000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
    // SCVTF (scalar, fixed-point)
    0b00011110_11000010_00000000_00000000 = [H, W, Imm] => [R(0), R(5), BUrange(1, 32), Usub(10, 6, 64)];
    0b00011110_00000010_00000000_00000000 = [S, W, Imm] => [R(0), R(5), Usub(10, 6, 64)];
    0b00011110_01000010_00000000_00000000 = [D, W, Imm] => [R(0), R(5), BUrange(1, 32), Usub(10, 6, 64)];
    0b10011110_11000010_00000000_00000000 = [H, X, Imm] => [R(0), R(5), Usub(10, 6, 64)];
    0b10011110_00000010_00000000_00000000 = [S, X, Imm] => [R(0), R(5), BUrange(1, 32), Usub(10, 6, 64)];
    0b10011110_01000010_00000000_00000000 = [D, X, Imm] => [R(0), R(5), Usub(10, 6, 64)];
    // SCVTF (scalar, integer)
    0b00011110_11100010_00000000_00000000 = [H, W] => [R(0), R(5)];
    0b00011110_00100010_00000000_00000000 = [S, W] => [R(0), R(5)];
    0b00011110_01100010_00000000_00000000 = [D, W] => [R(0), R(5)];
    0b10011110_11100010_00000000_00000000 = [H, X] => [R(0), R(5)];
    0b10011110_00100010_00000000_00000000 = [S, X] => [R(0), R(5)];
    0b10011110_01100010_00000000_00000000 = [D, X] => [R(0), R(5)];
]
"sdiv" = [
    0b00011010_11000000_00001100_00000000 = [W, W, W] => [R(0), R(5), R(16)];
    0b10011010_11000000_00001100_00000000 = [X, X, X] => [R(0), R(5), R(16)];
]
"sdot" = [
    // SDOT (by element)
    0b00001111_10000000_11100000_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(BYTE, 8), VLanes(BYTE)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    0b01001111_10000000_11100000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(BYTE, 16), VLanes(BYTE)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    // SDOT (vector)
    0b00001110_10000000_10010100_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(BYTE, 8), VSizedStatic(BYTE, 8)] => [R(0), R(5), R(16)];
    0b01001110_10000000_10010100_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16)] => [R(0), R(5), R(16)];
]
"setf16" = [
    0b00111010_00000000_01001000_00001101 = [W] => [R(5)];
]
"setf8" = [
    0b00111010_00000000_00001000_00001101 = [W] => [R(5)];
]
"sev" = [
    0b11010101_00000011_00100000_10011111 = [] => [];
]
"sevl" = [
    0b11010101_00000011_00100000_10111111 = [] => [];
]
"sha1c" = [
    0b01011110_00000000_00000000_00000000 = [Q, S, VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"sha1h" = [
    0b01011110_00101000_00001000_00000000 = [S, S] => [R(0), R(5)];
]
"sha1m" = [
    0b01011110_00000000_00100000_00000000 = [Q, S, VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"sha1p" = [
    0b01011110_00000000_00010000_00000000 = [Q, S, VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"sha1su0" = [
    0b01011110_00000000_00110000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"sha1su1" = [
    0b01011110_00101000_00011000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5)];
]
"sha256h" = [
    0b01011110_00000000_01000000_00000000 = [Q, Q, VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"sha256h2" = [
    0b01011110_00000000_01010000_00000000 = [Q, Q, VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"sha256su0" = [
    0b01011110_00101000_00101000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5)];
]
"sha256su1" = [
    0b01011110_00000000_01100000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"sha512h" = [
    0b11001110_01100000_10000000_00000000 = [Q, Q, VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16)];
]
"sha512h2" = [
    0b11001110_01100000_10000100_00000000 = [Q, Q, VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16)];
]
"sha512su0" = [
    0b11001110_11000000_10000000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5)];
]
"sha512su1" = [
    0b11001110_01100000_10001000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16)];
]
"shadd" = [
    0b00001110_00100000_00000100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01100000_00000100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10100000_00000100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
]
"shl" = [
    0b01011111_01000000_01010100_00000000 = [D, D, Imm] => [R(0), R(5), Ubits(16, 6)];
    0b00001111_00001000_01010100_00000000 = [VSized(BYTE), VSized(BYTE), Imm] => [R(0), R(5), Ubits(16, 3), Rwidth(30)];
    0b00001111_00010000_01010100_00000000 = [VSized(WORD), VSized(WORD), Imm] => [R(0), R(5), Ubits(16, 4), Rwidth(30)];
    0b00001111_00100000_01010100_00000000 = [VSized(DWORD), VSized(DWORD), Imm] => [R(0), R(5), Ubits(16, 5), Rwidth(30)];
    0b00001111_01000000_01010100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Ubits(16, 6), Rwidth(30)];
]
"shll" = [
    0b00101110_00100001_00111000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 8), LitInt(8)] => [R(0), R(5)];
    0b00101110_01100001_00111000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), LitInt(16)] => [R(0), R(5)];
    0b00101110_10100001_00111000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2), LitInt(32)] => [R(0), R(5)];
]
"shll2" = [
    0b01101110_00100001_00111000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 16), LitInt(8)] => [R(0), R(5)];
    0b01101110_01100001_00111000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8), LitInt(16)] => [R(0), R(5)];
    0b01101110_10100001_00111000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4), LitInt(32)] => [R(0), R(5)];
]
"shrn" = [
    0b00001111_00001000_10000100_00000000 = [VSizedStatic(BYTE, 8), VSizedStatic(WORD, 8), Imm] => [R(0), R(5), Usub(16, 3, 8)];
    0b00001111_00010000_10000100_00000000 = [VSizedStatic(WORD, 4), VSizedStatic(DWORD, 4), Imm] => [R(0), R(5), Usub(16, 4, 16)];
    0b00001111_00100000_10000100_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Usub(16, 5, 32)];
]
"shrn2" = [
    0b01001111_00001000_10000100_00000000 = [VSizedStatic(BYTE, 16), VSizedStatic(WORD, 8), Imm] => [R(0), R(5), Usub(16, 3, 8)];
    0b01001111_00010000_10000100_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(DWORD, 4), Imm] => [R(0), R(5), Usub(16, 4, 16)];
    0b01001111_00100000_10000100_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Usub(16, 5, 32)];
]
"shsub" = [
    0b00001110_00100000_00100100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01100000_00100100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10100000_00100100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
]
"sli" = [
    0b01111111_01000000_01010100_00000000 = [D, D, Imm] => [R(0), R(5), Ubits(16, 6)];
    0b00101111_00001000_01010100_00000000 = [VSized(BYTE), VSized(BYTE), Imm] => [R(0), R(5), Ubits(16, 3), Rwidth(30)];
    0b00101111_00010000_01010100_00000000 = [VSized(WORD), VSized(WORD), Imm] => [R(0), R(5), Ubits(16, 4), Rwidth(30)];
    0b00101111_00100000_01010100_00000000 = [VSized(DWORD), VSized(DWORD), Imm] => [R(0), R(5), Ubits(16, 5), Rwidth(30)];
    0b00101111_01000000_01010100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Ubits(16, 6), Rwidth(30)];
]
"sm3partw1" = [
    0b11001110_01100000_11000000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"sm3partw2" = [
    0b11001110_01100000_11000100_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"sm3ss1" = [
    0b11001110_01000000_00000000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16), R(10)];
]
"sm3tt1a" = [
    0b11001110_01000000_10000000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4), VLanes(DWORD)] => [R(0), R(5), R(16), Ubits(12, 2)];
]
"sm3tt1b" = [
    0b11001110_01000000_10000100_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4), VLanes(DWORD)] => [R(0), R(5), R(16), Ubits(12, 2)];
]
"sm3tt2a" = [
    0b11001110_01000000_10001000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4), VLanes(DWORD)] => [R(0), R(5), R(16), Ubits(12, 2)];
]
"sm3tt2b" = [
    0b11001110_01000000_10001100_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4), VLanes(DWORD)] => [R(0), R(5), R(16), Ubits(12, 2)];
]
"sm4e" = [
    0b11001110_11000000_10000100_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5)];
]
"sm4ekey" = [
    0b11001110_01100000_11001000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"smaddl" = [
    0b10011011_00100000_00000000_00000000 = [X, W, W, X] => [R(0), R(5), R(16), R(10)];
]
"smax" = [
    0b00001110_00100000_01100100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01100000_01100100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10100000_01100100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
]
"smaxp" = [
    0b00001110_00100000_10100100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01100000_10100100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10100000_10100100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
]
"smaxv" = [
    0b00001110_00110000_10101000_00000000 = [B, VSized(BYTE)] => [R(0), R(5), Rwidth(30)];
    0b00001110_01110000_10101000_00000000 = [H, VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_10110000_10101000_00000000 = [S, VSizedStatic(DWORD, 4)] => [R(0), R(5), Rwidth(30)];
]
"smc" = [
    0b11010100_00000000_00000000_00000011 = [Imm] => [Ubits(5, 16)];
]
"smin" = [
    0b00001110_00100000_01101100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01100000_01101100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10100000_01101100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
]
"sminp" = [
    0b00001110_00100000_10101100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01100000_10101100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10100000_10101100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
]
"sminv" = [
    0b00001110_00110001_10101000_00000000 = [B, VSized(BYTE)] => [R(0), R(5), Rwidth(30)];
    0b00001110_01110001_10101000_00000000 = [H, VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_10110001_10101000_00000000 = [S, VSizedStatic(DWORD, 4)] => [R(0), R(5), Rwidth(30)];
]
"smlal" = [
    // SMLAL, SMLAL2 (by element)
    0b00001111_01000000_00100000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20])];
    0b00001111_10000000_00100000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2), VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    // SMLAL, SMLAL2 (vector)
    0b00001110_00100000_10000000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 8), VSizedStatic(BYTE, 8)] => [R(0), R(5), R(16)];
    0b00001110_01100000_10000000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VSizedStatic(WORD, 4)] => [R(0), R(5), R(16)];
    0b00001110_10100000_10000000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2), VSizedStatic(DWORD, 2)] => [R(0), R(5), R(16)];
]
"smlal2" = [
    // SMLAL, SMLAL2 (by element)
    0b01001111_01000000_00100000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8), VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20])];
    0b01001111_10000000_00100000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4), VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    // SMLAL, SMLAL2 (vector)
    0b01001110_00100000_10000000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16)] => [R(0), R(5), R(16)];
    0b01001110_01100000_10000000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8), VSizedStatic(WORD, 8)] => [R(0), R(5), R(16)];
    0b01001110_10100000_10000000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"smlsl" = [
    // SMLSL, SMLSL2 (by element)
    0b00001111_01000000_01100000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20])];
    0b00001111_10000000_01100000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2), VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    // SMLSL, SMLSL2 (vector)
    0b00001110_00100000_10100000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 8), VSizedStatic(BYTE, 8)] => [R(0), R(5), R(16)];
    0b00001110_01100000_10100000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VSizedStatic(WORD, 4)] => [R(0), R(5), R(16)];
    0b00001110_10100000_10100000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2), VSizedStatic(DWORD, 2)] => [R(0), R(5), R(16)];
]
"smlsl2" = [
    // SMLSL, SMLSL2 (by element)
    0b01001111_01000000_01100000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8), VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20])];
    0b01001111_10000000_01100000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4), VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    // SMLSL, SMLSL2 (vector)
    0b01001110_00100000_10100000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16)] => [R(0), R(5), R(16)];
    0b01001110_01100000_10100000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8), VSizedStatic(WORD, 8)] => [R(0), R(5), R(16)];
    0b01001110_10100000_10100000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"smnegl" = [
    0b10011011_00100000_11111100_00000000 = [X, W, W] => [R(0), R(5), R(16)];
]
"smov" = [
    0b00001110_00000001_00101100_00000000 = [W, VLanes(BYTE)] => [R(0), R(5), Ubits(17, 4)];
    0b00001110_00000010_00101100_00000000 = [W, VLanes(WORD)] => [R(0), R(5), Ubits(18, 3)];
    0b01001110_00000001_00101100_00000000 = [X, VLanes(BYTE)] => [R(0), R(5), Ubits(17, 4)];
    0b01001110_00000010_00101100_00000000 = [X, VLanes(WORD)] => [R(0), R(5), Ubits(18, 3)];
    0b01001110_00000100_00101100_00000000 = [X, VLanes(DWORD)] => [R(0), R(5), Ubits(19, 2)];
]
"smsubl" = [
    0b10011011_00100000_10000000_00000000 = [X, W, W, X] => [R(0), R(5), R(16), R(10)];
]
"smulh" = [
    0b10011011_01000000_01111100_00000000 = [X, X, X] => [R(0), R(5), R(16)];
]
"smull" = [
    // SMULL, SMULL2 (by element)
    0b00001111_01000000_10100000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20])];
    0b00001111_10000000_10100000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2), VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    // SMULL, SMULL2 (vector)
    0b00001110_00100000_11000000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 8), VSizedStatic(BYTE, 8)] => [R(0), R(5), R(16)];
    0b00001110_01100000_11000000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VSizedStatic(WORD, 4)] => [R(0), R(5), R(16)];
    0b00001110_10100000_11000000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2), VSizedStatic(DWORD, 2)] => [R(0), R(5), R(16)];
    // SMULL
    0b10011011_00100000_01111100_00000000 = [X, W, W] => [R(0), R(5), R(16)];
]
"smull2" = [
    // SMULL, SMULL2 (by element)
    0b01001111_01000000_10100000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8), VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20])];
    0b01001111_10000000_10100000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4), VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    // SMULL, SMULL2 (vector)
    0b01001110_00100000_11000000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16)] => [R(0), R(5), R(16)];
    0b01001110_01100000_11000000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8), VSizedStatic(WORD, 8)] => [R(0), R(5), R(16)];
    0b01001110_10100000_11000000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"sqabs" = [
    0b01011110_00100000_01111000_00000000 = [B, B] => [R(0), R(5)];
    0b01011110_01100000_01111000_00000000 = [H, H] => [R(0), R(5)];
    0b01011110_10100000_01111000_00000000 = [S, S] => [R(0), R(5)];
    0b01011110_11100000_01111000_00000000 = [D, D] => [R(0), R(5)];
    0b00001110_00100000_01111000_00000000 = [VSized(BYTE), VSized(BYTE)] => [R(0), R(5), Rwidth(30)];
    0b00001110_01100000_01111000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_10100000_01111000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_11100000_01111000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
]
"sqadd" = [
    0b01011110_00100000_00001100_00000000 = [B, B, B] => [R(0), R(5), R(16)];
    0b01011110_01100000_00001100_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b01011110_10100000_00001100_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b01011110_11100000_00001100_00000000 = [D, D, D] => [R(0), R(5), R(16)];
    0b00001110_00100000_00001100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01100000_00001100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10100000_00001100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_11100000_00001100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"sqdmlal" = [
    // SQDMLAL, SQDMLAL2 (by element)
    0b01011111_01000000_00110000_00000000 = [S, H, VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20])];
    0b01011111_10000000_00110000_00000000 = [D, S, VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    0b00001111_01000000_00110000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20])];
    0b00001111_10000000_00110000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2), VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    // SQDMLAL, SQDMLAL2 (vector)
    0b01011110_01100000_10010000_00000000 = [S, H, H] => [R(0), R(5), R(16)];
    0b01011110_10100000_10010000_00000000 = [D, S, S] => [R(0), R(5), R(16)];
    0b00001110_01100000_10010000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VSizedStatic(WORD, 4)] => [R(0), R(5), R(16)];
    0b00001110_10100000_10010000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2), VSizedStatic(DWORD, 2)] => [R(0), R(5), R(16)];
]
"sqdmlal2" = [
    // SQDMLAL, SQDMLAL2 (by element)
    0b01001111_01000000_00110000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8), VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20])];
    0b01001111_10000000_00110000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4), VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    // SQDMLAL, SQDMLAL2 (vector)
    0b01001110_01100000_10010000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8), VSizedStatic(WORD, 8)] => [R(0), R(5), R(16)];
    0b01001110_10100000_10010000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"sqdmlsl" = [
    // SQDMLSL, SQDMLSL2 (by element)
    0b01011111_01000000_01110000_00000000 = [S, H, VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20])];
    0b01011111_10000000_01110000_00000000 = [D, S, VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    0b00001111_01000000_01110000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20])];
    0b00001111_10000000_01110000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2), VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    // SQDMLSL, SQDMLSL2 (vector)
    0b01011110_01100000_10110000_00000000 = [S, H, H] => [R(0), R(5), R(16)];
    0b01011110_10100000_10110000_00000000 = [D, S, S] => [R(0), R(5), R(16)];
    0b00001110_01100000_10110000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VSizedStatic(WORD, 4)] => [R(0), R(5), R(16)];
    0b00001110_10100000_10110000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2), VSizedStatic(DWORD, 2)] => [R(0), R(5), R(16)];
]
"sqdmlsl2" = [
    // SQDMLSL, SQDMLSL2 (by element)
    0b01001111_01000000_01110000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8), VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20])];
    0b01001111_10000000_01110000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4), VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    // SQDMLSL, SQDMLSL2 (vector)
    0b01001110_01100000_10110000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8), VSizedStatic(WORD, 8)] => [R(0), R(5), R(16)];
    0b01001110_10100000_10110000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"sqdmulh" = [
    // SQDMULH (by element)
    0b01011111_01000000_11000000_00000000 = [H, H, VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20])];
    0b01011111_10000000_11000000_00000000 = [S, S, VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    0b00001111_01000000_11000000_00000000 = [VSized(WORD), VSized(WORD), VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20]), Rwidth(30)];
    0b00001111_10000000_11000000_00000000 = [VSized(DWORD), VSized(DWORD), VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21]), Rwidth(30)];
    // SQDMULH (vector)
    0b01011110_01100000_10110100_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b01011110_10100000_10110100_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b00001110_01100000_10110100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10100000_10110100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
]
"sqdmull" = [
    // SQDMULL, SQDMULL2 (by element)
    0b01011111_01000000_10110000_00000000 = [S, H, VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20])];
    0b01011111_10000000_10110000_00000000 = [D, S, VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    0b00001111_01000000_10110000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20])];
    0b00001111_10000000_10110000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2), VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    // SQDMULL, SQDMULL2 (vector)
    0b01011110_01100000_11010000_00000000 = [S, H, H] => [R(0), R(5), R(16)];
    0b01011110_10100000_11010000_00000000 = [D, S, S] => [R(0), R(5), R(16)];
    0b00001110_01100000_11010000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VSizedStatic(WORD, 4)] => [R(0), R(5), R(16)];
    0b00001110_10100000_11010000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2), VSizedStatic(DWORD, 2)] => [R(0), R(5), R(16)];
]
"sqdmull2" = [
    // SQDMULL, SQDMULL2 (by element)
    0b01001111_01000000_10110000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8), VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20])];
    0b01001111_10000000_10110000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4), VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    // SQDMULL, SQDMULL2 (vector)
    0b01001110_01100000_11010000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8), VSizedStatic(WORD, 8)] => [R(0), R(5), R(16)];
    0b01001110_10100000_11010000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"sqneg" = [
    0b01111110_00100000_01111000_00000000 = [B, B] => [R(0), R(5)];
    0b01111110_01100000_01111000_00000000 = [H, H] => [R(0), R(5)];
    0b01111110_10100000_01111000_00000000 = [S, S] => [R(0), R(5)];
    0b01111110_11100000_01111000_00000000 = [D, D] => [R(0), R(5)];
    0b00101110_00100000_01111000_00000000 = [VSized(BYTE), VSized(BYTE)] => [R(0), R(5), Rwidth(30)];
    0b00101110_01100000_01111000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_10100000_01111000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_11100000_01111000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
]
"sqrdmlah" = [
    // SQRDMLAH (by element)
    0b01111111_01000000_11010000_00000000 = [H, H, VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20])];
    0b01111111_10000000_11010000_00000000 = [S, S, VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    0b00101111_01000000_11010000_00000000 = [VSized(WORD), VSized(WORD), VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20]), Rwidth(30)];
    0b00101111_10000000_11010000_00000000 = [VSized(DWORD), VSized(DWORD), VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21]), Rwidth(30)];
    // SQRDMLAH (vector)
    0b01111110_01000000_10000100_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b01111110_10000000_10000100_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b00101110_01000000_10000100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_10000000_10000100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
]
"sqrdmlsh" = [
    // SQRDMLSH (by element)
    0b01111111_01000000_11110000_00000000 = [H, H, VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20])];
    0b01111111_10000000_11110000_00000000 = [S, S, VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    0b00101111_01000000_11110000_00000000 = [VSized(WORD), VSized(WORD), VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20]), Rwidth(30)];
    0b00101111_10000000_11110000_00000000 = [VSized(DWORD), VSized(DWORD), VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21]), Rwidth(30)];
    // SQRDMLSH (vector)
    0b01111110_01000000_10001100_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b01111110_10000000_10001100_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b00101110_01000000_10001100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_10000000_10001100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
]
"sqrdmulh" = [
    // SQRDMULH (by element)
    0b01011111_01000000_11010000_00000000 = [H, H, VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20])];
    0b01011111_10000000_11010000_00000000 = [S, S, VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    0b00001111_01000000_11010000_00000000 = [VSized(WORD), VSized(WORD), VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20]), Rwidth(30)];
    0b00001111_10000000_11010000_00000000 = [VSized(DWORD), VSized(DWORD), VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21]), Rwidth(30)];
    // SQRDMULH (vector)
    0b01111110_01100000_10110100_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b01111110_10100000_10110100_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b00101110_01100000_10110100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_10100000_10110100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
]
"sqrshl" = [
    0b01011110_00100000_01011100_00000000 = [B, B, B] => [R(0), R(5), R(16)];
    0b01011110_01100000_01011100_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b01011110_10100000_01011100_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b01011110_11100000_01011100_00000000 = [D, D, D] => [R(0), R(5), R(16)];
    0b00001110_00100000_01011100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01100000_01011100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10100000_01011100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_11100000_01011100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"sqrshrn" = [
    0b01011111_00000000_10011100_00000000 = [B, H, Imm] => [R(0), R(5), BUrange(1, 8), Usub(16, 4, 16)];
    0b01011111_00000000_10011100_00000000 = [H, S, Imm] => [R(0), R(5), BUrange(1, 16), Usub(16, 5, 32)];
    0b01011111_00000000_10011100_00000000 = [S, D, Imm] => [R(0), R(5), BUrange(1, 32), Usub(16, 6, 64)];
    0b00001111_00001000_10011100_00000000 = [VSizedStatic(BYTE, 8), VSizedStatic(WORD, 8), Imm] => [R(0), R(5), Usub(16, 3, 8)];
    0b00001111_00010000_10011100_00000000 = [VSizedStatic(WORD, 4), VSizedStatic(DWORD, 4), Imm] => [R(0), R(5), Usub(16, 4, 16)];
    0b00001111_00100000_10011100_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Usub(16, 5, 32)];
]
"sqrshrn2" = [
    0b01001111_00001000_10011100_00000000 = [VSizedStatic(BYTE, 16), VSizedStatic(WORD, 8), Imm] => [R(0), R(5), Usub(16, 3, 8)];
    0b01001111_00010000_10011100_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(DWORD, 4), Imm] => [R(0), R(5), Usub(16, 4, 16)];
    0b01001111_00100000_10011100_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Usub(16, 5, 32)];
]
"sqrshrun" = [
    0b01111111_00000000_10001100_00000000 = [B, H, Imm] => [R(0), R(5), BUrange(1, 8), Usub(16, 4, 16)];
    0b01111111_00000000_10001100_00000000 = [H, S, Imm] => [R(0), R(5), BUrange(1, 16), Usub(16, 5, 32)];
    0b01111111_00000000_10001100_00000000 = [S, D, Imm] => [R(0), R(5), BUrange(1, 32), Usub(16, 6, 64)];
    0b00101111_00001000_10001100_00000000 = [VSizedStatic(BYTE, 8), VSizedStatic(WORD, 8), Imm] => [R(0), R(5), Usub(16, 3, 8)];
    0b00101111_00010000_10001100_00000000 = [VSizedStatic(WORD, 4), VSizedStatic(DWORD, 4), Imm] => [R(0), R(5), Usub(16, 4, 16)];
    0b00101111_00100000_10001100_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Usub(16, 5, 32)];
]
"sqrshrun2" = [
    0b01101111_00001000_10001100_00000000 = [VSizedStatic(BYTE, 16), VSizedStatic(WORD, 8), Imm] => [R(0), R(5), Usub(16, 3, 8)];
    0b01101111_00010000_10001100_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(DWORD, 4), Imm] => [R(0), R(5), Usub(16, 4, 16)];
    0b01101111_00100000_10001100_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Usub(16, 5, 32)];
]
"sqshl" = [
    // SQSHL (immediate)
    0b01011111_00001000_01110100_00000000 = [B, B, Imm] => [R(0), R(5), Ubits(16, 3)];
    0b01011111_00010000_01110100_00000000 = [H, H, Imm] => [R(0), R(5), Ubits(16, 4)];
    0b01011111_00100000_01110100_00000000 = [S, S, Imm] => [R(0), R(5), Ubits(16, 5)];
    0b01011111_01000000_01110100_00000000 = [D, D, Imm] => [R(0), R(5), Ubits(16, 6)];
    0b00001111_00001000_01110100_00000000 = [VSized(BYTE), VSized(BYTE), Imm] => [R(0), R(5), Ubits(16, 3), Rwidth(30)];
    0b00001111_00010000_01110100_00000000 = [VSized(WORD), VSized(WORD), Imm] => [R(0), R(5), Ubits(16, 4), Rwidth(30)];
    0b00001111_00100000_01110100_00000000 = [VSized(DWORD), VSized(DWORD), Imm] => [R(0), R(5), Ubits(16, 5), Rwidth(30)];
    0b00001111_01000000_01110100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Ubits(16, 6), Rwidth(30)];
    // SQSHL (register)
    0b01011110_00100000_01001100_00000000 = [B, B, B] => [R(0), R(5), R(16)];
    0b01011110_01100000_01001100_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b01011110_10100000_01001100_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b01011110_11100000_01001100_00000000 = [D, D, D] => [R(0), R(5), R(16)];
    0b00001110_00100000_01001100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01100000_01001100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10100000_01001100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_11100000_01001100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"sqshlu" = [
    0b01111111_00001000_01100100_00000000 = [B, B, Imm] => [R(0), R(5), Ubits(16, 3)];
    0b01111111_00010000_01100100_00000000 = [H, H, Imm] => [R(0), R(5), Ubits(16, 4)];
    0b01111111_00100000_01100100_00000000 = [S, S, Imm] => [R(0), R(5), Ubits(16, 5)];
    0b01111111_01000000_01100100_00000000 = [D, D, Imm] => [R(0), R(5), Ubits(16, 6)];
    0b00101111_00001000_01100100_00000000 = [VSized(BYTE), VSized(BYTE), Imm] => [R(0), R(5), Ubits(16, 3), Rwidth(30)];
    0b00101111_00010000_01100100_00000000 = [VSized(WORD), VSized(WORD), Imm] => [R(0), R(5), Ubits(16, 4), Rwidth(30)];
    0b00101111_00100000_01100100_00000000 = [VSized(DWORD), VSized(DWORD), Imm] => [R(0), R(5), Ubits(16, 5), Rwidth(30)];
    0b00101111_01000000_01100100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Ubits(16, 6), Rwidth(30)];
]
"sqshrn" = [
    0b01011111_00000000_10010100_00000000 = [B, H, Imm] => [R(0), R(5), BUrange(1, 8), Usub(16, 4, 16)];
    0b01011111_00000000_10010100_00000000 = [H, S, Imm] => [R(0), R(5), BUrange(1, 16), Usub(16, 5, 32)];
    0b01011111_00000000_10010100_00000000 = [S, D, Imm] => [R(0), R(5), BUrange(1, 32), Usub(16, 6, 64)];
    0b00001111_00001000_10010100_00000000 = [VSizedStatic(BYTE, 8), VSizedStatic(WORD, 8), Imm] => [R(0), R(5), Usub(16, 3, 8)];
    0b00001111_00010000_10010100_00000000 = [VSizedStatic(WORD, 4), VSizedStatic(DWORD, 4), Imm] => [R(0), R(5), Usub(16, 4, 16)];
    0b00001111_00100000_10010100_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Usub(16, 5, 32)];
]
"sqshrn2" = [
    0b01001111_00001000_10010100_00000000 = [VSizedStatic(BYTE, 16), VSizedStatic(WORD, 8), Imm] => [R(0), R(5), Usub(16, 3, 8)];
    0b01001111_00010000_10010100_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(DWORD, 4), Imm] => [R(0), R(5), Usub(16, 4, 16)];
    0b01001111_00100000_10010100_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Usub(16, 5, 32)];
]
"sqshrun" = [
    0b01111111_00000000_10000100_00000000 = [B, H, Imm] => [R(0), R(5), BUrange(1, 8), Usub(16, 4, 16)];
    0b01111111_00000000_10000100_00000000 = [H, S, Imm] => [R(0), R(5), BUrange(1, 16), Usub(16, 5, 32)];
    0b01111111_00000000_10000100_00000000 = [S, D, Imm] => [R(0), R(5), BUrange(1, 32), Usub(16, 6, 64)];
    0b00101111_00001000_10000100_00000000 = [VSizedStatic(BYTE, 8), VSizedStatic(WORD, 8), Imm] => [R(0), R(5), Usub(16, 3, 8)];
    0b00101111_00010000_10000100_00000000 = [VSizedStatic(WORD, 4), VSizedStatic(DWORD, 4), Imm] => [R(0), R(5), Usub(16, 4, 16)];
    0b00101111_00100000_10000100_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Usub(16, 5, 32)];
]
"sqshrun2" = [
    0b01101111_00001000_10000100_00000000 = [VSizedStatic(BYTE, 16), VSizedStatic(WORD, 8), Imm] => [R(0), R(5), Usub(16, 3, 8)];
    0b01101111_00010000_10000100_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(DWORD, 4), Imm] => [R(0), R(5), Usub(16, 4, 16)];
    0b01101111_00100000_10000100_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Usub(16, 5, 32)];
]
"sqsub" = [
    0b01011110_00100000_00101100_00000000 = [B, B, B] => [R(0), R(5), R(16)];
    0b01011110_01100000_00101100_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b01011110_10100000_00101100_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b01011110_11100000_00101100_00000000 = [D, D, D] => [R(0), R(5), R(16)];
    0b00001110_00100000_00101100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01100000_00101100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10100000_00101100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_11100000_00101100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"sqxtn" = [
    0b01011110_00100001_01001000_00000000 = [B, H] => [R(0), R(5)];
    0b01011110_01100001_01001000_00000000 = [H, S] => [R(0), R(5)];
    0b01011110_10100001_01001000_00000000 = [S, D] => [R(0), R(5)];
    0b00001110_00100001_01001000_00000000 = [VSizedStatic(BYTE, 8), VSizedStatic(WORD, 8)] => [R(0), R(5)];
    0b00001110_01100001_01001000_00000000 = [VSizedStatic(WORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5)];
    0b00001110_10100001_01001000_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5)];
]
"sqxtn2" = [
    0b01001110_00100001_01001000_00000000 = [VSizedStatic(BYTE, 16), VSizedStatic(WORD, 8)] => [R(0), R(5)];
    0b01001110_01100001_01001000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(DWORD, 4)] => [R(0), R(5)];
    0b01001110_10100001_01001000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(QWORD, 2)] => [R(0), R(5)];
]
"sqxtun" = [
    0b01111110_00100001_00101000_00000000 = [B, H] => [R(0), R(5)];
    0b01111110_01100001_00101000_00000000 = [H, S] => [R(0), R(5)];
    0b01111110_10100001_00101000_00000000 = [S, D] => [R(0), R(5)];
    0b00101110_00100001_00101000_00000000 = [VSizedStatic(BYTE, 8), VSizedStatic(WORD, 8)] => [R(0), R(5)];
    0b00101110_01100001_00101000_00000000 = [VSizedStatic(WORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5)];
    0b00101110_10100001_00101000_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5)];
]
"sqxtun2" = [
    0b01101110_00100001_00101000_00000000 = [VSizedStatic(BYTE, 16), VSizedStatic(WORD, 8)] => [R(0), R(5)];
    0b01101110_01100001_00101000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(DWORD, 4)] => [R(0), R(5)];
    0b01101110_10100001_00101000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(QWORD, 2)] => [R(0), R(5)];
]
"srhadd" = [
    0b00001110_00100000_00010100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01100000_00010100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10100000_00010100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
]
"sri" = [
    0b01111111_00000000_01000100_00000000 = [D, D, Imm] => [R(0), R(5), BUrange(1, 64), Usub(16, 7, 128)];
    0b00101111_00001000_01000100_00000000 = [VSized(BYTE), VSized(BYTE), Imm] => [R(0), R(5), Usub(16, 3, 8), Rwidth(30)];
    0b00101111_00010000_01000100_00000000 = [VSized(WORD), VSized(WORD), Imm] => [R(0), R(5), Usub(16, 4, 16), Rwidth(30)];
    0b00101111_00100000_01000100_00000000 = [VSized(DWORD), VSized(DWORD), Imm] => [R(0), R(5), Usub(16, 5, 32), Rwidth(30)];
    0b00101111_01000000_01000100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Usub(16, 6, 64), Rwidth(30)];
]
"srshl" = [
    0b01011110_11100000_01010100_00000000 = [D, D, D] => [R(0), R(5), R(16)];
    0b00001110_00100000_01010100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01100000_01010100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10100000_01010100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_11100000_01010100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"srshr" = [
    0b01011111_00000000_00100100_00000000 = [D, D, Imm] => [R(0), R(5), BUrange(1, 64), Usub(16, 7, 128)];
    0b00001111_00001000_00100100_00000000 = [VSized(BYTE), VSized(BYTE), Imm] => [R(0), R(5), Usub(16, 3, 8), Rwidth(30)];
    0b00001111_00010000_00100100_00000000 = [VSized(WORD), VSized(WORD), Imm] => [R(0), R(5), Usub(16, 4, 16), Rwidth(30)];
    0b00001111_00100000_00100100_00000000 = [VSized(DWORD), VSized(DWORD), Imm] => [R(0), R(5), Usub(16, 5, 32), Rwidth(30)];
    0b00001111_01000000_00100100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Usub(16, 6, 64), Rwidth(30)];
]
"srsra" = [
    0b01011111_00000000_00110100_00000000 = [D, D, Imm] => [R(0), R(5), BUrange(1, 64), Usub(16, 7, 128)];
    0b00001111_00001000_00110100_00000000 = [VSized(BYTE), VSized(BYTE), Imm] => [R(0), R(5), Usub(16, 3, 8), Rwidth(30)];
    0b00001111_00010000_00110100_00000000 = [VSized(WORD), VSized(WORD), Imm] => [R(0), R(5), Usub(16, 4, 16), Rwidth(30)];
    0b00001111_00100000_00110100_00000000 = [VSized(DWORD), VSized(DWORD), Imm] => [R(0), R(5), Usub(16, 5, 32), Rwidth(30)];
    0b00001111_01000000_00110100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Usub(16, 6, 64), Rwidth(30)];
]
"ssbb" = [
    0b11010101_00000011_00110000_10011111 = [] => [];
]
"sshl" = [
    0b01011110_11100000_01000100_00000000 = [D, D, D] => [R(0), R(5), R(16)];
    0b00001110_00100000_01000100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01100000_01000100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10100000_01000100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_11100000_01000100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"sshll" = [
    0b00001111_00001000_10100100_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 8), Imm] => [R(0), R(5), Ubits(16, 3)];
    0b00001111_00010000_10100100_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), Imm] => [R(0), R(5), Ubits(16, 4)];
    0b00001111_00100000_10100100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2), Imm] => [R(0), R(5), Ubits(16, 5)];
]
"sshll2" = [
    0b01001111_00001000_10100100_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 16), Imm] => [R(0), R(5), Ubits(16, 3)];
    0b01001111_00010000_10100100_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8), Imm] => [R(0), R(5), Ubits(16, 4)];
    0b01001111_00100000_10100100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4), Imm] => [R(0), R(5), Ubits(16, 5)];
]
"sshr" = [
    0b01011111_00000000_00000100_00000000 = [D, D, Imm] => [R(0), R(5), BUrange(1, 64), Usub(16, 7, 128)];
    0b00001111_00001000_00000100_00000000 = [VSized(BYTE), VSized(BYTE), Imm] => [R(0), R(5), Usub(16, 3, 8), Rwidth(30)];
    0b00001111_00010000_00000100_00000000 = [VSized(WORD), VSized(WORD), Imm] => [R(0), R(5), Usub(16, 4, 16), Rwidth(30)];
    0b00001111_00100000_00000100_00000000 = [VSized(DWORD), VSized(DWORD), Imm] => [R(0), R(5), Usub(16, 5, 32), Rwidth(30)];
    0b00001111_01000000_00000100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Usub(16, 6, 64), Rwidth(30)];
]
"ssra" = [
    0b01011111_00000000_00010100_00000000 = [D, D, Imm] => [R(0), R(5), BUrange(1, 64), Usub(16, 7, 128)];
    0b00001111_00001000_00010100_00000000 = [VSized(BYTE), VSized(BYTE), Imm] => [R(0), R(5), Usub(16, 3, 8), Rwidth(30)];
    0b00001111_00010000_00010100_00000000 = [VSized(WORD), VSized(WORD), Imm] => [R(0), R(5), Usub(16, 4, 16), Rwidth(30)];
    0b00001111_00100000_00010100_00000000 = [VSized(DWORD), VSized(DWORD), Imm] => [R(0), R(5), Usub(16, 5, 32), Rwidth(30)];
    0b00001111_01000000_00010100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Usub(16, 6, 64), Rwidth(30)];
]
"ssubl" = [
    0b00001110_00100000_00100000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 8), VSizedStatic(BYTE, 8)] => [R(0), R(5), R(16)];
    0b00001110_01100000_00100000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VSizedStatic(WORD, 4)] => [R(0), R(5), R(16)];
    0b00001110_10100000_00100000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2), VSizedStatic(DWORD, 2)] => [R(0), R(5), R(16)];
]
"ssubl2" = [
    0b01001110_00100000_00100000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16)] => [R(0), R(5), R(16)];
    0b01001110_01100000_00100000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8), VSizedStatic(WORD, 8)] => [R(0), R(5), R(16)];
    0b01001110_10100000_00100000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"ssubw" = [
    0b00001110_00100000_00110000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(WORD, 8), VSizedStatic(BYTE, 8)] => [R(0), R(5), R(16)];
    0b00001110_01100000_00110000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4)] => [R(0), R(5), R(16)];
    0b00001110_10100000_00110000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2)] => [R(0), R(5), R(16)];
]
"ssubw2" = [
    0b01001110_00100000_00110000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(WORD, 8), VSizedStatic(BYTE, 16)] => [R(0), R(5), R(16)];
    0b01001110_01100000_00110000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8)] => [R(0), R(5), R(16)];
    0b01001110_10100000_00110000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"st1" = [
    // ST1 (multiple structures)
    0b00001100_00000000_01110000_00000000 = [RegList(1, BYTE), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_00000000_01110100_00000000 = [RegList(1, WORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_00000000_01111000_00000000 = [RegList(1, DWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_00000000_01111100_00000000 = [RegList(1, QWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_00000000_10100000_00000000 = [RegList(2, BYTE), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_00000000_10100100_00000000 = [RegList(2, WORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_00000000_10101000_00000000 = [RegList(2, DWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_00000000_10101100_00000000 = [RegList(2, QWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_00000000_01100000_00000000 = [RegList(3, BYTE), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_00000000_01100100_00000000 = [RegList(3, WORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_00000000_01101000_00000000 = [RegList(3, DWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_00000000_01101100_00000000 = [RegList(3, QWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_00000000_00100000_00000000 = [RegList(4, BYTE), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_00000000_00100100_00000000 = [RegList(4, WORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_00000000_00101000_00000000 = [RegList(4, DWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_00000000_00101100_00000000 = [RegList(4, QWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_10011111_01110000_00000000 = [RegListSized(1, BYTE, 8), RefBase, LitInt(8)] => [R(0), R(5)];
    0b00001100_10011111_01110100_00000000 = [RegListSized(1, WORD, 4), RefBase, LitInt(8)] => [R(0), R(5)];
    0b00001100_10011111_01111000_00000000 = [RegListSized(1, DWORD, 2), RefBase, LitInt(8)] => [R(0), R(5)];
    0b00001100_10011111_01111100_00000000 = [RegListSized(1, QWORD, 1), RefBase, LitInt(8)] => [R(0), R(5)];
    0b01001100_10011111_01110000_00000000 = [RegListSized(1, BYTE, 16), RefBase, LitInt(16)] => [R(0), R(5)];
    0b01001100_10011111_01110100_00000000 = [RegListSized(1, WORD, 8), RefBase, LitInt(16)] => [R(0), R(5)];
    0b01001100_10011111_01111000_00000000 = [RegListSized(1, DWORD, 4), RefBase, LitInt(16)] => [R(0), R(5)];
    0b01001100_10011111_01111100_00000000 = [RegListSized(1, QWORD, 2), RefBase, LitInt(16)] => [R(0), R(5)];
    0b00001100_10000000_01110000_00000000 = [RegList(1, BYTE), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_10000000_01110100_00000000 = [RegList(1, WORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_10000000_01111000_00000000 = [RegList(1, DWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_10000000_01111100_00000000 = [RegList(1, QWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_10011111_10100000_00000000 = [RegListSized(2, BYTE, 8), RefBase, LitInt(16)] => [R(0), R(5)];
    0b00001100_10011111_10100100_00000000 = [RegListSized(2, WORD, 4), RefBase, LitInt(16)] => [R(0), R(5)];
    0b00001100_10011111_10101000_00000000 = [RegListSized(2, DWORD, 2), RefBase, LitInt(16)] => [R(0), R(5)];
    0b00001100_10011111_10101100_00000000 = [RegListSized(2, QWORD, 1), RefBase, LitInt(16)] => [R(0), R(5)];
    0b01001100_10011111_10100000_00000000 = [RegListSized(2, BYTE, 16), RefBase, LitInt(32)] => [R(0), R(5)];
    0b01001100_10011111_10100100_00000000 = [RegListSized(2, WORD, 8), RefBase, LitInt(32)] => [R(0), R(5)];
    0b01001100_10011111_10101000_00000000 = [RegListSized(2, DWORD, 4), RefBase, LitInt(32)] => [R(0), R(5)];
    0b01001100_10011111_10101100_00000000 = [RegListSized(2, QWORD, 2), RefBase, LitInt(32)] => [R(0), R(5)];
    0b00001100_10000000_10100000_00000000 = [RegList(2, BYTE), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_10000000_10100100_00000000 = [RegList(2, WORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_10000000_10101000_00000000 = [RegList(2, DWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_10000000_10101100_00000000 = [RegList(2, QWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_10011111_01100000_00000000 = [RegListSized(3, BYTE, 8), RefBase, LitInt(24)] => [R(0), R(5)];
    0b00001100_10011111_01100100_00000000 = [RegListSized(3, WORD, 4), RefBase, LitInt(24)] => [R(0), R(5)];
    0b00001100_10011111_01101000_00000000 = [RegListSized(3, DWORD, 2), RefBase, LitInt(24)] => [R(0), R(5)];
    0b00001100_10011111_01101100_00000000 = [RegListSized(3, QWORD, 1), RefBase, LitInt(24)] => [R(0), R(5)];
    0b01001100_10011111_01100000_00000000 = [RegListSized(3, BYTE, 16), RefBase, LitInt(48)] => [R(0), R(5)];
    0b01001100_10011111_01100100_00000000 = [RegListSized(3, WORD, 8), RefBase, LitInt(48)] => [R(0), R(5)];
    0b01001100_10011111_01101000_00000000 = [RegListSized(3, DWORD, 4), RefBase, LitInt(48)] => [R(0), R(5)];
    0b01001100_10011111_01101100_00000000 = [RegListSized(3, QWORD, 2), RefBase, LitInt(48)] => [R(0), R(5)];
    0b00001100_10000000_01100000_00000000 = [RegList(3, BYTE), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_10000000_01100100_00000000 = [RegList(3, WORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_10000000_01101000_00000000 = [RegList(3, DWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_10000000_01101100_00000000 = [RegList(3, QWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_10011111_00100000_00000000 = [RegListSized(4, BYTE, 8), RefBase, LitInt(32)] => [R(0), R(5)];
    0b00001100_10011111_00100100_00000000 = [RegListSized(4, WORD, 4), RefBase, LitInt(32)] => [R(0), R(5)];
    0b00001100_10011111_00101000_00000000 = [RegListSized(4, DWORD, 2), RefBase, LitInt(32)] => [R(0), R(5)];
    0b00001100_10011111_00101100_00000000 = [RegListSized(4, QWORD, 1), RefBase, LitInt(32)] => [R(0), R(5)];
    0b01001100_10011111_00100000_00000000 = [RegListSized(4, BYTE, 16), RefBase, LitInt(64)] => [R(0), R(5)];
    0b01001100_10011111_00100100_00000000 = [RegListSized(4, WORD, 8), RefBase, LitInt(64)] => [R(0), R(5)];
    0b01001100_10011111_00101000_00000000 = [RegListSized(4, DWORD, 4), RefBase, LitInt(64)] => [R(0), R(5)];
    0b01001100_10011111_00101100_00000000 = [RegListSized(4, QWORD, 2), RefBase, LitInt(64)] => [R(0), R(5)];
    0b00001100_10000000_00100000_00000000 = [RegList(4, BYTE), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_10000000_00100100_00000000 = [RegList(4, WORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_10000000_00101000_00000000 = [RegList(4, DWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_10000000_00101100_00000000 = [RegList(4, QWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    // ST1 (single structure)
    0b00001101_00000000_00000000_00000000 = [RegListLanes(1, BYTE), RefBase] => [R(0), Ufields(&[30, 12, 11, 10]), R(5)];
    0b00001101_00000000_01000000_00000000 = [RegListLanes(1, WORD), RefBase] => [R(0), Ufields(&[30, 12, 11]), R(5)];
    0b00001101_00000000_10000000_00000000 = [RegListLanes(1, DWORD), RefBase] => [R(0), Ufields(&[30, 12]), R(5)];
    0b00001101_00000000_10000100_00000000 = [RegListLanes(1, QWORD), RefBase] => [R(0), Ufields(&[30]), R(5)];
    0b00001101_10011111_00000000_00000000 = [RegListLanes(1, BYTE), RefBase, LitInt(1)] => [R(0), Ufields(&[30, 12, 11, 10]), R(5)];
    0b00001101_10000000_00000000_00000000 = [RegListLanes(1, BYTE), RefBase, X] => [R(0), Ufields(&[30, 12, 11, 10]), R(5), R(16)];
    0b00001101_10011111_01000000_00000000 = [RegListLanes(1, WORD), RefBase, LitInt(2)] => [R(0), Ufields(&[30, 12, 11]), R(5)];
    0b00001101_10000000_01000000_00000000 = [RegListLanes(1, WORD), RefBase, X] => [R(0), Ufields(&[30, 12, 11]), R(5), R(16)];
    0b00001101_10011111_10000000_00000000 = [RegListLanes(1, DWORD), RefBase, LitInt(4)] => [R(0), Ufields(&[30, 12]), R(5)];
    0b00001101_10000000_10000000_00000000 = [RegListLanes(1, DWORD), RefBase, X] => [R(0), Ufields(&[30, 12]), R(5), R(16)];
    0b00001101_10011111_10000100_00000000 = [RegListLanes(1, QWORD), RefBase, LitInt(8)] => [R(0), Ufields(&[30]), R(5)];
    0b00001101_10000000_10000100_00000000 = [RegListLanes(1, QWORD), RefBase, X] => [R(0), Ufields(&[30]), R(5), R(16)];
]
"st2" = [
    // ST2 (multiple structures)
    0b00001100_00000000_10000000_00000000 = [RegList(2, BYTE), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_00000000_10000100_00000000 = [RegList(2, WORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_00000000_10001000_00000000 = [RegList(2, DWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_00000000_10001100_00000000 = [RegListSized(2, QWORD, 2), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_10011111_10000000_00000000 = [RegListSized(2, BYTE, 8), RefBase, LitInt(16)] => [R(0), R(5)];
    0b00001100_10011111_10000100_00000000 = [RegListSized(2, WORD, 4), RefBase, LitInt(16)] => [R(0), R(5)];
    0b00001100_10011111_10001000_00000000 = [RegListSized(2, DWORD, 2), RefBase, LitInt(16)] => [R(0), R(5)];
    0b01001100_10011111_10000000_00000000 = [RegListSized(2, BYTE, 16), RefBase, LitInt(32)] => [R(0), R(5)];
    0b01001100_10011111_10000100_00000000 = [RegListSized(2, WORD, 8), RefBase, LitInt(32)] => [R(0), R(5)];
    0b01001100_10011111_10001000_00000000 = [RegListSized(2, DWORD, 4), RefBase, LitInt(32)] => [R(0), R(5)];
    0b01001100_10011111_10001100_00000000 = [RegListSized(2, QWORD, 2), RefBase, LitInt(32)] => [R(0), R(5)];
    0b00001100_10000000_10000000_00000000 = [RegList(2, BYTE), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_10000000_10000100_00000000 = [RegList(2, WORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_10000000_10001000_00000000 = [RegList(2, DWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_10000000_10001100_00000000 = [RegListSized(2, QWORD, 2), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    // ST2 (single structure)
    0b00001101_00100000_00000000_00000000 = [RegListLanes(2, BYTE), RefBase] => [R(0), Ufields(&[30, 12, 11, 10]), R(5)];
    0b00001101_00100000_01000000_00000000 = [RegListLanes(2, WORD), RefBase] => [R(0), Ufields(&[30, 12, 11]), R(5)];
    0b00001101_00100000_10000000_00000000 = [RegListLanes(2, DWORD), RefBase] => [R(0), Ufields(&[30, 12]), R(5)];
    0b00001101_00100000_10000100_00000000 = [RegListLanes(2, QWORD), RefBase] => [R(0), Ufields(&[30]), R(5)];
    0b00001101_10111111_00000000_00000000 = [RegListLanes(2, BYTE), RefBase, LitInt(2)] => [R(0), Ufields(&[30, 12, 11, 10]), R(5)];
    0b00001101_10100000_00000000_00000000 = [RegListLanes(2, BYTE), RefBase, X] => [R(0), Ufields(&[30, 12, 11, 10]), R(5), R(16)];
    0b00001101_10111111_01000000_00000000 = [RegListLanes(2, WORD), RefBase, LitInt(4)] => [R(0), Ufields(&[30, 12, 11]), R(5)];
    0b00001101_10100000_01000000_00000000 = [RegListLanes(2, WORD), RefBase, X] => [R(0), Ufields(&[30, 12, 11]), R(5), R(16)];
    0b00001101_10111111_10000000_00000000 = [RegListLanes(2, DWORD), RefBase, LitInt(8)] => [R(0), Ufields(&[30, 12]), R(5)];
    0b00001101_10100000_10000000_00000000 = [RegListLanes(2, DWORD), RefBase, X] => [R(0), Ufields(&[30, 12]), R(5), R(16)];
    0b00001101_10111111_10000100_00000000 = [RegListLanes(2, QWORD), RefBase, LitInt(16)] => [R(0), Ufields(&[30]), R(5)];
    0b00001101_10100000_10000100_00000000 = [RegListLanes(2, QWORD), RefBase, X] => [R(0), Ufields(&[30]), R(5), R(16)];
]
"st3" = [
    // ST3 (multiple structures)
    0b00001100_00000000_01000000_00000000 = [RegList(3, BYTE), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_00000000_01000100_00000000 = [RegList(3, WORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_00000000_01001000_00000000 = [RegList(3, DWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_00000000_01001100_00000000 = [RegListSized(3, QWORD, 2), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_10011111_01000000_00000000 = [RegListSized(3, BYTE, 8), RefBase, LitInt(24)] => [R(0), R(5)];
    0b00001100_10011111_01000100_00000000 = [RegListSized(3, WORD, 4), RefBase, LitInt(24)] => [R(0), R(5)];
    0b00001100_10011111_01001000_00000000 = [RegListSized(3, DWORD, 2), RefBase, LitInt(24)] => [R(0), R(5)];
    0b01001100_10011111_01000000_00000000 = [RegListSized(3, BYTE, 16), RefBase, LitInt(48)] => [R(0), R(5)];
    0b01001100_10011111_01000100_00000000 = [RegListSized(3, WORD, 8), RefBase, LitInt(48)] => [R(0), R(5)];
    0b01001100_10011111_01001000_00000000 = [RegListSized(3, DWORD, 4), RefBase, LitInt(48)] => [R(0), R(5)];
    0b01001100_10011111_01001100_00000000 = [RegListSized(3, QWORD, 2), RefBase, LitInt(48)] => [R(0), R(5)];
    0b00001100_10000000_01000000_00000000 = [RegList(3, BYTE), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_10000000_01000100_00000000 = [RegList(3, WORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_10000000_01001000_00000000 = [RegList(3, DWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_10000000_01001100_00000000 = [RegListSized(3, QWORD, 2), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    // ST3 (single structure)
    0b00001101_00000000_00100000_00000000 = [RegListLanes(3, BYTE), RefBase] => [R(0), Ufields(&[30, 12, 11, 10]), R(5)];
    0b00001101_00000000_01100000_00000000 = [RegListLanes(3, WORD), RefBase] => [R(0), Ufields(&[30, 12, 11]), R(5)];
    0b00001101_00000000_10100000_00000000 = [RegListLanes(3, DWORD), RefBase] => [R(0), Ufields(&[30, 12]), R(5)];
    0b00001101_00000000_10100100_00000000 = [RegListLanes(3, QWORD), RefBase] => [R(0), Ufields(&[30]), R(5)];
    0b00001101_10011111_00100000_00000000 = [RegListLanes(3, BYTE), RefBase, LitInt(3)] => [R(0), Ufields(&[30, 12, 11, 10]), R(5)];
    0b00001101_10000000_00100000_00000000 = [RegListLanes(3, BYTE), RefBase, X] => [R(0), Ufields(&[30, 12, 11, 10]), R(5), R(16)];
    0b00001101_10011111_01100000_00000000 = [RegListLanes(3, WORD), RefBase, LitInt(6)] => [R(0), Ufields(&[30, 12, 11]), R(5)];
    0b00001101_10000000_01100000_00000000 = [RegListLanes(3, WORD), RefBase, X] => [R(0), Ufields(&[30, 12, 11]), R(5), R(16)];
    0b00001101_10011111_10100000_00000000 = [RegListLanes(3, DWORD), RefBase, LitInt(12)] => [R(0), Ufields(&[30, 12]), R(5)];
    0b00001101_10000000_10100000_00000000 = [RegListLanes(3, DWORD), RefBase, X] => [R(0), Ufields(&[30, 12]), R(5), R(16)];
    0b00001101_10011111_10100100_00000000 = [RegListLanes(3, QWORD), RefBase, LitInt(24)] => [R(0), Ufields(&[30]), R(5)];
    0b00001101_10000000_10100100_00000000 = [RegListLanes(3, QWORD), RefBase, X] => [R(0), Ufields(&[30]), R(5), R(16)];
]
"st4" = [
    // ST4 (multiple structures)
    0b00001100_00000000_00000000_00000000 = [RegList(4, BYTE), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_00000000_00000100_00000000 = [RegList(4, WORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_00000000_00001000_00000000 = [RegList(4, DWORD), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_00000000_00001100_00000000 = [RegListSized(4, QWORD, 2), RefBase] => [R(0), R(5), Rwidth(30)];
    0b00001100_10011111_00000000_00000000 = [RegListSized(4, BYTE, 8), RefBase, LitInt(32)] => [R(0), R(5)];
    0b00001100_10011111_00000100_00000000 = [RegListSized(4, WORD, 4), RefBase, LitInt(32)] => [R(0), R(5)];
    0b00001100_10011111_00001000_00000000 = [RegListSized(4, DWORD, 2), RefBase, LitInt(32)] => [R(0), R(5)];
    0b01001100_10011111_00000000_00000000 = [RegListSized(4, BYTE, 16), RefBase, LitInt(64)] => [R(0), R(5)];
    0b01001100_10011111_00000100_00000000 = [RegListSized(4, WORD, 8), RefBase, LitInt(64)] => [R(0), R(5)];
    0b01001100_10011111_00001000_00000000 = [RegListSized(4, DWORD, 4), RefBase, LitInt(64)] => [R(0), R(5)];
    0b01001100_10011111_00001100_00000000 = [RegListSized(4, QWORD, 2), RefBase, LitInt(64)] => [R(0), R(5)];
    0b00001100_10000000_00000000_00000000 = [RegList(4, BYTE), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_10000000_00000100_00000000 = [RegList(4, WORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_10000000_00001000_00000000 = [RegList(4, DWORD), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001100_10000000_00001100_00000000 = [RegListSized(4, QWORD, 2), RefBase, X] => [R(0), R(5), R(16), Rwidth(30)];
    // ST4 (single structure)
    0b00001101_00100000_00100000_00000000 = [RegListLanes(4, BYTE), RefBase] => [R(0), Ufields(&[30, 12, 11, 10]), R(5)];
    0b00001101_00100000_01100000_00000000 = [RegListLanes(4, WORD), RefBase] => [R(0), Ufields(&[30, 12, 11]), R(5)];
    0b00001101_00100000_10100000_00000000 = [RegListLanes(4, DWORD), RefBase] => [R(0), Ufields(&[30, 12]), R(5)];
    0b00001101_00100000_10100100_00000000 = [RegListLanes(4, QWORD), RefBase] => [R(0), Ufields(&[30]), R(5)];
    0b00001101_10111111_00100000_00000000 = [RegListLanes(4, BYTE), RefBase, LitInt(4)] => [R(0), Ufields(&[30, 12, 11, 10]), R(5)];
    0b00001101_10100000_00100000_00000000 = [RegListLanes(4, BYTE), RefBase, X] => [R(0), Ufields(&[30, 12, 11, 10]), R(5), R(16)];
    0b00001101_10111111_01100000_00000000 = [RegListLanes(4, WORD), RefBase, LitInt(8)] => [R(0), Ufields(&[30, 12, 11]), R(5)];
    0b00001101_10100000_01100000_00000000 = [RegListLanes(4, WORD), RefBase, X] => [R(0), Ufields(&[30, 12, 11]), R(5), R(16)];
    0b00001101_10111111_10100000_00000000 = [RegListLanes(4, DWORD), RefBase, LitInt(16)] => [R(0), Ufields(&[30, 12]), R(5)];
    0b00001101_10100000_10100000_00000000 = [RegListLanes(4, DWORD), RefBase, X] => [R(0), Ufields(&[30, 12]), R(5), R(16)];
    0b00001101_10111111_10100100_00000000 = [RegListLanes(4, QWORD), RefBase, LitInt(32)] => [R(0), Ufields(&[30]), R(5)];
    0b00001101_10100000_10100100_00000000 = [RegListLanes(4, QWORD), RefBase, X] => [R(0), Ufields(&[30]), R(5), R(16)];
]
"stadd" = [
    0b10111000_00100000_00000000_00011111 = [W, RefBase] => [R(16), R(5)];
    0b11111000_00100000_00000000_00011111 = [X, RefBase] => [R(16), R(5)];
]
"staddb" = [
    0b00111000_00100000_00000000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"staddh" = [
    0b01111000_00100000_00000000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"staddl" = [
    0b10111000_01100000_00000000_00011111 = [W, RefBase] => [R(16), R(5)];
    0b11111000_01100000_00000000_00011111 = [X, RefBase] => [R(16), R(5)];
]
"staddlb" = [
    0b00111000_01100000_00000000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"staddlh" = [
    0b01111000_01100000_00000000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"stclr" = [
    0b10111000_00100000_00010000_00011111 = [W, RefBase] => [R(16), R(5)];
    0b11111000_00100000_00010000_00011111 = [X, RefBase] => [R(16), R(5)];
]
"stclrb" = [
    0b00111000_00100000_00010000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"stclrh" = [
    0b01111000_00100000_00010000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"stclrl" = [
    0b10111000_01100000_00010000_00011111 = [W, RefBase] => [R(16), R(5)];
    0b11111000_01100000_00010000_00011111 = [X, RefBase] => [R(16), R(5)];
]
"stclrlb" = [
    0b00111000_01100000_00010000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"stclrlh" = [
    0b01111000_01100000_00010000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"steor" = [
    0b10111000_00100000_00100000_00011111 = [W, RefBase] => [R(16), R(5)];
    0b11111000_00100000_00100000_00011111 = [X, RefBase] => [R(16), R(5)];
]
"steorb" = [
    0b00111000_00100000_00100000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"steorh" = [
    0b01111000_00100000_00100000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"steorl" = [
    0b10111000_01100000_00100000_00011111 = [W, RefBase] => [R(16), R(5)];
    0b11111000_01100000_00100000_00011111 = [X, RefBase] => [R(16), R(5)];
]
"steorlb" = [
    0b00111000_01100000_00100000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"steorlh" = [
    0b01111000_01100000_00100000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"stllr" = [
    0b10001000_10011111_01111100_00000000 = [W, RefBase] => [R(0), R(5)];
    0b11001000_10011111_01111100_00000000 = [X, RefBase] => [R(0), R(5)];
]
"stllrb" = [
    0b00001000_10011111_01111100_00000000 = [W, RefBase] => [R(0), R(5)];
]
"stllrh" = [
    0b01001000_10011111_01111100_00000000 = [W, RefBase] => [R(0), R(5)];
]
"stlr" = [
    0b10001000_10011111_11111100_00000000 = [W, RefBase] => [R(0), R(5)];
    0b11001000_10011111_11111100_00000000 = [X, RefBase] => [R(0), R(5)];
]
"stlrb" = [
    0b00001000_10011111_11111100_00000000 = [W, RefBase] => [R(0), R(5)];
]
"stlrh" = [
    0b01001000_10011111_11111100_00000000 = [W, RefBase] => [R(0), R(5)];
]
"stlur" = [
    0b10011001_00000000_00000000_00000000 = [W, RefOffset] => [R(0), R(5), Sbits(12, 9)];
    0b11011001_00000000_00000000_00000000 = [X, RefOffset] => [R(0), R(5), Sbits(12, 9)];
]
"stlurb" = [
    0b00011001_00000000_00000000_00000000 = [W, RefOffset] => [R(0), R(5), Sbits(12, 9)];
]
"stlurh" = [
    0b01011001_00000000_00000000_00000000 = [W, RefOffset] => [R(0), R(5), Sbits(12, 9)];
]
"stlxp" = [
    0b10001000_00100000_10000000_00000000 = [W, W, W, RefBase] => [R(16), R(0), R(10), R(5)];
    0b11001000_00100000_10000000_00000000 = [W, X, X, RefBase] => [R(16), R(0), R(10), R(5)];
]
"stlxr" = [
    0b10001000_00000000_11111100_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11001000_00000000_11111100_00000000 = [W, X, RefBase] => [R(16), R(0), R(5)];
]
"stlxrb" = [
    0b00001000_00000000_11111100_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"stlxrh" = [
    0b01001000_00000000_11111100_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"stnp" = [
    // STNP (SIMD&FP)
    0b00101100_00000000_00000000_00000000 = [S, S, RefOffset] => [R(0), R(10), R(5), Sscaled(15, 7, 2)];
    0b01101100_00000000_00000000_00000000 = [D, D, RefOffset] => [R(0), R(10), R(5), Sscaled(15, 7, 3)];
    0b10101100_00000000_00000000_00000000 = [Q, Q, RefOffset] => [R(0), R(10), R(5), Sscaled(15, 7, 4)];
    // STNP
    0b00101000_00000000_00000000_00000000 = [W, W, RefOffset] => [R(0), R(10), R(5), Sscaled(15, 7, 2)];
    0b10101000_00000000_00000000_00000000 = [X, X, RefOffset] => [R(0), R(10), R(5), Sscaled(15, 7, 3)];
]
"stp" = [
    // STP (SIMD&FP)
    0b00101100_10000000_00000000_00000000 = [S, S, RefBase, Imm] => [R(0), R(10), R(5), Sscaled(15, 7, 2)];
    0b01101100_10000000_00000000_00000000 = [D, D, RefBase, Imm] => [R(0), R(10), R(5), Sscaled(15, 7, 3)];
    0b10101100_10000000_00000000_00000000 = [Q, Q, RefBase, Imm] => [R(0), R(10), R(5), Sscaled(15, 7, 4)];
    0b00101101_10000000_00000000_00000000 = [S, S, RefPre] => [R(0), R(10), R(5), Sscaled(15, 7, 2)];
    0b01101101_10000000_00000000_00000000 = [D, D, RefPre] => [R(0), R(10), R(5), Sscaled(15, 7, 3)];
    0b10101101_10000000_00000000_00000000 = [Q, Q, RefPre] => [R(0), R(10), R(5), Sscaled(15, 7, 4)];
    0b00101101_00000000_00000000_00000000 = [S, S, RefOffset] => [R(0), R(10), R(5), Sscaled(15, 7, 2)];
    0b01101101_00000000_00000000_00000000 = [D, D, RefOffset] => [R(0), R(10), R(5), Sscaled(15, 7, 3)];
    0b10101101_00000000_00000000_00000000 = [Q, Q, RefOffset] => [R(0), R(10), R(5), Sscaled(15, 7, 4)];
    // STP
    0b00101000_10000000_00000000_00000000 = [W, W, RefBase, Imm] => [R(0), R(10), R(5), Sscaled(15, 7, 2)];
    0b10101000_10000000_00000000_00000000 = [X, X, RefBase, Imm] => [R(0), R(10), R(5), Sscaled(15, 7, 3)];
    0b00101001_10000000_00000000_00000000 = [W, W, RefPre] => [R(0), R(10), R(5), Sscaled(15, 7, 2)];
    0b10101001_10000000_00000000_00000000 = [X, X, RefPre] => [R(0), R(10), R(5), Sscaled(15, 7, 3)];
    0b00101001_00000000_00000000_00000000 = [W, W, RefOffset] => [R(0), R(10), R(5), Sscaled(15, 7, 2)];
    0b10101001_00000000_00000000_00000000 = [X, X, RefOffset] => [R(0), R(10), R(5), Sscaled(15, 7, 3)];
]
"str" = [
    // STR (immediate, SIMD&FP)
    0b00111100_00000000_00000100_00000000 = [B, RefBase, Imm] => [R(0), R(5), Sbits(12, 9)];
    0b01111100_00000000_00000100_00000000 = [H, RefBase, Imm] => [R(0), R(5), Sbits(12, 9)];
    0b10111100_00000000_00000100_00000000 = [S, RefBase, Imm] => [R(0), R(5), Sbits(12, 9)];
    0b11111100_00000000_00000100_00000000 = [B, RefBase, Imm] => [R(0), R(5), Sbits(12, 9)];
    0b00111100_10000000_00000100_00000000 = [Q, RefBase, Imm] => [R(0), R(5), Sbits(12, 9)];
    0b00111100_00000000_00001100_00000000 = [B, RefPre] => [R(0), R(5), Sbits(12, 9)];
    0b01111100_00000000_00001100_00000000 = [H, RefPre] => [R(0), R(5), Sbits(12, 9)];
    0b10111100_00000000_00001100_00000000 = [S, RefPre] => [R(0), R(5), Sbits(12, 9)];
    0b11111100_00000000_00001100_00000000 = [D, RefPre] => [R(0), R(5), Sbits(12, 9)];
    0b00111100_10000000_00001100_00000000 = [Q, RefPre] => [R(0), R(5), Sbits(12, 9)];
    0b00111101_00000000_00000000_00000000 = [B, RefOffset] => [R(0), R(5), Ubits(10, 12)];
    0b01111101_00000000_00000000_00000000 = [H, RefOffset] => [R(0), R(5), Uscaled(10, 12, 1)];
    0b10111101_00000000_00000000_00000000 = [Q, RefOffset] => [R(0), R(5), Uscaled(10, 12, 2)];
    0b11111101_00000000_00000000_00000000 = [B, RefOffset] => [R(0), R(5), Uscaled(10, 12, 3)];
    0b00111101_10000000_00000000_00000000 = [Q, RefOffset] => [R(0), R(5), Uscaled(10, 12, 4)];
    // STR (immediate)
    0b10111000_00000000_00000100_00000000 = [W, RefBase, Imm] => [R(0), R(5), Sbits(12, 9)];
    0b11111000_00000000_00000100_00000000 = [X, RefBase, Imm] => [R(0), R(5), Sbits(12, 9)];
    0b10111000_00000000_00001100_00000000 = [W, RefPre] => [R(0), R(5), Sbits(12, 9)];
    0b11111000_00000000_00001100_00000000 = [X, RefPre] => [R(0), R(5), Sbits(12, 9)];
    0b10111001_00000000_00000000_00000000 = [W, RefOffset] => [R(0), R(5), Uscaled(10, 12, 2)];
    0b11111001_00000000_00000000_00000000 = [X, RefOffset] => [R(0), R(5), Uscaled(10, 12, 3)];
    // STR (register, SIMD&FP)
    0b00111100_00100000_00001000_00000000 = [B, RefIndex] => [R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 0])];
    0b01111100_00100000_00001000_00000000 = [H, RefIndex] => [R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 1])];
    0b10111100_00100000_00001000_00000000 = [S, RefIndex] => [R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 2])];
    0b11111100_00100000_00001000_00000000 = [D, RefIndex] => [R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 3])];
    0b00111100_10100000_00001000_00000000 = [Q, RefIndex] => [R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 4])];
    // STR (register)
    0b10111000_00100000_00001000_00000000 = [W, RefIndex] => [R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 2])];
    0b11111000_00100000_00001000_00000000 = [X, RefIndex] => [R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 3])];
]
"strb" = [
    // STRB (immediate)
    0b00111000_00000000_00000100_00000000 = [W, RefBase, Imm] => [R(0), R(5), Sbits(12, 9)];
    0b00111000_00000000_00001100_00000000 = [W, RefPre] => [R(0), R(5), Sbits(12, 9)];
    0b00111001_00000000_00000000_00000000 = [W, RefOffset] => [R(0), R(5), Ubits(10, 12)];
    // STRB (register)
    0b00111000_00100000_00001000_00000000 = [W, RefIndex] => [R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 0])];
]
"strh" = [
    // STRH (immediate)
    0b01111000_00000000_00000100_00000000 = [W, RefBase, Imm] => [R(0), R(5), Sbits(12, 9)];
    0b01111000_00000000_00001100_00000000 = [W, RefPre] => [R(0), R(5), Sbits(12, 9)];
    0b01111001_00000000_00000000_00000000 = [W, RefOffset] => [R(0), R(5), Uscaled(10, 12, 1)];
    // STRH (register)
    0b01111000_00100000_00001000_00000000 = [W, RefIndex] => [R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 1])];
]
"stset" = [
    0b10111000_00100000_00110000_00011111 = [W, RefBase] => [R(16), R(5)];
    0b11111000_00100000_00110000_00011111 = [X, RefBase] => [R(16), R(5)];
]
"stsetb" = [
    0b00111000_00100000_00110000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"stseth" = [
    0b01111000_00100000_00110000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"stsetl" = [
    0b10111000_01100000_00110000_00011111 = [W, RefBase] => [R(16), R(5)];
    0b11111000_01100000_00110000_00011111 = [X, RefBase] => [R(16), R(5)];
]
"stsetlb" = [
    0b00111000_01100000_00110000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"stsetlh" = [
    0b01111000_01100000_00110000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"stsmax" = [
    0b10111000_00100000_01000000_00011111 = [W, RefBase] => [R(16), R(5)];
    0b11111000_00100000_01000000_00011111 = [X, RefBase] => [R(16), R(5)];
]
"stsmaxb" = [
    0b00111000_00100000_01000000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"stsmaxh" = [
    0b01111000_00100000_01000000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"stsmaxl" = [
    0b10111000_01100000_01000000_00011111 = [W, RefBase] => [R(16), R(5)];
    0b11111000_01100000_01000000_00011111 = [X, RefBase] => [R(16), R(5)];
]
"stsmaxlb" = [
    0b00111000_01100000_01000000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"stsmaxlh" = [
    0b01111000_01100000_01000000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"stsmin" = [
    0b10111000_00100000_01010000_00011111 = [W, RefBase] => [R(16), R(5)];
    0b11111000_00100000_01010000_00011111 = [X, RefBase] => [R(16), R(5)];
]
"stsminb" = [
    0b00111000_00100000_01010000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"stsminh" = [
    0b01111000_00100000_01010000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"stsminl" = [
    0b10111000_01100000_01010000_00011111 = [W, RefBase] => [R(16), R(5)];
    0b11111000_01100000_01010000_00011111 = [X, RefBase] => [R(16), R(5)];
]
"stsminlb" = [
    0b00111000_01100000_01010000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"stsminlh" = [
    0b01111000_01100000_01010000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"sttr" = [
    0b10111000_00000000_00001000_00000000 = [W, RefOffset] => [R(0), R(5), Sbits(12, 9)];
    0b11111000_00000000_00001000_00000000 = [X, RefOffset] => [R(0), R(5), Sbits(12, 9)];
]
"sttrb" = [
    0b00111000_00000000_00001000_00000000 = [W, RefOffset] => [R(0), R(5), Sbits(12, 9)];
]
"sttrh" = [
    0b01111000_00000000_00001000_00000000 = [W, RefOffset] => [R(0), R(5), Sbits(12, 9)];
]
"stumax" = [
    0b10111000_00100000_01100000_00011111 = [W, RefBase] => [R(16), R(5)];
    0b11111000_00100000_01100000_00011111 = [X, RefBase] => [R(16), R(5)];
]
"stumaxb" = [
    0b00111000_00100000_01100000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"stumaxh" = [
    0b01111000_00100000_01100000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"stumaxl" = [
    0b10111000_01100000_01100000_00011111 = [W, RefBase] => [R(16), R(5)];
    0b11111000_01100000_01100000_00011111 = [X, RefBase] => [R(16), R(5)];
]
"stumaxlb" = [
    0b00111000_01100000_01100000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"stumaxlh" = [
    0b01111000_01100000_01100000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"stumin" = [
    0b10111000_00100000_01110000_00011111 = [W, RefBase] => [R(16), R(5)];
    0b11111000_00100000_01110000_00011111 = [X, RefBase] => [R(16), R(5)];
]
"stuminb" = [
    0b00111000_00100000_01110000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"stuminh" = [
    0b01111000_00100000_01110000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"stuminl" = [
    0b10111000_01100000_01110000_00011111 = [W, RefBase] => [R(16), R(5)];
    0b11111000_01100000_01110000_00011111 = [X, RefBase] => [R(16), R(5)];
]
"stuminlb" = [
    0b00111000_01100000_01110000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"stuminlh" = [
    0b01111000_01100000_01110000_00011111 = [W, RefBase] => [R(16), R(5)];
]
"stur" = [
    // STUR (SIMD&FP)
    0b00111100_00000000_00000000_00000000 = [B, RefOffset] => [R(0), R(5), Sbits(12, 9)];
    0b01111100_00000000_00000000_00000000 = [H, RefOffset] => [R(0), R(5), Sbits(12, 9)];
    0b10111100_00000000_00000000_00000000 = [S, RefOffset] => [R(0), R(5), Sbits(12, 9)];
    0b11111100_00000000_00000000_00000000 = [D, RefOffset] => [R(0), R(5), Sbits(12, 9)];
    0b00111100_10000000_00000000_00000000 = [Q, RefOffset] => [R(0), R(5), Sbits(12, 9)];
    // STUR
    0b10111000_00000000_00000000_00000000 = [W, RefOffset] => [R(0), R(5), Sbits(12, 9)];
    0b11111000_00000000_00000000_00000000 = [X, RefOffset] => [R(0), R(5), Sbits(12, 9)];
]
"sturb" = [
    0b00111000_00000000_00000000_00000000 = [W, RefOffset] => [R(0), R(5), Sbits(12, 9)];
]
"sturh" = [
    0b01111000_00000000_00000000_00000000 = [W, RefOffset] => [R(0), R(5), Sbits(12, 9)];
]
"stxp" = [
    0b10001000_00100000_00000000_00000000 = [W, W, W, RefBase] => [R(16), R(0), R(10), R(5)];
    0b11001000_00100000_00000000_00000000 = [W, X, X, RefBase] => [R(16), R(0), R(10), R(5)];
]
"stxr" = [
    0b10001000_00000000_01111100_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11001000_00000000_01111100_00000000 = [W, X, RefBase] => [R(16), R(0), R(5)];
]
"stxrb" = [
    0b00001000_00000000_01111100_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"stxrh" = [
    0b01001000_00000000_01111100_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"sub" = [
    // SUB (shifted register)
    0b01001011_00000000_00000000_00000000 = [W, W, W, End, Mod(SHIFTS)] => [R(0), R(5), R(16), Rotates(22), Ubits(10, 5)];
    0b11001011_00000000_00000000_00000000 = [X, X, X, End, Mod(SHIFTS)] => [R(0), R(5), R(16), Rotates(22), Ubits(10, 6)];
    // SUB (extended register)
    0b01001011_00100000_00000000_00000000 = [WSP, WSP, W, End, Mod(EXTENDS)] => [R(0), R(5), R(16), ExtendsX(13), Urange(10, 0, 4)];
    0b11001011_00100000_00000000_00000000 = [XSP, XSP, W, End, Mod(EXTENDS_W)] => [R(0), R(5), R(16), ExtendsX(13), Urange(10, 0, 4)];
    0b11001011_00100000_00000000_00000000 = [XSP, XSP, X, End, Mod(EXTENDS_X)] => [R(0), R(5), R(16), ExtendsX(13), Urange(10, 0, 4)];
    // SUB (immediate)
    0b01010001_00000000_00000000_00000000 = [WSP, WSP, Imm, End, Mod(&[LSL])] => [R(0), R(5), Ubits(10, 12), A, Ulist(22, &[0, 12])];
    0b11010001_00000000_00000000_00000000 = [XSP, XSP, Imm, End, Mod(&[LSL])] => [R(0), R(5), Ubits(10, 12), A, Ulist(22, &[0, 12])];
    // SUB (vector)
    0b01111110_11100000_10000100_00000000 = [D, D, D] => [R(0), R(5), R(16)];
    0b00101110_00100000_10000100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_01100000_10000100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_10100000_10000100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_11100000_10000100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"subhn" = [
    0b00001110_00100000_01100000_00000000 = [VSizedStatic(BYTE, 8), VSizedStatic(WORD, 8), VSizedStatic(WORD, 8)] => [R(0), R(5), R(16)];
    0b00001110_01100000_01100000_00000000 = [VSizedStatic(WORD, 4), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
    0b00001110_10100000_01100000_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16)];
]
"subhn2" = [
    0b01001110_00100000_01100000_00000000 = [VSizedStatic(BYTE, 16), VSizedStatic(WORD, 8), VSizedStatic(WORD, 8)] => [R(0), R(5), R(16)];
    0b01001110_01100000_01100000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
    0b01001110_10100000_01100000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16)];
]
"subs" = [
    // SUBS (shifted register)
    0b01101011_00000000_00000000_00000000 = [W, W, W, End, Mod(SHIFTS)] => [R(0), R(5), R(16), Rotates(22), Ubits(10, 5)];
    0b11101011_00000000_00000000_00000000 = [X, X, X, End, Mod(SHIFTS)] => [R(0), R(5), R(16), Rotates(22), Ubits(10, 6)];
    // SUBS (extended register)
    0b01101011_00100000_00000000_00000000 = [W, WSP, W, End, Mod(EXTENDS)] => [R(0), R(5), R(16), ExtendsW(13), Urange(10, 0, 4)];
    0b11101011_00100000_00000000_00000000 = [X, XSP, W, End, Mod(EXTENDS_W)] => [R(0), R(5), R(16), ExtendsX(13), Urange(10, 0, 4)];
    0b11101011_00100000_00000000_00000000 = [X, XSP, X, End, Mod(EXTENDS_X)] => [R(0), R(5), R(16), ExtendsX(13), Urange(10, 0, 4)];
    // SUBS (immediate)
    0b01110001_00000000_00000000_00000000 = [W, WSP, Imm, End, Mod(&[LSL])] => [R(0), R(5), Ubits(10, 12), A, Ulist(22, &[0, 12])];
    0b11110001_00000000_00000000_00000000 = [X, XSP, Imm, End, Mod(&[LSL])] => [R(0), R(5), Ubits(10, 12), A, Ulist(22, &[0, 12])];
]
"suqadd" = [
    0b01011110_00100000_00111000_00000000 = [B, B] => [R(0), R(5)];
    0b01011110_01100000_00111000_00000000 = [H, H] => [R(0), R(5)];
    0b01011110_10100000_00111000_00000000 = [S, S] => [R(0), R(5)];
    0b01011110_11100000_00111000_00000000 = [D, D] => [R(0), R(5)];
    0b00001110_00100000_00111000_00000000 = [VSized(BYTE), VSized(BYTE)] => [R(0), R(5), Rwidth(30)];
    0b00001110_01100000_00111000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_10100000_00111000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_11100000_00111000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
]
"svc" = [
    0b11010100_00000000_00000000_00000001 = [Imm] => [Ubits(5, 16)];
]
"swp" = [
    0b10111000_00100000_10000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_00100000_10000000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"swpa" = [
    0b10111000_10100000_10000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_10100000_10000000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"swpab" = [
    0b00111000_10100000_10000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"swpah" = [
    0b01111000_10100000_10000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"swpal" = [
    0b10111000_11100000_10000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_11100000_10000000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"swpalb" = [
    0b00111000_11100000_10000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"swpalh" = [
    0b01111000_11100000_10000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"swpb" = [
    0b00111000_00100000_10000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"swph" = [
    0b01111000_00100000_10000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"swpl" = [
    0b10111000_01100000_10000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
    0b11111000_01100000_10000000_00000000 = [X, X, RefBase] => [R(16), R(0), R(5)];
]
"swplb" = [
    0b00111000_01100000_10000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"swplh" = [
    0b01111000_01100000_10000000_00000000 = [W, W, RefBase] => [R(16), R(0), R(5)];
]
"sxtb" = [
    0b00010011_00000000_00011100_00000000 = [W, W] => [R(0), R(5)];
    0b10010011_01000000_00011100_00000000 = [X, W] => [R(0), R(5)];
]
"sxth" = [
    0b00010011_00000000_00111100_00000000 = [W, W] => [R(0), R(5)];
    0b10010011_01000000_00111100_00000000 = [X, W] => [R(0), R(5)];
]
"sxtl" = [
    0b00001111_00001000_10100100_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 8)] => [R(0), R(5)];
    0b00001111_00010000_10100100_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4)] => [R(0), R(5)];
    0b00001111_00100000_10100100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2)] => [R(0), R(5)];
]
"sxtl2" = [
    0b01001111_00001000_10100100_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 16)] => [R(0), R(5)];
    0b01001111_00010000_10100100_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8)] => [R(0), R(5)];
    0b01001111_00100000_10100100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4)] => [R(0), R(5)];
]
"sxtw" = [
    0b10010011_01000000_01111100_00000000 = [X, W] => [R(0), R(5)];
]
"sys" = [
    0b11010101_00001000_00000000_00000000 = [Imm, Ident, Ident, Imm, End, X] => [Ubits(16, 3), LitList(12, "CONTROL_REGS"), LitList(8, "CONTROL_REGS"), Ubits(5, 3), R(0)];
]
"sysl" = [
    0b11010101_00101000_00000000_00000000 = [X, Imm, Ident, Ident, Imm, End] => [R(0), Ubits(16, 3), LitList(12, "CONTROL_REGS"), LitList(8, "CONTROL_REGS"), Ubits(5, 3)];
]
"tbl" = [
    0b00001110_00000000_00100000_00000000 = [VSized(BYTE), RegListSized(2, BYTE, 16), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_00000000_01000000_00000000 = [VSized(BYTE), RegListSized(3, BYTE, 16), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_00000000_01100000_00000000 = [VSized(BYTE), RegListSized(4, BYTE, 16), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_00000000_00000000_00000000 = [VSized(BYTE), RegListSized(1, BYTE, 16), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
]
"tbnz" = [
    0b00110111_00000000_00000000_00000000 = [W, Imm, Offset] => [R(0), Ubits(19, 5), Sscaled(5, 14, 2)];
    0b00110111_00000000_00000000_00000000 = [X, Imm, Offset] => [R(0), BUbits(6), BUslice(19, 5, 0), BUslice(31, 1, 5), A, Sscaled(5, 14, 2)];
]
"tbx" = [
    0b00001110_00000000_00110000_00000000 = [VSized(BYTE), RegListSized(2, BYTE, 16), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_00000000_01010000_00000000 = [VSized(BYTE), RegListSized(3, BYTE, 16), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_00000000_01110000_00000000 = [VSized(BYTE), RegListSized(4, BYTE, 16), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_00000000_00010000_00000000 = [VSized(BYTE), RegListSized(1, BYTE, 16), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
]
"tbz" = [
    0b00110110_00000000_00000000_00000000 = [W, Imm, Offset] => [R(0), Ubits(19, 5), Sscaled(5, 14, 2)];
    0b00110110_00000000_00000000_00000000 = [X, Imm, Offset] => [R(0), BUbits(6), BUslice(19, 5, 0), BUslice(31, 1, 5), A, Sscaled(5, 14, 2)];
]
"tlbi" = [
    0b11010101_00001000_10000000_00000000 = [Ident, End, X] => [LitList(5, "TLBI_OPS"), R(0)];
]
"trn1" = [
    0b00001110_00000000_00101000_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01000000_00101000_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10000000_00101000_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_11000000_00101000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"trn2" = [
    0b00001110_00000000_01101000_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01000000_01101000_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10000000_01101000_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_11000000_01101000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"tsb" = [
    0b11010101_00000011_00100010_01011111 = [Lit("CSYNC")] => [];
]
"tst" = [
    // TST (immediate)
    0b01110010_00000000_00000000_00011111 = [W, Imm] => [R(5), UlogicalW(10)];
    0b11110010_00000000_00000000_00011111 = [X, Imm] => [R(5), UlogicalX(10)];
    // TST (shifted register)
    0b01101010_00000000_00000000_00011111 = [W, W, End, Mod(ROTATES)] => [R(5), R(16), Rotates(22), Ubits(10, 5)];
    0b11101010_00000000_00000000_00011111 = [X, X, End, Mod(ROTATES)] => [R(5), R(16), Rotates(22), Ubits(10, 6)];
]
"uaba" = [
    0b00101110_00100000_01111100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_01100000_01111100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_10100000_01111100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
]
"uabal" = [
    0b00101110_00100000_01010000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 8), VSizedStatic(BYTE, 8)] => [R(0), R(5), R(16)];
    0b00101110_01100000_01010000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VSizedStatic(WORD, 4)] => [R(0), R(5), R(16)];
    0b00101110_10100000_01010000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2), VSizedStatic(DWORD, 2)] => [R(0), R(5), R(16)];
]
"uabal2" = [
    0b01101110_00100000_01010000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16)] => [R(0), R(5), R(16)];
    0b01101110_01100000_01010000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8), VSizedStatic(WORD, 8)] => [R(0), R(5), R(16)];
    0b01101110_10100000_01010000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"uabd" = [
    0b00101110_00100000_01110100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_01100000_01110100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_10100000_01110100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
]
"uabdl" = [
    0b00101110_00100000_01110000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 8), VSizedStatic(BYTE, 8)] => [R(0), R(5), R(16)];
    0b00101110_01100000_01110000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VSizedStatic(WORD, 4)] => [R(0), R(5), R(16)];
    0b00101110_10100000_01110000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2), VSizedStatic(DWORD, 2)] => [R(0), R(5), R(16)];
]
"uabdl2" = [
    0b01101110_00100000_01110000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16)] => [R(0), R(5), R(16)];
    0b01101110_01100000_01110000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8), VSizedStatic(WORD, 8)] => [R(0), R(5), R(16)];
    0b01101110_10100000_01110000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"uadalp" = [
    0b00101110_00100000_01101000_00000000 = [VSized(WORD), VSized(BYTE)] => [R(0), R(5), Rwidth(30)];
    0b00101110_01100000_01101000_00000000 = [VSized(DWORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_10100000_01101000_00000000 = [VSized(QWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
]
"uaddl" = [
    0b00101110_00100000_00000000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 8), VSizedStatic(BYTE, 8)] => [R(0), R(5), R(16)];
    0b00101110_01100000_00000000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VSizedStatic(WORD, 4)] => [R(0), R(5), R(16)];
    0b00101110_10100000_00000000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2), VSizedStatic(DWORD, 2)] => [R(0), R(5), R(16)];
]
"uaddl2" = [
    0b01101110_00100000_00000000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16)] => [R(0), R(5), R(16)];
    0b01101110_01100000_00000000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8), VSizedStatic(WORD, 8)] => [R(0), R(5), R(16)];
    0b01101110_10100000_00000000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"uaddlp" = [
    0b00101110_00100000_00101000_00000000 = [VSized(WORD), VSized(BYTE)] => [R(0), R(5), Rwidth(30)];
    0b00101110_01100000_00101000_00000000 = [VSized(DWORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_10100000_00101000_00000000 = [VSized(QWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
]
"uaddlv" = [
    0b00101110_00110000_00111000_00000000 = [B, VSized(BYTE)] => [R(0), R(5), Rwidth(30)];
    0b00101110_01110000_00111000_00000000 = [H, VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_10110000_00111000_00000000 = [S, VSizedStatic(DWORD, 4)] => [R(0), R(5), Rwidth(30)];
]
"uaddw" = [
    0b00101110_00100000_00010000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(WORD, 8), VSizedStatic(BYTE, 8)] => [R(0), R(5), R(16)];
    0b00101110_01100000_00010000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4)] => [R(0), R(5), R(16)];
    0b00101110_10100000_00010000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2)] => [R(0), R(5), R(16)];
]
"uaddw2" = [
    0b01101110_00100000_00010000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(WORD, 8), VSizedStatic(BYTE, 16)] => [R(0), R(5), R(16)];
    0b01101110_01100000_00010000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8)] => [R(0), R(5), R(16)];
    0b01101110_10100000_00010000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"ubfiz" = [
    0b01010011_00000000_00000000_00000000 = [W, W, Imm, Imm] => [R(0), R(5), Usub(16, 5, 32), Urange(10, 1, 32)];
    0b11010011_01000000_00000000_00000000 = [X, X, Imm, Imm] => [R(0), R(5), Usub(16, 6, 64), Urange(10, 1, 64)];
]
"ubfm" = [
    0b01010011_00000000_00000000_00000000 = [W, W, Imm, Imm] => [R(0), R(5), Ubits(16, 5), Ubits(10, 5)];
    0b11010011_01000000_00000000_00000000 = [X, X, Imm, Imm] => [R(0), R(5), Ubits(16, 6), Ubits(10, 6)];
]
"ubfx" = [
    0b01010011_00000000_00000000_00000000 = [W, W, Imm, Imm] => [R(0), R(5), Ubits(16, 5), Usumdec(10, 5)];
    0b11010011_01000000_00000000_00000000 = [X, X, Imm, Imm] => [R(0), R(5), Ubits(16, 6), Usumdec(10, 6)];
]
"ucvtf" = [
    // UCVTF (vector, fixed-point)
    0b01111111_00000000_11100100_00000000 = [H, H, Imm] => [R(0), R(5), BUrange(1, 16), Usub(16, 5, 32)];
    0b01111111_00000000_11100100_00000000 = [S, S, Imm] => [R(0), R(5), BUrange(1, 32), Usub(16, 6, 64)];
    0b01111111_00000000_11100100_00000000 = [D, D, Imm] => [R(0), R(5), BUrange(1, 64), Usub(16, 7, 128)];
    0b00101111_00010000_11100100_00000000 = [VSized(WORD), VSized(WORD), Imm] => [R(0), R(5), R(16), Usub(16, 4, 16), Rwidth(30)];
    0b00101111_00100000_11100100_00000000 = [VSized(DWORD), VSized(DWORD), Imm] => [R(0), R(5), R(16), Usub(16, 5, 32), Rwidth(30)];
    0b00101111_01000000_11100100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), R(16), Usub(16, 6, 64), Rwidth(30)];
    // UCVTF (vector, integer)
    0b01111110_01111001_11011000_00000000 = [H, H] => [R(0), R(5)];
    0b01111110_00100001_11011000_00000000 = [S, S] => [R(0), R(5)];
    0b01111110_01100001_11011000_00000000 = [D, D] => [R(0), R(5)];
    0b00101110_01111001_11011000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_00100001_11011000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_01100001_11011000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
    // UCVTF (scalar, fixed-point)
    0b00011110_11000011_00000000_00000000 = [H, W, Imm] => [R(0), R(5), BUrange(1, 32), Usub(10, 6, 64)];
    0b00011110_00000011_00000000_00000000 = [S, W, Imm] => [R(0), R(5), Usub(10, 6, 64)];
    0b00011110_01000011_00000000_00000000 = [D, W, Imm] => [R(0), R(5), BUrange(1, 32), Usub(10, 6, 64)];
    0b10011110_11000011_00000000_00000000 = [H, X, Imm] => [R(0), R(5), Usub(10, 6, 64)];
    0b10011110_00000011_00000000_00000000 = [S, X, Imm] => [R(0), R(5), BUrange(1, 32), Usub(10, 6, 64)];
    0b10011110_01000011_00000000_00000000 = [D, X, Imm] => [R(0), R(5), Usub(10, 6, 64)];
    // UCVTF (scalar, integer)
    0b00011110_11100011_00000000_00000000 = [H, W] => [R(0), R(5)];
    0b00011110_00100011_00000000_00000000 = [S, W] => [R(0), R(5)];
    0b00011110_01100011_00000000_00000000 = [D, W] => [R(0), R(5)];
    0b10011110_11100011_00000000_00000000 = [H, X] => [R(0), R(5)];
    0b10011110_00100011_00000000_00000000 = [S, X] => [R(0), R(5)];
    0b10011110_01100011_00000000_00000000 = [D, X] => [R(0), R(5)];
]
"udf" = [
    0b00000000_00000000_00000000_00000000 = [Imm] => [Ubits(0, 16)];
]
"udiv" = [
    0b00011010_11000000_00001000_00000000 = [W, W, W] => [R(0), R(5), R(16)];
    0b10011010_11000000_00001000_00000000 = [X, X, X] => [R(0), R(5), R(16)];
]
"udot" = [
    // UDOT (by element)
    0b00101111_10000000_11100000_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(BYTE, 8), VLanes(BYTE)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    0b01101111_10000000_11100000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(BYTE, 16), VLanes(BYTE)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    // UDOT (vector)
    0b00101110_10000000_10010100_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(BYTE, 8), VSizedStatic(BYTE, 8)] => [R(0), R(5), R(16)];
    0b01101110_10000000_10010100_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16)] => [R(0), R(5), R(16)];
]
"uhadd" = [
    0b00101110_00100000_00000100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_01100000_00000100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_10100000_00000100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
]
"uhsub" = [
    0b00101110_00100000_00100100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_01100000_00100100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_10100000_00100100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
]
"umaddl" = [
    0b10011011_10100000_00000000_00000000 = [X, W, W, X] => [R(0), R(5), R(16), R(10)];
]
"umax" = [
    0b00101110_00100000_01100100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_01100000_01100100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_10100000_01100100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
]
"umaxp" = [
    0b00101110_00100000_10100100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_01100000_10100100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_10100000_10100100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
]
"umaxv" = [
    0b00101110_00110000_10101000_00000000 = [B, VSized(BYTE)] => [R(0), R(5), Rwidth(30)];
    0b00101110_01110000_10101000_00000000 = [H, VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_10110000_10101000_00000000 = [S, VSizedStatic(DWORD, 4)] => [R(0), R(5), Rwidth(30)];
]
"umin" = [
    0b00101110_00100000_01101100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_01100000_01101100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_10100000_01101100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
]
"uminp" = [
    0b00101110_00100000_10101100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_01100000_10101100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_10100000_10101100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
]
"uminv" = [
    0b00101110_00110001_10101000_00000000 = [B, VSized(BYTE)] => [R(0), R(5), Rwidth(30)];
    0b00101110_01110001_10101000_00000000 = [H, VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_10110001_10101000_00000000 = [S, VSizedStatic(DWORD, 4)] => [R(0), R(5), Rwidth(30)];
]
"umlal" = [
    // UMLAL, UMLAL2 (by element)
    0b00101111_01000000_00100000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20])];
    0b00101111_10000000_00100000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2), VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    // UMLAL, UMLAL2 (vector)
    0b00101110_00100000_10000000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 8), VSizedStatic(BYTE, 8)] => [R(0), R(5), R(16)];
    0b00101110_01100000_10000000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VSizedStatic(WORD, 4)] => [R(0), R(5), R(16)];
    0b00101110_10100000_10000000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2), VSizedStatic(DWORD, 2)] => [R(0), R(5), R(16)];
]
"umlal2" = [
    // UMLAL, UMLAL2 (by element)
    0b01101111_01000000_00100000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8), VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20])];
    0b01101111_10000000_00100000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4), VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    // UMLAL, UMLAL2 (vector)
    0b01101110_00100000_10000000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16)] => [R(0), R(5), R(16)];
    0b01101110_01100000_10000000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8), VSizedStatic(WORD, 8)] => [R(0), R(5), R(16)];
    0b01101110_10100000_10000000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"umlsl" = [
    // UMLSL, UMLSL2 (by element)
    0b00101111_01000000_01100000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20])];
    0b00101111_10000000_01100000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2), VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    // UMLSL, UMLSL2 (vector)
    0b00101110_00100000_10100000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 8), VSizedStatic(BYTE, 8)] => [R(0), R(5), R(16)];
    0b00101110_01100000_10100000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VSizedStatic(WORD, 4)] => [R(0), R(5), R(16)];
    0b00101110_10100000_10100000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2), VSizedStatic(DWORD, 2)] => [R(0), R(5), R(16)];
]
"umlsl2" = [
    // UMLSL, UMLSL2 (by element)
    0b01101111_01000000_01100000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8), VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20])];
    0b01101111_10000000_01100000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4), VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    // UMLSL, UMLSL2 (vector)
    0b01101110_00100000_10100000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16)] => [R(0), R(5), R(16)];
    0b01101110_01100000_10100000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8), VSizedStatic(WORD, 8)] => [R(0), R(5), R(16)];
    0b01101110_10100000_10100000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"umnegl" = [
    0b10011011_10100000_11111100_00000000 = [X, W, W] => [R(0), R(5), R(16)];
]
"umov" = [
    0b00001110_00000001_00111100_00000000 = [W, VLanes(BYTE)] => [R(0), R(5), Ubits(17, 4)];
    0b00001110_00000010_00111100_00000000 = [W, VLanes(WORD)] => [R(0), R(5), Ubits(18, 3)];
    0b01001110_00000001_00111100_00000000 = [X, VLanes(BYTE)] => [R(0), R(5), Ubits(17, 4)];
    0b01001110_00000010_00111100_00000000 = [X, VLanes(WORD)] => [R(0), R(5), Ubits(18, 3)];
    0b01001110_00000100_00111100_00000000 = [X, VLanes(DWORD)] => [R(0), R(5), Ubits(19, 2)];
]
"umsubl" = [
    0b10011011_10100000_10000000_00000000 = [X, W, W, X] => [R(0), R(5), R(16), R(10)];
]
"umulh" = [
    0b10011011_11000000_01111100_00000000 = [X, X, X] => [R(0), R(5), R(16)];
]
"umull" = [
    // UMULL, UMULL2 (by element)
    0b00101111_01000000_10100000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20])];
    0b00101111_10000000_10100000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2), VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    // UMULL, UMULL2 (vector)
    0b00101110_00100000_11000000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 8), VSizedStatic(BYTE, 8)] => [R(0), R(5), R(16)];
    0b00101110_01100000_11000000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VSizedStatic(WORD, 4)] => [R(0), R(5), R(16)];
    0b00101110_10100000_11000000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2), VSizedStatic(DWORD, 2)] => [R(0), R(5), R(16)];
    // UMULL
    0b10011011_10100000_01111100_00000000 = [X, W, W] => [R(0), R(5), R(16)];
]
"umull2" = [
    // UMULL, UMULL2 (by element)
    0b01101111_01000000_10100000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8), VLanes(WORD)] => [R(0), R(5), R4(16), Ufields(&[11, 21, 20])];
    0b01101111_10000000_10100000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4), VLanes(DWORD)] => [R(0), R(5), R(16), Ufields(&[11, 21])];
    // UMULL, UMULL2 (vector)
    0b01101110_00100000_11000000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16)] => [R(0), R(5), R(16)];
    0b01101110_01100000_11000000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8), VSizedStatic(WORD, 8)] => [R(0), R(5), R(16)];
    0b01101110_10100000_11000000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"uqadd" = [
    0b01111110_00100000_00001100_00000000 = [B, B, B] => [R(0), R(5), R(16)];
    0b01111110_01100000_00001100_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b01111110_10100000_00001100_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b01111110_11100000_00001100_00000000 = [D, D, D] => [R(0), R(5), R(16)];
    0b00101110_00100000_00001100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_01100000_00001100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_10100000_00001100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_11100000_00001100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"uqrshl" = [
    0b01111110_00100000_01011100_00000000 = [B, B, B] => [R(0), R(5), R(16)];
    0b01111110_01100000_01011100_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b01111110_10100000_01011100_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b01111110_11100000_01011100_00000000 = [D, D, D] => [R(0), R(5), R(16)];
    0b00101110_00100000_01011100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_01100000_01011100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_10100000_01011100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_11100000_01011100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"uqrshrn" = [
    0b01111111_00000000_10011100_00000000 = [B, H, Imm] => [R(0), R(5), BUrange(1, 8), Usub(16, 4, 16)];
    0b01111111_00000000_10011100_00000000 = [H, S, Imm] => [R(0), R(5), BUrange(1, 16), Usub(16, 5, 32)];
    0b01111111_00000000_10011100_00000000 = [S, D, Imm] => [R(0), R(5), BUrange(1, 32), Usub(16, 6, 64)];
    0b00101111_00001000_10011100_00000000 = [VSizedStatic(BYTE, 8), VSizedStatic(WORD, 8), Imm] => [R(0), R(5), Usub(16, 3, 8)];
    0b00101111_00010000_10011100_00000000 = [VSizedStatic(WORD, 4), VSizedStatic(DWORD, 4), Imm] => [R(0), R(5), Usub(16, 4, 16)];
    0b00101111_00100000_10011100_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Usub(16, 5, 32)];
]
"uqrshrn2" = [
    0b01101111_00001000_10011100_00000000 = [VSizedStatic(BYTE, 16), VSizedStatic(WORD, 8), Imm] => [R(0), R(5), Usub(16, 3, 8)];
    0b01101111_00010000_10011100_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(DWORD, 4), Imm] => [R(0), R(5), Usub(16, 4, 16)];
    0b01101111_00100000_10011100_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Usub(16, 5, 32)];
]
"uqshl" = [
    // UQSHL (immediate)
    0b01111111_00001000_01110100_00000000 = [B, B, Imm] => [R(0), R(5), Ubits(16, 3)];
    0b01111111_00010000_01110100_00000000 = [H, H, Imm] => [R(0), R(5), Ubits(16, 4)];
    0b01111111_00100000_01110100_00000000 = [S, S, Imm] => [R(0), R(5), Ubits(16, 5)];
    0b01111111_01000000_01110100_00000000 = [D, D, Imm] => [R(0), R(5), Ubits(16, 6)];
    0b00101111_00001000_01110100_00000000 = [VSized(BYTE), VSized(BYTE), Imm] => [R(0), R(5), Ubits(16, 3), Rwidth(30)];
    0b00101111_00010000_01110100_00000000 = [VSized(WORD), VSized(WORD), Imm] => [R(0), R(5), Ubits(16, 4), Rwidth(30)];
    0b00101111_00100000_01110100_00000000 = [VSized(DWORD), VSized(DWORD), Imm] => [R(0), R(5), Ubits(16, 5), Rwidth(30)];
    0b00101111_01000000_01110100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Ubits(16, 6), Rwidth(30)];
    // UQSHL (register)
    0b01111110_00100000_01001100_00000000 = [B, B, B] => [R(0), R(5), R(16)];
    0b01111110_01100000_01001100_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b01111110_10100000_01001100_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b01111110_11100000_01001100_00000000 = [D, D, D] => [R(0), R(5), R(16)];
    0b00101110_00100000_01001100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_01100000_01001100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_10100000_01001100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_11100000_01001100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"uqshrn" = [
    0b01111111_00000000_10010100_00000000 = [B, H, Imm] => [R(0), R(5), BUrange(1, 8), Usub(16, 4, 16)];
    0b01111111_00000000_10010100_00000000 = [H, S, Imm] => [R(0), R(5), BUrange(1, 16), Usub(16, 5, 32)];
    0b01111111_00000000_10010100_00000000 = [S, D, Imm] => [R(0), R(5), BUrange(1, 32), Usub(16, 6, 64)];
    0b00101111_00001000_10010100_00000000 = [VSizedStatic(BYTE, 8), VSizedStatic(WORD, 8), Imm] => [R(0), R(5), Usub(16, 3, 8)];
    0b00101111_00010000_10010100_00000000 = [VSizedStatic(WORD, 4), VSizedStatic(DWORD, 4), Imm] => [R(0), R(5), Usub(16, 4, 16)];
    0b00101111_00100000_10010100_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Usub(16, 5, 32)];
]
"uqshrn2" = [
    0b01101111_00001000_10010100_00000000 = [VSizedStatic(BYTE, 16), VSizedStatic(WORD, 8), Imm] => [R(0), R(5), Usub(16, 3, 8)];
    0b01101111_00010000_10010100_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(DWORD, 4), Imm] => [R(0), R(5), Usub(16, 4, 16)];
    0b01101111_00100000_10010100_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Usub(16, 5, 32)];
]
"uqsub" = [
    0b01111110_00100000_00101100_00000000 = [B, B, B] => [R(0), R(5), R(16)];
    0b01111110_01100000_00101100_00000000 = [H, H, H] => [R(0), R(5), R(16)];
    0b01111110_10100000_00101100_00000000 = [S, S, S] => [R(0), R(5), R(16)];
    0b01111110_11100000_00101100_00000000 = [D, D, D] => [R(0), R(5), R(16)];
    0b00101110_00100000_00101100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_01100000_00101100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_10100000_00101100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_11100000_00101100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"uqxtn" = [
    0b01111110_00100001_01001000_00000000 = [B, H] => [R(0), R(5)];
    0b01111110_01100001_01001000_00000000 = [H, S] => [R(0), R(5)];
    0b01111110_10100001_01001000_00000000 = [S, D] => [R(0), R(5)];
    0b00101110_00100001_01001000_00000000 = [VSizedStatic(BYTE, 8), VSizedStatic(WORD, 8)] => [R(0), R(5)];
    0b00101110_01100001_01001000_00000000 = [VSizedStatic(WORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5)];
    0b00101110_10100001_01001000_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5)];
]
"uqxtn2" = [
    0b01101110_00100001_01001000_00000000 = [VSizedStatic(BYTE, 16), VSizedStatic(WORD, 8)] => [R(0), R(5)];
    0b01101110_01100001_01001000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(DWORD, 4)] => [R(0), R(5)];
    0b01101110_10100001_01001000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(QWORD, 2)] => [R(0), R(5)];
]
"urecpe" = [
    0b00001110_10100001_11001000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00001110_11100001_11001000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
]
"urhadd" = [
    0b00101110_00100000_00010100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_01100000_00010100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_10100000_00010100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
]
"urshl" = [
    0b01111110_11100000_01010100_00000000 = [D, D, D] => [R(0), R(5), R(16)];
    0b00101110_00100000_01010100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_01100000_01010100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_10100000_01010100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_11100000_01010100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"urshr" = [
    0b01111111_00000000_00100100_00000000 = [D, D, Imm] => [R(0), R(5), BUrange(1, 64), Usub(16, 7, 128)];
    0b00101111_00001000_00100100_00000000 = [VSized(BYTE), VSized(BYTE), Imm] => [R(0), R(5), Usub(16, 3, 8), Rwidth(30)];
    0b00101111_00010000_00100100_00000000 = [VSized(WORD), VSized(WORD), Imm] => [R(0), R(5), Usub(16, 4, 16), Rwidth(30)];
    0b00101111_00100000_00100100_00000000 = [VSized(DWORD), VSized(DWORD), Imm] => [R(0), R(5), Usub(16, 5, 32), Rwidth(30)];
    0b00101111_01000000_00100100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Usub(16, 6, 64), Rwidth(30)];
]
"ursqrte" = [
    0b00101110_10100001_11001000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_11100001_11001000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
]
"ursra" = [
    0b01111111_00000000_00110100_00000000 = [D, D, Imm] => [R(0), R(5), BUrange(1, 64), Usub(16, 7, 128)];
    0b00101111_00001000_00110100_00000000 = [VSized(BYTE), VSized(BYTE), Imm] => [R(0), R(5), Usub(16, 3, 8), Rwidth(30)];
    0b00101111_00010000_00110100_00000000 = [VSized(WORD), VSized(WORD), Imm] => [R(0), R(5), Usub(16, 4, 16), Rwidth(30)];
    0b00101111_00100000_00110100_00000000 = [VSized(DWORD), VSized(DWORD), Imm] => [R(0), R(5), Usub(16, 5, 32), Rwidth(30)];
    0b00101111_01000000_00110100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Usub(16, 6, 64), Rwidth(30)];
]
"ushl" = [
    0b01111110_11100000_01000100_00000000 = [D, D, D] => [R(0), R(5), R(16)];
    0b00101110_00100000_01000100_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_01100000_01000100_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_10100000_01000100_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00101110_11100000_01000100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"ushll" = [
    0b00101111_00001000_10100100_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 8), Imm] => [R(0), R(5), Ubits(16, 3)];
    0b00101111_00010000_10100100_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), Imm] => [R(0), R(5), Ubits(16, 4)];
    0b00101111_00100000_10100100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2), Imm] => [R(0), R(5), Ubits(16, 5)];
]
"ushll2" = [
    0b01101111_00001000_10100100_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 16), Imm] => [R(0), R(5), Ubits(16, 3)];
    0b01101111_00010000_10100100_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8), Imm] => [R(0), R(5), Ubits(16, 4)];
    0b01101111_00100000_10100100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4), Imm] => [R(0), R(5), Ubits(16, 5)];
]
"ushr" = [
    0b01111111_00000000_00000100_00000000 = [D, D, Imm] => [R(0), R(5), BUrange(1, 64), Usub(16, 7, 128)];
    0b00101111_00001000_00000100_00000000 = [VSized(BYTE), VSized(BYTE), Imm] => [R(0), R(5), Usub(16, 3, 8), Rwidth(30)];
    0b00101111_00010000_00000100_00000000 = [VSized(WORD), VSized(WORD), Imm] => [R(0), R(5), Usub(16, 4, 16), Rwidth(30)];
    0b00101111_00100000_00000100_00000000 = [VSized(DWORD), VSized(DWORD), Imm] => [R(0), R(5), Usub(16, 5, 32), Rwidth(30)];
    0b00101111_01000000_00000100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Usub(16, 6, 64), Rwidth(30)];
]
"usqadd" = [
    0b01111110_00100000_00111000_00000000 = [B, B] => [R(0), R(5)];
    0b01111110_01100000_00111000_00000000 = [H, H] => [R(0), R(5)];
    0b01111110_10100000_00111000_00000000 = [S, S] => [R(0), R(5)];
    0b01111110_11100000_00111000_00000000 = [D, D] => [R(0), R(5)];
    0b00101110_00100000_00111000_00000000 = [VSized(BYTE), VSized(BYTE)] => [R(0), R(5), Rwidth(30)];
    0b00101110_01100000_00111000_00000000 = [VSized(WORD), VSized(WORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_10100000_00111000_00000000 = [VSized(DWORD), VSized(DWORD)] => [R(0), R(5), Rwidth(30)];
    0b00101110_11100000_00111000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), Rwidth(30)];
]
"usra" = [
    0b01111111_00000000_00010100_00000000 = [D, D, Imm] => [R(0), R(5), BUrange(1, 64), Usub(16, 7, 128)];
    0b00101111_00001000_00010100_00000000 = [VSized(BYTE), VSized(BYTE), Imm] => [R(0), R(5), Usub(16, 3, 8), Rwidth(30)];
    0b00101111_00010000_00010100_00000000 = [VSized(WORD), VSized(WORD), Imm] => [R(0), R(5), Usub(16, 4, 16), Rwidth(30)];
    0b00101111_00100000_00010100_00000000 = [VSized(DWORD), VSized(DWORD), Imm] => [R(0), R(5), Usub(16, 5, 32), Rwidth(30)];
    0b00101111_01000000_00010100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), Usub(16, 6, 64), Rwidth(30)];
]
"usubl" = [
    0b00101110_00100000_00100000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 8), VSizedStatic(BYTE, 8)] => [R(0), R(5), R(16)];
    0b00101110_01100000_00100000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4), VSizedStatic(WORD, 4)] => [R(0), R(5), R(16)];
    0b00101110_10100000_00100000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2), VSizedStatic(DWORD, 2)] => [R(0), R(5), R(16)];
]
"usubl2" = [
    0b01101110_00100000_00100000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16)] => [R(0), R(5), R(16)];
    0b01101110_01100000_00100000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8), VSizedStatic(WORD, 8)] => [R(0), R(5), R(16)];
    0b01101110_10100000_00100000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"usubw" = [
    0b00101110_00100000_00110000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(WORD, 8), VSizedStatic(BYTE, 8)] => [R(0), R(5), R(16)];
    0b00101110_01100000_00110000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4)] => [R(0), R(5), R(16)];
    0b00101110_10100000_00110000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2)] => [R(0), R(5), R(16)];
]
"usubw2" = [
    0b01101110_00100000_00110000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(WORD, 8), VSizedStatic(BYTE, 16)] => [R(0), R(5), R(16)];
    0b01101110_01100000_00110000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8)] => [R(0), R(5), R(16)];
    0b01101110_10100000_00110000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4)] => [R(0), R(5), R(16)];
]
"uxtb" = [
    0b01010011_00000000_00011100_00000000 = [W, W] => [R(0), R(5)];
]
"uxth" = [
    0b01010011_00000000_00111100_00000000 = [W, W] => [R(0), R(5)];
]
"uxtl" = [
    0b00101111_00001000_10100100_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 8)] => [R(0), R(5)];
    0b00101111_00010000_10100100_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 4)] => [R(0), R(5)];
    0b00101111_00100000_10100100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 2)] => [R(0), R(5)];
]
"uxtl2" = [
    0b01101111_00001000_10100100_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(BYTE, 16)] => [R(0), R(5)];
    0b01101111_00010000_10100100_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(WORD, 8)] => [R(0), R(5)];
    0b01101111_00100000_10100100_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(DWORD, 4)] => [R(0), R(5)];
]
"uzp1" = [
    0b00001110_00000000_00011000_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01000000_00011000_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10000000_00011000_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_11000000_00011000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"uzp2" = [
    0b00001110_00000000_01011000_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01000000_01011000_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10000000_01011000_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_11000000_01011000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"wfe" = [
    0b11010101_00000011_00100000_01011111 = [] => [];
]
"wfi" = [
    0b11010101_00000011_00100000_01111111 = [] => [];
]
"xar" = [
    0b11001110_10000000_00000000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), Imm] => [R(0), R(5), R(16), Ubits(10, 6)];
]
"xpacd" = [
    0b11011010_11000001_01000111_11100000 = [X] => [R(0)];
]
"xpaci" = [
    0b11011010_11000001_01000011_11100000 = [X] => [R(0)];
]
"xpaclri" = [
    0b11010101_00000011_00100000_11111111 = [] => [];
]
"xtn" = [
    0b00001110_00100001_00101000_00000000 = [VSizedStatic(BYTE, 8), VSizedStatic(WORD, 8)] => [R(0), R(5)];
    0b00001110_01100001_00101000_00000000 = [VSizedStatic(WORD, 4), VSizedStatic(DWORD, 4)] => [R(0), R(5)];
    0b00001110_10100001_00101000_00000000 = [VSizedStatic(DWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5)];
]
"xtn2" = [
    0b01001110_00100001_00101000_00000000 = [VSizedStatic(BYTE, 16), VSizedStatic(WORD, 8)] => [R(0), R(5)];
    0b01001110_01100001_00101000_00000000 = [VSizedStatic(WORD, 8), VSizedStatic(DWORD, 4)] => [R(0), R(5)];
    0b01001110_10100001_00101000_00000000 = [VSizedStatic(DWORD, 4), VSizedStatic(QWORD, 2)] => [R(0), R(5)];
]
"yield" = [
    0b11010101_00000011_00100000_00111111 = [] => [];
]
"zip1" = [
    0b00001110_00000000_00111000_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01000000_00111000_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10000000_00111000_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_11000000_00111000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]
"zip2" = [
    0b00001110_00000000_01111000_00000000 = [VSized(BYTE), VSized(BYTE), VSized(BYTE)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_01000000_01111000_00000000 = [VSized(WORD), VSized(WORD), VSized(WORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_10000000_01111000_00000000 = [VSized(DWORD), VSized(DWORD), VSized(DWORD)] => [R(0), R(5), R(16), Rwidth(30)];
    0b00001110_11000000_01111000_00000000 = [VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)] => [R(0), R(5), R(16), Rwidth(30)];
]

);
