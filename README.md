# RustyRook-
Chess Engine written in Rust

pre go live todos:

- minimax
- alpha beta prunning
- dont calculate douplicate positions?
- Order of Moves when Calculating!

v2:

- improve calculation/evaluation performance!
- only calculate pins when rook/queen/bishop left
- undo move for performance?
- move "has_moved" to board (board tracking possible castles)
- Rayon


Engines:

v1 Problems:
- Pawn Structure not valued
- Rooks on open Lines?
- Active King in Early Endgame
- Throws Pawns at opponent without value
- Field Value for each side individually


Known Issues:

- Input String Castle Rights are not read
- position startpos moves e2e4 d7d5 e4d5 c8g4 d1g4 resulting in capture of own king
- 5rkR/r1p1qb2/p4p2/1p1pp3/3P4/1PP1PNP1/1PQ2PP1/2K4R b - - 1 20 
---> Own m8 resulting in np

todos:
- Refactor Queen, Rook and Bishop and make them use same Function




