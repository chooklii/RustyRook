use rustc_hash::FxHashMap;

use crate::{
    board::bitboard::Bitboard,
    helper::moves_by_field::{
        get_bishop_blockers_for_field, get_moves_for_each_field, get_rook_blockers_for_field,
        MoveInEveryDirection,
    },
    BISHOP_MAGIC_BITBOARDS, ROOK_MAGIC_BITBOARDS,
};

use super::{
    helper::{get_magic_index, get_valid_moves_for_position_with_given_blockers},
    magic_bitboard::MagicBitboard,
};

// precalculated magics from find to prevent needing to find them on every startup
const ROOK_MAGIC_NUMBERS: [u64; 64] = [
    4647719420192360576,
    90073368279130112,
    2377909743478964288,
    9259425091857418240,
    9295448322724602922,
    2377906103292789288,
    7061670750025449732,
    72075324786999426,
    4644372549247105,
    2307039287796834305,
    703824889122816,
    11529355817924988929,
    140754676615168,
    562984607024640,
    577586686637195792,
    117234349274843264,
    9167178198695936,
    4508272587442368,
    27024346811736192,
    24912185377689600,
    2311504394870327296,
    2486269568833290376,
    2882484081558438224,
    434669931825774724,
    2594425231233787008,
    9223935132841345158,
    9223935545155002436,
    90089586889328640,
    20266750226990080,
    180216555012096128,
    9372571383886250242,
    9223513899626676356,
    198160764098316419,
    18031991773470720,
    9225131392914960640,
    4504150591668480,
    146369223428408320,
    140754676614656,
    2410133850167478,
    864695529756427649,
    140876001083393,
    162130858164117504,
    13837873080204132384,
    90353536245760040,
    2891315358952620160,
    7318557851516932,
    423382978134050,
    72343468145508353,
    140781511778432,
    576607270869148160,
    2305914478006440192,
    3459186932445610112,
    1180506087221367296,
    10736616970971514881,
    648799898627760384,
    4683679924736,
    71468834619409,
    9372009657998065669,
    2364741691050590258,
    1153203070315790361,
    1689262464698370,
    522980577076070410,
    2486567540854621700,
    9223408322953102758,
];
const BISHOP_MAGIC_NUMBERS: [u64; 64] = [
    18581798595985920,
    225182738905571460,
    6757787516273664,
    19144810413687869,
    5634035288172288,
    4972539206380290592,
    4835185984818257984,
    28430630773654530,
    9818421395134612256,
    9009776373563520,
    2401335559456000,
    2555282211766464,
    1268879637090305,
    2594920043729387520,
    6917530201510380553,
    1155475674489687108,
    9227879210164486464,
    1242157902694187540,
    580964442129334528,
    294985792793686018,
    9233540346191872000,
    4645591514875176,
    18577902648035840,
    290763681561051777,
    1153635673113110528,
    22535590627770624,
    4611835587710706208,
    18023332142252096,
    9288743001276416,
    565148981395976,
    5630066537136452,
    288514136067819776,
    9223970196951142432,
    10412465413526924289,
    9223935691202267136,
    85570594092351616,
    1747396930599846144,
    2315186695537428480,
    73184594530403332,
    9228037300125303298,
    5371176992639041544,
    81489239677749008,
    324364866544480256,
    1171012602776520322,
    2323927916355780866,
    577058903859201056,
    1730516955124998404,
    2252629023392257,
    9547776398712375298,
    871587866948538377,
    2551315872874498,
    36063983605907584,
    17662684037184,
    144264859230438794,
    1130366712020996,
    27171136886308864,
    1408614787190784,
    1127550317371395,
    45064723301402626,
    11605776399524300304,
    2305843009282933249,
    74315040728023168,
    579141517412139520,
    1206966108893020232,
];

pub fn init_bishop_magic_moves_array() -> [Vec<Bitboard>; 64] {
    let mut magic_positions = [const { Vec::new() }; 64];
    let moves_by_field = get_moves_for_each_field();
    for column in 0..8 {
        for row in 0..8 {
            let position: usize = column *8 + row;
            let magic_bitboard = BISHOP_MAGIC_BITBOARDS[position];
            magic_positions[position] =
                create_moves_vec(&magic_bitboard, position, &moves_by_field, false);
        }
    }
    magic_positions
}

pub fn init_bishop_magic_arrays() -> [MagicBitboard; 64] {
    let mut magic_bitboards: [MagicBitboard; 64] = [Default::default(); 64];
    for column in 0..8 {
        for row in 0..8 {
            let position: usize = column *8 + row;
            let blockers = get_bishop_blockers_for_field(column, row);
            let shift: u8 = 64 - blockers.get_used_fields().len() as u8;
            let magic_key = BISHOP_MAGIC_NUMBERS[position];
            magic_bitboards[position] = MagicBitboard {
                relevant_fields: blockers,
                magic_key,
                index: shift,
            };
        }
    }
    magic_bitboards
}

pub fn init_rook_magic_moves_array() -> [Vec<Bitboard>; 64] {
    let mut magic_positions = [const { Vec::new() }; 64];
    let moves_by_field = get_moves_for_each_field();
    for column in 0..8 {
        for row in 0..8 {
            let position: usize = column *8 + row;
            let magic_bitboard = ROOK_MAGIC_BITBOARDS[position];
            magic_positions[position] =
                create_moves_vec(&magic_bitboard, position, &moves_by_field, true);
        }
    }
    magic_positions
}

pub fn init_rook_magic_arrays() -> [MagicBitboard; 64] {
    let mut magic_bitboards: [MagicBitboard; 64] = [Default::default(); 64];
    for column in 0..8 {
        for row in 0..8 {
            let position: usize = column *8 + row;
            let blockers = get_rook_blockers_for_field(column, row);
            let shift: u8 = 64 - blockers.get_used_fields().len() as u8;
            let magic_key = ROOK_MAGIC_NUMBERS[position];
            magic_bitboards[position] = MagicBitboard {
                relevant_fields: blockers,
                magic_key,
                index: shift,
            };
        }
    }
    magic_bitboards
}

fn create_moves_vec(
    magic_bitboard: &MagicBitboard,
    own_position: usize,
    moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
    is_rook: bool,
) -> Vec<Bitboard> {
    let index_bits = 64 - magic_bitboard.index;
    let mut table = vec![Bitboard::new(); 1 << index_bits];
    let mut blockers = Bitboard::new();
    loop {
        let moves = get_valid_moves_for_position_with_given_blockers(
            blockers,
            own_position,
            moves_by_field,
            is_rook,
        );
        let table_entry = &mut table[get_magic_index(blockers, magic_bitboard)];
        *table_entry = moves;
        blockers.board = blockers
            .board
            .wrapping_sub(magic_bitboard.relevant_fields.board)
            & magic_bitboard.relevant_fields.board;
        if blockers.board == 0 {
            break;
        }
    }
    table
}
