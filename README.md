# RustyRook-
Chess Engine written in Rust

Known Issues:
-> Repetition does not work as intended in some cases

todos: 
-> Makes moves incremently by depth and use time given by user
-> Test and Improve Evaluation
-> Make prev. best Move sequential to have baseline alpha/beta


todos maybe later:
-> Refactor Fields of prevent check to magic bitboards
-> Refacotor Pinned Pieces
-> Maybe Refactor Pawn EnPassant, but does not effect performance by much
--> to field figure can move to to en passant
-> Test Rayon
-> Test Move Ordering by Transposition Table 
---> Own Takes where Opponent Cannot Take > Takes > Silent Moves
---> Save Top 3 Moves in Trans. Table
-> Killer Moves & Null Moves
-> Negamax