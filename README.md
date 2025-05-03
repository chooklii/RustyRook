# RustyRook-
Chess Engine written in Rust

todos: 
-> Test Undo Moves for Performance
-> Makes moves incremently by depth and use time given by user
-> Test Move Ordering by Transposition Table 
---> For Board get best Move and make it first
---> Own Takes where Opponent Cannot Take > Takes > Silent Moves
-> Test Move Ordering by Takes first
-> Repetition
-> Test and Improve Evaluation
-> Change en-passant to be the field the pawn moves to!

todos later:
-> Refactor Fields to prevent check to magic bitboards
-> Refacotor Pinned Pieces
-> Maybe Refactor Pawn EnPassant, but does not effect performance by much
-> Test Rayon



Check for Check first
-> If Double Check - Only King Moves
-> Calculate Pinned Pieces Prior to Own Move Generation