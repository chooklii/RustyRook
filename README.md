# RustyRook-
Chess Engine written in Rust

Known Issues:
-> Repetition does not work as intended in some cases

todos: 
-> Makes moves incremently by depth and use time given by user
-> Test Move Ordering by Transposition Table 
---> Own Takes where Opponent Cannot Take > Takes > Silent Moves
-> Test and Improve Evaluation
-> Change en-passant to be the field the pawn moves to!

todos later:
-> Refactor Fields to prevent check to magic bitboards
-> Refacotor Pinned Pieces
-> Maybe Refactor Pawn EnPassant, but does not effect performance by much
-> Test Rayon



Check for Check first
-> Calculate Pinned Pieces Prior to Own Move Generation