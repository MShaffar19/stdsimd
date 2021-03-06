define_integer_vector! {
    /// Vector of two `u128` values
    struct u128x2([u128; 2]);
}

define_integer_vector! {
    /// Vector of four `u128` values
    struct u128x4([u128; 4]);
}

from_transmute_x86! { unsafe u128x2 => __m256i }
//from_transmute_x86! { unsafe u128x4 => __m512i }
