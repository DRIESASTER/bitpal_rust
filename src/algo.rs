// src/algo.rs

use crate::delta_structs::{DeltaH, DeltaV}; // Import DeltaH and DeltaV

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
    // Step 1: Calculate dV[4]
    let dV_max_init = dh.get(-3) & matches_vectors[match_index];
    let plus_dH_min = dV_max_init + dh.get(-3);
    let xor_dH_min = plus_dH_min ^ dh.get(-3);
    dv.set(4, (xor_dH_min ^ dV_max_init) >> 1);

    // Step 2: Calculate dV[3]
    let dH_neg_2: u64 = 0;
    let dH_pos_3_init = (dv.get(4) | matches_vectors[match_index]) & dH_neg_2;
    let remain_dH_neg_3 = (dv.get(4) >> 1) ^ dh.get(-3);
    dv.set(3, ((dH_pos_3_init << 1) + remain_dH_neg_3) ^ remain_dH_neg_3);

    // Step 3: Calculate dV[2] down to dV[-3]
    let dV_pos_4_shifted_or_match = dv.get(4) | matches_vectors[match_index];
    let dV_not_4_to_3_shifted_or_match = !(dV_pos_4_shifted_or_match | dv.get(3));

    dv.set(2, (dh.get(-1) & dV_pos_4_shifted_or_match) | (dh.get(-2) & dv.get(3)) | ((dh.get(-3) & dV_not_4_to_3_shifted_or_match) << 1));
    dv.set(1, (dh.get(0) & dV_pos_4_shifted_or_match) | (dh.get(-1) & dv.get(3)) | ((dh.get(-2) & dV_not_4_to_3_shifted_or_match) << 1));
    dv.set(0, (dh.get(1) & dV_pos_4_shifted_or_match) | (dh.get(0) & dv.get(3)) | ((dh.get(-1) & dV_not_4_to_3_shifted_or_match) << 1));
    dv.set(-1, (dh.get(2) & dV_pos_4_shifted_or_match) | (dh.get(1) & dv.get(3)) | ((dh.get(0) & dV_not_4_to_3_shifted_or_match) << 1));
    dv.set(-2, (dh.get(3) & dV_pos_4_shifted_or_match) | (dh.get(2) & dv.get(3)) | ((dh.get(1) & dV_not_4_to_3_shifted_or_match) << 1));

    let mut all_ones: u64 = 0;
    for i in 0..seq1_len {
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
    dh.set(4, (dh.get(4) | matches_vectors[match_index]));
    let dh_not_4_to_3 = !(dh.get(4) | dh.get(3));

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