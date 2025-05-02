# RustyRook-
Chess Engine written in Rust

pre go live todos:

- Order of Moves when Calculating!

-> Zugwiederholung -> Resolve with Trans Table Position Unique Key

v2:
- improve calculation/evaluation performance!
- only calculate pins when rook/queen/bishop left
- undo move for performance?
- Rayon

todo:

Refactor Fields to prevent check to magic bitboards
Refacotor Pinned Pieces
Maybe Refactor Pawn EnPassant, but does not effect performance by much

f32 Evaluation -> u64???

Problem:
Order Moves
-> Go by Depth