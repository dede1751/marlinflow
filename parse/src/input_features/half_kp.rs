use cozy_chess::{Board, Color, Piece};

use crate::batch::EntryFeatureWriter;

use super::InputFeatureSet;

pub struct HalfKp;

impl InputFeatureSet for HalfKp {
    const MAX_FEATURES: usize = 30;

    fn add_features(board: Board, mut entry: EntryFeatureWriter) {
        let stm = board.side_to_move();

        let white = board.colors(Color::White);
        let black = board.colors(Color::Black);

        let pawns = board.pieces(Piece::Pawn);
        let knights = board.pieces(Piece::Knight);
        let bishops = board.pieces(Piece::Bishop);
        let rooks = board.pieces(Piece::Rook);
        let queens = board.pieces(Piece::Queen);

        let w_king = board.king(Color::White);
        let b_king = board.king(Color::Black).flip_rank();
        let (stm_king, nstm_king) = match stm {
            Color::White => (w_king, b_king),
            Color::Black => (b_king, w_king),
        };

        let array = [
            (white & pawns),
            (white & knights),
            (white & bishops),
            (white & rooks),
            (white & queens),
            (black & pawns),
            (black & knights),
            (black & bishops),
            (black & rooks),
            (black & queens),
        ];
        for (index, &pieces) in array.iter().enumerate() {
            for sq in pieces {
                let (stm_index, stm_sq, nstm_index, nstm_sq) = match stm {
                    Color::White => (index, sq as usize, ((index + 5) % 10), sq as usize ^ 56),
                    Color::Black => (((index + 5) % 10), sq as usize ^ 56, index, sq as usize),
                };
                let stm_feature = (stm_king as usize * 640 + stm_index * 64 + stm_sq) as i64;
                let nstm_feature = (nstm_king as usize * 640 + nstm_index * 64 + nstm_sq) as i64;
                entry.add_feature(stm_feature, nstm_feature);
            }
        }
    }
}