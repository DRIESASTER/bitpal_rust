//bulk of the algorithm
use crate::delta_structs::{DeltaH, DeltaV};

//creates match vectors, 1 where letter is present 0 when not
pub fn generate_match_vectors(sequence: &str) -> [u64; 4] {
    let mut match_vectors: [u64; 4] = [0; 4];
    for (i, char) in sequence.chars().enumerate() {
        match char {
            'A' => match_vectors[0] |= 1 << i,
            'C' => match_vectors[1] |= 1 << i,
            'G' => match_vectors[2] |= 1 << i,
            'T' => match_vectors[3] |= 1 << i,
            _ => panic!("Invalid character in sequence"),
        }
    }
    match_vectors
}
pub fn calculate_new_dv_values(
    seq1_len: usize,
    matches_vectors: &[u64; 4],
    match_index: usize,
    dh: &DeltaH,
    dv: &mut DeltaV,
) {
    // step 1 (dV 4)
    let d_v_max_init = dh.get(-3) & matches_vectors[match_index];
    let plus_d_h_min = d_v_max_init + dh.get(-3);
    let xor_d_h_min = plus_d_h_min ^ dh.get(-3);
    dv.set(4, xor_d_h_min ^ d_v_max_init);

    // step 2 (in this case only dV 3)
    let d_v_pos_4_shifted_or_match = dv.get(4) | matches_vectors[match_index];
    let d_h_pos_3_init = d_v_pos_4_shifted_or_match & dh.get(-2);
    let remain_d_h_neg_3 = (dv.get(4) >> 1) ^ dh.get(-3);
    dv.set(3, ((d_h_pos_3_init << 1) + remain_d_h_neg_3) ^ remain_d_h_neg_3);

    // step 3 (calc rest)
    //from high to low cause dependencies
    let d_v_not_4_to_3_shifted_or_match = !(d_v_pos_4_shifted_or_match | dv.get(3));

    //the reason we shift it to the right is that by our earlier definition there can not be a single bit a 1 at a a place where dH-3 is a 1 so we need to shift to align them to start run propagation
    dv.set(2, ((dh.get(-1) & d_v_pos_4_shifted_or_match) | (dh.get(-2) & (dv.get(3) & !matches_vectors[match_index])) | (dh.get(-3) & d_v_not_4_to_3_shifted_or_match)) << 1);
    dv.set(1, ((dh.get(0) & d_v_pos_4_shifted_or_match) | (dh.get(-1) & (dv.get(3) & !matches_vectors[match_index])) | (dh.get(-2) & d_v_not_4_to_3_shifted_or_match)) << 1);
    dv.set(0, ((dh.get(1) & d_v_pos_4_shifted_or_match) | (dh.get(0) & (dv.get(3) & !matches_vectors[match_index])) | (dh.get(-1) & d_v_not_4_to_3_shifted_or_match)) << 1);
    dv.set(-1, ((dh.get(2) & d_v_pos_4_shifted_or_match) | (dh.get(1) & (dv.get(3) & !matches_vectors[match_index])) | (dh.get(0) & d_v_not_4_to_3_shifted_or_match)) << 1);
    dv.set(-2, ((dh.get(3) & d_v_pos_4_shifted_or_match) | (dh.get(2) & (dv.get(3) & !matches_vectors[match_index])) | (dh.get(1) & d_v_not_4_to_3_shifted_or_match)) << 1);

    let mut all_ones: u64 = 0;
    for i in 0..seq1_len+1 {
        all_ones |= 1 << i;
    }

    dv.set(-3, all_ones ^ (dv.get(4) | dv.get(3) | dv.get(2) | dv.get(1) | dv.get(0) | dv.get(-1) | dv.get(-2)));
}

pub fn calculate_new_dh_values(
    seq1_len: usize,
    matches_vectors: &[u64; 4],
    match_index: usize,
    dh: &mut DeltaH,
    dv: &DeltaV,
) {
    dh.set(4, dh.get(4) | matches_vectors[match_index]);

    for i in -3..=3 {
        dh.set(i, dh.get(i) & !matches_vectors[match_index]);
    }

    //mask is necessary to not introduce extra 1's outside of sequence length's reach
    let mut mask: u64 = u64::MAX;
    mask >>= 64 - seq1_len;

    //calculating new delta H values using transposed function table, fairly straightforward
    let dh_not_4_to_3 = (!(dh.get(4) | dh.get(3))) & mask;
    let new_dh_pos_4 = dh.get(4) & dv.get(-3);
    let new_dh_pos_3 = (dh.get(3) & dv.get(-3)) | (dh.get(4) & dv.get(-2));
    let new_dh_pos_2 = (dh_not_4_to_3 & dv.get(-3)) | (dh.get(3) & dv.get(-2)) | (dh.get(4) & dv.get(-1));
    let new_dh_pos_1 = (dh_not_4_to_3 & dv.get(-2)) | (dh.get(3) & dv.get(-1)) | (dh.get(4) & dv.get(0));
    let new_dh_0 = (dh_not_4_to_3 & dv.get(-1)) | (dh.get(3) & dv.get(0)) | (dh.get(4) & dv.get(1));
    let new_dh_neg_1 = (dh_not_4_to_3 & dv.get(0)) | (dh.get(3) & dv.get(1)) | (dh.get(4) & dv.get(2));
    let new_dh_neg_2 = (dh_not_4_to_3 & dv.get(1)) | (dh.get(3) & dv.get(2)) | (dh.get(4) & dv.get(3));

    let mut all_ones: u64 = 0;
    for i in 0..seq1_len {
        all_ones |= 1 << i;
    }

    let new_dh_neg_3 = all_ones ^ (new_dh_neg_2 | new_dh_neg_1 | new_dh_0 | new_dh_pos_1 | new_dh_pos_2 | new_dh_pos_3 | new_dh_pos_4);

    dh.set(4, new_dh_pos_4);
    dh.set(3, new_dh_pos_3);
    dh.set(2, new_dh_pos_2);
    dh.set(1, new_dh_pos_1);
    dh.set(0, new_dh_0);
    dh.set(-1, new_dh_neg_1);
    dh.set(-2, new_dh_neg_2);
    dh.set(-3, new_dh_neg_3);
}