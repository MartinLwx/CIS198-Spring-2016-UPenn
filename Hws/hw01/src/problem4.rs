/// #[derive(...)] statements define certain properties on the enum for you for
/// free (printing, equality testing, the ability to copy values). More on this
/// when we cover Enums in detail.

/// You can use any of the variants of the `Peg` enum by writing `Peg::B`, etc.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Peg {
    A,
    B,
    C,
}

/// A move between two pegs: (source, destination).
pub type Move = (Peg, Peg);

/// Solves for the sequence of moves required to move all discs from `src` to
/// `dst`.
pub fn hanoi(num_discs: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {
    let mut current_moves = Vec::new();
    if num_discs == 1 {
        current_moves.push((src, dst));
        return current_moves;
    }
    current_moves.extend(&hanoi(num_discs - 1, src, dst, aux));
    current_moves.push((src, dst));
    current_moves.extend(&hanoi(num_discs - 1, aux, src, dst));
    current_moves
}