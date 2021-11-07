use macros::layout_member;

///
/// ```compile_fail
/// fn TestLayoutMemberAttr() {
///     #[layout_member(1, 2)]
///     union Foo {
///         a: u8,
///     }
/// }
/// ```
#[allow(dead_code)]
struct TestLayoutMemberAttr {}

#[test]
fn test_layout_member_attr_single_8() {
    #[layout_member(x86, 128)]
    union Foo {}
}
