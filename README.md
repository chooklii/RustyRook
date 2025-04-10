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


Known Issues:

- Input String Castle Rights are not read
- position startpos moves e2e4 d7d5 e4d5 c8g4 d1g4 resulting in capture of own king

todos:
- Refactor Queen, Rook and Bishop and make them use same Function




