// This file was generated by `cargo dev update_lints`.
// Use that command to update this file and do not edit by hand.
// Manual edits will be overwritten.

store.register_group(true, "clippy::style", Some("clippy_style"), vec![
    LintId::of(assertions_on_constants::ASSERTIONS_ON_CONSTANTS),
    LintId::of(assign_ops::ASSIGN_OP_PATTERN),
    LintId::of(blacklisted_name::BLACKLISTED_NAME),
    LintId::of(blocks_in_if_conditions::BLOCKS_IN_IF_CONDITIONS),
    LintId::of(bool_assert_comparison::BOOL_ASSERT_COMPARISON),
    LintId::of(casts::FN_TO_NUMERIC_CAST),
    LintId::of(casts::FN_TO_NUMERIC_CAST_WITH_TRUNCATION),
    LintId::of(collapsible_if::COLLAPSIBLE_ELSE_IF),
    LintId::of(collapsible_if::COLLAPSIBLE_IF),
    LintId::of(comparison_chain::COMPARISON_CHAIN),
    LintId::of(default::FIELD_REASSIGN_WITH_DEFAULT),
    LintId::of(dereference::NEEDLESS_BORROW),
    LintId::of(derive::DERIVE_PARTIAL_EQ_WITHOUT_EQ),
    LintId::of(disallowed_methods::DISALLOWED_METHODS),
    LintId::of(disallowed_types::DISALLOWED_TYPES),
    LintId::of(doc::MISSING_SAFETY_DOC),
    LintId::of(doc::NEEDLESS_DOCTEST_MAIN),
    LintId::of(enum_variants::ENUM_VARIANT_NAMES),
    LintId::of(enum_variants::MODULE_INCEPTION),
    LintId::of(eq_op::OP_REF),
    LintId::of(eta_reduction::REDUNDANT_CLOSURE),
    LintId::of(float_literal::EXCESSIVE_PRECISION),
    LintId::of(from_over_into::FROM_OVER_INTO),
    LintId::of(from_str_radix_10::FROM_STR_RADIX_10),
    LintId::of(functions::DOUBLE_MUST_USE),
    LintId::of(functions::MUST_USE_UNIT),
    LintId::of(functions::RESULT_UNIT_ERR),
    LintId::of(get_first::GET_FIRST),
    LintId::of(inherent_to_string::INHERENT_TO_STRING),
    LintId::of(init_numbered_fields::INIT_NUMBERED_FIELDS),
    LintId::of(len_zero::COMPARISON_TO_EMPTY),
    LintId::of(len_zero::LEN_WITHOUT_IS_EMPTY),
    LintId::of(len_zero::LEN_ZERO),
    LintId::of(literal_representation::INCONSISTENT_DIGIT_GROUPING),
    LintId::of(literal_representation::UNUSUAL_BYTE_GROUPINGS),
    LintId::of(loops::FOR_KV_MAP),
    LintId::of(loops::NEEDLESS_RANGE_LOOP),
    LintId::of(loops::SAME_ITEM_PUSH),
    LintId::of(loops::WHILE_LET_ON_ITERATOR),
    LintId::of(main_recursion::MAIN_RECURSION),
    LintId::of(manual_async_fn::MANUAL_ASYNC_FN),
    LintId::of(manual_bits::MANUAL_BITS),
    LintId::of(manual_non_exhaustive::MANUAL_NON_EXHAUSTIVE),
    LintId::of(map_clone::MAP_CLONE),
    LintId::of(match_result_ok::MATCH_RESULT_OK),
    LintId::of(matches::COLLAPSIBLE_MATCH),
    LintId::of(matches::INFALLIBLE_DESTRUCTURING_MATCH),
    LintId::of(matches::MANUAL_MAP),
    LintId::of(matches::MATCH_LIKE_MATCHES_MACRO),
    LintId::of(matches::MATCH_OVERLAPPING_ARM),
    LintId::of(matches::MATCH_REF_PATS),
    LintId::of(matches::REDUNDANT_PATTERN_MATCHING),
    LintId::of(matches::SINGLE_MATCH),
    LintId::of(mem_replace::MEM_REPLACE_OPTION_WITH_NONE),
    LintId::of(mem_replace::MEM_REPLACE_WITH_DEFAULT),
    LintId::of(methods::BYTES_NTH),
    LintId::of(methods::CHARS_LAST_CMP),
    LintId::of(methods::CHARS_NEXT_CMP),
    LintId::of(methods::ERR_EXPECT),
    LintId::of(methods::INTO_ITER_ON_REF),
    LintId::of(methods::IS_DIGIT_ASCII_RADIX),
    LintId::of(methods::ITER_CLONED_COLLECT),
    LintId::of(methods::ITER_NEXT_SLICE),
    LintId::of(methods::ITER_NTH_ZERO),
    LintId::of(methods::ITER_SKIP_NEXT),
    LintId::of(methods::MANUAL_SATURATING_ARITHMETIC),
    LintId::of(methods::MAP_COLLECT_RESULT_UNIT),
    LintId::of(methods::NEW_RET_NO_SELF),
    LintId::of(methods::OK_EXPECT),
    LintId::of(methods::OPTION_MAP_OR_NONE),
    LintId::of(methods::RESULT_MAP_OR_INTO_OPTION),
    LintId::of(methods::SHOULD_IMPLEMENT_TRAIT),
    LintId::of(methods::SINGLE_CHAR_ADD_STR),
    LintId::of(methods::STRING_EXTEND_CHARS),
    LintId::of(methods::UNNECESSARY_FOLD),
    LintId::of(methods::UNNECESSARY_LAZY_EVALUATIONS),
    LintId::of(methods::UNWRAP_OR_ELSE_DEFAULT),
    LintId::of(methods::WRONG_SELF_CONVENTION),
    LintId::of(misc::TOPLEVEL_REF_ARG),
    LintId::of(misc::ZERO_PTR),
    LintId::of(misc_early::BUILTIN_TYPE_SHADOW),
    LintId::of(misc_early::DOUBLE_NEG),
    LintId::of(misc_early::DUPLICATE_UNDERSCORE_ARGUMENT),
    LintId::of(misc_early::MIXED_CASE_HEX_LITERALS),
    LintId::of(misc_early::REDUNDANT_PATTERN),
    LintId::of(mut_mutex_lock::MUT_MUTEX_LOCK),
    LintId::of(mut_reference::UNNECESSARY_MUT_PASSED),
    LintId::of(needless_late_init::NEEDLESS_LATE_INIT),
    LintId::of(needless_parens_on_range_literals::NEEDLESS_PARENS_ON_RANGE_LITERALS),
    LintId::of(neg_multiply::NEG_MULTIPLY),
    LintId::of(new_without_default::NEW_WITHOUT_DEFAULT),
    LintId::of(non_copy_const::BORROW_INTERIOR_MUTABLE_CONST),
    LintId::of(non_copy_const::DECLARE_INTERIOR_MUTABLE_CONST),
    LintId::of(non_expressive_names::JUST_UNDERSCORES_AND_DIGITS),
    LintId::of(ptr::CMP_NULL),
    LintId::of(ptr::PTR_ARG),
    LintId::of(ptr_eq::PTR_EQ),
    LintId::of(question_mark::QUESTION_MARK),
    LintId::of(ranges::MANUAL_RANGE_CONTAINS),
    LintId::of(redundant_field_names::REDUNDANT_FIELD_NAMES),
    LintId::of(redundant_static_lifetimes::REDUNDANT_STATIC_LIFETIMES),
    LintId::of(returns::LET_AND_RETURN),
    LintId::of(returns::NEEDLESS_RETURN),
    LintId::of(self_named_constructors::SELF_NAMED_CONSTRUCTORS),
    LintId::of(single_component_path_imports::SINGLE_COMPONENT_PATH_IMPORTS),
    LintId::of(strings::TRIM_SPLIT_WHITESPACE),
    LintId::of(tabs_in_doc_comments::TABS_IN_DOC_COMMENTS),
    LintId::of(to_digit_is_some::TO_DIGIT_IS_SOME),
    LintId::of(unit_types::LET_UNIT_VALUE),
    LintId::of(unnecessary_owned_empty_strings::UNNECESSARY_OWNED_EMPTY_STRINGS),
    LintId::of(unsafe_removed_from_name::UNSAFE_REMOVED_FROM_NAME),
    LintId::of(unused_unit::UNUSED_UNIT),
    LintId::of(upper_case_acronyms::UPPER_CASE_ACRONYMS),
    LintId::of(write::PRINTLN_EMPTY_STRING),
    LintId::of(write::PRINT_LITERAL),
    LintId::of(write::PRINT_WITH_NEWLINE),
    LintId::of(write::WRITELN_EMPTY_STRING),
    LintId::of(write::WRITE_LITERAL),
    LintId::of(write::WRITE_WITH_NEWLINE),
])
