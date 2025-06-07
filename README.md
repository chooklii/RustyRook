# RustyRook-
Chess Engine written in Rust

Known Issues:
-> Black not working as intended (initial alpha/beta wrong)
---> Due to transposition table being in the perspective of white -> change to negamax

todos: 
-> Test and Improve Evaluation
-> Cleanup Engine 


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