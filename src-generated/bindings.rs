/* automatically generated by rust-bindgen 0.71.1 */

use libc::FILE;

pub const BITWUZLA_SAT: BitwuzlaResult = 10;
pub const BITWUZLA_UNSAT: BitwuzlaResult = 20;
pub const BITWUZLA_UNKNOWN: BitwuzlaResult = 0;
pub type BitwuzlaResult = ::std::os::raw::c_uint;
pub const BITWUZLA_RM_RNE: BitwuzlaRoundingMode = 0;
pub const BITWUZLA_RM_RNA: BitwuzlaRoundingMode = 1;
pub const BITWUZLA_RM_RTN: BitwuzlaRoundingMode = 2;
pub const BITWUZLA_RM_RTP: BitwuzlaRoundingMode = 3;
pub const BITWUZLA_RM_RTZ: BitwuzlaRoundingMode = 4;
pub const BITWUZLA_RM_MAX: BitwuzlaRoundingMode = 5;
pub type BitwuzlaRoundingMode = ::std::os::raw::c_uint;
pub const BITWUZLA_KIND_CONSTANT: BitwuzlaKind = 0;
pub const BITWUZLA_KIND_CONST_ARRAY: BitwuzlaKind = 1;
pub const BITWUZLA_KIND_VALUE: BitwuzlaKind = 2;
pub const BITWUZLA_KIND_VARIABLE: BitwuzlaKind = 3;
pub const BITWUZLA_KIND_AND: BitwuzlaKind = 4;
pub const BITWUZLA_KIND_DISTINCT: BitwuzlaKind = 5;
pub const BITWUZLA_KIND_EQUAL: BitwuzlaKind = 6;
pub const BITWUZLA_KIND_IFF: BitwuzlaKind = 7;
pub const BITWUZLA_KIND_IMPLIES: BitwuzlaKind = 8;
pub const BITWUZLA_KIND_NOT: BitwuzlaKind = 9;
pub const BITWUZLA_KIND_OR: BitwuzlaKind = 10;
pub const BITWUZLA_KIND_XOR: BitwuzlaKind = 11;
pub const BITWUZLA_KIND_ITE: BitwuzlaKind = 12;
pub const BITWUZLA_KIND_EXISTS: BitwuzlaKind = 13;
pub const BITWUZLA_KIND_FORALL: BitwuzlaKind = 14;
pub const BITWUZLA_KIND_APPLY: BitwuzlaKind = 15;
pub const BITWUZLA_KIND_LAMBDA: BitwuzlaKind = 16;
pub const BITWUZLA_KIND_ARRAY_SELECT: BitwuzlaKind = 17;
pub const BITWUZLA_KIND_ARRAY_STORE: BitwuzlaKind = 18;
pub const BITWUZLA_KIND_BV_ADD: BitwuzlaKind = 19;
pub const BITWUZLA_KIND_BV_AND: BitwuzlaKind = 20;
pub const BITWUZLA_KIND_BV_ASHR: BitwuzlaKind = 21;
pub const BITWUZLA_KIND_BV_COMP: BitwuzlaKind = 22;
pub const BITWUZLA_KIND_BV_CONCAT: BitwuzlaKind = 23;
pub const BITWUZLA_KIND_BV_DEC: BitwuzlaKind = 24;
pub const BITWUZLA_KIND_BV_INC: BitwuzlaKind = 25;
pub const BITWUZLA_KIND_BV_MUL: BitwuzlaKind = 26;
pub const BITWUZLA_KIND_BV_NAND: BitwuzlaKind = 27;
pub const BITWUZLA_KIND_BV_NEG: BitwuzlaKind = 28;
pub const BITWUZLA_KIND_BV_NEG_OVERFLOW: BitwuzlaKind = 29;
pub const BITWUZLA_KIND_BV_NOR: BitwuzlaKind = 30;
pub const BITWUZLA_KIND_BV_NOT: BitwuzlaKind = 31;
pub const BITWUZLA_KIND_BV_OR: BitwuzlaKind = 32;
pub const BITWUZLA_KIND_BV_REDAND: BitwuzlaKind = 33;
pub const BITWUZLA_KIND_BV_REDOR: BitwuzlaKind = 34;
pub const BITWUZLA_KIND_BV_REDXOR: BitwuzlaKind = 35;
pub const BITWUZLA_KIND_BV_ROL: BitwuzlaKind = 36;
pub const BITWUZLA_KIND_BV_ROR: BitwuzlaKind = 37;
pub const BITWUZLA_KIND_BV_SADD_OVERFLOW: BitwuzlaKind = 38;
pub const BITWUZLA_KIND_BV_SDIV_OVERFLOW: BitwuzlaKind = 39;
pub const BITWUZLA_KIND_BV_SDIV: BitwuzlaKind = 40;
pub const BITWUZLA_KIND_BV_SGE: BitwuzlaKind = 41;
pub const BITWUZLA_KIND_BV_SGT: BitwuzlaKind = 42;
pub const BITWUZLA_KIND_BV_SHL: BitwuzlaKind = 43;
pub const BITWUZLA_KIND_BV_SHR: BitwuzlaKind = 44;
pub const BITWUZLA_KIND_BV_SLE: BitwuzlaKind = 45;
pub const BITWUZLA_KIND_BV_SLT: BitwuzlaKind = 46;
pub const BITWUZLA_KIND_BV_SMOD: BitwuzlaKind = 47;
pub const BITWUZLA_KIND_BV_SMUL_OVERFLOW: BitwuzlaKind = 48;
pub const BITWUZLA_KIND_BV_SREM: BitwuzlaKind = 49;
pub const BITWUZLA_KIND_BV_SSUB_OVERFLOW: BitwuzlaKind = 50;
pub const BITWUZLA_KIND_BV_SUB: BitwuzlaKind = 51;
pub const BITWUZLA_KIND_BV_UADD_OVERFLOW: BitwuzlaKind = 52;
pub const BITWUZLA_KIND_BV_UDIV: BitwuzlaKind = 53;
pub const BITWUZLA_KIND_BV_UGE: BitwuzlaKind = 54;
pub const BITWUZLA_KIND_BV_UGT: BitwuzlaKind = 55;
pub const BITWUZLA_KIND_BV_ULE: BitwuzlaKind = 56;
pub const BITWUZLA_KIND_BV_ULT: BitwuzlaKind = 57;
pub const BITWUZLA_KIND_BV_UMUL_OVERFLOW: BitwuzlaKind = 58;
pub const BITWUZLA_KIND_BV_UREM: BitwuzlaKind = 59;
pub const BITWUZLA_KIND_BV_USUB_OVERFLOW: BitwuzlaKind = 60;
pub const BITWUZLA_KIND_BV_XNOR: BitwuzlaKind = 61;
pub const BITWUZLA_KIND_BV_XOR: BitwuzlaKind = 62;
pub const BITWUZLA_KIND_BV_EXTRACT: BitwuzlaKind = 63;
pub const BITWUZLA_KIND_BV_REPEAT: BitwuzlaKind = 64;
pub const BITWUZLA_KIND_BV_ROLI: BitwuzlaKind = 65;
pub const BITWUZLA_KIND_BV_RORI: BitwuzlaKind = 66;
pub const BITWUZLA_KIND_BV_SIGN_EXTEND: BitwuzlaKind = 67;
pub const BITWUZLA_KIND_BV_ZERO_EXTEND: BitwuzlaKind = 68;
pub const BITWUZLA_KIND_FP_ABS: BitwuzlaKind = 69;
pub const BITWUZLA_KIND_FP_ADD: BitwuzlaKind = 70;
pub const BITWUZLA_KIND_FP_DIV: BitwuzlaKind = 71;
pub const BITWUZLA_KIND_FP_EQUAL: BitwuzlaKind = 72;
pub const BITWUZLA_KIND_FP_FMA: BitwuzlaKind = 73;
pub const BITWUZLA_KIND_FP_FP: BitwuzlaKind = 74;
pub const BITWUZLA_KIND_FP_GEQ: BitwuzlaKind = 75;
pub const BITWUZLA_KIND_FP_GT: BitwuzlaKind = 76;
pub const BITWUZLA_KIND_FP_IS_INF: BitwuzlaKind = 77;
pub const BITWUZLA_KIND_FP_IS_NAN: BitwuzlaKind = 78;
pub const BITWUZLA_KIND_FP_IS_NEG: BitwuzlaKind = 79;
pub const BITWUZLA_KIND_FP_IS_NORMAL: BitwuzlaKind = 80;
pub const BITWUZLA_KIND_FP_IS_POS: BitwuzlaKind = 81;
pub const BITWUZLA_KIND_FP_IS_SUBNORMAL: BitwuzlaKind = 82;
pub const BITWUZLA_KIND_FP_IS_ZERO: BitwuzlaKind = 83;
pub const BITWUZLA_KIND_FP_LEQ: BitwuzlaKind = 84;
pub const BITWUZLA_KIND_FP_LT: BitwuzlaKind = 85;
pub const BITWUZLA_KIND_FP_MAX: BitwuzlaKind = 86;
pub const BITWUZLA_KIND_FP_MIN: BitwuzlaKind = 87;
pub const BITWUZLA_KIND_FP_MUL: BitwuzlaKind = 88;
pub const BITWUZLA_KIND_FP_NEG: BitwuzlaKind = 89;
pub const BITWUZLA_KIND_FP_REM: BitwuzlaKind = 90;
pub const BITWUZLA_KIND_FP_RTI: BitwuzlaKind = 91;
pub const BITWUZLA_KIND_FP_SQRT: BitwuzlaKind = 92;
pub const BITWUZLA_KIND_FP_SUB: BitwuzlaKind = 93;
pub const BITWUZLA_KIND_FP_TO_FP_FROM_BV: BitwuzlaKind = 94;
pub const BITWUZLA_KIND_FP_TO_FP_FROM_FP: BitwuzlaKind = 95;
pub const BITWUZLA_KIND_FP_TO_FP_FROM_SBV: BitwuzlaKind = 96;
pub const BITWUZLA_KIND_FP_TO_FP_FROM_UBV: BitwuzlaKind = 97;
pub const BITWUZLA_KIND_FP_TO_SBV: BitwuzlaKind = 98;
pub const BITWUZLA_KIND_FP_TO_UBV: BitwuzlaKind = 99;
pub const BITWUZLA_KIND_NUM_KINDS: BitwuzlaKind = 100;
pub type BitwuzlaKind = ::std::os::raw::c_uint;
pub const BITWUZLA_OPT_LOGLEVEL: BitwuzlaOption = 0;
pub const BITWUZLA_OPT_PRODUCE_MODELS: BitwuzlaOption = 1;
pub const BITWUZLA_OPT_PRODUCE_UNSAT_ASSUMPTIONS: BitwuzlaOption = 2;
pub const BITWUZLA_OPT_PRODUCE_UNSAT_CORES: BitwuzlaOption = 3;
pub const BITWUZLA_OPT_SEED: BitwuzlaOption = 4;
pub const BITWUZLA_OPT_VERBOSITY: BitwuzlaOption = 5;
pub const BITWUZLA_OPT_TIME_LIMIT_PER: BitwuzlaOption = 6;
pub const BITWUZLA_OPT_MEMORY_LIMIT: BitwuzlaOption = 7;
pub const BITWUZLA_OPT_NTHREADS: BitwuzlaOption = 8;
pub const BITWUZLA_OPT_RELEVANT_TERMS: BitwuzlaOption = 9;
pub const BITWUZLA_OPT_BV_SOLVER: BitwuzlaOption = 10;
pub const BITWUZLA_OPT_REWRITE_LEVEL: BitwuzlaOption = 11;
pub const BITWUZLA_OPT_SAT_SOLVER: BitwuzlaOption = 12;
pub const BITWUZLA_OPT_WRITE_AIGER: BitwuzlaOption = 13;
pub const BITWUZLA_OPT_WRITE_CNF: BitwuzlaOption = 14;
pub const BITWUZLA_OPT_PROP_CONST_BITS: BitwuzlaOption = 15;
pub const BITWUZLA_OPT_PROP_INFER_INEQ_BOUNDS: BitwuzlaOption = 16;
pub const BITWUZLA_OPT_PROP_NPROPS: BitwuzlaOption = 17;
pub const BITWUZLA_OPT_PROP_NUPDATES: BitwuzlaOption = 18;
pub const BITWUZLA_OPT_PROP_OPT_LT_CONCAT_SEXT: BitwuzlaOption = 19;
pub const BITWUZLA_OPT_PROP_PATH_SEL: BitwuzlaOption = 20;
pub const BITWUZLA_OPT_PROP_PROB_RANDOM_INPUT: BitwuzlaOption = 21;
pub const BITWUZLA_OPT_PROP_PROB_USE_INV_VALUE: BitwuzlaOption = 22;
pub const BITWUZLA_OPT_PROP_SEXT: BitwuzlaOption = 23;
pub const BITWUZLA_OPT_PROP_NORMALIZE: BitwuzlaOption = 24;
pub const BITWUZLA_OPT_ABSTRACTION: BitwuzlaOption = 25;
pub const BITWUZLA_OPT_ABSTRACTION_BV_SIZE: BitwuzlaOption = 26;
pub const BITWUZLA_OPT_ABSTRACTION_EAGER_REFINE: BitwuzlaOption = 27;
pub const BITWUZLA_OPT_ABSTRACTION_VALUE_LIMIT: BitwuzlaOption = 28;
pub const BITWUZLA_OPT_ABSTRACTION_VALUE_ONLY: BitwuzlaOption = 29;
pub const BITWUZLA_OPT_ABSTRACTION_ASSERT: BitwuzlaOption = 30;
pub const BITWUZLA_OPT_ABSTRACTION_ASSERT_REFS: BitwuzlaOption = 31;
pub const BITWUZLA_OPT_ABSTRACTION_INITIAL_LEMMAS: BitwuzlaOption = 32;
pub const BITWUZLA_OPT_ABSTRACTION_INC_BITBLAST: BitwuzlaOption = 33;
pub const BITWUZLA_OPT_ABSTRACTION_BV_ADD: BitwuzlaOption = 34;
pub const BITWUZLA_OPT_ABSTRACTION_BV_MUL: BitwuzlaOption = 35;
pub const BITWUZLA_OPT_ABSTRACTION_BV_UDIV: BitwuzlaOption = 36;
pub const BITWUZLA_OPT_ABSTRACTION_BV_UREM: BitwuzlaOption = 37;
pub const BITWUZLA_OPT_ABSTRACTION_EQUAL: BitwuzlaOption = 38;
pub const BITWUZLA_OPT_ABSTRACTION_ITE: BitwuzlaOption = 39;
pub const BITWUZLA_OPT_PREPROCESS: BitwuzlaOption = 40;
pub const BITWUZLA_OPT_PP_CONTRADICTING_ANDS: BitwuzlaOption = 41;
pub const BITWUZLA_OPT_PP_ELIM_BV_EXTRACTS: BitwuzlaOption = 42;
pub const BITWUZLA_OPT_PP_ELIM_BV_UDIV: BitwuzlaOption = 43;
pub const BITWUZLA_OPT_PP_EMBEDDED_CONSTR: BitwuzlaOption = 44;
pub const BITWUZLA_OPT_PP_FLATTEN_AND: BitwuzlaOption = 45;
pub const BITWUZLA_OPT_PP_NORMALIZE: BitwuzlaOption = 46;
pub const BITWUZLA_OPT_PP_SKELETON_PREPROC: BitwuzlaOption = 47;
pub const BITWUZLA_OPT_PP_VARIABLE_SUBST: BitwuzlaOption = 48;
pub const BITWUZLA_OPT_PP_VARIABLE_SUBST_NORM_EQ: BitwuzlaOption = 49;
pub const BITWUZLA_OPT_PP_VARIABLE_SUBST_NORM_DISEQ: BitwuzlaOption = 50;
pub const BITWUZLA_OPT_PP_VARIABLE_SUBST_NORM_BV_INEQ: BitwuzlaOption = 51;
pub const BITWUZLA_OPT_DBG_RW_NODE_THRESH: BitwuzlaOption = 52;
pub const BITWUZLA_OPT_DBG_PP_NODE_THRESH: BitwuzlaOption = 53;
pub const BITWUZLA_OPT_DBG_CHECK_MODEL: BitwuzlaOption = 54;
pub const BITWUZLA_OPT_DBG_CHECK_UNSAT_CORE: BitwuzlaOption = 55;
pub const BITWUZLA_OPT_NUM_OPTS: BitwuzlaOption = 56;
pub type BitwuzlaOption = ::std::os::raw::c_uint;
unsafe extern "C" {
    pub fn bitwuzla_copyright() -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn bitwuzla_version() -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn bitwuzla_git_id() -> *const ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BitwuzlaOptionInfo {
    pub opt: BitwuzlaOption,
    pub shrt: *const ::std::os::raw::c_char,
    pub lng: *const ::std::os::raw::c_char,
    pub description: *const ::std::os::raw::c_char,
    pub is_numeric: bool,
    pub is_mode: bool,
    pub is_string: bool,
    pub numeric: BitwuzlaOptionInfo_NumericValue,
    pub mode: BitwuzlaOptionInfo_ModeValue,
    pub string: BitwuzlaOptionInfo_StringValue,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BitwuzlaOptionInfo_NumericValue {
    pub cur: u64,
    pub dflt: u64,
    pub min: u64,
    pub max: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of BitwuzlaOptionInfo_NumericValue"]
        [::std::mem::size_of::<BitwuzlaOptionInfo_NumericValue>() - 32usize];
    ["Alignment of BitwuzlaOptionInfo_NumericValue"]
        [::std::mem::align_of::<BitwuzlaOptionInfo_NumericValue>() - 8usize];
    ["Offset of field: BitwuzlaOptionInfo_NumericValue::cur"]
        [::std::mem::offset_of!(BitwuzlaOptionInfo_NumericValue, cur) - 0usize];
    ["Offset of field: BitwuzlaOptionInfo_NumericValue::dflt"]
        [::std::mem::offset_of!(BitwuzlaOptionInfo_NumericValue, dflt) - 8usize];
    ["Offset of field: BitwuzlaOptionInfo_NumericValue::min"]
        [::std::mem::offset_of!(BitwuzlaOptionInfo_NumericValue, min) - 16usize];
    ["Offset of field: BitwuzlaOptionInfo_NumericValue::max"]
        [::std::mem::offset_of!(BitwuzlaOptionInfo_NumericValue, max) - 24usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BitwuzlaOptionInfo_ModeValue {
    pub cur: *const ::std::os::raw::c_char,
    pub dflt: *const ::std::os::raw::c_char,
    pub num_modes: usize,
    pub modes: *mut *const ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of BitwuzlaOptionInfo_ModeValue"]
        [::std::mem::size_of::<BitwuzlaOptionInfo_ModeValue>() - 32usize];
    ["Alignment of BitwuzlaOptionInfo_ModeValue"]
        [::std::mem::align_of::<BitwuzlaOptionInfo_ModeValue>() - 8usize];
    ["Offset of field: BitwuzlaOptionInfo_ModeValue::cur"]
        [::std::mem::offset_of!(BitwuzlaOptionInfo_ModeValue, cur) - 0usize];
    ["Offset of field: BitwuzlaOptionInfo_ModeValue::dflt"]
        [::std::mem::offset_of!(BitwuzlaOptionInfo_ModeValue, dflt) - 8usize];
    ["Offset of field: BitwuzlaOptionInfo_ModeValue::num_modes"]
        [::std::mem::offset_of!(BitwuzlaOptionInfo_ModeValue, num_modes) - 16usize];
    ["Offset of field: BitwuzlaOptionInfo_ModeValue::modes"]
        [::std::mem::offset_of!(BitwuzlaOptionInfo_ModeValue, modes) - 24usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BitwuzlaOptionInfo_StringValue {
    pub cur: *const ::std::os::raw::c_char,
    pub dflt: *const ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of BitwuzlaOptionInfo_StringValue"]
        [::std::mem::size_of::<BitwuzlaOptionInfo_StringValue>() - 16usize];
    ["Alignment of BitwuzlaOptionInfo_StringValue"]
        [::std::mem::align_of::<BitwuzlaOptionInfo_StringValue>() - 8usize];
    ["Offset of field: BitwuzlaOptionInfo_StringValue::cur"]
        [::std::mem::offset_of!(BitwuzlaOptionInfo_StringValue, cur) - 0usize];
    ["Offset of field: BitwuzlaOptionInfo_StringValue::dflt"]
        [::std::mem::offset_of!(BitwuzlaOptionInfo_StringValue, dflt) - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of BitwuzlaOptionInfo"][::std::mem::size_of::<BitwuzlaOptionInfo>() - 120usize];
    ["Alignment of BitwuzlaOptionInfo"][::std::mem::align_of::<BitwuzlaOptionInfo>() - 8usize];
    ["Offset of field: BitwuzlaOptionInfo::opt"]
        [::std::mem::offset_of!(BitwuzlaOptionInfo, opt) - 0usize];
    ["Offset of field: BitwuzlaOptionInfo::shrt"]
        [::std::mem::offset_of!(BitwuzlaOptionInfo, shrt) - 8usize];
    ["Offset of field: BitwuzlaOptionInfo::lng"]
        [::std::mem::offset_of!(BitwuzlaOptionInfo, lng) - 16usize];
    ["Offset of field: BitwuzlaOptionInfo::description"]
        [::std::mem::offset_of!(BitwuzlaOptionInfo, description) - 24usize];
    ["Offset of field: BitwuzlaOptionInfo::is_numeric"]
        [::std::mem::offset_of!(BitwuzlaOptionInfo, is_numeric) - 32usize];
    ["Offset of field: BitwuzlaOptionInfo::is_mode"]
        [::std::mem::offset_of!(BitwuzlaOptionInfo, is_mode) - 33usize];
    ["Offset of field: BitwuzlaOptionInfo::is_string"]
        [::std::mem::offset_of!(BitwuzlaOptionInfo, is_string) - 34usize];
    ["Offset of field: BitwuzlaOptionInfo::numeric"]
        [::std::mem::offset_of!(BitwuzlaOptionInfo, numeric) - 40usize];
    ["Offset of field: BitwuzlaOptionInfo::mode"]
        [::std::mem::offset_of!(BitwuzlaOptionInfo, mode) - 72usize];
    ["Offset of field: BitwuzlaOptionInfo::string"]
        [::std::mem::offset_of!(BitwuzlaOptionInfo, string) - 104usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BitwuzlaOptions {
    _unused: [u8; 0],
}
unsafe extern "C" {
    pub fn bitwuzla_options_new() -> *mut BitwuzlaOptions;
}
unsafe extern "C" {
    pub fn bitwuzla_options_delete(options: *mut BitwuzlaOptions);
}
unsafe extern "C" {
    pub fn bitwuzla_option_is_valid(
        options: *mut BitwuzlaOptions,
        name: *const ::std::os::raw::c_char,
    ) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_option_is_numeric(
        options: *mut BitwuzlaOptions,
        option: BitwuzlaOption,
    ) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_option_is_mode(options: *mut BitwuzlaOptions, option: BitwuzlaOption) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_option_is_string(options: *mut BitwuzlaOptions, option: BitwuzlaOption)
        -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_set_option(options: *mut BitwuzlaOptions, option: BitwuzlaOption, val: u64);
}
unsafe extern "C" {
    pub fn bitwuzla_set_option_mode(
        options: *mut BitwuzlaOptions,
        option: BitwuzlaOption,
        val: *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn bitwuzla_set_option_string(
        options: *mut BitwuzlaOptions,
        option: BitwuzlaOption,
        val: *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn bitwuzla_get_option(options: *mut BitwuzlaOptions, option: BitwuzlaOption) -> u64;
}
unsafe extern "C" {
    pub fn bitwuzla_get_option_mode(
        options: *mut BitwuzlaOptions,
        option: BitwuzlaOption,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn bitwuzla_get_option_string(
        options: *mut BitwuzlaOptions,
        option: BitwuzlaOption,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn bitwuzla_get_option_info(
        options: *mut BitwuzlaOptions,
        option: BitwuzlaOption,
        info: *mut BitwuzlaOptionInfo,
    );
}
unsafe extern "C" {
    pub fn bitwuzla_result_to_string(result: BitwuzlaResult) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn bitwuzla_rm_to_string(rm: BitwuzlaRoundingMode) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn bitwuzla_kind_to_string(kind: BitwuzlaKind) -> *const ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bitwuzla_sort_t {
    _unused: [u8; 0],
}
pub type BitwuzlaSort = *mut bitwuzla_sort_t;
unsafe extern "C" {
    pub fn bitwuzla_sort_hash(sort: BitwuzlaSort) -> usize;
}
unsafe extern "C" {
    pub fn bitwuzla_sort_copy(sort: BitwuzlaSort) -> BitwuzlaSort;
}
unsafe extern "C" {
    pub fn bitwuzla_sort_release(sort: BitwuzlaSort);
}
unsafe extern "C" {
    pub fn bitwuzla_sort_bv_get_size(sort: BitwuzlaSort) -> u64;
}
unsafe extern "C" {
    pub fn bitwuzla_sort_fp_get_exp_size(sort: BitwuzlaSort) -> u64;
}
unsafe extern "C" {
    pub fn bitwuzla_sort_fp_get_sig_size(sort: BitwuzlaSort) -> u64;
}
unsafe extern "C" {
    pub fn bitwuzla_sort_array_get_index(sort: BitwuzlaSort) -> BitwuzlaSort;
}
unsafe extern "C" {
    pub fn bitwuzla_sort_array_get_element(sort: BitwuzlaSort) -> BitwuzlaSort;
}
unsafe extern "C" {
    pub fn bitwuzla_sort_fun_get_domain_sorts(
        sort: BitwuzlaSort,
        size: *mut usize,
    ) -> *mut BitwuzlaSort;
}
unsafe extern "C" {
    pub fn bitwuzla_sort_fun_get_codomain(sort: BitwuzlaSort) -> BitwuzlaSort;
}
unsafe extern "C" {
    pub fn bitwuzla_sort_fun_get_arity(sort: BitwuzlaSort) -> u64;
}
unsafe extern "C" {
    pub fn bitwuzla_sort_get_uninterpreted_symbol(
        sort: BitwuzlaSort,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn bitwuzla_sort_is_array(sort: BitwuzlaSort) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_sort_is_bool(sort: BitwuzlaSort) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_sort_is_bv(sort: BitwuzlaSort) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_sort_is_fp(sort: BitwuzlaSort) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_sort_is_fun(sort: BitwuzlaSort) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_sort_is_rm(sort: BitwuzlaSort) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_sort_is_uninterpreted(sort: BitwuzlaSort) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_sort_to_string(sort: BitwuzlaSort) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn bitwuzla_sort_print(sort: BitwuzlaSort, file: *mut FILE);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bitwuzla_term_t {
    _unused: [u8; 0],
}
pub type BitwuzlaTerm = *mut bitwuzla_term_t;
unsafe extern "C" {
    pub fn bitwuzla_term_hash(term: BitwuzlaTerm) -> usize;
}
unsafe extern "C" {
    pub fn bitwuzla_term_copy(term: BitwuzlaTerm) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_term_release(term: BitwuzlaTerm);
}
unsafe extern "C" {
    pub fn bitwuzla_term_get_kind(term: BitwuzlaTerm) -> BitwuzlaKind;
}
unsafe extern "C" {
    pub fn bitwuzla_term_get_children(term: BitwuzlaTerm, size: *mut usize) -> *mut BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_term_get_indices(term: BitwuzlaTerm, size: *mut usize) -> *mut u64;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_indexed(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_get_sort(term: BitwuzlaTerm) -> BitwuzlaSort;
}
unsafe extern "C" {
    pub fn bitwuzla_term_array_get_index_sort(term: BitwuzlaTerm) -> BitwuzlaSort;
}
unsafe extern "C" {
    pub fn bitwuzla_term_array_get_element_sort(term: BitwuzlaTerm) -> BitwuzlaSort;
}
unsafe extern "C" {
    pub fn bitwuzla_term_fun_get_domain_sorts(
        term: BitwuzlaTerm,
        size: *mut usize,
    ) -> *mut BitwuzlaSort;
}
unsafe extern "C" {
    pub fn bitwuzla_term_fun_get_codomain_sort(term: BitwuzlaTerm) -> BitwuzlaSort;
}
unsafe extern "C" {
    pub fn bitwuzla_term_bv_get_size(term: BitwuzlaTerm) -> u64;
}
unsafe extern "C" {
    pub fn bitwuzla_term_fp_get_exp_size(term: BitwuzlaTerm) -> u64;
}
unsafe extern "C" {
    pub fn bitwuzla_term_fp_get_sig_size(term: BitwuzlaTerm) -> u64;
}
unsafe extern "C" {
    pub fn bitwuzla_term_fun_get_arity(term: BitwuzlaTerm) -> u64;
}
unsafe extern "C" {
    pub fn bitwuzla_term_get_symbol(term: BitwuzlaTerm) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_equal_sort(term0: BitwuzlaTerm, term1: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_array(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_const(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_fun(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_var(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_value(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_bv_value(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_fp_value(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_rm_value(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_bool(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_bv(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_fp(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_rm(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_uninterpreted(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_true(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_false(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_bv_value_zero(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_bv_value_one(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_bv_value_ones(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_bv_value_min_signed(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_bv_value_max_signed(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_fp_value_pos_zero(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_fp_value_neg_zero(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_fp_value_pos_inf(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_fp_value_neg_inf(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_fp_value_nan(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_rm_value_rna(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_rm_value_rne(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_rm_value_rtn(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_rm_value_rtp(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_is_rm_value_rtz(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_value_get_bool(term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_term_value_get_str(term: BitwuzlaTerm) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn bitwuzla_term_value_get_str_fmt(
        term: BitwuzlaTerm,
        base: u8,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn bitwuzla_term_value_get_fp_ieee(
        term: BitwuzlaTerm,
        sign: *mut *const ::std::os::raw::c_char,
        exponent: *mut *const ::std::os::raw::c_char,
        significand: *mut *const ::std::os::raw::c_char,
        base: u8,
    );
}
unsafe extern "C" {
    pub fn bitwuzla_term_value_get_rm(term: BitwuzlaTerm) -> BitwuzlaRoundingMode;
}
unsafe extern "C" {
    pub fn bitwuzla_term_to_string(term: BitwuzlaTerm) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn bitwuzla_term_to_string_fmt(
        term: BitwuzlaTerm,
        base: u8,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn bitwuzla_term_fp_value_to_real_string(
        term: BitwuzlaTerm,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn bitwuzla_term_print(term: BitwuzlaTerm, file: *mut FILE);
}
unsafe extern "C" {
    pub fn bitwuzla_term_print_fmt(term: BitwuzlaTerm, file: *mut FILE, base: u8);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BitwuzlaTermManager {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Bitwuzla {
    _unused: [u8; 0],
}
unsafe extern "C" {
    pub fn bitwuzla_new(
        tm: *mut BitwuzlaTermManager,
        options: *const BitwuzlaOptions,
    ) -> *mut Bitwuzla;
}
unsafe extern "C" {
    pub fn bitwuzla_delete(bitwuzla: *mut Bitwuzla);
}
unsafe extern "C" {
    pub fn bitwuzla_set_termination_callback(
        bitwuzla: *mut Bitwuzla,
        fun: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> i32>,
        state: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn bitwuzla_get_termination_callback_state(
        bitwuzla: *mut Bitwuzla,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn bitwuzla_set_abort_callback(
        fun: ::std::option::Option<unsafe extern "C" fn(msg: *const ::std::os::raw::c_char)>,
    );
}
unsafe extern "C" {
    pub fn bitwuzla_push(bitwuzla: *mut Bitwuzla, nlevels: u64);
}
unsafe extern "C" {
    pub fn bitwuzla_pop(bitwuzla: *mut Bitwuzla, nlevels: u64);
}
unsafe extern "C" {
    pub fn bitwuzla_assert(bitwuzla: *mut Bitwuzla, term: BitwuzlaTerm);
}
unsafe extern "C" {
    pub fn bitwuzla_get_assertions(
        bitwuzla: *mut Bitwuzla,
        size: *mut usize,
    ) -> *const BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_is_unsat_assumption(bitwuzla: *mut Bitwuzla, term: BitwuzlaTerm) -> bool;
}
unsafe extern "C" {
    pub fn bitwuzla_get_unsat_assumptions(
        bitwuzla: *mut Bitwuzla,
        size: *mut usize,
    ) -> *const BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_get_unsat_core(
        bitwuzla: *mut Bitwuzla,
        size: *mut usize,
    ) -> *const BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_simplify(bitwuzla: *mut Bitwuzla);
}
unsafe extern "C" {
    pub fn bitwuzla_simplify_term(bitwuzla: *mut Bitwuzla, term: BitwuzlaTerm) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_check_sat(bitwuzla: *mut Bitwuzla) -> BitwuzlaResult;
}
unsafe extern "C" {
    pub fn bitwuzla_check_sat_assuming(
        bitwuzla: *mut Bitwuzla,
        argc: u32,
        args: *mut BitwuzlaTerm,
    ) -> BitwuzlaResult;
}
unsafe extern "C" {
    pub fn bitwuzla_get_value(bitwuzla: *mut Bitwuzla, term: BitwuzlaTerm) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_print_formula(
        bitwuzla: *mut Bitwuzla,
        format: *const ::std::os::raw::c_char,
        file: *mut FILE,
        base: u8,
    );
}
unsafe extern "C" {
    pub fn bitwuzla_print_unsat_core(
        bitwuzla: *mut Bitwuzla,
        format: *const ::std::os::raw::c_char,
        file: *mut FILE,
        base: u8,
    );
}
unsafe extern "C" {
    pub fn bitwuzla_get_statistics(
        bitwuzla: *mut Bitwuzla,
        keys: *mut *mut *const ::std::os::raw::c_char,
        values: *mut *mut *const ::std::os::raw::c_char,
        size: *mut usize,
    );
}
unsafe extern "C" {
    pub fn bitwuzla_get_term_mgr(bitwuzla: *mut Bitwuzla) -> *mut BitwuzlaTermManager;
}
unsafe extern "C" {
    pub fn bitwuzla_term_manager_new() -> *mut BitwuzlaTermManager;
}
unsafe extern "C" {
    pub fn bitwuzla_term_manager_delete(tm: *mut BitwuzlaTermManager);
}
unsafe extern "C" {
    pub fn bitwuzla_term_manager_release(tm: *mut BitwuzlaTermManager);
}
unsafe extern "C" {
    pub fn bitwuzla_mk_array_sort(
        tm: *mut BitwuzlaTermManager,
        index: BitwuzlaSort,
        element: BitwuzlaSort,
    ) -> BitwuzlaSort;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_bool_sort(tm: *mut BitwuzlaTermManager) -> BitwuzlaSort;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_bv_sort(tm: *mut BitwuzlaTermManager, size: u64) -> BitwuzlaSort;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_fp_sort(
        tm: *mut BitwuzlaTermManager,
        exp_size: u64,
        sig_size: u64,
    ) -> BitwuzlaSort;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_fun_sort(
        tm: *mut BitwuzlaTermManager,
        arity: u64,
        domain: *mut BitwuzlaSort,
        codomain: BitwuzlaSort,
    ) -> BitwuzlaSort;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_rm_sort(tm: *mut BitwuzlaTermManager) -> BitwuzlaSort;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_uninterpreted_sort(
        tm: *mut BitwuzlaTermManager,
        symbol: *const ::std::os::raw::c_char,
    ) -> BitwuzlaSort;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_true(tm: *mut BitwuzlaTermManager) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_false(tm: *mut BitwuzlaTermManager) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_bv_zero(tm: *mut BitwuzlaTermManager, sort: BitwuzlaSort) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_bv_one(tm: *mut BitwuzlaTermManager, sort: BitwuzlaSort) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_bv_ones(tm: *mut BitwuzlaTermManager, sort: BitwuzlaSort) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_bv_min_signed(
        tm: *mut BitwuzlaTermManager,
        sort: BitwuzlaSort,
    ) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_bv_max_signed(
        tm: *mut BitwuzlaTermManager,
        sort: BitwuzlaSort,
    ) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_fp_pos_zero(
        tm: *mut BitwuzlaTermManager,
        sort: BitwuzlaSort,
    ) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_fp_neg_zero(
        tm: *mut BitwuzlaTermManager,
        sort: BitwuzlaSort,
    ) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_fp_pos_inf(tm: *mut BitwuzlaTermManager, sort: BitwuzlaSort)
        -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_fp_neg_inf(tm: *mut BitwuzlaTermManager, sort: BitwuzlaSort)
        -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_fp_nan(tm: *mut BitwuzlaTermManager, sort: BitwuzlaSort) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_bv_value(
        tm: *mut BitwuzlaTermManager,
        sort: BitwuzlaSort,
        value: *const ::std::os::raw::c_char,
        base: u8,
    ) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_bv_value_uint64(
        tm: *mut BitwuzlaTermManager,
        sort: BitwuzlaSort,
        value: u64,
    ) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_bv_value_int64(
        tm: *mut BitwuzlaTermManager,
        sort: BitwuzlaSort,
        value: i64,
    ) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_fp_value(
        tm: *mut BitwuzlaTermManager,
        bv_sign: BitwuzlaTerm,
        bv_exponent: BitwuzlaTerm,
        bv_significand: BitwuzlaTerm,
    ) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_fp_from_real(
        tm: *mut BitwuzlaTermManager,
        sort: BitwuzlaSort,
        rm: BitwuzlaTerm,
        real: *const ::std::os::raw::c_char,
    ) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_fp_from_rational(
        tm: *mut BitwuzlaTermManager,
        sort: BitwuzlaSort,
        rm: BitwuzlaTerm,
        num: *const ::std::os::raw::c_char,
        den: *const ::std::os::raw::c_char,
    ) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_rm_value(
        tm: *mut BitwuzlaTermManager,
        rm: BitwuzlaRoundingMode,
    ) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_term1(
        tm: *mut BitwuzlaTermManager,
        kind: BitwuzlaKind,
        arg: BitwuzlaTerm,
    ) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_term2(
        tm: *mut BitwuzlaTermManager,
        kind: BitwuzlaKind,
        arg0: BitwuzlaTerm,
        arg1: BitwuzlaTerm,
    ) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_term3(
        tm: *mut BitwuzlaTermManager,
        kind: BitwuzlaKind,
        arg0: BitwuzlaTerm,
        arg1: BitwuzlaTerm,
        arg2: BitwuzlaTerm,
    ) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_term(
        tm: *mut BitwuzlaTermManager,
        kind: BitwuzlaKind,
        argc: u32,
        args: *mut BitwuzlaTerm,
    ) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_term1_indexed1(
        tm: *mut BitwuzlaTermManager,
        kind: BitwuzlaKind,
        arg: BitwuzlaTerm,
        idx: u64,
    ) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_term1_indexed2(
        tm: *mut BitwuzlaTermManager,
        kind: BitwuzlaKind,
        arg: BitwuzlaTerm,
        idx0: u64,
        idx1: u64,
    ) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_term2_indexed1(
        tm: *mut BitwuzlaTermManager,
        kind: BitwuzlaKind,
        arg0: BitwuzlaTerm,
        arg1: BitwuzlaTerm,
        idx: u64,
    ) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_term2_indexed2(
        tm: *mut BitwuzlaTermManager,
        kind: BitwuzlaKind,
        arg0: BitwuzlaTerm,
        arg1: BitwuzlaTerm,
        idx0: u64,
        idx1: u64,
    ) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_term_indexed(
        tm: *mut BitwuzlaTermManager,
        kind: BitwuzlaKind,
        argc: u32,
        args: *mut BitwuzlaTerm,
        idxc: u32,
        idxs: *const u64,
    ) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_const(
        tm: *mut BitwuzlaTermManager,
        sort: BitwuzlaSort,
        symbol: *const ::std::os::raw::c_char,
    ) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_const_array(
        tm: *mut BitwuzlaTermManager,
        sort: BitwuzlaSort,
        value: BitwuzlaTerm,
    ) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_mk_var(
        tm: *mut BitwuzlaTermManager,
        sort: BitwuzlaSort,
        symbol: *const ::std::os::raw::c_char,
    ) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_substitute_term(
        term: BitwuzlaTerm,
        map_size: usize,
        map_keys: *mut BitwuzlaTerm,
        map_values: *mut BitwuzlaTerm,
    ) -> BitwuzlaTerm;
}
unsafe extern "C" {
    pub fn bitwuzla_substitute_terms(
        terms_size: usize,
        terms: *mut BitwuzlaTerm,
        map_size: usize,
        map_keys: *mut BitwuzlaTerm,
        map_values: *mut BitwuzlaTerm,
    );
}
