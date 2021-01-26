static INPUT: [&[u8]; 375] = [b"9 * 7 + ((6 + 9 * 2 + 6 + 7 + 5) * (5 + 9 + 7) * 6) * (4 + 8) + 5 * 8",b"3 * (4 * 9 + 8 + 4 * 4 + 7) + 7 * (8 * 9)",b"4 * ((6 * 5 * 9) + 2) + 9 * 4",b"((4 * 6 * 2 * 9) + 7 * 4) + 4 * 6 + 5 * 6 + 6",b"9 * 8 * ((5 + 7 * 8 * 2) * 6) + 3",b"6 + (3 * 7) * 3 + (9 * (6 * 6 + 5 + 9 + 9) * 7 * 9 + (4 * 8) + 8)",b"6 * 4 + 2 + 8 + 7",b"(5 * (9 * 8) * 6) + 2 + 8 * 5 * 4 * 3",b"(3 + 2 * 2 * 8 * 5) + (7 + 6 * 4 * 8) * (6 * 9 * 5 * 9)",b"(4 * 2 + (9 + 2 * 4) * 5 + 9) + (9 * 3)",b"7 * 6 + 6",b"4 + (8 + (5 * 9) + 9 * 8)",b"9 + 8 + 5 + 4 * 2 * 9",b"8 * (5 + (8 + 2) * 9 + 4 + 7) + 8",b"2 * 2 + 9 * 6 + (2 * (8 * 2 + 3 * 8) * (9 + 7 * 4 * 8 * 2)) * 2",b"4 + (5 * 6 + 7 * 5) + 4",b"4 + (9 * 4) + 4 * (4 + 7 + 9 * 4) * 3",b"(4 * (2 + 2) * 9 * 8 * 3) + 9 * 3",b"6 * (5 * 3 + 4 + (4 + 4 + 5 * 7 * 4)) * 4",b"8 * (6 * 9 + 3 * (6 * 6 * 2) + 4 + 9) * (4 * 8 * (9 * 4 + 8 + 7 * 7) * 5) + 4 + 7",b"(7 * 3 * 2) + 3 + 8 * (5 + 7 * 3 + 4 * 2 * (8 + 6 * 4)) * 3 * (8 + 9 + 7 * 3 * 3)",b"8 * 7 + 4 + 7 * (3 * 3 + 7 + (3 * 5 * 8 + 8 + 3 * 4))",b"(7 * (4 * 3 + 8 * 7 * 3 + 9) * 4 + (4 * 7)) * 8 + ((2 * 9) + 6 * 7 + (9 + 3) * 5 + 2) * 8 + 6",b"6 + 8 * (7 * (5 + 3 + 8 * 5 + 2 * 7) * 4 + 2 + 7 + 7)",b"(6 * 3 * 7) * 7 * (7 * 7 * 9) + 8",b"((8 * 7 + 2 + 3) * 4) + 4 * 7 * 6 + 5",b"2 * 5 * 4 * 9 + ((2 * 4 * 4) + 4 * (5 + 4))",b"((3 + 5 * 5) + (5 * 9 + 5)) * 3",b"8 * (3 * (9 * 3 * 9) + 3 + 5) * 2",b"4 * (8 * 9 * 7 * 2 + 3) + ((9 * 8) * 9) * (4 * 5 + 6 * 8) * 3",b"(5 + 2 * 9 + 5 + 8) + 9 + 6 * 2",b"(5 * 9 + 9 + 3 + (5 + 9 + 4 + 3)) + 8 * 7",b"(7 + 7 * 2 + 8) * (2 * 4 + 3 * 4) + 5 * 4",b"3 + (4 + 6) + 8 * 5",b"(6 + 5 * (8 + 2 + 6) * (2 + 9)) * ((3 + 2 * 8 + 4 * 9 * 3) * 6 * 9)",b"9 + (5 * (2 + 2 * 2) + 5 * 8) * 7 + 3 + 9",b"2 + 8 * ((2 + 2) * 2 + 4 * 3 + 3) * 5",b"(2 + 5 * 3 * (4 * 3 + 8 + 6) + 6) + ((7 + 9) + 4 * 3) + 6 * 9",b"(8 + 7 * (8 + 3 * 7 + 4 + 7) + (5 + 6)) + 4 * (9 * 2 + (8 + 7 * 4 * 6 + 3) + 7 + 2) + (3 * (2 + 2 * 3))",b"(3 * 6 * 4 * 8 + 4) + 6 + 3 * 3 + 4",b"8 + ((6 * 2) + 6 * (8 + 5 * 2 * 2 * 9 + 8)) * 5 * 7",b"7 + 3 + 5 + ((3 + 8 + 2) * 5 * 9 * 4 * 9 + 5)",b"4 * 5 * (8 + 6)",b"9 * 7 * 4 + (4 + 5) * 4",b"9 + 6 + (8 * 5 * (3 + 2 * 7 + 2 + 2 + 9) + 8 * (8 * 5 + 5 * 5 * 3 * 7) * (5 * 3 + 8 + 7 * 4)) + 7 + 2 * 5",b"6 + ((2 * 3 + 9 * 8 * 8) + 6 * (5 * 5)) * 2 + 3 * (2 + (7 * 8 * 9 * 4 + 7 * 3) * 2 * 7 * (7 + 3 + 2 * 7 + 9 + 2)) + (3 * 9 + 2 * 8 * 4 * 2)",b"(7 * 4) * 2 + 7 + 2 + 3",b"(5 + 6 + 8) + 8 + 3 + (5 + 8 * 5 + 5 + 8) + 3",b"2 * (6 * (3 + 5)) + 4 + (3 + 9 * 5 + 8 + (6 + 4 + 2 + 2 * 2 * 4)) * 3 + 8",b"(2 + 7 + 3) + 7 + 2 + 6 * (6 * (4 + 7 * 8 * 5)) * 4",b"7 * 5 + 3 + 3",b"4 * (2 + 3 + 5) * 2 * 3",b"(8 * 8 + (7 * 4 * 6 * 6 + 2 * 6) + (8 + 6 + 2) * 7 * 8) + (3 * (8 + 9 + 6) + 4 * 7) + 5 * 8",b"(7 * 8 * 2 + 3 * 9) * ((4 * 4 + 5) + (6 + 4 + 9 + 5) + 3 * 2 * (2 + 5 * 7)) + 4 + 8 * ((8 * 8 + 5 * 4 * 3 + 4) * 5 + 4 * 4 + 4 + 7)",b"(9 + 8 + (4 + 2 * 9) + 7 + 7) + 7 * 8",b"(3 * 5 + (7 + 5 * 2 + 2)) * 4",b"(5 + 8) * 6 + (6 + (7 + 4 + 3 + 2 + 6 + 9))",b"4 * 2 + 2 + 9 + 3 + (2 * 6 * (2 * 2 * 8) + 6 * (3 + 2 * 3 * 4 + 9) * 7)",b"(7 + (9 + 8 + 6 * 6 * 2)) + 6 * (8 * 9 * (5 * 7 + 6 + 3)) + 4",b"(3 * (9 + 2) + 6 + 6) + 4 * 4 * 4 * 7",b"4 * 4 + 6 * 3 * 2 * 7",b"3 + (7 + 3 + (4 + 4 * 3 + 5 + 4) * 9 + (2 + 5 + 7 * 7 * 2) * 4)",b"5 * (3 * 6 * (7 + 2 * 2) + 7 + 7) + (2 + 7 + 3 * 7 * 6) + 3 * 4 + 4",b"2 * 4 * 2 * (8 * (7 * 7) * 7 * (2 * 8 + 7)) + 5",b"4 + 6 + (8 * 9 + 4 + 8 + 4) * 3",b"5 * 9 * 9 + 4 + 2 + (3 + (5 + 9))",b"(9 + 8 + (6 * 5 + 7 * 5 * 4) + (6 + 7 * 2 + 8 + 6)) * 4 + 8 * 6 + 8",b"7 * 2 + ((7 + 3 * 7 * 7 * 2) * (9 * 9 * 8 + 7) * (7 + 9 * 4 * 2 + 3) + 6) * 6",b"8 * 4 + 9 + 7 * (6 + 9)",b"6 + 3 + 3 * 7 + ((8 + 9) * 4) * 6",b"7 + 6 * (2 * (6 + 7) * (8 * 6)) + 5 + 7",b"7 * (5 + 9 * 6 * 5) * 7 + 6 * 2 * 2",b"7 * 3 * (2 + (9 + 3 * 6) + 8 * (9 * 2 * 2 + 2) * 3 * 5)",b"6 + 5 * 7 * 9 * (8 * 2 * 6 * 5 + 9 * 6) * 2",b"5 * 3 + (8 * (9 + 3 * 2 + 9 * 4) * 2 * 7) + 2 * (2 + 8 * 6 + 5 + 7 * 3)",b"9 * 4 * (6 + (2 + 6 + 4 + 2 * 5) * 8 + 2 + 8) + 4 * 3 * (9 + 8)",b"3 * 6 + 9 + 6 * 3 + (7 + 7 * 2)",b"7 * (8 + (3 + 9 * 3) * 3 + 2 + 9 * 5) + (3 + (3 + 3 + 8 * 9))",b"2 * 2 * ((3 * 6) + 7 + 2 * (9 + 8 * 3 * 6) + 5) * (4 + (2 * 2 + 8 + 8 + 6 + 4) + (7 * 2 * 2 + 9 * 8 + 8) + 9 + 3)",b"(7 + 2 * 8 * 3) + 9",b"8 * 8 + (9 + 2 + 3 + 2)",b"(3 + 2 * 4 * 6 * 3) + 7",b"8 * 4 * 3 + (7 + 5 + 6 * 5 * 6) + 9",b"(2 * 3) + (8 * 2 * (6 * 3) * 5) * ((7 * 9 + 4 * 4) + 2 * (4 * 9 + 3) + 9 * (5 * 5) * 4)",b"2 + 6 * (3 + 5 * 7) * 2",b"(6 + (3 + 4) * 6 * 8) + 8",b"8 * 5 * 2 + (7 * (5 + 3 + 4 + 4) * 4 + 9)",b"5 * 9 * (5 + 8 + 9 * (2 + 7 * 2 * 6)) * 9",b"(5 + 9 * (6 * 7 + 9 + 6 * 8) * 8) * (3 + 7 + 2) * 2 + (9 + 6 * 4 * (9 * 3 * 4) * 5 * (7 * 4 + 3)) * 8",b"7 * 3 * 7 * 3 * ((3 + 2 * 9) + 6 * (3 * 2) + 4)",b"7 + 4 * (7 + 9 * 8) * 7 * 5 + 7",b"(7 + 9 * (2 + 4 * 9 * 4 * 7 + 4) * 3) * (8 * (2 * 4 + 6 * 2) * 6)",b"((3 * 6 + 4 * 6 * 4) * 6 * 8 * (9 * 4 + 2 + 5 + 3 + 9) * 5) * 6 + (9 + 8)",b"(3 * 3 + 4) * 4",b"(4 * (7 * 6 + 4 * 4 * 5) * 3 + 3) + 6 * 6 * (2 * 3 * (8 * 3 + 2 * 5 * 6 + 7) + 9 + 5) * 4 * 7",b"(2 * (5 * 6 + 9) * 7) + 3",b"5 + 9 + 2 * ((6 * 6) * (3 + 7 + 6 + 6) + 3 + 5)",b"4 * 2 * 9 + (3 + 4 + 5 * (4 + 8 * 5) * 9 + 5)",b"7 + 4 * 6 + 9 * 7 + 3",b"(8 * 7) + 7 * 3 * (7 * 9 * (4 + 4 + 7 * 5 + 7 * 2) + 9 * (5 * 5 * 8 + 7) * 3) + ((9 * 8 + 8) + 2 * 2) + 6",b"2 + 7 * 4 * 7 * (4 * 3 + 6 + 8 * 6) + 7",b"6 + (9 + 8 * (5 * 7) * 4) * (5 * 8 * 2 * (8 * 6 + 6) * 5 + 3) + 4 * 7",b"6 + 5 + (7 + (8 + 2) * 4 * 3 * 2 * 9) * 7",b"7 + 3 * 9",b"(5 * 5 + 8 * (9 * 4 * 3 + 6 + 8 * 2)) * (2 * 6 + 8) + 6 * 7",b"(2 + 8) * 9 + 4 + 4 * 2 + (3 * 3 + 9 + 2 + 9 + 5)",b"((5 * 4) + (3 * 4 + 3 + 4) + (8 + 3 + 3 + 9 + 2) + 4) * 9 * 7 * 9 + 6",b"9 + 3 * 4 * (9 + 6 * (9 + 8) + (8 * 4) * 5 + 2)",b"(4 + (3 + 2 * 4 + 7) + 4 + 2) * 4 * 7 * 3 * (2 * 8 * 9 * 5 + (3 * 3 + 8) * 3)",b"2 * 7 * (2 * 5 + 5 * (2 + 7 + 4)) * 7 + 2 + 4",b"4 * 2 * (6 * 6 + 2 + 2 + 6 * (2 * 2 * 8)) + 3 * 9",b"(5 * 9) + 2 + (3 * 8 * (8 + 4 * 3 + 7 * 9 * 5) * 2 * 6)",b"2 * 9",b"6 * 9 * 2 + 9 * (4 + 3) * (8 + 6 * 9 * 3 + 7 * (2 * 2 + 7 + 8 + 3 * 7))",b"3 * 8 + 9 + 4",b"9 + (4 * 9 + (6 * 9 + 6) + 2 * 6 * 9) * (2 * 4 * (3 * 5 * 2 + 2 + 8 * 6)) * 6",b"(3 * (3 * 6 + 8 + 3 + 7) * 9 * (8 * 3 * 4) + 6 + 3) + 8 * (4 + 9 + 3 + 2 + (4 + 4 * 6 + 2))",b"6 + 5 * 2 + 7 + ((9 + 2 + 4) * 6 + 5 * 8) * ((6 + 5 + 6 + 2) + (6 * 3 + 7) + 7 * 2 + (7 * 7 + 3 * 4 * 3 + 2))",b"((2 + 3 * 6 * 5) * 7 * 9 * 5 * 2 * 8) + 8 + 3 * 3 * 6",b"7 * (8 * 6) * 4 + 2 * 7 * 8",b"2 + 4 + (9 * 5 + 3)",b"5 + ((7 * 8) + 8 + 6 + 6 * 2 * 3) * 2 * 3",b"9 + (4 * 5 * (6 * 6 * 9 * 5 * 6 * 8)) * (8 + 5 + 3 * 7) + 9 * 3 * 4",b"8 + 4 * 5 * 2",b"5 + 2 + 8 + 4 * (9 * 2 * 6) * 3",b"4 * 6 * (5 * 2 + 3 + 5 + (8 + 5 * 7 * 4) + 4) * 8 + 2",b"(9 * 7 + 2 + (3 * 5 * 4 + 6 + 3 * 2) * (4 * 8)) * 3 + ((9 * 2 + 6) * 6 + 5 + 5 + 5) * 8",b"(8 + (2 + 2)) * 7 + (9 * 9 + 3) + ((5 * 4 * 9) + (2 * 6 + 9)) + 3 * 3",b"6 * 8 + 8 + 2 + 7 + (5 + 3)",b"2 + 2 * (4 + 5) * 6 + 4 * (6 * 8 + 4 + 2 * (9 + 5) * 3)",b"(5 * 5 + 6) + 8 + 3 * (2 * 3 * 3 + 6 * 7 * (6 + 7 + 8 * 6 * 4))",b"7 * 8 + 2 + 5 * (9 + 6 + (5 + 5 * 6 * 3 + 7) * 2 * 5 + (7 + 8))",b"6 * (9 + 5 * 3) * 7 * (9 * 5) + 3 + 4",b"(9 + (3 + 2 * 2) * 6) * 2 + 5 * 6 + ((9 * 2 + 4 + 5 * 7) * 2 * 7 + 5)",b"6 + (5 + 7 + 5 + 5 + 9 + 5) * (8 * 4 * 8 + 4) + 4 * 3 * 9",b"2 * 9 * 9 * (4 + 2) * 7 + (2 * (4 * 7 + 3) + 8 * 9 * 3 + 9)",b"5 * 5 * (6 * (8 * 2) + 8 * 8)",b"(4 * (5 * 9 * 2 * 8 * 5) * 8 + (6 + 3)) + (8 * 9 + 4 * (5 * 5 + 6)) * 4 + 6 * 4",b"9 * 6 + 6 + (7 * (3 + 3 * 9 + 7 * 7 + 5) + 3)",b"5 * ((5 * 3 + 5 + 7 + 2 + 3) + 8) * 6 + 9 * 7",b"(4 * 9 + 4 + 2 * (8 + 9)) * 6 * 4 + 9 + 5",b"5 * 4 + 3 + 2",b"(5 * (9 + 7 + 3 + 4) * (8 + 3 + 9 + 9 + 9) + 5 + 9) * (3 * 3) * 7 + (2 * 6) + 8",b"(7 * 6 + 7) * 7 + 9",b"((6 + 4 * 9 * 7 * 8) * 3 + 3 * 5 + 7) * 5 * 9 + (5 * 2 + (2 * 7 + 8 + 3)) + 5",b"(3 + 8 * 4 * 3 * 5 + 9) + ((8 + 3 + 9 + 3) * 8 * 5 * 5 * 3) * 4 + 9 * 6",b"4 * 3 + 2 * (9 * 6 * 3 + (9 + 3 + 4 + 8 * 4 * 4) + 3 + 6) * ((2 * 9) + 7) + 5",b"(6 * 9) * 2",b"8 * (5 * (7 * 4 * 5 * 3 + 4))",b"2 * 9 + 7 + (8 + 5 * 8 * 8)",b"8 + 5 + (2 + 3 * (7 * 8 + 3)) * 4 + 8 + 6",b"(6 * 4 + (3 * 6 + 7) + 9 + 5) + 4 + 4 + (5 * (9 * 7)) + 7 * 6",b"6 * (7 * 9 + 3 * 3 + 4) * 4 * 4 + 7 * 6",b"(2 * 5 + (5 + 5 * 9 * 3 * 4 * 9) * 7) * 8 + 9 * (6 + (9 * 4 + 3 + 2 + 9 * 3) * 4)",b"(6 + 6 * 6 + 6) + 3 + 6",b"3 + 5 * 6 + 8 * ((5 * 5 + 8 * 4) * 9 + 7 + 6)",b"8 * 9 + 5",b"4 * 3 + (8 * 2 + (3 + 5 * 4 + 9))",b"(4 + 9) * 3 * 3 + (7 * (6 + 5 + 7 + 9) * 4 * 2 * 2 + (9 * 4 + 7 + 4 + 7 * 3))",b"(2 + 8 * 6 + 2 + 8 * 6) * 2 + 7 * 5 + 3",b"8 + 3 + 4 + 7 * (5 + 8 + 3 * 8 * 2 * 3)",b"(2 * 3 + (2 * 6 * 2 + 2 + 8 + 2)) * 9 + (2 + 9) + 4 + 3 + 9",b"9 + 4 * 4 * ((4 * 6 + 2 + 8 * 7) * (3 + 8 + 6 + 3) * 3 * 3) + 4 + 6",b"(7 + 8 * 9 + 5) + (4 * 5 * 4 * 9 * 5) * 9",b"7 * (2 * 5 + 9) * 6 * 3 * 3",b"(9 * (4 * 9 * 9)) + (5 + (7 * 9 * 2 * 7 + 4) + (3 * 8 * 2 * 2 * 6 * 4) * 6 + (3 + 3 * 2 + 2)) + 2 + (5 * (2 * 5 + 3 * 3 * 4 * 7) * (8 * 4 + 7 + 8 * 5 * 2) * 3)",b"9 + ((2 * 6) * 3) + (9 * 2 * 8 * (7 + 3 * 5 * 3 * 7 * 8) * 7) + 4 * 4",b"6 + 9 * (2 + 7 + (7 + 5 + 2) * 5)",b"(7 * 2 * 4 + 8 * 2) + 8 * 4 + (7 * 3 * 2 * (5 + 8 + 6) * 3) * 6",b"5 + 9 + (4 * 4 * 3 * 5 + 9)",b"6 * 2 * 3",b"((6 + 5) + (8 * 7 + 3 + 6) + 5 + (3 + 2 + 9) + 9) * 6",b"7 * 9 + 2 + 7 + (8 * 9 * 9 + 2)",b"(8 + (7 * 4 * 2 + 5) * 8) * 3 + 8 + 2 + 9",b"((2 * 6 * 3) * (9 * 3) + (3 + 4 + 9) * 2 + (3 * 9 + 9 * 9 * 6 + 3) * (4 * 6 * 5 + 8 + 2)) * ((5 + 4 * 8) * 2 * 5) * 4",b"4 + (3 + 9 + 2 * 5 * 9) + 8 + (8 * 4 * (8 + 2 + 7 + 7 + 8 * 9) * 6 + 8 + 8) * 5",b"(4 * (7 + 5 + 8 * 4 * 4)) + 8 * 9 + 9",b"(7 + 2 * (7 * 4) * (7 * 4 + 6) + 5) + 3",b"9 * 4 * 5 + 3 + 9 + 5",b"4 * (5 + (4 * 8 + 4 * 5) * 6) + 4",b"6 + (7 + (7 * 8 * 3 * 3 * 9 * 3) + (7 + 9 * 7 * 9 + 3 * 6) * 4 + 3 + 4) + 6 + ((4 + 3 * 4 * 7 + 7) * 6 + 5) * 9",b"9 * 9 * 7 * (8 * (6 * 6 + 2 * 8 * 8 + 8) * 4 * 7 + 4)",b"(2 + 5 * 6 * (3 + 9)) * (3 + (7 + 9)) * 3 * 2",b"8 + 8 + 5 * (4 * (3 * 3) + 9 + 6 + 7 + 6) + ((5 * 4 + 7 + 5) + (6 * 9 + 9) + 5 * (9 + 9 + 2 * 4 + 6 * 4) + (4 + 5 + 7 + 8 + 6)) + 6",b"8 + (4 * (3 + 8) * 7) + 6",b"6 + 9",b"4 * (8 + 4 * 5 * 6) * 6 + 2 * 3",b"6 * 7 * 8 + 9 * 7",b"((9 * 5 * 2) + 9 * (7 + 6) + 4) + 5 + 3 + 3",b"9 + 4 * (5 + (8 * 3 + 5 * 9 + 9 * 5) * 2) + (3 + (5 + 4 * 3) * (5 + 2) + (7 * 9)) * 3",b"2 + (9 + 9 + 5 + 6 + (2 + 9 * 9) * 6) * (9 + 4 + 3 * 7 * 7 + 4)",b"3 + (8 * 9 * 6 * 2 + (2 + 9 * 3 + 9) * 5) + 7 * 6 + (8 * 5 + (8 + 3))",b"(6 * (3 + 5) * 2) + 6 * (4 * 7 * 2 + 2 + 8 * 3) + 8 + 4 * 9",b"3 * 9 * ((9 + 3 + 6 * 7 + 8) * 2 * (9 * 6 * 6 + 5 + 9 * 2)) * 6 * 2 + 3",b"9 * (6 + 8 + 6 * 6 * 2) + 5 * 3",b"(3 + 8 + 7 * 8 + 6 + (3 + 2)) * 4 + 2 + 4",b"((7 + 7 * 4 + 5) * 9 * 9 * 8) + 9 * 8 + 7",b"3 + ((4 * 9 + 4 * 5 * 2 * 7) + 7 + 2 + 8 * 2) + 5 * 6 + 8 + 5",b"(4 * 6 + 4 + 5) + 7 * ((6 + 9 * 2 + 8 * 3 * 4) * 9 * 9 + 4) + 3 + 3",b"6 + (9 * 7 + 2 * (7 + 6 + 6 + 4 * 8 * 9) + 3 * 3) * 2 + 8 * (4 * 9)",b"9 + 4 + 6 + ((6 + 8 * 2 * 7 * 2) * 8) + 8",b"(6 + 5) + 2 + 4",b"(4 * (2 + 5 * 9) + 4) + 4 + 2 + 3 * (5 + 8 + (9 + 9 + 4)) * 6",b"(7 + 8 * (9 + 8 + 8)) + 5 * 7 + 9",b"5 + 6 + (9 * 6 * (3 * 4 * 5 * 2) + (5 * 4 + 4 * 2 + 6 + 5)) + 9",b"((2 + 4) * 5) * 5 * 3 + 3 + 2",b"8 + (8 + 9 + (9 * 5 * 4)) * 8 * (6 + 8 * 2 + 6 * (3 + 6 * 9 + 2)) + 8 * (5 + (8 + 3 + 6 * 3))",b"6 * (2 * 7 * 5) + 8",b"4 * 5 + 8 * (7 + 4)",b"(7 + 4 + 8 + 4 * 4) * 8 * 5 + ((7 * 6 * 4) + 5 * 6)",b"7 * ((7 * 3 + 6 * 6 * 2 * 3) * 4 + 8 * 5 + 5) + (9 * 3 + 2 + 2 + 3 + 4) + (7 + 2 + (8 * 8 + 6 + 7 + 7)) * ((5 + 6 * 4 * 3 + 7 + 5) + 2 * 5 + 4 + 5) * 4",b"4 * (9 * 7 + (7 * 7 + 7 + 3) * 8)",b"3 + (8 * 8 * 9 + 5 * (3 * 2 + 2 + 2 * 2 + 8)) + (5 * (8 * 4) * 2 + 7 * (7 * 4 + 7 + 2 * 4 * 3) + 6) + 9 * 2 + (4 * (2 + 9 * 6 + 7 + 5 + 7) * 2 + (7 * 5 * 5 + 6 + 6) + 4 * (8 + 5))",b"((6 * 9 + 3 * 3 + 8) * (3 + 9) * 2 * 2 + 3 * 8) * 3",b"2 + 5 + (3 + 4 + 8 * 8) * 4 + 6 + 4",b"(6 * 6) + 9 * ((2 * 2 + 4) + 6 + 9 * 8 * (7 * 4 * 3 * 8 * 3)) * 7 * 2 + 3",b"9 * 4 + 2 * (3 * 6) * 8 + 3",b"(9 + 7) * 2 * 7 * 3",b"(8 * 7 + 2 + 8 * 3) * (3 * 7 * 2 * 2) + 4 * 3 * 3 + 6",b"(2 + 7 + 7) * 9 + 7 * 7",b"4 * (9 * 4 + 8 + 9 * 9) * 3 + (3 + 2 + (4 * 2 + 2 + 4 + 9 * 8) * 4) * 9 * 9",b"4 * 6 * ((4 * 7 + 4) * (2 + 8 + 7 * 5 + 2 * 7)) + 2 + 3 * 5",b"2 + 2 + 6 + (4 + (7 * 3)) + 5",b"(9 * (3 + 8 * 3 + 9) + (8 + 4 * 2 * 8 * 2 * 6) * 9 + (5 * 7 * 7) + 5) * 3 + 3",b"3 * 8 + (6 * 6 * 7 + 3 + 5 + (2 * 7 * 4 + 2 * 3 + 4)) + 8 + 7",b"7 * 7 + (6 * (3 * 4 * 4 + 7 + 4) * 7 * 7 + (7 + 4 * 4 + 6 * 9 * 7)) * 2 + 8",b"7 + 3 + 4 + (2 * 4 + 9 + 6 + 8) * 5 + 5",b"((5 * 7 + 4 + 8 + 8 * 5) + 3 * (3 + 3 + 5 * 9 + 2) * 6) + 2",b"6 * (8 * 4 + 7 + 7 * 8) * (5 * (2 + 2 + 4 + 8 + 6)) + 6 * 5",b"3 + 7 + 7 * (3 + 9 * 6 * 6) + 4 * 5",b"3 * ((4 + 3 * 9) * 4 + 2 * 5 + 6 + (5 + 2 + 2 + 4 + 8)) * 6 * 9 + 3 * (9 + 5 + (9 + 9 + 6 * 6 + 6 + 2) * 3)",b"(8 + 5 * 6 * 4 + 2 * 4) + (2 + 5 * (9 + 3 * 8 * 8) + 8 * 2 * 4) * (3 + 7 * (8 * 4 * 4 * 8)) * 9 + 8",b"6 * 6 + (6 + 3 + (8 * 8 * 8 + 5 + 3 * 6) + 7)",b"3 + 7 + ((3 * 6 + 4 + 9 * 3) * 9 * 7 + 2) + (8 + 2 * 3 * 8 * 7) * 6 + 5",b"2 + ((7 * 7 * 3) + (4 + 8 + 3 + 5) + 3 + 7 * 9 + 7)",b"9 * 9 * 7 + ((8 + 7 * 8 * 8 + 6 + 8) + 3 + 2 + 6 + 9)",b"3 + ((3 + 8 * 5) + 7 * 2) + (2 + 9)",b"((7 * 2 * 3 * 3 * 9) * 6 * 2 * (3 * 4) + 5) * 4 + 6 + 6 + 2",b"6 * 3 * 2 + 2 + 4 * 9",b"4 * 4 * ((2 * 4 * 4 * 2 + 9 + 7) + 9 + 8 + 5) * 9",b"6 + 4 * 4 * (7 + 6 + (3 * 7 * 6)) + 5",b"9 + 3 * ((7 * 9 * 7 * 7 + 5 * 4) * 6 * 3 + 3) * 7 * 5",b"3 + (8 + 6 + (7 * 3 * 9 + 4) + 9 + (3 + 8 + 9 * 8 + 5 * 6) + 8) + ((9 * 2 + 3 * 9 + 6) * 5 * 2 * 7 + 2 + 3) * (6 + (5 * 6 * 5 * 5 * 2) + 4 + 4 + 9) * 7 + (9 * (5 + 3 + 3 * 4 * 7) * 7 * 9 * 8 + 4)",b"2 * 4 * 3 + (4 + 7 * 4)",b"(4 * (6 + 4)) * 2 * 9",b"((8 + 8 + 2 + 4) + 5 * 8 + 2 + 5) * (5 + 9 * 6 + 5 + 9) * 8 * 4 + 7",b"2 + (7 * 6 * 7 * 4 * 2)",b"((7 * 5 + 4 * 8) * 5) + 4 + 7 * 8 * 8 + 3",b"4 + ((6 + 6 + 5 + 8) * 5 + 5 * 2)",b"(9 + 6 * (6 * 6 + 9 + 5) + 4) * 3 + ((6 + 5 * 2 * 7) * 2)",b"(3 * (6 * 9 + 9) * 9 * 3) + 2 + 7 + 5",b"5 * ((6 + 9 + 6) + 7)",b"2 + 5 + ((9 + 3 + 6 + 4 + 2 + 7) * 7 * 8 * 4 + 8 * 6) * 2 * 2",b"((4 * 7) + 9 + 7 + 8 + 3) + 6 * 5",b"(3 + 6 + 7 + (7 * 5)) * 6 + 8 * 9",b"9 + 5 * 6 * 9 + (8 + 6 * 6)",b"(7 * (9 + 8 * 4 * 7 + 3) + 8 * 3 * 3 * (6 * 8 * 8 * 3 + 9)) + 8 * 5 + 6 * 6",b"3 * 4 * ((9 + 2) * (7 * 4 * 9 + 5 + 7)) + (6 + 7 + 8) + 2 * 9",b"4 + 4 + 4 + 3",b"6 + 6 * 6 * ((5 + 4 + 5 * 5 + 6 * 9) * (6 * 3 * 9) + 7 + (2 * 4 + 3 * 4) + 6) + 9 + 3",b"4 * 5 + 5 * (8 + 4 * 7 + (3 * 6 + 3 * 6 + 5 + 9) * (4 + 5 + 6 * 7 + 7 * 8)) * 3",b"4 * (5 + (2 + 8 + 8 * 7 * 3 * 9) * 4 + 4 * 4) + 3",b"5 + (3 + (9 + 5 * 7) + 9) + 2",b"3 * (4 + 9)",b"9 + 5 * 7 + (7 + 5 + 8 * 3 * 7) + 8 + (2 + 3 * 9 * 4 + 3)",b"8 + (9 + 8) + 5 * 2 + 9 * ((4 * 8 + 5) * 3 * (8 + 3 + 6 + 5 + 7 * 4) * 5)",b"7 * 8 * 9 + 9 * (4 + (7 * 9 * 7 + 9))",b"7 * ((5 * 6 * 4 * 2) * 9 * 7 + 3) + 3",b"2 + 9 * 4",b"2 * 2 + (5 + 2 + 4 * 5 + 5) + 5 * 5",b"4 * 8 + 9 * (7 * 9 + 9 + 8) + 8",b"9 + ((6 + 6 + 2 * 2 * 9 * 9) * 8 * 9 * 4 * 6)",b"((2 + 8 + 9 * 4 + 7) * 7 * 5 + 3 + 9) * 4 * 3 + 7",b"(8 * 8 + (5 + 3 + 9 * 9)) + ((7 + 5 + 5) + 2 + 7) * (9 + 4 + 4) + 8 + 4 * 8",b"(9 * 6 * 3) * 3 * (6 * 2 + (3 + 4 + 2) * (7 + 9 * 5 + 5)) + 8 + 6",b"(8 + (3 + 8 * 3) * 3 * 6 + 5) + 5 + 7",b"((9 + 8) + (6 + 2) * 7) * 2 + 6 * 4 * 2",b"3 + 6 + (3 + 2 + 2 + 6 * 5 + 3)",b"(6 + 4 + (2 + 3 * 4 + 3) + 4 + (3 * 2 + 5) * 8) + 9",b"(3 + 3 + (9 * 4) * 2) + 5 * 7 * 3 * (8 + 4) + 6",b"(3 + 7) + 7 * (4 + 7 * (9 + 2 * 7) + 8 * 5 * 8) * 3",b"((2 * 4 * 5 * 4) + 3 + 3 * 7 + (8 * 2 + 6)) + 8",b"(5 + 6 + 9 + 8 + 8 + 3) * 2 + 6",b"4 + 6 + 8 + (9 * 8 * 2 + 6 * 4 + 3) * 5",b"(2 * 2 * 5 * 7 + 4) * 6 + 7 + 5 * (9 + 7)",b"5 * 9 + (7 * 4 + (9 * 7) * 5 * 7) + 5",b"(3 * 6 + 8) + 7 + 8 + 3 + 4",b"4 + 3 * 3 * 3 + 7 * (5 + 9 + 6 + 5 + 6)",b"3 + 9 + 8 * 4 + (6 * 7 * 3 + 7 * 5)",b"8 * 3 * (9 * (4 * 8 * 7 * 9 + 3 * 9) * 2 + 5 * 7) * 4 * 4",b"5 * 7 * (4 * (9 * 7 * 7 + 5) * (7 + 5 * 9 * 6 + 8) + 4 * 6 * (4 + 6 + 6)) + 3",b"4 + ((4 + 2) + 7 * (3 * 4 + 4 * 9 + 6 + 8)) + 7 * 4 + 5 * 7",b"((6 + 2 * 6) * 2) + (5 * 9 + 9 + 7) + 7 * 4 * 2",b"4 + 7 * 8 * 7 + (3 * 7 + (4 * 6 * 5 * 2)) * 5",b"6 + (9 + 6 * 4 + 6 * 6 + 3) * 5 * 2",b"8 * 3 * 7 * (6 + 3 + (7 * 6 * 5 + 9 + 4) + 6 * (2 * 7 + 5 + 6) + 3) * (6 * 4 + 2 * 9 * 7 + 6)",b"2 * (5 * (9 + 6)) * (4 + (5 * 2 + 3) + 7 * 6 * (8 * 4 * 4) * 4) * 5",b"2 * 2 * (3 + 8 + (4 * 3) + (6 + 5 + 6 + 8 * 5 * 4) + 8) + 3 * 3 + 8",b"4 * (8 * 4 * (3 * 6) + (5 + 3 + 4 * 6 + 4 + 7) * 4) + ((8 * 2) * (2 * 7) + (9 + 7 * 2 + 2))",b"3 * 4 + (7 * 8) + (5 + 7 * 8 + 9) * 9",b"(9 * 4) * 5 * 3 * 9 + 5 + 7",b"(7 * (9 * 4 + 9 + 5) * (2 + 3 * 3 + 8 + 2)) * 5",b"((9 * 2 * 9 + 8 + 8 * 3) * 5 * 7 + (2 * 2 * 3 * 2) + 6) + (7 + 8 * (4 + 2 * 9 * 6 + 3 * 9) * 4 * 2) + ((4 + 9 * 9 * 5 * 5) + 2 + 8 * 9 * 2) + 9 * 6",b"9 + 7 + 9 * (9 + 4 * 5) * 7",b"7 + 8 + ((2 + 3) * 4) * 6",b"6 * 4",b"(9 + 4 * (6 * 2 + 2 + 7 * 6) * 4) * ((6 + 8 + 7 + 4 + 9) + (7 + 9 + 8 + 6)) + (2 * (3 + 8) + 7 * (4 + 3) * 5)",b"(9 + (2 + 4 * 3 + 3 + 2 + 2) + 4 + (2 + 3) + 3) * ((6 + 6 + 7 * 6) * 2 + 3 + 9 * 4 + 8) * 2 * 2 + (9 + (5 * 6 * 8 * 3))",b"(8 * (4 + 9 * 3 + 6 + 6) * 7 * 6) * 5 + 4 + 8 * 8",b"3 * (8 * (4 * 2 + 6 * 6) * 5 + (9 * 6 * 3 + 4 * 4) + (5 * 2 + 7 + 5 + 5))",b"3 * ((6 + 7 + 7 * 9 + 4) * 6 + 8) + 8 + ((7 * 9 + 5) + 5 + 6) * (5 * 6 + 2 * 6 * 6 + 7)",b"(3 + (8 + 9) * 5 * 6 + 8) * (2 + 6 + (7 + 4 + 6) * 4 + 3) + 6 + 2 * (7 + 8 * 5 + 7)",b"5 * (2 + 2 * 6 * (6 + 2 * 4 + 8) + 4 * 5)",b"4 * 9 * 7 + 7 * 3",b"((7 + 7 * 6) * (5 * 9 * 6)) * 5 + 5 + 3 + 8 + 4",b"(4 + (5 + 7 + 7 + 7)) * 4 * (7 * 7 + 3) * 6 * 3 * ((4 * 7) + 4 * 9)",b"2 * 2 + 6 * ((8 * 9 + 7 * 7) * (5 * 9 * 6 * 6 * 7 + 6) + 5 * 4) + (8 * 7 + 5 * 5)",b"9 + 6 * ((9 + 5 + 2 + 7 + 3) * 3 * 6) + 9 + 7 * 9",b"4 * (8 * (4 + 6 * 4 + 8 + 2 + 3) * 5 + 8 * 5) * ((5 + 4 + 2 + 3 * 2 + 9) * (4 * 8 + 9 + 4 * 4 + 4)) + 2",b"9 + (3 * 3 + 4 + 5 + 6) + 3 + (5 + 7) + (3 + 5)",b"((3 * 3) + 9 + 4 + 8 + 9 + 2) * (7 * (8 + 8 * 7) * 4 + (4 * 4 + 2 * 5 * 2) * 9)",b"2 + (7 + (6 * 2 * 4 + 2 * 9 * 2) * 4 * 5) + (4 + 5)",b"2 + (4 + 3 * 5 * 3 + 3) + 5",b"6 * 6 + 4 + 8 + 4",b"((3 + 4 + 2 + 6) + 8) + 2",b"(4 + 2 * 4 + 6 + 8 * 3) + (3 * 8 + 9 + 7 * 5 + 2) + 7 * 6 * (6 * 3 * 5 * 6) * 6",b"5 * 6 + (8 * (6 + 6 * 4 + 2) * (2 + 5 + 4 * 7 + 3 * 6) + 8 + 4) * 6 * 6",b"3 * 6 * 5 * (3 + 4 + 3) * 3 * 5",b"9 * (4 * (3 * 9 * 4 * 9 + 6 * 7) * 7 + 7 * 7)",b"7 * 4 + 2 + (7 + (3 * 4) * 9 * 8) * 2",b"(3 * (9 * 3 + 4) * 3) + 2 + ((3 + 7 + 6 + 4 * 6 + 9) + (3 * 7)) * 8",b"(4 + 3 + 6 * 7) * (7 * 7 * (8 * 4 + 2 * 8 * 2 + 3) + 4 + 8 * 3) * 8 + 2",b"3 + (9 + 2 * 3)",b"7 * 7 * 5 + (2 * (3 + 8 + 2) + 7)",b"4 * (7 + 7 * 9) * ((8 + 2 * 4 * 6 + 4 + 9) * 5 * 6 + 3 + 3) + (4 + 4 * 2 * (4 + 4 + 3 + 3 + 7) + 2)",b"2 * 2 + 2 * 9 + 9",b"9 * 4 + (2 * 5) + 8 * 5",b"(8 + 9 * (2 * 2 + 3 * 9 * 3 + 3) * 8 * 8) * (9 * 4 * 6 * 7)",b"7 * (6 * 8 * 8 * (3 * 2 + 4 * 7 + 9 + 6) + (4 + 9 * 3 * 3 + 7 + 3)) + (4 * 3) * 5 * 8 * 4",b"4 * 8",b"7 * 2 * (4 + 2 * 7) + ((3 * 2 + 2 * 9 * 6 + 7) + 8 * 6 * 6 + 9) * 7 + 5",b"(4 * (2 + 4 * 4 + 4) * (8 + 7)) * 7 * 8 * 9 * 9",b"9 + 5 * 3 * (8 + (9 * 5 + 4 * 3) * 7 * 7 * (6 + 4 * 5 + 7 * 2 + 5)) + 3",b"(4 * 4 * 9 * 8 + (7 * 2 + 9 * 6 + 4)) + 5",b"(8 * 4 * 8) + 9 * (7 * 5) * 5",b"(3 * 4 * 9 * 9 + (5 * 8 * 5 + 4 + 7) + (7 * 8 * 5 + 9 + 5 + 5)) + 2",b"(8 * 3 + 2 + 9 + 2 + 2) + 3",b"8 * (9 + 7) + 7 * 2 * (8 + 3 + 5 + 7 + 6) + 2",b"2 * (2 * 6 + 8 + (7 + 6 + 2) + 3 * 6) * (5 + 8 + 7 + (9 + 9 * 3 * 7)) * 7 + 4",b"(5 + 5 + 8 * (5 + 2 + 3)) * 9 * 9 + ((2 * 6 * 6 * 9) * 5 + 4 * 6) + 5",b"5 + 3 * 4 * (3 + 4 + 8) * (3 * 9 + 9 + 7 * 8)",b"6 * 4 + ((7 + 3 * 3 + 3) + (3 * 7) * 3 + 9 * 2)",b"(5 * 3 * 7 * 2 + 6) * (2 * 6) + 6",b"7 * 4 + (6 + 2 * 3 * 9 + (8 + 9) * (5 + 2 + 2))",b"2 + 2 + ((9 * 2 * 4 + 8 + 3 * 7) * 4 + (2 + 6 + 5 * 5 * 2 + 8) + 5 * 2 * 8) + 4",b"((2 + 3 + 2 * 8 * 4 * 6) * 4 * 2 * 8 * 5 + 5) + 8 * 9",b"5 + 4 * (5 + (4 * 4 * 8) * 3 + 3)",b"4 * 3 + (7 * 4) + 8",b"4 + (9 * 8 * 5 * 4) + 2 + 4",b"4 + (7 + 9 * 6) * 5 + 4 + 4 + 6",b"(2 + 9 * 3) * 2 * (7 + (5 + 8 * 7) + 8)",b"4 * 3 + 6 + 3 + ((5 + 6 + 7) * 5 * 2)",b"9 + 8 * (4 + 3) * 4 + 6 * 9",b"4 * (6 + 5 * 5 * 8 * 6 * 6)",b"(8 + 7 * 4 + 4 + (2 + 8 * 2 + 4 + 6 + 2)) + 3",b"2 + (9 * 4) * 7",b"4 + 7",b"(9 * (2 * 2 + 4) + 3 * 3 + (7 * 4 * 6 * 4 + 5 + 4) * (8 * 8 * 7 + 8)) * 6 + 3 + 7 + (6 * 5 * 8 * 6) + 2",b"2 + 6 + (4 + 2 * 7 + 3 * 9 * 9) * ((8 + 2 * 4 * 4 * 2 + 3) + (3 + 9 + 4) * 3 * 4) + 8",b"8 + (6 * 5 + 2 * 6 * (2 * 3 + 2 * 9 * 7 * 2) + 6) * 2 * 8 + 7 + (6 * 5 * 9 * 6)",b"6 + (3 + 9 + 3 * 6)",b"(2 + 5 * 9 * 2) * 2 * 5 + ((4 * 2 * 7 + 8 + 7) + 7 + 6 * 4 * 3 * 6)",b"(7 + 3 * 6 * (2 * 7) + 7 * 8) * 7 * 2 + 4 + 5 * 2",b"(6 * 2 * 5) + (7 + 7) * 9 * 5 * 2 * 5",b"3 * (8 + 3 * 3 * 2) + 5 + (9 * (5 + 8 * 8 * 5 + 8) * 4 + 7 * 8 + 3)"];

fn find_matching_par(s: &[u8]) -> usize {
  let mut d = 0;
  for j in 0..s.len() {
    match s[j] {
      b'(' => d += 1,
      b')' => d -= 1,
      _ => {}
    }
    if d == 0 {
      return j;
    }
  }
  unreachable!()
}

fn eval_p1(s: &[u8]) -> usize {
  let (mut val, mut i) = (0,0);
  let mut op = '+';
  while i < s.len() {
    match s[i] as char {
      '+' => op = '+',
      '*' => op = '*',
      '0'..='9' => match op {
        '+' => val += (s[i] - b'0') as usize,
        '*' => val *= (s[i] - b'0') as usize,
        _ => unreachable!()
      },
      '(' => {
        let end = i + find_matching_par(&s[i..]);
        let v = eval_p1(&s[(i+1)..end]);
        match op {
          '+' => val += v,
          '*' => val *= v,
          _ => unreachable!()
        }
        i = end;
      }
      _ => unreachable!()
    }
    i += 2;
  }
  val
}

fn eval_term(s: &[u8]) -> (usize, usize) {
  if s[0] == b'(' {
    let j = find_matching_par(s);
    (eval_p2(&s[1..j]), j)
  } else {
    ((s[0] - b'0') as usize, 0)
  }
}

fn eval_p2(s: &[u8]) -> usize {
  let (mut val, mut i) = (1,0);
  while i < s.len() {
    let (mut v, step) = eval_term(&s[i..]);
    i += step;

    // eagerly perform all add operations!
    while let Some(b'+') = s.get(i+2) {
      let (tmp, step) = eval_term(&s[(i+4)..]);
      v += tmp;
      i += step + 4;
    }

    val *= v;
    i += 4;
  }
  val
}

aoc2020::main! {
  let p1 = INPUT.iter().map(|s| eval_p1(s)).sum::<usize>();
  let p2 = INPUT.iter().map(|s| eval_p2(s)).sum::<usize>();
  (p1,p2)
}