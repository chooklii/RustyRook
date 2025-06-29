# RustyRook
Chess Engine written in Rust

This Engine has no UI - It implements the UCI Chess Protocol. Use the [Lichess Bot](https://lichess.org/@/RustyRookChessBot) or any other Chess Software to play against it. 

Read More about RustyRook in my [Blog](https://bzenth.de/blog/rustyrook)


use 
```
cargo build 
or
cargo build --release
```

to create an executable to use with your local chess software. 

You need to copy the 'openings.txt' file to the folder you are running the engine in. If you dont want to use an opening book just create 
an empty txt file with this name (Config to use no Openings Book will be implemented in the future)

To verify the correctness of changes run the Unit-Tests

```
cargo test
```

Advanced Unit-Tests based on [Chess Programming Wiki](https://www.chessprogramming.org/Perft_Results) to verify the correctness of the move generation can be used with

```
cargo test -- --ignored
```

## Chess Programming
Rusty Rook implements Basic Chess Engine Algorithms like

- Minimax
- Alpha Beta Prunning
- Magic Bitboards
- Parallel Iterative Deepening with prev. best Move sequentially
- Transpositional Table
- Some Move Ordering

___

Possible Improvements for the Future

- Refactor Pinned Pieces
- Refactor En-Passant (Does not effect performance)
- Improve Move-Ordering
- Add more Stuff like Killer Moves & Null Moves
- Improve convertion from UCI-Position to Position usize (a1 -> 0)
