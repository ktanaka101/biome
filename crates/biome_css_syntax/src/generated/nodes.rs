//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::enum_variant_names)]
#![allow(clippy::match_like_matches_macro)]
use crate::{
    macros::map_syntax_node,
    CssLanguage as Language, CssSyntaxElement as SyntaxElement,
    CssSyntaxElementChildren as SyntaxElementChildren,
    CssSyntaxKind::{self as SyntaxKind, *},
    CssSyntaxList as SyntaxList, CssSyntaxNode as SyntaxNode, CssSyntaxToken as SyntaxToken,
};
use biome_rowan::{support, AstNode, RawSyntaxKind, SyntaxKindSet, SyntaxResult};
#[allow(unused)]
use biome_rowan::{
    AstNodeList, AstNodeListIterator, AstSeparatedList, AstSeparatedListNodesIterator,
};
#[cfg(feature = "serde")]
use serde::ser::SerializeSeq;
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};
use std::fmt::{Debug, Formatter};
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAnyFunction {
    pub(crate) syntax: SyntaxNode,
}
impl CssAnyFunction {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssAnyFunctionFields {
        CssAnyFunctionFields {
            css_simple_function: self.css_simple_function(),
        }
    }
    pub fn css_simple_function(&self) -> SyntaxResult<CssSimpleFunction> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAnyFunction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAnyFunctionFields {
    pub css_simple_function: SyntaxResult<CssSimpleFunction>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssAtRuleFields {
        CssAtRuleFields {
            at_token: self.at_token(),
            rule: self.rule(),
        }
    }
    pub fn at_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn rule(&self) -> SyntaxResult<AnyCssAtRule> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAtRuleFields {
    pub at_token: SyntaxResult<SyntaxToken>,
    pub rule: SyntaxResult<AnyCssAtRule>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAttributeMatcher {
    pub(crate) syntax: SyntaxNode,
}
impl CssAttributeMatcher {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssAttributeMatcherFields {
        CssAttributeMatcherFields {
            operator: self.operator(),
            value: self.value(),
            modifier: self.modifier(),
        }
    }
    pub fn operator(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<CssAttributeMatcherValue> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn modifier(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAttributeMatcher {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAttributeMatcherFields {
    pub operator: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<CssAttributeMatcherValue>,
    pub modifier: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAttributeMatcherValue {
    pub(crate) syntax: SyntaxNode,
}
impl CssAttributeMatcherValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssAttributeMatcherValueFields {
        CssAttributeMatcherValueFields { name: self.name() }
    }
    pub fn name(&self) -> SyntaxResult<AnyCssAttributeMatcherValue> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAttributeMatcherValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAttributeMatcherValueFields {
    pub name: SyntaxResult<AnyCssAttributeMatcherValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAttributeName {
    pub(crate) syntax: SyntaxNode,
}
impl CssAttributeName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssAttributeNameFields {
        CssAttributeNameFields {
            namespace: self.namespace(),
            name: self.name(),
        }
    }
    pub fn namespace(&self) -> Option<CssNamespace> {
        support::node(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAttributeName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAttributeNameFields {
    pub namespace: Option<CssNamespace>,
    pub name: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAttributeSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssAttributeSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssAttributeSelectorFields {
        CssAttributeSelectorFields {
            l_brack_token: self.l_brack_token(),
            name: self.name(),
            matcher: self.matcher(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<CssAttributeName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn matcher(&self) -> Option<CssAttributeMatcher> {
        support::node(&self.syntax, 2usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAttributeSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAttributeSelectorFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<CssAttributeName>,
    pub matcher: Option<CssAttributeMatcher>,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssCharsetAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssCharsetAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssCharsetAtRuleFields {
        CssCharsetAtRuleFields {
            charset_token: self.charset_token(),
            encoding: self.encoding(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn charset_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn encoding(&self) -> SyntaxResult<CssString> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn semicolon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssCharsetAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssCharsetAtRuleFields {
    pub charset_token: SyntaxResult<SyntaxToken>,
    pub encoding: SyntaxResult<CssString>,
    pub semicolon_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssClassSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssClassSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssClassSelectorFields {
        CssClassSelectorFields {
            dot_token: self.dot_token(),
            name: self.name(),
        }
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssClassSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssClassSelectorFields {
    pub dot_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssColorProfileAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssColorProfileAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssColorProfileAtRuleFields {
        CssColorProfileAtRuleFields {
            color_profile_token: self.color_profile_token(),
            name: self.name(),
            block: self.block(),
        }
    }
    pub fn color_profile_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssDeclarationListBlock> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssColorProfileAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssColorProfileAtRuleFields {
    pub color_profile_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<CssIdentifier>,
    pub block: SyntaxResult<AnyCssDeclarationListBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssComplexSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssComplexSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssComplexSelectorFields {
        CssComplexSelectorFields {
            left: self.left(),
            combinator: self.combinator(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyCssSelector> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn combinator(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyCssSelector> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssComplexSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssComplexSelectorFields {
    pub left: SyntaxResult<AnyCssSelector>,
    pub combinator: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyCssSelector>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssCompoundSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssCompoundSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssCompoundSelectorFields {
        CssCompoundSelectorFields {
            nesting_selector_token: self.nesting_selector_token(),
            simple_selector: self.simple_selector(),
            sub_selectors: self.sub_selectors(),
        }
    }
    pub fn nesting_selector_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn simple_selector(&self) -> Option<AnyCssSimpleSelector> {
        support::node(&self.syntax, 1usize)
    }
    pub fn sub_selectors(&self) -> CssSubSelectorList {
        support::list(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssCompoundSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssCompoundSelectorFields {
    pub nesting_selector_token: Option<SyntaxToken>,
    pub simple_selector: Option<AnyCssSimpleSelector>,
    pub sub_selectors: CssSubSelectorList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssContainerAndQuery {
    pub(crate) syntax: SyntaxNode,
}
impl CssContainerAndQuery {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssContainerAndQueryFields {
        CssContainerAndQueryFields {
            left: self.left(),
            and_token: self.and_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyCssContainerQueryInParens> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn and_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyCssContainerAndCombinableQuery> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssContainerAndQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssContainerAndQueryFields {
    pub left: SyntaxResult<AnyCssContainerQueryInParens>,
    pub and_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyCssContainerAndCombinableQuery>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssContainerAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssContainerAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssContainerAtRuleFields {
        CssContainerAtRuleFields {
            container_token: self.container_token(),
            name: self.name(),
            query: self.query(),
            block: self.block(),
        }
    }
    pub fn container_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> Option<CssIdentifier> {
        support::node(&self.syntax, 1usize)
    }
    pub fn query(&self) -> SyntaxResult<AnyCssContainerQuery> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssRuleListBlock> {
        support::required_node(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssContainerAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssContainerAtRuleFields {
    pub container_token: SyntaxResult<SyntaxToken>,
    pub name: Option<CssIdentifier>,
    pub query: SyntaxResult<AnyCssContainerQuery>,
    pub block: SyntaxResult<AnyCssRuleListBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssContainerNotQuery {
    pub(crate) syntax: SyntaxNode,
}
impl CssContainerNotQuery {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssContainerNotQueryFields {
        CssContainerNotQueryFields {
            not_token: self.not_token(),
            query: self.query(),
        }
    }
    pub fn not_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn query(&self) -> SyntaxResult<AnyCssContainerQueryInParens> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssContainerNotQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssContainerNotQueryFields {
    pub not_token: SyntaxResult<SyntaxToken>,
    pub query: SyntaxResult<AnyCssContainerQueryInParens>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssContainerOrQuery {
    pub(crate) syntax: SyntaxNode,
}
impl CssContainerOrQuery {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssContainerOrQueryFields {
        CssContainerOrQueryFields {
            left: self.left(),
            or_token: self.or_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyCssContainerQueryInParens> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn or_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyCssContainerOrCombinableQuery> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssContainerOrQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssContainerOrQueryFields {
    pub left: SyntaxResult<AnyCssContainerQueryInParens>,
    pub or_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyCssContainerOrCombinableQuery>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssContainerQueryInParens {
    pub(crate) syntax: SyntaxNode,
}
impl CssContainerQueryInParens {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssContainerQueryInParensFields {
        CssContainerQueryInParensFields {
            l_paren_token: self.l_paren_token(),
            query: self.query(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn query(&self) -> SyntaxResult<AnyCssContainerQuery> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssContainerQueryInParens {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssContainerQueryInParensFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub query: SyntaxResult<AnyCssContainerQuery>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssContainerSizeFeatureInParens {
    pub(crate) syntax: SyntaxNode,
}
impl CssContainerSizeFeatureInParens {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssContainerSizeFeatureInParensFields {
        CssContainerSizeFeatureInParensFields {
            l_paren_token: self.l_paren_token(),
            feature: self.feature(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn feature(&self) -> SyntaxResult<AnyCssQueryFeature> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssContainerSizeFeatureInParens {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssContainerSizeFeatureInParensFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub feature: SyntaxResult<AnyCssQueryFeature>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssContainerStyleAndQuery {
    pub(crate) syntax: SyntaxNode,
}
impl CssContainerStyleAndQuery {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssContainerStyleAndQueryFields {
        CssContainerStyleAndQueryFields {
            left: self.left(),
            and_token: self.and_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<CssContainerStyleInParens> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn and_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyCssContainerStyleAndCombinableQuery> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssContainerStyleAndQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssContainerStyleAndQueryFields {
    pub left: SyntaxResult<CssContainerStyleInParens>,
    pub and_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyCssContainerStyleAndCombinableQuery>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssContainerStyleInParens {
    pub(crate) syntax: SyntaxNode,
}
impl CssContainerStyleInParens {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssContainerStyleInParensFields {
        CssContainerStyleInParensFields {
            l_paren_token: self.l_paren_token(),
            query: self.query(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn query(&self) -> SyntaxResult<AnyCssContainerStyleInParens> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssContainerStyleInParens {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssContainerStyleInParensFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub query: SyntaxResult<AnyCssContainerStyleInParens>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssContainerStyleNotQuery {
    pub(crate) syntax: SyntaxNode,
}
impl CssContainerStyleNotQuery {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssContainerStyleNotQueryFields {
        CssContainerStyleNotQueryFields {
            not_token: self.not_token(),
            query: self.query(),
        }
    }
    pub fn not_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn query(&self) -> SyntaxResult<CssContainerStyleInParens> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssContainerStyleNotQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssContainerStyleNotQueryFields {
    pub not_token: SyntaxResult<SyntaxToken>,
    pub query: SyntaxResult<CssContainerStyleInParens>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssContainerStyleOrQuery {
    pub(crate) syntax: SyntaxNode,
}
impl CssContainerStyleOrQuery {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssContainerStyleOrQueryFields {
        CssContainerStyleOrQueryFields {
            left: self.left(),
            or_token: self.or_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<CssContainerStyleInParens> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn or_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyCssContainerStyleOrCombinableQuery> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssContainerStyleOrQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssContainerStyleOrQueryFields {
    pub left: SyntaxResult<CssContainerStyleInParens>,
    pub or_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyCssContainerStyleOrCombinableQuery>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssContainerStyleQueryInParens {
    pub(crate) syntax: SyntaxNode,
}
impl CssContainerStyleQueryInParens {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssContainerStyleQueryInParensFields {
        CssContainerStyleQueryInParensFields {
            style_token: self.style_token(),
            l_paren_token: self.l_paren_token(),
            query: self.query(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn style_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn query(&self) -> SyntaxResult<AnyCssContainerStyleQuery> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssContainerStyleQueryInParens {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssContainerStyleQueryInParensFields {
    pub style_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub query: SyntaxResult<AnyCssContainerStyleQuery>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssCounterStyleAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssCounterStyleAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssCounterStyleAtRuleFields {
        CssCounterStyleAtRuleFields {
            counter_style_token: self.counter_style_token(),
            name: self.name(),
            block: self.block(),
        }
    }
    pub fn counter_style_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssDeclarationListBlock> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssCounterStyleAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssCounterStyleAtRuleFields {
    pub counter_style_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<CssIdentifier>,
    pub block: SyntaxResult<AnyCssDeclarationListBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssCustomProperty {
    pub(crate) syntax: SyntaxNode,
}
impl CssCustomProperty {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssCustomPropertyFields {
        CssCustomPropertyFields {
            value: self.value(),
        }
    }
    pub fn value(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssCustomProperty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssCustomPropertyFields {
    pub value: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssDeclaration {
    pub(crate) syntax: SyntaxNode,
}
impl CssDeclaration {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssDeclarationFields {
        CssDeclarationFields {
            name: self.name(),
            colon_token: self.colon_token(),
            value: self.value(),
            important: self.important(),
        }
    }
    pub fn name(&self) -> SyntaxResult<AnyCssDeclarationName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> CssComponentValueList {
        support::list(&self.syntax, 2usize)
    }
    pub fn important(&self) -> Option<CssDeclarationImportant> {
        support::node(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssDeclaration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssDeclarationFields {
    pub name: SyntaxResult<AnyCssDeclarationName>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub value: CssComponentValueList,
    pub important: Option<CssDeclarationImportant>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssDeclarationImportant {
    pub(crate) syntax: SyntaxNode,
}
impl CssDeclarationImportant {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssDeclarationImportantFields {
        CssDeclarationImportantFields {
            excl_token: self.excl_token(),
            important_token: self.important_token(),
        }
    }
    pub fn excl_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn important_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssDeclarationImportant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssDeclarationImportantFields {
    pub excl_token: SyntaxResult<SyntaxToken>,
    pub important_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssDeclarationListBlock {
    pub(crate) syntax: SyntaxNode,
}
impl CssDeclarationListBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssDeclarationListBlockFields {
        CssDeclarationListBlockFields {
            l_curly_token: self.l_curly_token(),
            declarations: self.declarations(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn declarations(&self) -> CssDeclarationList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssDeclarationListBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssDeclarationListBlockFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub declarations: CssDeclarationList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssFontFaceAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssFontFaceAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssFontFaceAtRuleFields {
        CssFontFaceAtRuleFields {
            font_face_token: self.font_face_token(),
            block: self.block(),
        }
    }
    pub fn font_face_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssDeclarationListBlock> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssFontFaceAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssFontFaceAtRuleFields {
    pub font_face_token: SyntaxResult<SyntaxToken>,
    pub block: SyntaxResult<AnyCssDeclarationListBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssFontPaletteValuesAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssFontPaletteValuesAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssFontPaletteValuesAtRuleFields {
        CssFontPaletteValuesAtRuleFields {
            font_palette_values_token: self.font_palette_values_token(),
            name: self.name(),
            block: self.block(),
        }
    }
    pub fn font_palette_values_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssDeclarationListBlock> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssFontPaletteValuesAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssFontPaletteValuesAtRuleFields {
    pub font_palette_values_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<CssIdentifier>,
    pub block: SyntaxResult<AnyCssDeclarationListBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssIdSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssIdSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssIdSelectorFields {
        CssIdSelectorFields {
            hash_token: self.hash_token(),
            name: self.name(),
        }
    }
    pub fn hash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssIdSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssIdSelectorFields {
    pub hash_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssIdentifier {
    pub(crate) syntax: SyntaxNode,
}
impl CssIdentifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssIdentifierFields {
        CssIdentifierFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssIdentifierFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssKeyframesAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssKeyframesAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssKeyframesAtRuleFields {
        CssKeyframesAtRuleFields {
            keyframes_token: self.keyframes_token(),
            name: self.name(),
            block: self.block(),
        }
    }
    pub fn keyframes_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<AnyCssKeyframeName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssKeyframesBlock> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssKeyframesAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssKeyframesAtRuleFields {
    pub keyframes_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<AnyCssKeyframeName>,
    pub block: SyntaxResult<AnyCssKeyframesBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssKeyframesBlock {
    pub(crate) syntax: SyntaxNode,
}
impl CssKeyframesBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssKeyframesBlockFields {
        CssKeyframesBlockFields {
            l_curly_token: self.l_curly_token(),
            items: self.items(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> CssKeyframesItemList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssKeyframesBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssKeyframesBlockFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub items: CssKeyframesItemList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssKeyframesIdentSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssKeyframesIdentSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssKeyframesIdentSelectorFields {
        CssKeyframesIdentSelectorFields {
            selector: self.selector(),
        }
    }
    pub fn selector(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssKeyframesIdentSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssKeyframesIdentSelectorFields {
    pub selector: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssKeyframesItem {
    pub(crate) syntax: SyntaxNode,
}
impl CssKeyframesItem {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssKeyframesItemFields {
        CssKeyframesItemFields {
            selectors: self.selectors(),
            block: self.block(),
        }
    }
    pub fn selectors(&self) -> CssKeyframesSelectorList {
        support::list(&self.syntax, 0usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssDeclarationListBlock> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssKeyframesItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssKeyframesItemFields {
    pub selectors: CssKeyframesSelectorList,
    pub block: SyntaxResult<AnyCssDeclarationListBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssKeyframesPercentageSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssKeyframesPercentageSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssKeyframesPercentageSelectorFields {
        CssKeyframesPercentageSelectorFields {
            selector: self.selector(),
        }
    }
    pub fn selector(&self) -> SyntaxResult<CssPercentage> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssKeyframesPercentageSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssKeyframesPercentageSelectorFields {
    pub selector: SyntaxResult<CssPercentage>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssMediaAndCondition {
    pub(crate) syntax: SyntaxNode,
}
impl CssMediaAndCondition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMediaAndConditionFields {
        CssMediaAndConditionFields {
            left: self.left(),
            and_token: self.and_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyCssMediaInParens> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn and_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyCssMediaAndCombinableCondition> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssMediaAndCondition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssMediaAndConditionFields {
    pub left: SyntaxResult<AnyCssMediaInParens>,
    pub and_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyCssMediaAndCombinableCondition>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssMediaAndTypeQuery {
    pub(crate) syntax: SyntaxNode,
}
impl CssMediaAndTypeQuery {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMediaAndTypeQueryFields {
        CssMediaAndTypeQueryFields {
            left: self.left(),
            and_token: self.and_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<CssMediaTypeQuery> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn and_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyCssMediaTypeCondition> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssMediaAndTypeQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssMediaAndTypeQueryFields {
    pub left: SyntaxResult<CssMediaTypeQuery>,
    pub and_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyCssMediaTypeCondition>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssMediaAtRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssMediaAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMediaAtRuleFields {
        CssMediaAtRuleFields {
            media_token: self.media_token(),
            queries: self.queries(),
            block: self.block(),
        }
    }
    pub fn media_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn queries(&self) -> CssMediaQueryList {
        support::list(&self.syntax, 1usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssRuleListBlock> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssMediaAtRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssMediaAtRuleFields {
    pub media_token: SyntaxResult<SyntaxToken>,
    pub queries: CssMediaQueryList,
    pub block: SyntaxResult<AnyCssRuleListBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssMediaConditionInParens {
    pub(crate) syntax: SyntaxNode,
}
impl CssMediaConditionInParens {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMediaConditionInParensFields {
        CssMediaConditionInParensFields {
            l_paren_token: self.l_paren_token(),
            condition: self.condition(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn condition(&self) -> SyntaxResult<AnyCssMediaCondition> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssMediaConditionInParens {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssMediaConditionInParensFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub condition: SyntaxResult<AnyCssMediaCondition>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssMediaConditionQuery {
    pub(crate) syntax: SyntaxNode,
}
impl CssMediaConditionQuery {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMediaConditionQueryFields {
        CssMediaConditionQueryFields {
            condition: self.condition(),
        }
    }
    pub fn condition(&self) -> SyntaxResult<AnyCssMediaCondition> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssMediaConditionQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssMediaConditionQueryFields {
    pub condition: SyntaxResult<AnyCssMediaCondition>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssMediaFeatureInParens {
    pub(crate) syntax: SyntaxNode,
}
impl CssMediaFeatureInParens {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMediaFeatureInParensFields {
        CssMediaFeatureInParensFields {
            l_paren_token: self.l_paren_token(),
            feature: self.feature(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn feature(&self) -> SyntaxResult<AnyCssQueryFeature> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssMediaFeatureInParens {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssMediaFeatureInParensFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub feature: SyntaxResult<AnyCssQueryFeature>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssMediaNotCondition {
    pub(crate) syntax: SyntaxNode,
}
impl CssMediaNotCondition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMediaNotConditionFields {
        CssMediaNotConditionFields {
            not_token: self.not_token(),
            condition: self.condition(),
        }
    }
    pub fn not_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn condition(&self) -> SyntaxResult<AnyCssMediaInParens> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssMediaNotCondition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssMediaNotConditionFields {
    pub not_token: SyntaxResult<SyntaxToken>,
    pub condition: SyntaxResult<AnyCssMediaInParens>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssMediaOrCondition {
    pub(crate) syntax: SyntaxNode,
}
impl CssMediaOrCondition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMediaOrConditionFields {
        CssMediaOrConditionFields {
            left: self.left(),
            or_token: self.or_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyCssMediaInParens> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn or_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyCssMediaOrCombinableCondition> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssMediaOrCondition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssMediaOrConditionFields {
    pub left: SyntaxResult<AnyCssMediaInParens>,
    pub or_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyCssMediaOrCombinableCondition>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssMediaType {
    pub(crate) syntax: SyntaxNode,
}
impl CssMediaType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMediaTypeFields {
        CssMediaTypeFields {
            value: self.value(),
        }
    }
    pub fn value(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssMediaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssMediaTypeFields {
    pub value: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssMediaTypeQuery {
    pub(crate) syntax: SyntaxNode,
}
impl CssMediaTypeQuery {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssMediaTypeQueryFields {
        CssMediaTypeQueryFields {
            modifier: self.modifier(),
            ty: self.ty(),
        }
    }
    pub fn modifier(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn ty(&self) -> SyntaxResult<CssMediaType> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssMediaTypeQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssMediaTypeQueryFields {
    pub modifier: Option<SyntaxToken>,
    pub ty: SyntaxResult<CssMediaType>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssNamedNamespacePrefix {
    pub(crate) syntax: SyntaxNode,
}
impl CssNamedNamespacePrefix {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssNamedNamespacePrefixFields {
        CssNamedNamespacePrefixFields { name: self.name() }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssNamedNamespacePrefix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssNamedNamespacePrefixFields {
    pub name: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssNamespace {
    pub(crate) syntax: SyntaxNode,
}
impl CssNamespace {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssNamespaceFields {
        CssNamespaceFields {
            prefix: self.prefix(),
            bitwise_or_token: self.bitwise_or_token(),
        }
    }
    pub fn prefix(&self) -> Option<AnyCssNamespacePrefix> {
        support::node(&self.syntax, 0usize)
    }
    pub fn bitwise_or_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssNamespace {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssNamespaceFields {
    pub prefix: Option<AnyCssNamespacePrefix>,
    pub bitwise_or_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssNthOffset {
    pub(crate) syntax: SyntaxNode,
}
impl CssNthOffset {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssNthOffsetFields {
        CssNthOffsetFields {
            sign: self.sign(),
            value: self.value(),
        }
    }
    pub fn sign(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<CssNumber> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssNthOffset {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssNthOffsetFields {
    pub sign: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<CssNumber>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssNumber {
    pub(crate) syntax: SyntaxNode,
}
impl CssNumber {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssNumberFields {
        CssNumberFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssNumberFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssParameter {
    pub(crate) syntax: SyntaxNode,
}
impl CssParameter {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssParameterFields {
        CssParameterFields {
            css_component_value_list: self.css_component_value_list(),
        }
    }
    pub fn css_component_value_list(&self) -> CssComponentValueList {
        support::list(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssParameter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssParameterFields {
    pub css_component_value_list: CssComponentValueList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPercentDimension {
    pub(crate) syntax: SyntaxNode,
}
impl CssPercentDimension {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPercentDimensionFields {
        CssPercentDimensionFields {
            value: self.value(),
            unit_token: self.unit_token(),
        }
    }
    pub fn value(&self) -> SyntaxResult<CssNumber> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn unit_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPercentDimension {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPercentDimensionFields {
    pub value: SyntaxResult<CssNumber>,
    pub unit_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPercentage {
    pub(crate) syntax: SyntaxNode,
}
impl CssPercentage {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPercentageFields {
        CssPercentageFields {
            value: self.value(),
            reminder_token: self.reminder_token(),
        }
    }
    pub fn value(&self) -> SyntaxResult<CssNumber> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn reminder_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPercentage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPercentageFields {
    pub value: SyntaxResult<CssNumber>,
    pub reminder_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassFunctionCompoundSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassFunctionCompoundSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassFunctionCompoundSelectorFields {
        CssPseudoClassFunctionCompoundSelectorFields {
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            selector: self.selector(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn selector(&self) -> SyntaxResult<AnyCssCompoundSelector> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassFunctionCompoundSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassFunctionCompoundSelectorFields {
    pub name: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub selector: SyntaxResult<AnyCssCompoundSelector>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassFunctionCompoundSelectorList {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassFunctionCompoundSelectorList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassFunctionCompoundSelectorListFields {
        CssPseudoClassFunctionCompoundSelectorListFields {
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            compound_selectors: self.compound_selectors(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn compound_selectors(&self) -> CssCompoundSelectorList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassFunctionCompoundSelectorList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassFunctionCompoundSelectorListFields {
    pub name: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub compound_selectors: CssCompoundSelectorList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassFunctionIdentifier {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassFunctionIdentifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassFunctionIdentifierFields {
        CssPseudoClassFunctionIdentifierFields {
            name_token: self.name_token(),
            l_paren_token: self.l_paren_token(),
            ident: self.ident(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn ident(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassFunctionIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassFunctionIdentifierFields {
    pub name_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub ident: SyntaxResult<CssIdentifier>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassFunctionNth {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassFunctionNth {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassFunctionNthFields {
        CssPseudoClassFunctionNthFields {
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            selector: self.selector(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn selector(&self) -> SyntaxResult<AnyCssPseudoClassNthSelector> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassFunctionNth {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassFunctionNthFields {
    pub name: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub selector: SyntaxResult<AnyCssPseudoClassNthSelector>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassFunctionRelativeSelectorList {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassFunctionRelativeSelectorList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassFunctionRelativeSelectorListFields {
        CssPseudoClassFunctionRelativeSelectorListFields {
            name_token: self.name_token(),
            l_paren_token: self.l_paren_token(),
            relative_selectors: self.relative_selectors(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn relative_selectors(&self) -> CssRelativeSelectorList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassFunctionRelativeSelectorList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassFunctionRelativeSelectorListFields {
    pub name_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub relative_selectors: CssRelativeSelectorList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassFunctionSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassFunctionSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassFunctionSelectorFields {
        CssPseudoClassFunctionSelectorFields {
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            selector: self.selector(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn selector(&self) -> SyntaxResult<AnyCssSelector> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassFunctionSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassFunctionSelectorFields {
    pub name: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub selector: SyntaxResult<AnyCssSelector>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassFunctionSelectorList {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassFunctionSelectorList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassFunctionSelectorListFields {
        CssPseudoClassFunctionSelectorListFields {
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            selectors: self.selectors(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn selectors(&self) -> CssSelectorList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassFunctionSelectorList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassFunctionSelectorListFields {
    pub name: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub selectors: CssSelectorList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassFunctionValueList {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassFunctionValueList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassFunctionValueListFields {
        CssPseudoClassFunctionValueListFields {
            name_token: self.name_token(),
            l_paren_token: self.l_paren_token(),
            values: self.values(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn values(&self) -> CssPseudoValueList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassFunctionValueList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassFunctionValueListFields {
    pub name_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub values: CssPseudoValueList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassIdentifier {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassIdentifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassIdentifierFields {
        CssPseudoClassIdentifierFields { name: self.name() }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassIdentifierFields {
    pub name: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassNth {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassNth {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassNthFields {
        CssPseudoClassNthFields {
            sign: self.sign(),
            value: self.value(),
            symbol_token: self.symbol_token(),
            offset: self.offset(),
        }
    }
    pub fn sign(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> Option<CssNumber> {
        support::node(&self.syntax, 1usize)
    }
    pub fn symbol_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn offset(&self) -> Option<CssNthOffset> {
        support::node(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassNth {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassNthFields {
    pub sign: Option<SyntaxToken>,
    pub value: Option<CssNumber>,
    pub symbol_token: SyntaxResult<SyntaxToken>,
    pub offset: Option<CssNthOffset>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassNthIdentifier {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassNthIdentifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassNthIdentifierFields {
        CssPseudoClassNthIdentifierFields {
            value: self.value(),
        }
    }
    pub fn value(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassNthIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassNthIdentifierFields {
    pub value: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassNthNumber {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassNthNumber {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassNthNumberFields {
        CssPseudoClassNthNumberFields {
            sign: self.sign(),
            value: self.value(),
        }
    }
    pub fn sign(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<CssNumber> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassNthNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassNthNumberFields {
    pub sign: Option<SyntaxToken>,
    pub value: SyntaxResult<CssNumber>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassNthSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassNthSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassNthSelectorFields {
        CssPseudoClassNthSelectorFields {
            nth: self.nth(),
            of_selector: self.of_selector(),
        }
    }
    pub fn nth(&self) -> SyntaxResult<AnyCssPseudoClassNth> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn of_selector(&self) -> Option<CssPseudoClassOfNthSelector> {
        support::node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassNthSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassNthSelectorFields {
    pub nth: SyntaxResult<AnyCssPseudoClassNth>,
    pub of_selector: Option<CssPseudoClassOfNthSelector>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassOfNthSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassOfNthSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassOfNthSelectorFields {
        CssPseudoClassOfNthSelectorFields {
            of_token: self.of_token(),
            selectors: self.selectors(),
        }
    }
    pub fn of_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn selectors(&self) -> CssSelectorList {
        support::list(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassOfNthSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassOfNthSelectorFields {
    pub of_token: SyntaxResult<SyntaxToken>,
    pub selectors: CssSelectorList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoClassSelectorFields {
        CssPseudoClassSelectorFields {
            colon_token: self.colon_token(),
            class: self.class(),
        }
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn class(&self) -> SyntaxResult<AnyCssPseudoClass> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassSelectorFields {
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub class: SyntaxResult<AnyCssPseudoClass>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoElementFunctionIdentifier {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoElementFunctionIdentifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoElementFunctionIdentifierFields {
        CssPseudoElementFunctionIdentifierFields {
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            ident: self.ident(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn ident(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoElementFunctionIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoElementFunctionIdentifierFields {
    pub name: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub ident: SyntaxResult<CssIdentifier>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoElementFunctionSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoElementFunctionSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoElementFunctionSelectorFields {
        CssPseudoElementFunctionSelectorFields {
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            selector: self.selector(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn selector(&self) -> SyntaxResult<AnyCssSelector> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoElementFunctionSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoElementFunctionSelectorFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub selector: SyntaxResult<AnyCssSelector>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoElementIdentifier {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoElementIdentifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoElementIdentifierFields {
        CssPseudoElementIdentifierFields { name: self.name() }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoElementIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoElementIdentifierFields {
    pub name: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoElementSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoElementSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssPseudoElementSelectorFields {
        CssPseudoElementSelectorFields {
            double_colon_token: self.double_colon_token(),
            element: self.element(),
        }
    }
    pub fn double_colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn element(&self) -> SyntaxResult<AnyCssPseudoElement> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoElementSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoElementSelectorFields {
    pub double_colon_token: SyntaxResult<SyntaxToken>,
    pub element: SyntaxResult<AnyCssPseudoElement>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssQueryFeatureBoolean {
    pub(crate) syntax: SyntaxNode,
}
impl CssQueryFeatureBoolean {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssQueryFeatureBooleanFields {
        CssQueryFeatureBooleanFields { name: self.name() }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssQueryFeatureBoolean {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssQueryFeatureBooleanFields {
    pub name: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssQueryFeaturePlain {
    pub(crate) syntax: SyntaxNode,
}
impl CssQueryFeaturePlain {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssQueryFeaturePlainFields {
        CssQueryFeaturePlainFields {
            name: self.name(),
            colon_token: self.colon_token(),
            value: self.value(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> SyntaxResult<AnyCssQueryFeatureValue> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssQueryFeaturePlain {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssQueryFeaturePlainFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AnyCssQueryFeatureValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssQueryFeatureRange {
    pub(crate) syntax: SyntaxNode,
}
impl CssQueryFeatureRange {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssQueryFeatureRangeFields {
        CssQueryFeatureRangeFields {
            left: self.left(),
            comparison: self.comparison(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn comparison(&self) -> SyntaxResult<CssQueryFeatureRangeComparison> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyCssQueryFeatureValue> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssQueryFeatureRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssQueryFeatureRangeFields {
    pub left: SyntaxResult<CssIdentifier>,
    pub comparison: SyntaxResult<CssQueryFeatureRangeComparison>,
    pub right: SyntaxResult<AnyCssQueryFeatureValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssQueryFeatureRangeComparison {
    pub(crate) syntax: SyntaxNode,
}
impl CssQueryFeatureRangeComparison {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssQueryFeatureRangeComparisonFields {
        CssQueryFeatureRangeComparisonFields {
            operator: self.operator(),
        }
    }
    pub fn operator(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssQueryFeatureRangeComparison {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssQueryFeatureRangeComparisonFields {
    pub operator: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssQueryFeatureRangeInterval {
    pub(crate) syntax: SyntaxNode,
}
impl CssQueryFeatureRangeInterval {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssQueryFeatureRangeIntervalFields {
        CssQueryFeatureRangeIntervalFields {
            left: self.left(),
            left_comparison: self.left_comparison(),
            name: self.name(),
            right_comparison: self.right_comparison(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyCssQueryFeatureValue> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn left_comparison(&self) -> SyntaxResult<CssQueryFeatureRangeComparison> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn right_comparison(&self) -> SyntaxResult<CssQueryFeatureRangeComparison> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyCssQueryFeatureValue> {
        support::required_node(&self.syntax, 4usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssQueryFeatureRangeInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssQueryFeatureRangeIntervalFields {
    pub left: SyntaxResult<AnyCssQueryFeatureValue>,
    pub left_comparison: SyntaxResult<CssQueryFeatureRangeComparison>,
    pub name: SyntaxResult<CssIdentifier>,
    pub right_comparison: SyntaxResult<CssQueryFeatureRangeComparison>,
    pub right: SyntaxResult<AnyCssQueryFeatureValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssQueryFeatureReverseRange {
    pub(crate) syntax: SyntaxNode,
}
impl CssQueryFeatureReverseRange {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssQueryFeatureReverseRangeFields {
        CssQueryFeatureReverseRangeFields {
            left: self.left(),
            comparison: self.comparison(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyCssQueryFeatureValue> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn comparison(&self) -> SyntaxResult<CssQueryFeatureRangeComparison> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssQueryFeatureReverseRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssQueryFeatureReverseRangeFields {
    pub left: SyntaxResult<AnyCssQueryFeatureValue>,
    pub comparison: SyntaxResult<CssQueryFeatureRangeComparison>,
    pub right: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssRatio {
    pub(crate) syntax: SyntaxNode,
}
impl CssRatio {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssRatioFields {
        CssRatioFields {
            numerator: self.numerator(),
            slash_token: self.slash_token(),
            denominator: self.denominator(),
        }
    }
    pub fn numerator(&self) -> SyntaxResult<CssNumber> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn slash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn denominator(&self) -> SyntaxResult<CssNumber> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssRatio {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssRatioFields {
    pub numerator: SyntaxResult<CssNumber>,
    pub slash_token: SyntaxResult<SyntaxToken>,
    pub denominator: SyntaxResult<CssNumber>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssRegularDimension {
    pub(crate) syntax: SyntaxNode,
}
impl CssRegularDimension {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssRegularDimensionFields {
        CssRegularDimensionFields {
            value: self.value(),
            unit: self.unit(),
        }
    }
    pub fn value(&self) -> SyntaxResult<CssNumber> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn unit(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssRegularDimension {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssRegularDimensionFields {
    pub value: SyntaxResult<CssNumber>,
    pub unit: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssRelativeSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssRelativeSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssRelativeSelectorFields {
        CssRelativeSelectorFields {
            combinator: self.combinator(),
            selector: self.selector(),
        }
    }
    pub fn combinator(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn selector(&self) -> SyntaxResult<AnyCssSelector> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssRelativeSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssRelativeSelectorFields {
    pub combinator: Option<SyntaxToken>,
    pub selector: SyntaxResult<AnyCssSelector>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssRoot {
    pub(crate) syntax: SyntaxNode,
}
impl CssRoot {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssRootFields {
        CssRootFields {
            bom_token: self.bom_token(),
            rules: self.rules(),
            eof_token: self.eof_token(),
        }
    }
    pub fn bom_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn rules(&self) -> CssRuleList {
        support::list(&self.syntax, 1usize)
    }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssRoot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssRootFields {
    pub bom_token: Option<SyntaxToken>,
    pub rules: CssRuleList,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssRuleFields {
        CssRuleFields {
            prelude: self.prelude(),
            block: self.block(),
        }
    }
    pub fn prelude(&self) -> CssSelectorList {
        support::list(&self.syntax, 0usize)
    }
    pub fn block(&self) -> SyntaxResult<AnyCssDeclarationListBlock> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssRuleFields {
    pub prelude: CssSelectorList,
    pub block: SyntaxResult<AnyCssDeclarationListBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssRuleListBlock {
    pub(crate) syntax: SyntaxNode,
}
impl CssRuleListBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssRuleListBlockFields {
        CssRuleListBlockFields {
            l_curly_token: self.l_curly_token(),
            rules: self.rules(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn rules(&self) -> CssRuleList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssRuleListBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssRuleListBlockFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub rules: CssRuleList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssSimpleFunction {
    pub(crate) syntax: SyntaxNode,
}
impl CssSimpleFunction {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssSimpleFunctionFields {
        CssSimpleFunctionFields {
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            items: self.items(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn items(&self) -> CssParameterList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssSimpleFunction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssSimpleFunctionFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub items: CssParameterList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssString {
    pub(crate) syntax: SyntaxNode,
}
impl CssString {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssStringFields {
        CssStringFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssStringFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssTypeSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssTypeSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssTypeSelectorFields {
        CssTypeSelectorFields {
            namespace: self.namespace(),
            ident: self.ident(),
        }
    }
    pub fn namespace(&self) -> Option<CssNamespace> {
        support::node(&self.syntax, 0usize)
    }
    pub fn ident(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssTypeSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssTypeSelectorFields {
    pub namespace: Option<CssNamespace>,
    pub ident: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssUniversalNamespacePrefix {
    pub(crate) syntax: SyntaxNode,
}
impl CssUniversalNamespacePrefix {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssUniversalNamespacePrefixFields {
        CssUniversalNamespacePrefixFields {
            star_token: self.star_token(),
        }
    }
    pub fn star_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssUniversalNamespacePrefix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssUniversalNamespacePrefixFields {
    pub star_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssUniversalSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssUniversalSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssUniversalSelectorFields {
        CssUniversalSelectorFields {
            namespace: self.namespace(),
            star_token: self.star_token(),
        }
    }
    pub fn namespace(&self) -> Option<CssNamespace> {
        support::node(&self.syntax, 0usize)
    }
    pub fn star_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssUniversalSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssUniversalSelectorFields {
    pub namespace: Option<CssNamespace>,
    pub star_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssVarFunction {
    pub(crate) syntax: SyntaxNode,
}
impl CssVarFunction {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssVarFunctionFields {
        CssVarFunctionFields {
            var_token: self.var_token(),
            l_paren_token: self.l_paren_token(),
            property: self.property(),
            value: self.value(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn var_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn property(&self) -> SyntaxResult<CssCustomProperty> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn value(&self) -> Option<CssVarFunctionValue> {
        support::node(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssVarFunction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssVarFunctionFields {
    pub var_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub property: SyntaxResult<CssCustomProperty>,
    pub value: Option<CssVarFunctionValue>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssVarFunctionValue {
    pub(crate) syntax: SyntaxNode,
}
impl CssVarFunctionValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CssVarFunctionValueFields {
        CssVarFunctionValueFields {
            comma_token: self.comma_token(),
            value: self.value(),
        }
    }
    pub fn comma_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssVarFunctionValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssVarFunctionValueFields {
    pub comma_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssAtRule {
    CssBogusAtRule(CssBogusAtRule),
    CssCharsetAtRule(CssCharsetAtRule),
    CssColorProfileAtRule(CssColorProfileAtRule),
    CssContainerAtRule(CssContainerAtRule),
    CssCounterStyleAtRule(CssCounterStyleAtRule),
    CssFontFaceAtRule(CssFontFaceAtRule),
    CssFontPaletteValuesAtRule(CssFontPaletteValuesAtRule),
    CssKeyframesAtRule(CssKeyframesAtRule),
    CssMediaAtRule(CssMediaAtRule),
}
impl AnyCssAtRule {
    pub fn as_css_bogus_at_rule(&self) -> Option<&CssBogusAtRule> {
        match &self {
            AnyCssAtRule::CssBogusAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_charset_at_rule(&self) -> Option<&CssCharsetAtRule> {
        match &self {
            AnyCssAtRule::CssCharsetAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_color_profile_at_rule(&self) -> Option<&CssColorProfileAtRule> {
        match &self {
            AnyCssAtRule::CssColorProfileAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_at_rule(&self) -> Option<&CssContainerAtRule> {
        match &self {
            AnyCssAtRule::CssContainerAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_counter_style_at_rule(&self) -> Option<&CssCounterStyleAtRule> {
        match &self {
            AnyCssAtRule::CssCounterStyleAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_font_face_at_rule(&self) -> Option<&CssFontFaceAtRule> {
        match &self {
            AnyCssAtRule::CssFontFaceAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_font_palette_values_at_rule(&self) -> Option<&CssFontPaletteValuesAtRule> {
        match &self {
            AnyCssAtRule::CssFontPaletteValuesAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_keyframes_at_rule(&self) -> Option<&CssKeyframesAtRule> {
        match &self {
            AnyCssAtRule::CssKeyframesAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_media_at_rule(&self) -> Option<&CssMediaAtRule> {
        match &self {
            AnyCssAtRule::CssMediaAtRule(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssAttributeMatcherValue {
    CssIdentifier(CssIdentifier),
    CssString(CssString),
}
impl AnyCssAttributeMatcherValue {
    pub fn as_css_identifier(&self) -> Option<&CssIdentifier> {
        match &self {
            AnyCssAttributeMatcherValue::CssIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_string(&self) -> Option<&CssString> {
        match &self {
            AnyCssAttributeMatcherValue::CssString(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssCompoundSelector {
    CssBogusSelector(CssBogusSelector),
    CssCompoundSelector(CssCompoundSelector),
}
impl AnyCssCompoundSelector {
    pub fn as_css_bogus_selector(&self) -> Option<&CssBogusSelector> {
        match &self {
            AnyCssCompoundSelector::CssBogusSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_compound_selector(&self) -> Option<&CssCompoundSelector> {
        match &self {
            AnyCssCompoundSelector::CssCompoundSelector(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssContainerAndCombinableQuery {
    AnyCssContainerQueryInParens(AnyCssContainerQueryInParens),
    CssContainerAndQuery(CssContainerAndQuery),
}
impl AnyCssContainerAndCombinableQuery {
    pub fn as_any_css_container_query_in_parens(&self) -> Option<&AnyCssContainerQueryInParens> {
        match &self {
            AnyCssContainerAndCombinableQuery::AnyCssContainerQueryInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_and_query(&self) -> Option<&CssContainerAndQuery> {
        match &self {
            AnyCssContainerAndCombinableQuery::CssContainerAndQuery(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssContainerOrCombinableQuery {
    AnyCssContainerQueryInParens(AnyCssContainerQueryInParens),
    CssContainerOrQuery(CssContainerOrQuery),
}
impl AnyCssContainerOrCombinableQuery {
    pub fn as_any_css_container_query_in_parens(&self) -> Option<&AnyCssContainerQueryInParens> {
        match &self {
            AnyCssContainerOrCombinableQuery::AnyCssContainerQueryInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_or_query(&self) -> Option<&CssContainerOrQuery> {
        match &self {
            AnyCssContainerOrCombinableQuery::CssContainerOrQuery(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssContainerQuery {
    AnyCssContainerQueryInParens(AnyCssContainerQueryInParens),
    CssContainerAndQuery(CssContainerAndQuery),
    CssContainerNotQuery(CssContainerNotQuery),
    CssContainerOrQuery(CssContainerOrQuery),
}
impl AnyCssContainerQuery {
    pub fn as_any_css_container_query_in_parens(&self) -> Option<&AnyCssContainerQueryInParens> {
        match &self {
            AnyCssContainerQuery::AnyCssContainerQueryInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_and_query(&self) -> Option<&CssContainerAndQuery> {
        match &self {
            AnyCssContainerQuery::CssContainerAndQuery(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_not_query(&self) -> Option<&CssContainerNotQuery> {
        match &self {
            AnyCssContainerQuery::CssContainerNotQuery(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_or_query(&self) -> Option<&CssContainerOrQuery> {
        match &self {
            AnyCssContainerQuery::CssContainerOrQuery(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssContainerQueryInParens {
    CssContainerQueryInParens(CssContainerQueryInParens),
    CssContainerSizeFeatureInParens(CssContainerSizeFeatureInParens),
    CssContainerStyleQueryInParens(CssContainerStyleQueryInParens),
}
impl AnyCssContainerQueryInParens {
    pub fn as_css_container_query_in_parens(&self) -> Option<&CssContainerQueryInParens> {
        match &self {
            AnyCssContainerQueryInParens::CssContainerQueryInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_size_feature_in_parens(
        &self,
    ) -> Option<&CssContainerSizeFeatureInParens> {
        match &self {
            AnyCssContainerQueryInParens::CssContainerSizeFeatureInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_style_query_in_parens(
        &self,
    ) -> Option<&CssContainerStyleQueryInParens> {
        match &self {
            AnyCssContainerQueryInParens::CssContainerStyleQueryInParens(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssContainerStyleAndCombinableQuery {
    CssContainerStyleAndQuery(CssContainerStyleAndQuery),
    CssContainerStyleInParens(CssContainerStyleInParens),
}
impl AnyCssContainerStyleAndCombinableQuery {
    pub fn as_css_container_style_and_query(&self) -> Option<&CssContainerStyleAndQuery> {
        match &self {
            AnyCssContainerStyleAndCombinableQuery::CssContainerStyleAndQuery(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_style_in_parens(&self) -> Option<&CssContainerStyleInParens> {
        match &self {
            AnyCssContainerStyleAndCombinableQuery::CssContainerStyleInParens(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssContainerStyleInParens {
    AnyCssContainerStyleQuery(AnyCssContainerStyleQuery),
    CssDeclaration(CssDeclaration),
}
impl AnyCssContainerStyleInParens {
    pub fn as_any_css_container_style_query(&self) -> Option<&AnyCssContainerStyleQuery> {
        match &self {
            AnyCssContainerStyleInParens::AnyCssContainerStyleQuery(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_declaration(&self) -> Option<&CssDeclaration> {
        match &self {
            AnyCssContainerStyleInParens::CssDeclaration(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssContainerStyleOrCombinableQuery {
    CssContainerStyleInParens(CssContainerStyleInParens),
    CssContainerStyleOrQuery(CssContainerStyleOrQuery),
}
impl AnyCssContainerStyleOrCombinableQuery {
    pub fn as_css_container_style_in_parens(&self) -> Option<&CssContainerStyleInParens> {
        match &self {
            AnyCssContainerStyleOrCombinableQuery::CssContainerStyleInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_style_or_query(&self) -> Option<&CssContainerStyleOrQuery> {
        match &self {
            AnyCssContainerStyleOrCombinableQuery::CssContainerStyleOrQuery(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssContainerStyleQuery {
    CssContainerStyleAndQuery(CssContainerStyleAndQuery),
    CssContainerStyleInParens(CssContainerStyleInParens),
    CssContainerStyleNotQuery(CssContainerStyleNotQuery),
    CssContainerStyleOrQuery(CssContainerStyleOrQuery),
    CssDeclaration(CssDeclaration),
}
impl AnyCssContainerStyleQuery {
    pub fn as_css_container_style_and_query(&self) -> Option<&CssContainerStyleAndQuery> {
        match &self {
            AnyCssContainerStyleQuery::CssContainerStyleAndQuery(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_style_in_parens(&self) -> Option<&CssContainerStyleInParens> {
        match &self {
            AnyCssContainerStyleQuery::CssContainerStyleInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_style_not_query(&self) -> Option<&CssContainerStyleNotQuery> {
        match &self {
            AnyCssContainerStyleQuery::CssContainerStyleNotQuery(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_container_style_or_query(&self) -> Option<&CssContainerStyleOrQuery> {
        match &self {
            AnyCssContainerStyleQuery::CssContainerStyleOrQuery(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_declaration(&self) -> Option<&CssDeclaration> {
        match &self {
            AnyCssContainerStyleQuery::CssDeclaration(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssDeclarationListBlock {
    CssBogusBlock(CssBogusBlock),
    CssDeclarationListBlock(CssDeclarationListBlock),
}
impl AnyCssDeclarationListBlock {
    pub fn as_css_bogus_block(&self) -> Option<&CssBogusBlock> {
        match &self {
            AnyCssDeclarationListBlock::CssBogusBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_declaration_list_block(&self) -> Option<&CssDeclarationListBlock> {
        match &self {
            AnyCssDeclarationListBlock::CssDeclarationListBlock(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssDeclarationName {
    CssCustomProperty(CssCustomProperty),
    CssIdentifier(CssIdentifier),
}
impl AnyCssDeclarationName {
    pub fn as_css_custom_property(&self) -> Option<&CssCustomProperty> {
        match &self {
            AnyCssDeclarationName::CssCustomProperty(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_identifier(&self) -> Option<&CssIdentifier> {
        match &self {
            AnyCssDeclarationName::CssIdentifier(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssDimension {
    CssPercentage(CssPercentage),
    CssRegularDimension(CssRegularDimension),
}
impl AnyCssDimension {
    pub fn as_css_percentage(&self) -> Option<&CssPercentage> {
        match &self {
            AnyCssDimension::CssPercentage(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_regular_dimension(&self) -> Option<&CssRegularDimension> {
        match &self {
            AnyCssDimension::CssRegularDimension(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssKeyframeName {
    CssIdentifier(CssIdentifier),
    CssString(CssString),
}
impl AnyCssKeyframeName {
    pub fn as_css_identifier(&self) -> Option<&CssIdentifier> {
        match &self {
            AnyCssKeyframeName::CssIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_string(&self) -> Option<&CssString> {
        match &self {
            AnyCssKeyframeName::CssString(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssKeyframesBlock {
    CssBogusBlock(CssBogusBlock),
    CssKeyframesBlock(CssKeyframesBlock),
}
impl AnyCssKeyframesBlock {
    pub fn as_css_bogus_block(&self) -> Option<&CssBogusBlock> {
        match &self {
            AnyCssKeyframesBlock::CssBogusBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_keyframes_block(&self) -> Option<&CssKeyframesBlock> {
        match &self {
            AnyCssKeyframesBlock::CssKeyframesBlock(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssKeyframesItem {
    CssBogusKeyframesItem(CssBogusKeyframesItem),
    CssKeyframesItem(CssKeyframesItem),
}
impl AnyCssKeyframesItem {
    pub fn as_css_bogus_keyframes_item(&self) -> Option<&CssBogusKeyframesItem> {
        match &self {
            AnyCssKeyframesItem::CssBogusKeyframesItem(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_keyframes_item(&self) -> Option<&CssKeyframesItem> {
        match &self {
            AnyCssKeyframesItem::CssKeyframesItem(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssKeyframesSelector {
    CssBogusSelector(CssBogusSelector),
    CssKeyframesIdentSelector(CssKeyframesIdentSelector),
    CssKeyframesPercentageSelector(CssKeyframesPercentageSelector),
}
impl AnyCssKeyframesSelector {
    pub fn as_css_bogus_selector(&self) -> Option<&CssBogusSelector> {
        match &self {
            AnyCssKeyframesSelector::CssBogusSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_keyframes_ident_selector(&self) -> Option<&CssKeyframesIdentSelector> {
        match &self {
            AnyCssKeyframesSelector::CssKeyframesIdentSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_keyframes_percentage_selector(&self) -> Option<&CssKeyframesPercentageSelector> {
        match &self {
            AnyCssKeyframesSelector::CssKeyframesPercentageSelector(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssMediaAndCombinableCondition {
    AnyCssMediaInParens(AnyCssMediaInParens),
    CssMediaAndCondition(CssMediaAndCondition),
}
impl AnyCssMediaAndCombinableCondition {
    pub fn as_any_css_media_in_parens(&self) -> Option<&AnyCssMediaInParens> {
        match &self {
            AnyCssMediaAndCombinableCondition::AnyCssMediaInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_media_and_condition(&self) -> Option<&CssMediaAndCondition> {
        match &self {
            AnyCssMediaAndCombinableCondition::CssMediaAndCondition(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssMediaCondition {
    AnyCssMediaInParens(AnyCssMediaInParens),
    CssMediaAndCondition(CssMediaAndCondition),
    CssMediaNotCondition(CssMediaNotCondition),
    CssMediaOrCondition(CssMediaOrCondition),
}
impl AnyCssMediaCondition {
    pub fn as_any_css_media_in_parens(&self) -> Option<&AnyCssMediaInParens> {
        match &self {
            AnyCssMediaCondition::AnyCssMediaInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_media_and_condition(&self) -> Option<&CssMediaAndCondition> {
        match &self {
            AnyCssMediaCondition::CssMediaAndCondition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_media_not_condition(&self) -> Option<&CssMediaNotCondition> {
        match &self {
            AnyCssMediaCondition::CssMediaNotCondition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_media_or_condition(&self) -> Option<&CssMediaOrCondition> {
        match &self {
            AnyCssMediaCondition::CssMediaOrCondition(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssMediaInParens {
    CssMediaConditionInParens(CssMediaConditionInParens),
    CssMediaFeatureInParens(CssMediaFeatureInParens),
}
impl AnyCssMediaInParens {
    pub fn as_css_media_condition_in_parens(&self) -> Option<&CssMediaConditionInParens> {
        match &self {
            AnyCssMediaInParens::CssMediaConditionInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_media_feature_in_parens(&self) -> Option<&CssMediaFeatureInParens> {
        match &self {
            AnyCssMediaInParens::CssMediaFeatureInParens(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssMediaOrCombinableCondition {
    AnyCssMediaInParens(AnyCssMediaInParens),
    CssMediaOrCondition(CssMediaOrCondition),
}
impl AnyCssMediaOrCombinableCondition {
    pub fn as_any_css_media_in_parens(&self) -> Option<&AnyCssMediaInParens> {
        match &self {
            AnyCssMediaOrCombinableCondition::AnyCssMediaInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_media_or_condition(&self) -> Option<&CssMediaOrCondition> {
        match &self {
            AnyCssMediaOrCombinableCondition::CssMediaOrCondition(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssMediaQuery {
    AnyCssMediaTypeQuery(AnyCssMediaTypeQuery),
    CssBogusMediaQuery(CssBogusMediaQuery),
    CssMediaConditionQuery(CssMediaConditionQuery),
}
impl AnyCssMediaQuery {
    pub fn as_any_css_media_type_query(&self) -> Option<&AnyCssMediaTypeQuery> {
        match &self {
            AnyCssMediaQuery::AnyCssMediaTypeQuery(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_bogus_media_query(&self) -> Option<&CssBogusMediaQuery> {
        match &self {
            AnyCssMediaQuery::CssBogusMediaQuery(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_media_condition_query(&self) -> Option<&CssMediaConditionQuery> {
        match &self {
            AnyCssMediaQuery::CssMediaConditionQuery(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssMediaTypeCondition {
    AnyCssMediaInParens(AnyCssMediaInParens),
    CssMediaAndCondition(CssMediaAndCondition),
    CssMediaNotCondition(CssMediaNotCondition),
}
impl AnyCssMediaTypeCondition {
    pub fn as_any_css_media_in_parens(&self) -> Option<&AnyCssMediaInParens> {
        match &self {
            AnyCssMediaTypeCondition::AnyCssMediaInParens(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_media_and_condition(&self) -> Option<&CssMediaAndCondition> {
        match &self {
            AnyCssMediaTypeCondition::CssMediaAndCondition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_media_not_condition(&self) -> Option<&CssMediaNotCondition> {
        match &self {
            AnyCssMediaTypeCondition::CssMediaNotCondition(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssMediaTypeQuery {
    CssMediaAndTypeQuery(CssMediaAndTypeQuery),
    CssMediaTypeQuery(CssMediaTypeQuery),
}
impl AnyCssMediaTypeQuery {
    pub fn as_css_media_and_type_query(&self) -> Option<&CssMediaAndTypeQuery> {
        match &self {
            AnyCssMediaTypeQuery::CssMediaAndTypeQuery(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_media_type_query(&self) -> Option<&CssMediaTypeQuery> {
        match &self {
            AnyCssMediaTypeQuery::CssMediaTypeQuery(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssNamespacePrefix {
    CssNamedNamespacePrefix(CssNamedNamespacePrefix),
    CssUniversalNamespacePrefix(CssUniversalNamespacePrefix),
}
impl AnyCssNamespacePrefix {
    pub fn as_css_named_namespace_prefix(&self) -> Option<&CssNamedNamespacePrefix> {
        match &self {
            AnyCssNamespacePrefix::CssNamedNamespacePrefix(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_universal_namespace_prefix(&self) -> Option<&CssUniversalNamespacePrefix> {
        match &self {
            AnyCssNamespacePrefix::CssUniversalNamespacePrefix(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssPseudoClass {
    CssBogusPseudoClass(CssBogusPseudoClass),
    CssPseudoClassFunctionCompoundSelector(CssPseudoClassFunctionCompoundSelector),
    CssPseudoClassFunctionCompoundSelectorList(CssPseudoClassFunctionCompoundSelectorList),
    CssPseudoClassFunctionIdentifier(CssPseudoClassFunctionIdentifier),
    CssPseudoClassFunctionNth(CssPseudoClassFunctionNth),
    CssPseudoClassFunctionRelativeSelectorList(CssPseudoClassFunctionRelativeSelectorList),
    CssPseudoClassFunctionSelector(CssPseudoClassFunctionSelector),
    CssPseudoClassFunctionSelectorList(CssPseudoClassFunctionSelectorList),
    CssPseudoClassFunctionValueList(CssPseudoClassFunctionValueList),
    CssPseudoClassIdentifier(CssPseudoClassIdentifier),
}
impl AnyCssPseudoClass {
    pub fn as_css_bogus_pseudo_class(&self) -> Option<&CssBogusPseudoClass> {
        match &self {
            AnyCssPseudoClass::CssBogusPseudoClass(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_function_compound_selector(
        &self,
    ) -> Option<&CssPseudoClassFunctionCompoundSelector> {
        match &self {
            AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_function_compound_selector_list(
        &self,
    ) -> Option<&CssPseudoClassFunctionCompoundSelectorList> {
        match &self {
            AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelectorList(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_function_identifier(
        &self,
    ) -> Option<&CssPseudoClassFunctionIdentifier> {
        match &self {
            AnyCssPseudoClass::CssPseudoClassFunctionIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_function_nth(&self) -> Option<&CssPseudoClassFunctionNth> {
        match &self {
            AnyCssPseudoClass::CssPseudoClassFunctionNth(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_function_relative_selector_list(
        &self,
    ) -> Option<&CssPseudoClassFunctionRelativeSelectorList> {
        match &self {
            AnyCssPseudoClass::CssPseudoClassFunctionRelativeSelectorList(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_function_selector(&self) -> Option<&CssPseudoClassFunctionSelector> {
        match &self {
            AnyCssPseudoClass::CssPseudoClassFunctionSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_function_selector_list(
        &self,
    ) -> Option<&CssPseudoClassFunctionSelectorList> {
        match &self {
            AnyCssPseudoClass::CssPseudoClassFunctionSelectorList(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_function_value_list(
        &self,
    ) -> Option<&CssPseudoClassFunctionValueList> {
        match &self {
            AnyCssPseudoClass::CssPseudoClassFunctionValueList(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_identifier(&self) -> Option<&CssPseudoClassIdentifier> {
        match &self {
            AnyCssPseudoClass::CssPseudoClassIdentifier(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssPseudoClassNth {
    CssPseudoClassNth(CssPseudoClassNth),
    CssPseudoClassNthIdentifier(CssPseudoClassNthIdentifier),
    CssPseudoClassNthNumber(CssPseudoClassNthNumber),
}
impl AnyCssPseudoClassNth {
    pub fn as_css_pseudo_class_nth(&self) -> Option<&CssPseudoClassNth> {
        match &self {
            AnyCssPseudoClassNth::CssPseudoClassNth(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_nth_identifier(&self) -> Option<&CssPseudoClassNthIdentifier> {
        match &self {
            AnyCssPseudoClassNth::CssPseudoClassNthIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_nth_number(&self) -> Option<&CssPseudoClassNthNumber> {
        match &self {
            AnyCssPseudoClassNth::CssPseudoClassNthNumber(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssPseudoClassNthSelector {
    CssBogusSelector(CssBogusSelector),
    CssPseudoClassNthSelector(CssPseudoClassNthSelector),
}
impl AnyCssPseudoClassNthSelector {
    pub fn as_css_bogus_selector(&self) -> Option<&CssBogusSelector> {
        match &self {
            AnyCssPseudoClassNthSelector::CssBogusSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_nth_selector(&self) -> Option<&CssPseudoClassNthSelector> {
        match &self {
            AnyCssPseudoClassNthSelector::CssPseudoClassNthSelector(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssPseudoElement {
    CssBogusPseudoElement(CssBogusPseudoElement),
    CssPseudoElementFunctionIdentifier(CssPseudoElementFunctionIdentifier),
    CssPseudoElementFunctionSelector(CssPseudoElementFunctionSelector),
    CssPseudoElementIdentifier(CssPseudoElementIdentifier),
}
impl AnyCssPseudoElement {
    pub fn as_css_bogus_pseudo_element(&self) -> Option<&CssBogusPseudoElement> {
        match &self {
            AnyCssPseudoElement::CssBogusPseudoElement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_element_function_identifier(
        &self,
    ) -> Option<&CssPseudoElementFunctionIdentifier> {
        match &self {
            AnyCssPseudoElement::CssPseudoElementFunctionIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_element_function_selector(
        &self,
    ) -> Option<&CssPseudoElementFunctionSelector> {
        match &self {
            AnyCssPseudoElement::CssPseudoElementFunctionSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_element_identifier(&self) -> Option<&CssPseudoElementIdentifier> {
        match &self {
            AnyCssPseudoElement::CssPseudoElementIdentifier(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssPseudoValue {
    CssIdentifier(CssIdentifier),
    CssString(CssString),
}
impl AnyCssPseudoValue {
    pub fn as_css_identifier(&self) -> Option<&CssIdentifier> {
        match &self {
            AnyCssPseudoValue::CssIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_string(&self) -> Option<&CssString> {
        match &self {
            AnyCssPseudoValue::CssString(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssQueryFeature {
    CssQueryFeatureBoolean(CssQueryFeatureBoolean),
    CssQueryFeaturePlain(CssQueryFeaturePlain),
    CssQueryFeatureRange(CssQueryFeatureRange),
    CssQueryFeatureRangeInterval(CssQueryFeatureRangeInterval),
    CssQueryFeatureReverseRange(CssQueryFeatureReverseRange),
}
impl AnyCssQueryFeature {
    pub fn as_css_query_feature_boolean(&self) -> Option<&CssQueryFeatureBoolean> {
        match &self {
            AnyCssQueryFeature::CssQueryFeatureBoolean(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_query_feature_plain(&self) -> Option<&CssQueryFeaturePlain> {
        match &self {
            AnyCssQueryFeature::CssQueryFeaturePlain(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_query_feature_range(&self) -> Option<&CssQueryFeatureRange> {
        match &self {
            AnyCssQueryFeature::CssQueryFeatureRange(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_query_feature_range_interval(&self) -> Option<&CssQueryFeatureRangeInterval> {
        match &self {
            AnyCssQueryFeature::CssQueryFeatureRangeInterval(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_query_feature_reverse_range(&self) -> Option<&CssQueryFeatureReverseRange> {
        match &self {
            AnyCssQueryFeature::CssQueryFeatureReverseRange(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssQueryFeatureValue {
    AnyCssDimension(AnyCssDimension),
    CssAnyFunction(CssAnyFunction),
    CssIdentifier(CssIdentifier),
    CssNumber(CssNumber),
    CssRatio(CssRatio),
}
impl AnyCssQueryFeatureValue {
    pub fn as_any_css_dimension(&self) -> Option<&AnyCssDimension> {
        match &self {
            AnyCssQueryFeatureValue::AnyCssDimension(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_any_function(&self) -> Option<&CssAnyFunction> {
        match &self {
            AnyCssQueryFeatureValue::CssAnyFunction(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_identifier(&self) -> Option<&CssIdentifier> {
        match &self {
            AnyCssQueryFeatureValue::CssIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_number(&self) -> Option<&CssNumber> {
        match &self {
            AnyCssQueryFeatureValue::CssNumber(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_ratio(&self) -> Option<&CssRatio> {
        match &self {
            AnyCssQueryFeatureValue::CssRatio(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssRelativeSelector {
    CssBogusSelector(CssBogusSelector),
    CssRelativeSelector(CssRelativeSelector),
}
impl AnyCssRelativeSelector {
    pub fn as_css_bogus_selector(&self) -> Option<&CssBogusSelector> {
        match &self {
            AnyCssRelativeSelector::CssBogusSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_relative_selector(&self) -> Option<&CssRelativeSelector> {
        match &self {
            AnyCssRelativeSelector::CssRelativeSelector(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssRule {
    CssAtRule(CssAtRule),
    CssBogusRule(CssBogusRule),
    CssRule(CssRule),
}
impl AnyCssRule {
    pub fn as_css_at_rule(&self) -> Option<&CssAtRule> {
        match &self {
            AnyCssRule::CssAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_bogus_rule(&self) -> Option<&CssBogusRule> {
        match &self {
            AnyCssRule::CssBogusRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_rule(&self) -> Option<&CssRule> {
        match &self {
            AnyCssRule::CssRule(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssRuleListBlock {
    CssBogusBlock(CssBogusBlock),
    CssRuleListBlock(CssRuleListBlock),
}
impl AnyCssRuleListBlock {
    pub fn as_css_bogus_block(&self) -> Option<&CssBogusBlock> {
        match &self {
            AnyCssRuleListBlock::CssBogusBlock(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_rule_list_block(&self) -> Option<&CssRuleListBlock> {
        match &self {
            AnyCssRuleListBlock::CssRuleListBlock(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssSelector {
    CssBogusSelector(CssBogusSelector),
    CssComplexSelector(CssComplexSelector),
    CssCompoundSelector(CssCompoundSelector),
}
impl AnyCssSelector {
    pub fn as_css_bogus_selector(&self) -> Option<&CssBogusSelector> {
        match &self {
            AnyCssSelector::CssBogusSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_complex_selector(&self) -> Option<&CssComplexSelector> {
        match &self {
            AnyCssSelector::CssComplexSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_compound_selector(&self) -> Option<&CssCompoundSelector> {
        match &self {
            AnyCssSelector::CssCompoundSelector(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssSimpleSelector {
    CssTypeSelector(CssTypeSelector),
    CssUniversalSelector(CssUniversalSelector),
}
impl AnyCssSimpleSelector {
    pub fn as_css_type_selector(&self) -> Option<&CssTypeSelector> {
        match &self {
            AnyCssSimpleSelector::CssTypeSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_universal_selector(&self) -> Option<&CssUniversalSelector> {
        match &self {
            AnyCssSimpleSelector::CssUniversalSelector(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssSubSelector {
    CssAttributeSelector(CssAttributeSelector),
    CssBogusSubSelector(CssBogusSubSelector),
    CssClassSelector(CssClassSelector),
    CssIdSelector(CssIdSelector),
    CssPseudoClassSelector(CssPseudoClassSelector),
    CssPseudoElementSelector(CssPseudoElementSelector),
}
impl AnyCssSubSelector {
    pub fn as_css_attribute_selector(&self) -> Option<&CssAttributeSelector> {
        match &self {
            AnyCssSubSelector::CssAttributeSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_bogus_sub_selector(&self) -> Option<&CssBogusSubSelector> {
        match &self {
            AnyCssSubSelector::CssBogusSubSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_class_selector(&self) -> Option<&CssClassSelector> {
        match &self {
            AnyCssSubSelector::CssClassSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_id_selector(&self) -> Option<&CssIdSelector> {
        match &self {
            AnyCssSubSelector::CssIdSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_selector(&self) -> Option<&CssPseudoClassSelector> {
        match &self {
            AnyCssSubSelector::CssPseudoClassSelector(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_element_selector(&self) -> Option<&CssPseudoElementSelector> {
        match &self {
            AnyCssSubSelector::CssPseudoElementSelector(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyCssValue {
    AnyCssDimension(AnyCssDimension),
    CssAnyFunction(CssAnyFunction),
    CssCustomProperty(CssCustomProperty),
    CssIdentifier(CssIdentifier),
    CssNumber(CssNumber),
    CssRatio(CssRatio),
    CssString(CssString),
}
impl AnyCssValue {
    pub fn as_any_css_dimension(&self) -> Option<&AnyCssDimension> {
        match &self {
            AnyCssValue::AnyCssDimension(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_any_function(&self) -> Option<&CssAnyFunction> {
        match &self {
            AnyCssValue::CssAnyFunction(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_custom_property(&self) -> Option<&CssCustomProperty> {
        match &self {
            AnyCssValue::CssCustomProperty(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_identifier(&self) -> Option<&CssIdentifier> {
        match &self {
            AnyCssValue::CssIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_number(&self) -> Option<&CssNumber> {
        match &self {
            AnyCssValue::CssNumber(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_ratio(&self) -> Option<&CssRatio> {
        match &self {
            AnyCssValue::CssRatio(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_string(&self) -> Option<&CssString> {
        match &self {
            AnyCssValue::CssString(item) => Some(item),
            _ => None,
        }
    }
}
impl AstNode for CssAnyFunction {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_ANY_FUNCTION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_ANY_FUNCTION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssAnyFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAnyFunction")
            .field(
                "css_simple_function",
                &support::DebugSyntaxResult(self.css_simple_function()),
            )
            .finish()
    }
}
impl From<CssAnyFunction> for SyntaxNode {
    fn from(n: CssAnyFunction) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssAnyFunction> for SyntaxElement {
    fn from(n: CssAnyFunction) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAtRule")
            .field("at_token", &support::DebugSyntaxResult(self.at_token()))
            .field("rule", &support::DebugSyntaxResult(self.rule()))
            .finish()
    }
}
impl From<CssAtRule> for SyntaxNode {
    fn from(n: CssAtRule) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssAtRule> for SyntaxElement {
    fn from(n: CssAtRule) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssAttributeMatcher {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_ATTRIBUTE_MATCHER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_ATTRIBUTE_MATCHER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssAttributeMatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAttributeMatcher")
            .field("operator", &support::DebugSyntaxResult(self.operator()))
            .field("value", &support::DebugSyntaxResult(self.value()))
            .field("modifier", &support::DebugOptionalElement(self.modifier()))
            .finish()
    }
}
impl From<CssAttributeMatcher> for SyntaxNode {
    fn from(n: CssAttributeMatcher) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssAttributeMatcher> for SyntaxElement {
    fn from(n: CssAttributeMatcher) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssAttributeMatcherValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_ATTRIBUTE_MATCHER_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_ATTRIBUTE_MATCHER_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssAttributeMatcherValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAttributeMatcherValue")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .finish()
    }
}
impl From<CssAttributeMatcherValue> for SyntaxNode {
    fn from(n: CssAttributeMatcherValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssAttributeMatcherValue> for SyntaxElement {
    fn from(n: CssAttributeMatcherValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssAttributeName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_ATTRIBUTE_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_ATTRIBUTE_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssAttributeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAttributeName")
            .field(
                "namespace",
                &support::DebugOptionalElement(self.namespace()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .finish()
    }
}
impl From<CssAttributeName> for SyntaxNode {
    fn from(n: CssAttributeName) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssAttributeName> for SyntaxElement {
    fn from(n: CssAttributeName) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssAttributeSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_ATTRIBUTE_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_ATTRIBUTE_SELECTOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssAttributeSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAttributeSelector")
            .field(
                "l_brack_token",
                &support::DebugSyntaxResult(self.l_brack_token()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field("matcher", &support::DebugOptionalElement(self.matcher()))
            .field(
                "r_brack_token",
                &support::DebugSyntaxResult(self.r_brack_token()),
            )
            .finish()
    }
}
impl From<CssAttributeSelector> for SyntaxNode {
    fn from(n: CssAttributeSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssAttributeSelector> for SyntaxElement {
    fn from(n: CssAttributeSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssCharsetAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CHARSET_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CHARSET_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssCharsetAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssCharsetAtRule")
            .field(
                "charset_token",
                &support::DebugSyntaxResult(self.charset_token()),
            )
            .field("encoding", &support::DebugSyntaxResult(self.encoding()))
            .field(
                "semicolon_token",
                &support::DebugSyntaxResult(self.semicolon_token()),
            )
            .finish()
    }
}
impl From<CssCharsetAtRule> for SyntaxNode {
    fn from(n: CssCharsetAtRule) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssCharsetAtRule> for SyntaxElement {
    fn from(n: CssCharsetAtRule) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssClassSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CLASS_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CLASS_SELECTOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssClassSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssClassSelector")
            .field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
            .field("name", &support::DebugSyntaxResult(self.name()))
            .finish()
    }
}
impl From<CssClassSelector> for SyntaxNode {
    fn from(n: CssClassSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssClassSelector> for SyntaxElement {
    fn from(n: CssClassSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssColorProfileAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_COLOR_PROFILE_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_COLOR_PROFILE_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssColorProfileAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssColorProfileAtRule")
            .field(
                "color_profile_token",
                &support::DebugSyntaxResult(self.color_profile_token()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field("block", &support::DebugSyntaxResult(self.block()))
            .finish()
    }
}
impl From<CssColorProfileAtRule> for SyntaxNode {
    fn from(n: CssColorProfileAtRule) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssColorProfileAtRule> for SyntaxElement {
    fn from(n: CssColorProfileAtRule) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssComplexSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_COMPLEX_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_COMPLEX_SELECTOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssComplexSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssComplexSelector")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field("combinator", &support::DebugSyntaxResult(self.combinator()))
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<CssComplexSelector> for SyntaxNode {
    fn from(n: CssComplexSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssComplexSelector> for SyntaxElement {
    fn from(n: CssComplexSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssCompoundSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_COMPOUND_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_COMPOUND_SELECTOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssCompoundSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssCompoundSelector")
            .field(
                "nesting_selector_token",
                &support::DebugOptionalElement(self.nesting_selector_token()),
            )
            .field(
                "simple_selector",
                &support::DebugOptionalElement(self.simple_selector()),
            )
            .field("sub_selectors", &self.sub_selectors())
            .finish()
    }
}
impl From<CssCompoundSelector> for SyntaxNode {
    fn from(n: CssCompoundSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssCompoundSelector> for SyntaxElement {
    fn from(n: CssCompoundSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssContainerAndQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CONTAINER_AND_QUERY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CONTAINER_AND_QUERY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssContainerAndQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssContainerAndQuery")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field("and_token", &support::DebugSyntaxResult(self.and_token()))
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<CssContainerAndQuery> for SyntaxNode {
    fn from(n: CssContainerAndQuery) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssContainerAndQuery> for SyntaxElement {
    fn from(n: CssContainerAndQuery) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssContainerAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CONTAINER_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CONTAINER_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssContainerAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssContainerAtRule")
            .field(
                "container_token",
                &support::DebugSyntaxResult(self.container_token()),
            )
            .field("name", &support::DebugOptionalElement(self.name()))
            .field("query", &support::DebugSyntaxResult(self.query()))
            .field("block", &support::DebugSyntaxResult(self.block()))
            .finish()
    }
}
impl From<CssContainerAtRule> for SyntaxNode {
    fn from(n: CssContainerAtRule) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssContainerAtRule> for SyntaxElement {
    fn from(n: CssContainerAtRule) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssContainerNotQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CONTAINER_NOT_QUERY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CONTAINER_NOT_QUERY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssContainerNotQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssContainerNotQuery")
            .field("not_token", &support::DebugSyntaxResult(self.not_token()))
            .field("query", &support::DebugSyntaxResult(self.query()))
            .finish()
    }
}
impl From<CssContainerNotQuery> for SyntaxNode {
    fn from(n: CssContainerNotQuery) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssContainerNotQuery> for SyntaxElement {
    fn from(n: CssContainerNotQuery) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssContainerOrQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CONTAINER_OR_QUERY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CONTAINER_OR_QUERY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssContainerOrQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssContainerOrQuery")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field("or_token", &support::DebugSyntaxResult(self.or_token()))
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<CssContainerOrQuery> for SyntaxNode {
    fn from(n: CssContainerOrQuery) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssContainerOrQuery> for SyntaxElement {
    fn from(n: CssContainerOrQuery) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssContainerQueryInParens {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CONTAINER_QUERY_IN_PARENS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CONTAINER_QUERY_IN_PARENS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssContainerQueryInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssContainerQueryInParens")
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("query", &support::DebugSyntaxResult(self.query()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssContainerQueryInParens> for SyntaxNode {
    fn from(n: CssContainerQueryInParens) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssContainerQueryInParens> for SyntaxElement {
    fn from(n: CssContainerQueryInParens) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssContainerSizeFeatureInParens {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CONTAINER_SIZE_FEATURE_IN_PARENS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CONTAINER_SIZE_FEATURE_IN_PARENS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssContainerSizeFeatureInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssContainerSizeFeatureInParens")
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("feature", &support::DebugSyntaxResult(self.feature()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssContainerSizeFeatureInParens> for SyntaxNode {
    fn from(n: CssContainerSizeFeatureInParens) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssContainerSizeFeatureInParens> for SyntaxElement {
    fn from(n: CssContainerSizeFeatureInParens) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssContainerStyleAndQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CONTAINER_STYLE_AND_QUERY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CONTAINER_STYLE_AND_QUERY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssContainerStyleAndQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssContainerStyleAndQuery")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field("and_token", &support::DebugSyntaxResult(self.and_token()))
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<CssContainerStyleAndQuery> for SyntaxNode {
    fn from(n: CssContainerStyleAndQuery) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssContainerStyleAndQuery> for SyntaxElement {
    fn from(n: CssContainerStyleAndQuery) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssContainerStyleInParens {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CONTAINER_STYLE_IN_PARENS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CONTAINER_STYLE_IN_PARENS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssContainerStyleInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssContainerStyleInParens")
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("query", &support::DebugSyntaxResult(self.query()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssContainerStyleInParens> for SyntaxNode {
    fn from(n: CssContainerStyleInParens) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssContainerStyleInParens> for SyntaxElement {
    fn from(n: CssContainerStyleInParens) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssContainerStyleNotQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CONTAINER_STYLE_NOT_QUERY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CONTAINER_STYLE_NOT_QUERY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssContainerStyleNotQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssContainerStyleNotQuery")
            .field("not_token", &support::DebugSyntaxResult(self.not_token()))
            .field("query", &support::DebugSyntaxResult(self.query()))
            .finish()
    }
}
impl From<CssContainerStyleNotQuery> for SyntaxNode {
    fn from(n: CssContainerStyleNotQuery) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssContainerStyleNotQuery> for SyntaxElement {
    fn from(n: CssContainerStyleNotQuery) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssContainerStyleOrQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CONTAINER_STYLE_OR_QUERY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CONTAINER_STYLE_OR_QUERY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssContainerStyleOrQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssContainerStyleOrQuery")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field("or_token", &support::DebugSyntaxResult(self.or_token()))
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<CssContainerStyleOrQuery> for SyntaxNode {
    fn from(n: CssContainerStyleOrQuery) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssContainerStyleOrQuery> for SyntaxElement {
    fn from(n: CssContainerStyleOrQuery) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssContainerStyleQueryInParens {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CONTAINER_STYLE_QUERY_IN_PARENS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CONTAINER_STYLE_QUERY_IN_PARENS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssContainerStyleQueryInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssContainerStyleQueryInParens")
            .field(
                "style_token",
                &support::DebugSyntaxResult(self.style_token()),
            )
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("query", &support::DebugSyntaxResult(self.query()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssContainerStyleQueryInParens> for SyntaxNode {
    fn from(n: CssContainerStyleQueryInParens) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssContainerStyleQueryInParens> for SyntaxElement {
    fn from(n: CssContainerStyleQueryInParens) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssCounterStyleAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_COUNTER_STYLE_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_COUNTER_STYLE_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssCounterStyleAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssCounterStyleAtRule")
            .field(
                "counter_style_token",
                &support::DebugSyntaxResult(self.counter_style_token()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field("block", &support::DebugSyntaxResult(self.block()))
            .finish()
    }
}
impl From<CssCounterStyleAtRule> for SyntaxNode {
    fn from(n: CssCounterStyleAtRule) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssCounterStyleAtRule> for SyntaxElement {
    fn from(n: CssCounterStyleAtRule) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssCustomProperty {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CUSTOM_PROPERTY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_CUSTOM_PROPERTY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssCustomProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssCustomProperty")
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<CssCustomProperty> for SyntaxNode {
    fn from(n: CssCustomProperty) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssCustomProperty> for SyntaxElement {
    fn from(n: CssCustomProperty) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssDeclaration {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_DECLARATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_DECLARATION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssDeclaration")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field("value", &self.value())
            .field(
                "important",
                &support::DebugOptionalElement(self.important()),
            )
            .finish()
    }
}
impl From<CssDeclaration> for SyntaxNode {
    fn from(n: CssDeclaration) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssDeclaration> for SyntaxElement {
    fn from(n: CssDeclaration) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssDeclarationImportant {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_DECLARATION_IMPORTANT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_DECLARATION_IMPORTANT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssDeclarationImportant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssDeclarationImportant")
            .field("excl_token", &support::DebugSyntaxResult(self.excl_token()))
            .field(
                "important_token",
                &support::DebugSyntaxResult(self.important_token()),
            )
            .finish()
    }
}
impl From<CssDeclarationImportant> for SyntaxNode {
    fn from(n: CssDeclarationImportant) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssDeclarationImportant> for SyntaxElement {
    fn from(n: CssDeclarationImportant) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssDeclarationListBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_DECLARATION_LIST_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_DECLARATION_LIST_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssDeclarationListBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssDeclarationListBlock")
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("declarations", &self.declarations())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<CssDeclarationListBlock> for SyntaxNode {
    fn from(n: CssDeclarationListBlock) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssDeclarationListBlock> for SyntaxElement {
    fn from(n: CssDeclarationListBlock) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssFontFaceAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_FONT_FACE_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_FONT_FACE_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssFontFaceAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssFontFaceAtRule")
            .field(
                "font_face_token",
                &support::DebugSyntaxResult(self.font_face_token()),
            )
            .field("block", &support::DebugSyntaxResult(self.block()))
            .finish()
    }
}
impl From<CssFontFaceAtRule> for SyntaxNode {
    fn from(n: CssFontFaceAtRule) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssFontFaceAtRule> for SyntaxElement {
    fn from(n: CssFontFaceAtRule) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssFontPaletteValuesAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_FONT_PALETTE_VALUES_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_FONT_PALETTE_VALUES_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssFontPaletteValuesAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssFontPaletteValuesAtRule")
            .field(
                "font_palette_values_token",
                &support::DebugSyntaxResult(self.font_palette_values_token()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field("block", &support::DebugSyntaxResult(self.block()))
            .finish()
    }
}
impl From<CssFontPaletteValuesAtRule> for SyntaxNode {
    fn from(n: CssFontPaletteValuesAtRule) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssFontPaletteValuesAtRule> for SyntaxElement {
    fn from(n: CssFontPaletteValuesAtRule) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssIdSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_ID_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_ID_SELECTOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssIdSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssIdSelector")
            .field("hash_token", &support::DebugSyntaxResult(self.hash_token()))
            .field("name", &support::DebugSyntaxResult(self.name()))
            .finish()
    }
}
impl From<CssIdSelector> for SyntaxNode {
    fn from(n: CssIdSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssIdSelector> for SyntaxElement {
    fn from(n: CssIdSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssIdentifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_IDENTIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_IDENTIFIER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssIdentifier")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<CssIdentifier> for SyntaxNode {
    fn from(n: CssIdentifier) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssIdentifier> for SyntaxElement {
    fn from(n: CssIdentifier) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssKeyframesAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_KEYFRAMES_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_KEYFRAMES_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssKeyframesAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssKeyframesAtRule")
            .field(
                "keyframes_token",
                &support::DebugSyntaxResult(self.keyframes_token()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field("block", &support::DebugSyntaxResult(self.block()))
            .finish()
    }
}
impl From<CssKeyframesAtRule> for SyntaxNode {
    fn from(n: CssKeyframesAtRule) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssKeyframesAtRule> for SyntaxElement {
    fn from(n: CssKeyframesAtRule) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssKeyframesBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_KEYFRAMES_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_KEYFRAMES_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssKeyframesBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssKeyframesBlock")
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("items", &self.items())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<CssKeyframesBlock> for SyntaxNode {
    fn from(n: CssKeyframesBlock) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssKeyframesBlock> for SyntaxElement {
    fn from(n: CssKeyframesBlock) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssKeyframesIdentSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_KEYFRAMES_IDENT_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_KEYFRAMES_IDENT_SELECTOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssKeyframesIdentSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssKeyframesIdentSelector")
            .field("selector", &support::DebugSyntaxResult(self.selector()))
            .finish()
    }
}
impl From<CssKeyframesIdentSelector> for SyntaxNode {
    fn from(n: CssKeyframesIdentSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssKeyframesIdentSelector> for SyntaxElement {
    fn from(n: CssKeyframesIdentSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssKeyframesItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_KEYFRAMES_ITEM as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_KEYFRAMES_ITEM
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssKeyframesItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssKeyframesItem")
            .field("selectors", &self.selectors())
            .field("block", &support::DebugSyntaxResult(self.block()))
            .finish()
    }
}
impl From<CssKeyframesItem> for SyntaxNode {
    fn from(n: CssKeyframesItem) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssKeyframesItem> for SyntaxElement {
    fn from(n: CssKeyframesItem) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssKeyframesPercentageSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_KEYFRAMES_PERCENTAGE_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_KEYFRAMES_PERCENTAGE_SELECTOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssKeyframesPercentageSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssKeyframesPercentageSelector")
            .field("selector", &support::DebugSyntaxResult(self.selector()))
            .finish()
    }
}
impl From<CssKeyframesPercentageSelector> for SyntaxNode {
    fn from(n: CssKeyframesPercentageSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssKeyframesPercentageSelector> for SyntaxElement {
    fn from(n: CssKeyframesPercentageSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssMediaAndCondition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_AND_CONDITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_AND_CONDITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMediaAndCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssMediaAndCondition")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field("and_token", &support::DebugSyntaxResult(self.and_token()))
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<CssMediaAndCondition> for SyntaxNode {
    fn from(n: CssMediaAndCondition) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssMediaAndCondition> for SyntaxElement {
    fn from(n: CssMediaAndCondition) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssMediaAndTypeQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_AND_TYPE_QUERY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_AND_TYPE_QUERY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMediaAndTypeQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssMediaAndTypeQuery")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field("and_token", &support::DebugSyntaxResult(self.and_token()))
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<CssMediaAndTypeQuery> for SyntaxNode {
    fn from(n: CssMediaAndTypeQuery) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssMediaAndTypeQuery> for SyntaxElement {
    fn from(n: CssMediaAndTypeQuery) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssMediaAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMediaAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssMediaAtRule")
            .field(
                "media_token",
                &support::DebugSyntaxResult(self.media_token()),
            )
            .field("queries", &self.queries())
            .field("block", &support::DebugSyntaxResult(self.block()))
            .finish()
    }
}
impl From<CssMediaAtRule> for SyntaxNode {
    fn from(n: CssMediaAtRule) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssMediaAtRule> for SyntaxElement {
    fn from(n: CssMediaAtRule) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssMediaConditionInParens {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_CONDITION_IN_PARENS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_CONDITION_IN_PARENS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMediaConditionInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssMediaConditionInParens")
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("condition", &support::DebugSyntaxResult(self.condition()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssMediaConditionInParens> for SyntaxNode {
    fn from(n: CssMediaConditionInParens) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssMediaConditionInParens> for SyntaxElement {
    fn from(n: CssMediaConditionInParens) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssMediaConditionQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_CONDITION_QUERY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_CONDITION_QUERY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMediaConditionQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssMediaConditionQuery")
            .field("condition", &support::DebugSyntaxResult(self.condition()))
            .finish()
    }
}
impl From<CssMediaConditionQuery> for SyntaxNode {
    fn from(n: CssMediaConditionQuery) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssMediaConditionQuery> for SyntaxElement {
    fn from(n: CssMediaConditionQuery) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssMediaFeatureInParens {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_FEATURE_IN_PARENS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_FEATURE_IN_PARENS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMediaFeatureInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssMediaFeatureInParens")
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("feature", &support::DebugSyntaxResult(self.feature()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssMediaFeatureInParens> for SyntaxNode {
    fn from(n: CssMediaFeatureInParens) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssMediaFeatureInParens> for SyntaxElement {
    fn from(n: CssMediaFeatureInParens) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssMediaNotCondition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_NOT_CONDITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_NOT_CONDITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMediaNotCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssMediaNotCondition")
            .field("not_token", &support::DebugSyntaxResult(self.not_token()))
            .field("condition", &support::DebugSyntaxResult(self.condition()))
            .finish()
    }
}
impl From<CssMediaNotCondition> for SyntaxNode {
    fn from(n: CssMediaNotCondition) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssMediaNotCondition> for SyntaxElement {
    fn from(n: CssMediaNotCondition) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssMediaOrCondition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_OR_CONDITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_OR_CONDITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMediaOrCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssMediaOrCondition")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field("or_token", &support::DebugSyntaxResult(self.or_token()))
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<CssMediaOrCondition> for SyntaxNode {
    fn from(n: CssMediaOrCondition) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssMediaOrCondition> for SyntaxElement {
    fn from(n: CssMediaOrCondition) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssMediaType {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_TYPE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_TYPE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssMediaType")
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<CssMediaType> for SyntaxNode {
    fn from(n: CssMediaType) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssMediaType> for SyntaxElement {
    fn from(n: CssMediaType) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssMediaTypeQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_TYPE_QUERY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_TYPE_QUERY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssMediaTypeQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssMediaTypeQuery")
            .field("modifier", &support::DebugOptionalElement(self.modifier()))
            .field("ty", &support::DebugSyntaxResult(self.ty()))
            .finish()
    }
}
impl From<CssMediaTypeQuery> for SyntaxNode {
    fn from(n: CssMediaTypeQuery) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssMediaTypeQuery> for SyntaxElement {
    fn from(n: CssMediaTypeQuery) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssNamedNamespacePrefix {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_NAMED_NAMESPACE_PREFIX as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_NAMED_NAMESPACE_PREFIX
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssNamedNamespacePrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssNamedNamespacePrefix")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .finish()
    }
}
impl From<CssNamedNamespacePrefix> for SyntaxNode {
    fn from(n: CssNamedNamespacePrefix) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssNamedNamespacePrefix> for SyntaxElement {
    fn from(n: CssNamedNamespacePrefix) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssNamespace {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_NAMESPACE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_NAMESPACE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssNamespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssNamespace")
            .field("prefix", &support::DebugOptionalElement(self.prefix()))
            .field(
                "bitwise_or_token",
                &support::DebugSyntaxResult(self.bitwise_or_token()),
            )
            .finish()
    }
}
impl From<CssNamespace> for SyntaxNode {
    fn from(n: CssNamespace) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssNamespace> for SyntaxElement {
    fn from(n: CssNamespace) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssNthOffset {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_NTH_OFFSET as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_NTH_OFFSET
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssNthOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssNthOffset")
            .field("sign", &support::DebugSyntaxResult(self.sign()))
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<CssNthOffset> for SyntaxNode {
    fn from(n: CssNthOffset) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssNthOffset> for SyntaxElement {
    fn from(n: CssNthOffset) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssNumber {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_NUMBER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_NUMBER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssNumber")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<CssNumber> for SyntaxNode {
    fn from(n: CssNumber) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssNumber> for SyntaxElement {
    fn from(n: CssNumber) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssParameter {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PARAMETER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PARAMETER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssParameter")
            .field("css_component_value_list", &self.css_component_value_list())
            .finish()
    }
}
impl From<CssParameter> for SyntaxNode {
    fn from(n: CssParameter) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssParameter> for SyntaxElement {
    fn from(n: CssParameter) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPercentDimension {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PERCENT_DIMENSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PERCENT_DIMENSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPercentDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPercentDimension")
            .field("value", &support::DebugSyntaxResult(self.value()))
            .field("unit_token", &support::DebugSyntaxResult(self.unit_token()))
            .finish()
    }
}
impl From<CssPercentDimension> for SyntaxNode {
    fn from(n: CssPercentDimension) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPercentDimension> for SyntaxElement {
    fn from(n: CssPercentDimension) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPercentage {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PERCENTAGE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PERCENTAGE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPercentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPercentage")
            .field("value", &support::DebugSyntaxResult(self.value()))
            .field(
                "reminder_token",
                &support::DebugSyntaxResult(self.reminder_token()),
            )
            .finish()
    }
}
impl From<CssPercentage> for SyntaxNode {
    fn from(n: CssPercentage) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPercentage> for SyntaxElement {
    fn from(n: CssPercentage) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassFunctionCompoundSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR as u16,
    ));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPseudoClassFunctionCompoundSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassFunctionCompoundSelector")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("selector", &support::DebugSyntaxResult(self.selector()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssPseudoClassFunctionCompoundSelector> for SyntaxNode {
    fn from(n: CssPseudoClassFunctionCompoundSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassFunctionCompoundSelector> for SyntaxElement {
    fn from(n: CssPseudoClassFunctionCompoundSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassFunctionCompoundSelectorList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_LIST as u16,
    ));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPseudoClassFunctionCompoundSelectorList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassFunctionCompoundSelectorList")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("compound_selectors", &self.compound_selectors())
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssPseudoClassFunctionCompoundSelectorList> for SyntaxNode {
    fn from(n: CssPseudoClassFunctionCompoundSelectorList) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassFunctionCompoundSelectorList> for SyntaxElement {
    fn from(n: CssPseudoClassFunctionCompoundSelectorList) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassFunctionIdentifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_CLASS_FUNCTION_IDENTIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_FUNCTION_IDENTIFIER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPseudoClassFunctionIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassFunctionIdentifier")
            .field("name_token", &support::DebugSyntaxResult(self.name_token()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("ident", &support::DebugSyntaxResult(self.ident()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssPseudoClassFunctionIdentifier> for SyntaxNode {
    fn from(n: CssPseudoClassFunctionIdentifier) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassFunctionIdentifier> for SyntaxElement {
    fn from(n: CssPseudoClassFunctionIdentifier) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassFunctionNth {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_CLASS_FUNCTION_NTH as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_FUNCTION_NTH
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPseudoClassFunctionNth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassFunctionNth")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("selector", &support::DebugSyntaxResult(self.selector()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssPseudoClassFunctionNth> for SyntaxNode {
    fn from(n: CssPseudoClassFunctionNth) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassFunctionNth> for SyntaxElement {
    fn from(n: CssPseudoClassFunctionNth) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassFunctionRelativeSelectorList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        CSS_PSEUDO_CLASS_FUNCTION_RELATIVE_SELECTOR_LIST as u16,
    ));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_FUNCTION_RELATIVE_SELECTOR_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPseudoClassFunctionRelativeSelectorList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassFunctionRelativeSelectorList")
            .field("name_token", &support::DebugSyntaxResult(self.name_token()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("relative_selectors", &self.relative_selectors())
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssPseudoClassFunctionRelativeSelectorList> for SyntaxNode {
    fn from(n: CssPseudoClassFunctionRelativeSelectorList) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassFunctionRelativeSelectorList> for SyntaxElement {
    fn from(n: CssPseudoClassFunctionRelativeSelectorList) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassFunctionSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_CLASS_FUNCTION_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_FUNCTION_SELECTOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPseudoClassFunctionSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassFunctionSelector")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("selector", &support::DebugSyntaxResult(self.selector()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssPseudoClassFunctionSelector> for SyntaxNode {
    fn from(n: CssPseudoClassFunctionSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassFunctionSelector> for SyntaxElement {
    fn from(n: CssPseudoClassFunctionSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassFunctionSelectorList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        CSS_PSEUDO_CLASS_FUNCTION_SELECTOR_LIST as u16,
    ));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_FUNCTION_SELECTOR_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPseudoClassFunctionSelectorList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassFunctionSelectorList")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("selectors", &self.selectors())
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssPseudoClassFunctionSelectorList> for SyntaxNode {
    fn from(n: CssPseudoClassFunctionSelectorList) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassFunctionSelectorList> for SyntaxElement {
    fn from(n: CssPseudoClassFunctionSelectorList) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassFunctionValueList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_CLASS_FUNCTION_VALUE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_FUNCTION_VALUE_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPseudoClassFunctionValueList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassFunctionValueList")
            .field("name_token", &support::DebugSyntaxResult(self.name_token()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("values", &self.values())
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssPseudoClassFunctionValueList> for SyntaxNode {
    fn from(n: CssPseudoClassFunctionValueList) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassFunctionValueList> for SyntaxElement {
    fn from(n: CssPseudoClassFunctionValueList) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassIdentifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_CLASS_IDENTIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_IDENTIFIER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPseudoClassIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassIdentifier")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .finish()
    }
}
impl From<CssPseudoClassIdentifier> for SyntaxNode {
    fn from(n: CssPseudoClassIdentifier) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassIdentifier> for SyntaxElement {
    fn from(n: CssPseudoClassIdentifier) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassNth {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_CLASS_NTH as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_NTH
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPseudoClassNth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassNth")
            .field("sign", &support::DebugOptionalElement(self.sign()))
            .field("value", &support::DebugOptionalElement(self.value()))
            .field(
                "symbol_token",
                &support::DebugSyntaxResult(self.symbol_token()),
            )
            .field("offset", &support::DebugOptionalElement(self.offset()))
            .finish()
    }
}
impl From<CssPseudoClassNth> for SyntaxNode {
    fn from(n: CssPseudoClassNth) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassNth> for SyntaxElement {
    fn from(n: CssPseudoClassNth) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassNthIdentifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_CLASS_NTH_IDENTIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_NTH_IDENTIFIER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPseudoClassNthIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassNthIdentifier")
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<CssPseudoClassNthIdentifier> for SyntaxNode {
    fn from(n: CssPseudoClassNthIdentifier) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassNthIdentifier> for SyntaxElement {
    fn from(n: CssPseudoClassNthIdentifier) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassNthNumber {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_CLASS_NTH_NUMBER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_NTH_NUMBER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPseudoClassNthNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassNthNumber")
            .field("sign", &support::DebugOptionalElement(self.sign()))
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<CssPseudoClassNthNumber> for SyntaxNode {
    fn from(n: CssPseudoClassNthNumber) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassNthNumber> for SyntaxElement {
    fn from(n: CssPseudoClassNthNumber) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassNthSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_CLASS_NTH_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_NTH_SELECTOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPseudoClassNthSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassNthSelector")
            .field("nth", &support::DebugSyntaxResult(self.nth()))
            .field(
                "of_selector",
                &support::DebugOptionalElement(self.of_selector()),
            )
            .finish()
    }
}
impl From<CssPseudoClassNthSelector> for SyntaxNode {
    fn from(n: CssPseudoClassNthSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassNthSelector> for SyntaxElement {
    fn from(n: CssPseudoClassNthSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassOfNthSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_CLASS_OF_NTH_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_OF_NTH_SELECTOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPseudoClassOfNthSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassOfNthSelector")
            .field("of_token", &support::DebugSyntaxResult(self.of_token()))
            .field("selectors", &self.selectors())
            .finish()
    }
}
impl From<CssPseudoClassOfNthSelector> for SyntaxNode {
    fn from(n: CssPseudoClassOfNthSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassOfNthSelector> for SyntaxElement {
    fn from(n: CssPseudoClassOfNthSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoClassSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_CLASS_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_CLASS_SELECTOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPseudoClassSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassSelector")
            .field(
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field("class", &support::DebugSyntaxResult(self.class()))
            .finish()
    }
}
impl From<CssPseudoClassSelector> for SyntaxNode {
    fn from(n: CssPseudoClassSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoClassSelector> for SyntaxElement {
    fn from(n: CssPseudoClassSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoElementFunctionIdentifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_ELEMENT_FUNCTION_IDENTIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_ELEMENT_FUNCTION_IDENTIFIER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPseudoElementFunctionIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoElementFunctionIdentifier")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("ident", &support::DebugSyntaxResult(self.ident()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssPseudoElementFunctionIdentifier> for SyntaxNode {
    fn from(n: CssPseudoElementFunctionIdentifier) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoElementFunctionIdentifier> for SyntaxElement {
    fn from(n: CssPseudoElementFunctionIdentifier) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoElementFunctionSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_ELEMENT_FUNCTION_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_ELEMENT_FUNCTION_SELECTOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPseudoElementFunctionSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoElementFunctionSelector")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("selector", &support::DebugSyntaxResult(self.selector()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssPseudoElementFunctionSelector> for SyntaxNode {
    fn from(n: CssPseudoElementFunctionSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoElementFunctionSelector> for SyntaxElement {
    fn from(n: CssPseudoElementFunctionSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoElementIdentifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_ELEMENT_IDENTIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_ELEMENT_IDENTIFIER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPseudoElementIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoElementIdentifier")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .finish()
    }
}
impl From<CssPseudoElementIdentifier> for SyntaxNode {
    fn from(n: CssPseudoElementIdentifier) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoElementIdentifier> for SyntaxElement {
    fn from(n: CssPseudoElementIdentifier) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssPseudoElementSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_ELEMENT_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_ELEMENT_SELECTOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssPseudoElementSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoElementSelector")
            .field(
                "double_colon_token",
                &support::DebugSyntaxResult(self.double_colon_token()),
            )
            .field("element", &support::DebugSyntaxResult(self.element()))
            .finish()
    }
}
impl From<CssPseudoElementSelector> for SyntaxNode {
    fn from(n: CssPseudoElementSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssPseudoElementSelector> for SyntaxElement {
    fn from(n: CssPseudoElementSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssQueryFeatureBoolean {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_QUERY_FEATURE_BOOLEAN as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_QUERY_FEATURE_BOOLEAN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssQueryFeatureBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssQueryFeatureBoolean")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .finish()
    }
}
impl From<CssQueryFeatureBoolean> for SyntaxNode {
    fn from(n: CssQueryFeatureBoolean) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssQueryFeatureBoolean> for SyntaxElement {
    fn from(n: CssQueryFeatureBoolean) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssQueryFeaturePlain {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_QUERY_FEATURE_PLAIN as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_QUERY_FEATURE_PLAIN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssQueryFeaturePlain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssQueryFeaturePlain")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<CssQueryFeaturePlain> for SyntaxNode {
    fn from(n: CssQueryFeaturePlain) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssQueryFeaturePlain> for SyntaxElement {
    fn from(n: CssQueryFeaturePlain) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssQueryFeatureRange {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_QUERY_FEATURE_RANGE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_QUERY_FEATURE_RANGE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssQueryFeatureRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssQueryFeatureRange")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field("comparison", &support::DebugSyntaxResult(self.comparison()))
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<CssQueryFeatureRange> for SyntaxNode {
    fn from(n: CssQueryFeatureRange) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssQueryFeatureRange> for SyntaxElement {
    fn from(n: CssQueryFeatureRange) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssQueryFeatureRangeComparison {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_QUERY_FEATURE_RANGE_COMPARISON as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_QUERY_FEATURE_RANGE_COMPARISON
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssQueryFeatureRangeComparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssQueryFeatureRangeComparison")
            .field("operator", &support::DebugSyntaxResult(self.operator()))
            .finish()
    }
}
impl From<CssQueryFeatureRangeComparison> for SyntaxNode {
    fn from(n: CssQueryFeatureRangeComparison) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssQueryFeatureRangeComparison> for SyntaxElement {
    fn from(n: CssQueryFeatureRangeComparison) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssQueryFeatureRangeInterval {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_QUERY_FEATURE_RANGE_INTERVAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_QUERY_FEATURE_RANGE_INTERVAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssQueryFeatureRangeInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssQueryFeatureRangeInterval")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field(
                "left_comparison",
                &support::DebugSyntaxResult(self.left_comparison()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "right_comparison",
                &support::DebugSyntaxResult(self.right_comparison()),
            )
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<CssQueryFeatureRangeInterval> for SyntaxNode {
    fn from(n: CssQueryFeatureRangeInterval) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssQueryFeatureRangeInterval> for SyntaxElement {
    fn from(n: CssQueryFeatureRangeInterval) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssQueryFeatureReverseRange {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_QUERY_FEATURE_REVERSE_RANGE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_QUERY_FEATURE_REVERSE_RANGE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssQueryFeatureReverseRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssQueryFeatureReverseRange")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field("comparison", &support::DebugSyntaxResult(self.comparison()))
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<CssQueryFeatureReverseRange> for SyntaxNode {
    fn from(n: CssQueryFeatureReverseRange) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssQueryFeatureReverseRange> for SyntaxElement {
    fn from(n: CssQueryFeatureReverseRange) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssRatio {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_RATIO as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_RATIO
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssRatio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssRatio")
            .field("numerator", &support::DebugSyntaxResult(self.numerator()))
            .field(
                "slash_token",
                &support::DebugSyntaxResult(self.slash_token()),
            )
            .field(
                "denominator",
                &support::DebugSyntaxResult(self.denominator()),
            )
            .finish()
    }
}
impl From<CssRatio> for SyntaxNode {
    fn from(n: CssRatio) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssRatio> for SyntaxElement {
    fn from(n: CssRatio) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssRegularDimension {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_REGULAR_DIMENSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_REGULAR_DIMENSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssRegularDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssRegularDimension")
            .field("value", &support::DebugSyntaxResult(self.value()))
            .field("unit", &support::DebugSyntaxResult(self.unit()))
            .finish()
    }
}
impl From<CssRegularDimension> for SyntaxNode {
    fn from(n: CssRegularDimension) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssRegularDimension> for SyntaxElement {
    fn from(n: CssRegularDimension) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssRelativeSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_RELATIVE_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_RELATIVE_SELECTOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssRelativeSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssRelativeSelector")
            .field(
                "combinator",
                &support::DebugOptionalElement(self.combinator()),
            )
            .field("selector", &support::DebugSyntaxResult(self.selector()))
            .finish()
    }
}
impl From<CssRelativeSelector> for SyntaxNode {
    fn from(n: CssRelativeSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssRelativeSelector> for SyntaxElement {
    fn from(n: CssRelativeSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssRoot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_ROOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_ROOT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssRoot")
            .field(
                "bom_token",
                &support::DebugOptionalElement(self.bom_token()),
            )
            .field("rules", &self.rules())
            .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
            .finish()
    }
}
impl From<CssRoot> for SyntaxNode {
    fn from(n: CssRoot) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssRoot> for SyntaxElement {
    fn from(n: CssRoot) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssRule")
            .field("prelude", &self.prelude())
            .field("block", &support::DebugSyntaxResult(self.block()))
            .finish()
    }
}
impl From<CssRule> for SyntaxNode {
    fn from(n: CssRule) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssRule> for SyntaxElement {
    fn from(n: CssRule) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssRuleListBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_RULE_LIST_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_RULE_LIST_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssRuleListBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssRuleListBlock")
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("rules", &self.rules())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<CssRuleListBlock> for SyntaxNode {
    fn from(n: CssRuleListBlock) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssRuleListBlock> for SyntaxElement {
    fn from(n: CssRuleListBlock) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssSimpleFunction {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SIMPLE_FUNCTION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_SIMPLE_FUNCTION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssSimpleFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssSimpleFunction")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("items", &self.items())
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssSimpleFunction> for SyntaxNode {
    fn from(n: CssSimpleFunction) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssSimpleFunction> for SyntaxElement {
    fn from(n: CssSimpleFunction) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssString {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_STRING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_STRING
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssString")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<CssString> for SyntaxNode {
    fn from(n: CssString) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssString> for SyntaxElement {
    fn from(n: CssString) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssTypeSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_TYPE_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_TYPE_SELECTOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssTypeSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssTypeSelector")
            .field(
                "namespace",
                &support::DebugOptionalElement(self.namespace()),
            )
            .field("ident", &support::DebugSyntaxResult(self.ident()))
            .finish()
    }
}
impl From<CssTypeSelector> for SyntaxNode {
    fn from(n: CssTypeSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssTypeSelector> for SyntaxElement {
    fn from(n: CssTypeSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssUniversalNamespacePrefix {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_UNIVERSAL_NAMESPACE_PREFIX as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_UNIVERSAL_NAMESPACE_PREFIX
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssUniversalNamespacePrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssUniversalNamespacePrefix")
            .field("star_token", &support::DebugSyntaxResult(self.star_token()))
            .finish()
    }
}
impl From<CssUniversalNamespacePrefix> for SyntaxNode {
    fn from(n: CssUniversalNamespacePrefix) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssUniversalNamespacePrefix> for SyntaxElement {
    fn from(n: CssUniversalNamespacePrefix) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssUniversalSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_UNIVERSAL_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_UNIVERSAL_SELECTOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssUniversalSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssUniversalSelector")
            .field(
                "namespace",
                &support::DebugOptionalElement(self.namespace()),
            )
            .field("star_token", &support::DebugSyntaxResult(self.star_token()))
            .finish()
    }
}
impl From<CssUniversalSelector> for SyntaxNode {
    fn from(n: CssUniversalSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssUniversalSelector> for SyntaxElement {
    fn from(n: CssUniversalSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssVarFunction {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_VAR_FUNCTION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_VAR_FUNCTION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssVarFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssVarFunction")
            .field("var_token", &support::DebugSyntaxResult(self.var_token()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("property", &support::DebugSyntaxResult(self.property()))
            .field("value", &support::DebugOptionalElement(self.value()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssVarFunction> for SyntaxNode {
    fn from(n: CssVarFunction) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssVarFunction> for SyntaxElement {
    fn from(n: CssVarFunction) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CssVarFunctionValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_VAR_FUNCTION_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_VAR_FUNCTION_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssVarFunctionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssVarFunctionValue")
            .field(
                "comma_token",
                &support::DebugSyntaxResult(self.comma_token()),
            )
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<CssVarFunctionValue> for SyntaxNode {
    fn from(n: CssVarFunctionValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssVarFunctionValue> for SyntaxElement {
    fn from(n: CssVarFunctionValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl From<CssBogusAtRule> for AnyCssAtRule {
    fn from(node: CssBogusAtRule) -> AnyCssAtRule {
        AnyCssAtRule::CssBogusAtRule(node)
    }
}
impl From<CssCharsetAtRule> for AnyCssAtRule {
    fn from(node: CssCharsetAtRule) -> AnyCssAtRule {
        AnyCssAtRule::CssCharsetAtRule(node)
    }
}
impl From<CssColorProfileAtRule> for AnyCssAtRule {
    fn from(node: CssColorProfileAtRule) -> AnyCssAtRule {
        AnyCssAtRule::CssColorProfileAtRule(node)
    }
}
impl From<CssContainerAtRule> for AnyCssAtRule {
    fn from(node: CssContainerAtRule) -> AnyCssAtRule {
        AnyCssAtRule::CssContainerAtRule(node)
    }
}
impl From<CssCounterStyleAtRule> for AnyCssAtRule {
    fn from(node: CssCounterStyleAtRule) -> AnyCssAtRule {
        AnyCssAtRule::CssCounterStyleAtRule(node)
    }
}
impl From<CssFontFaceAtRule> for AnyCssAtRule {
    fn from(node: CssFontFaceAtRule) -> AnyCssAtRule {
        AnyCssAtRule::CssFontFaceAtRule(node)
    }
}
impl From<CssFontPaletteValuesAtRule> for AnyCssAtRule {
    fn from(node: CssFontPaletteValuesAtRule) -> AnyCssAtRule {
        AnyCssAtRule::CssFontPaletteValuesAtRule(node)
    }
}
impl From<CssKeyframesAtRule> for AnyCssAtRule {
    fn from(node: CssKeyframesAtRule) -> AnyCssAtRule {
        AnyCssAtRule::CssKeyframesAtRule(node)
    }
}
impl From<CssMediaAtRule> for AnyCssAtRule {
    fn from(node: CssMediaAtRule) -> AnyCssAtRule {
        AnyCssAtRule::CssMediaAtRule(node)
    }
}
impl AstNode for AnyCssAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssBogusAtRule::KIND_SET
        .union(CssCharsetAtRule::KIND_SET)
        .union(CssColorProfileAtRule::KIND_SET)
        .union(CssContainerAtRule::KIND_SET)
        .union(CssCounterStyleAtRule::KIND_SET)
        .union(CssFontFaceAtRule::KIND_SET)
        .union(CssFontPaletteValuesAtRule::KIND_SET)
        .union(CssKeyframesAtRule::KIND_SET)
        .union(CssMediaAtRule::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_BOGUS_AT_RULE
                | CSS_CHARSET_AT_RULE
                | CSS_COLOR_PROFILE_AT_RULE
                | CSS_CONTAINER_AT_RULE
                | CSS_COUNTER_STYLE_AT_RULE
                | CSS_FONT_FACE_AT_RULE
                | CSS_FONT_PALETTE_VALUES_AT_RULE
                | CSS_KEYFRAMES_AT_RULE
                | CSS_MEDIA_AT_RULE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_AT_RULE => AnyCssAtRule::CssBogusAtRule(CssBogusAtRule { syntax }),
            CSS_CHARSET_AT_RULE => AnyCssAtRule::CssCharsetAtRule(CssCharsetAtRule { syntax }),
            CSS_COLOR_PROFILE_AT_RULE => {
                AnyCssAtRule::CssColorProfileAtRule(CssColorProfileAtRule { syntax })
            }
            CSS_CONTAINER_AT_RULE => {
                AnyCssAtRule::CssContainerAtRule(CssContainerAtRule { syntax })
            }
            CSS_COUNTER_STYLE_AT_RULE => {
                AnyCssAtRule::CssCounterStyleAtRule(CssCounterStyleAtRule { syntax })
            }
            CSS_FONT_FACE_AT_RULE => AnyCssAtRule::CssFontFaceAtRule(CssFontFaceAtRule { syntax }),
            CSS_FONT_PALETTE_VALUES_AT_RULE => {
                AnyCssAtRule::CssFontPaletteValuesAtRule(CssFontPaletteValuesAtRule { syntax })
            }
            CSS_KEYFRAMES_AT_RULE => {
                AnyCssAtRule::CssKeyframesAtRule(CssKeyframesAtRule { syntax })
            }
            CSS_MEDIA_AT_RULE => AnyCssAtRule::CssMediaAtRule(CssMediaAtRule { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssAtRule::CssBogusAtRule(it) => &it.syntax,
            AnyCssAtRule::CssCharsetAtRule(it) => &it.syntax,
            AnyCssAtRule::CssColorProfileAtRule(it) => &it.syntax,
            AnyCssAtRule::CssContainerAtRule(it) => &it.syntax,
            AnyCssAtRule::CssCounterStyleAtRule(it) => &it.syntax,
            AnyCssAtRule::CssFontFaceAtRule(it) => &it.syntax,
            AnyCssAtRule::CssFontPaletteValuesAtRule(it) => &it.syntax,
            AnyCssAtRule::CssKeyframesAtRule(it) => &it.syntax,
            AnyCssAtRule::CssMediaAtRule(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssAtRule::CssBogusAtRule(it) => it.syntax,
            AnyCssAtRule::CssCharsetAtRule(it) => it.syntax,
            AnyCssAtRule::CssColorProfileAtRule(it) => it.syntax,
            AnyCssAtRule::CssContainerAtRule(it) => it.syntax,
            AnyCssAtRule::CssCounterStyleAtRule(it) => it.syntax,
            AnyCssAtRule::CssFontFaceAtRule(it) => it.syntax,
            AnyCssAtRule::CssFontPaletteValuesAtRule(it) => it.syntax,
            AnyCssAtRule::CssKeyframesAtRule(it) => it.syntax,
            AnyCssAtRule::CssMediaAtRule(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssAtRule::CssBogusAtRule(it) => std::fmt::Debug::fmt(it, f),
            AnyCssAtRule::CssCharsetAtRule(it) => std::fmt::Debug::fmt(it, f),
            AnyCssAtRule::CssColorProfileAtRule(it) => std::fmt::Debug::fmt(it, f),
            AnyCssAtRule::CssContainerAtRule(it) => std::fmt::Debug::fmt(it, f),
            AnyCssAtRule::CssCounterStyleAtRule(it) => std::fmt::Debug::fmt(it, f),
            AnyCssAtRule::CssFontFaceAtRule(it) => std::fmt::Debug::fmt(it, f),
            AnyCssAtRule::CssFontPaletteValuesAtRule(it) => std::fmt::Debug::fmt(it, f),
            AnyCssAtRule::CssKeyframesAtRule(it) => std::fmt::Debug::fmt(it, f),
            AnyCssAtRule::CssMediaAtRule(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssAtRule> for SyntaxNode {
    fn from(n: AnyCssAtRule) -> SyntaxNode {
        match n {
            AnyCssAtRule::CssBogusAtRule(it) => it.into(),
            AnyCssAtRule::CssCharsetAtRule(it) => it.into(),
            AnyCssAtRule::CssColorProfileAtRule(it) => it.into(),
            AnyCssAtRule::CssContainerAtRule(it) => it.into(),
            AnyCssAtRule::CssCounterStyleAtRule(it) => it.into(),
            AnyCssAtRule::CssFontFaceAtRule(it) => it.into(),
            AnyCssAtRule::CssFontPaletteValuesAtRule(it) => it.into(),
            AnyCssAtRule::CssKeyframesAtRule(it) => it.into(),
            AnyCssAtRule::CssMediaAtRule(it) => it.into(),
        }
    }
}
impl From<AnyCssAtRule> for SyntaxElement {
    fn from(n: AnyCssAtRule) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssIdentifier> for AnyCssAttributeMatcherValue {
    fn from(node: CssIdentifier) -> AnyCssAttributeMatcherValue {
        AnyCssAttributeMatcherValue::CssIdentifier(node)
    }
}
impl From<CssString> for AnyCssAttributeMatcherValue {
    fn from(node: CssString) -> AnyCssAttributeMatcherValue {
        AnyCssAttributeMatcherValue::CssString(node)
    }
}
impl AstNode for AnyCssAttributeMatcherValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssIdentifier::KIND_SET.union(CssString::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_IDENTIFIER | CSS_STRING)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_IDENTIFIER => AnyCssAttributeMatcherValue::CssIdentifier(CssIdentifier { syntax }),
            CSS_STRING => AnyCssAttributeMatcherValue::CssString(CssString { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssAttributeMatcherValue::CssIdentifier(it) => &it.syntax,
            AnyCssAttributeMatcherValue::CssString(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssAttributeMatcherValue::CssIdentifier(it) => it.syntax,
            AnyCssAttributeMatcherValue::CssString(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssAttributeMatcherValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssAttributeMatcherValue::CssIdentifier(it) => std::fmt::Debug::fmt(it, f),
            AnyCssAttributeMatcherValue::CssString(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssAttributeMatcherValue> for SyntaxNode {
    fn from(n: AnyCssAttributeMatcherValue) -> SyntaxNode {
        match n {
            AnyCssAttributeMatcherValue::CssIdentifier(it) => it.into(),
            AnyCssAttributeMatcherValue::CssString(it) => it.into(),
        }
    }
}
impl From<AnyCssAttributeMatcherValue> for SyntaxElement {
    fn from(n: AnyCssAttributeMatcherValue) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusSelector> for AnyCssCompoundSelector {
    fn from(node: CssBogusSelector) -> AnyCssCompoundSelector {
        AnyCssCompoundSelector::CssBogusSelector(node)
    }
}
impl From<CssCompoundSelector> for AnyCssCompoundSelector {
    fn from(node: CssCompoundSelector) -> AnyCssCompoundSelector {
        AnyCssCompoundSelector::CssCompoundSelector(node)
    }
}
impl AstNode for AnyCssCompoundSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssBogusSelector::KIND_SET.union(CssCompoundSelector::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_BOGUS_SELECTOR | CSS_COMPOUND_SELECTOR)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_SELECTOR => {
                AnyCssCompoundSelector::CssBogusSelector(CssBogusSelector { syntax })
            }
            CSS_COMPOUND_SELECTOR => {
                AnyCssCompoundSelector::CssCompoundSelector(CssCompoundSelector { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssCompoundSelector::CssBogusSelector(it) => &it.syntax,
            AnyCssCompoundSelector::CssCompoundSelector(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssCompoundSelector::CssBogusSelector(it) => it.syntax,
            AnyCssCompoundSelector::CssCompoundSelector(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssCompoundSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssCompoundSelector::CssBogusSelector(it) => std::fmt::Debug::fmt(it, f),
            AnyCssCompoundSelector::CssCompoundSelector(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssCompoundSelector> for SyntaxNode {
    fn from(n: AnyCssCompoundSelector) -> SyntaxNode {
        match n {
            AnyCssCompoundSelector::CssBogusSelector(it) => it.into(),
            AnyCssCompoundSelector::CssCompoundSelector(it) => it.into(),
        }
    }
}
impl From<AnyCssCompoundSelector> for SyntaxElement {
    fn from(n: AnyCssCompoundSelector) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssContainerAndQuery> for AnyCssContainerAndCombinableQuery {
    fn from(node: CssContainerAndQuery) -> AnyCssContainerAndCombinableQuery {
        AnyCssContainerAndCombinableQuery::CssContainerAndQuery(node)
    }
}
impl AstNode for AnyCssContainerAndCombinableQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AnyCssContainerQueryInParens::KIND_SET.union(CssContainerAndQuery::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_CONTAINER_AND_QUERY => true,
            k if AnyCssContainerQueryInParens::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_CONTAINER_AND_QUERY => {
                AnyCssContainerAndCombinableQuery::CssContainerAndQuery(CssContainerAndQuery {
                    syntax,
                })
            }
            _ => {
                if let Some(any_css_container_query_in_parens) =
                    AnyCssContainerQueryInParens::cast(syntax)
                {
                    return Some(
                        AnyCssContainerAndCombinableQuery::AnyCssContainerQueryInParens(
                            any_css_container_query_in_parens,
                        ),
                    );
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssContainerAndCombinableQuery::CssContainerAndQuery(it) => &it.syntax,
            AnyCssContainerAndCombinableQuery::AnyCssContainerQueryInParens(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssContainerAndCombinableQuery::CssContainerAndQuery(it) => it.syntax,
            AnyCssContainerAndCombinableQuery::AnyCssContainerQueryInParens(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssContainerAndCombinableQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssContainerAndCombinableQuery::AnyCssContainerQueryInParens(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssContainerAndCombinableQuery::CssContainerAndQuery(it) => {
                std::fmt::Debug::fmt(it, f)
            }
        }
    }
}
impl From<AnyCssContainerAndCombinableQuery> for SyntaxNode {
    fn from(n: AnyCssContainerAndCombinableQuery) -> SyntaxNode {
        match n {
            AnyCssContainerAndCombinableQuery::AnyCssContainerQueryInParens(it) => it.into(),
            AnyCssContainerAndCombinableQuery::CssContainerAndQuery(it) => it.into(),
        }
    }
}
impl From<AnyCssContainerAndCombinableQuery> for SyntaxElement {
    fn from(n: AnyCssContainerAndCombinableQuery) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssContainerOrQuery> for AnyCssContainerOrCombinableQuery {
    fn from(node: CssContainerOrQuery) -> AnyCssContainerOrCombinableQuery {
        AnyCssContainerOrCombinableQuery::CssContainerOrQuery(node)
    }
}
impl AstNode for AnyCssContainerOrCombinableQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AnyCssContainerQueryInParens::KIND_SET.union(CssContainerOrQuery::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_CONTAINER_OR_QUERY => true,
            k if AnyCssContainerQueryInParens::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_CONTAINER_OR_QUERY => {
                AnyCssContainerOrCombinableQuery::CssContainerOrQuery(CssContainerOrQuery {
                    syntax,
                })
            }
            _ => {
                if let Some(any_css_container_query_in_parens) =
                    AnyCssContainerQueryInParens::cast(syntax)
                {
                    return Some(
                        AnyCssContainerOrCombinableQuery::AnyCssContainerQueryInParens(
                            any_css_container_query_in_parens,
                        ),
                    );
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssContainerOrCombinableQuery::CssContainerOrQuery(it) => &it.syntax,
            AnyCssContainerOrCombinableQuery::AnyCssContainerQueryInParens(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssContainerOrCombinableQuery::CssContainerOrQuery(it) => it.syntax,
            AnyCssContainerOrCombinableQuery::AnyCssContainerQueryInParens(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssContainerOrCombinableQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssContainerOrCombinableQuery::AnyCssContainerQueryInParens(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssContainerOrCombinableQuery::CssContainerOrQuery(it) => {
                std::fmt::Debug::fmt(it, f)
            }
        }
    }
}
impl From<AnyCssContainerOrCombinableQuery> for SyntaxNode {
    fn from(n: AnyCssContainerOrCombinableQuery) -> SyntaxNode {
        match n {
            AnyCssContainerOrCombinableQuery::AnyCssContainerQueryInParens(it) => it.into(),
            AnyCssContainerOrCombinableQuery::CssContainerOrQuery(it) => it.into(),
        }
    }
}
impl From<AnyCssContainerOrCombinableQuery> for SyntaxElement {
    fn from(n: AnyCssContainerOrCombinableQuery) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssContainerAndQuery> for AnyCssContainerQuery {
    fn from(node: CssContainerAndQuery) -> AnyCssContainerQuery {
        AnyCssContainerQuery::CssContainerAndQuery(node)
    }
}
impl From<CssContainerNotQuery> for AnyCssContainerQuery {
    fn from(node: CssContainerNotQuery) -> AnyCssContainerQuery {
        AnyCssContainerQuery::CssContainerNotQuery(node)
    }
}
impl From<CssContainerOrQuery> for AnyCssContainerQuery {
    fn from(node: CssContainerOrQuery) -> AnyCssContainerQuery {
        AnyCssContainerQuery::CssContainerOrQuery(node)
    }
}
impl AstNode for AnyCssContainerQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyCssContainerQueryInParens::KIND_SET
        .union(CssContainerAndQuery::KIND_SET)
        .union(CssContainerNotQuery::KIND_SET)
        .union(CssContainerOrQuery::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_CONTAINER_AND_QUERY | CSS_CONTAINER_NOT_QUERY | CSS_CONTAINER_OR_QUERY => true,
            k if AnyCssContainerQueryInParens::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_CONTAINER_AND_QUERY => {
                AnyCssContainerQuery::CssContainerAndQuery(CssContainerAndQuery { syntax })
            }
            CSS_CONTAINER_NOT_QUERY => {
                AnyCssContainerQuery::CssContainerNotQuery(CssContainerNotQuery { syntax })
            }
            CSS_CONTAINER_OR_QUERY => {
                AnyCssContainerQuery::CssContainerOrQuery(CssContainerOrQuery { syntax })
            }
            _ => {
                if let Some(any_css_container_query_in_parens) =
                    AnyCssContainerQueryInParens::cast(syntax)
                {
                    return Some(AnyCssContainerQuery::AnyCssContainerQueryInParens(
                        any_css_container_query_in_parens,
                    ));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssContainerQuery::CssContainerAndQuery(it) => &it.syntax,
            AnyCssContainerQuery::CssContainerNotQuery(it) => &it.syntax,
            AnyCssContainerQuery::CssContainerOrQuery(it) => &it.syntax,
            AnyCssContainerQuery::AnyCssContainerQueryInParens(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssContainerQuery::CssContainerAndQuery(it) => it.syntax,
            AnyCssContainerQuery::CssContainerNotQuery(it) => it.syntax,
            AnyCssContainerQuery::CssContainerOrQuery(it) => it.syntax,
            AnyCssContainerQuery::AnyCssContainerQueryInParens(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssContainerQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssContainerQuery::AnyCssContainerQueryInParens(it) => std::fmt::Debug::fmt(it, f),
            AnyCssContainerQuery::CssContainerAndQuery(it) => std::fmt::Debug::fmt(it, f),
            AnyCssContainerQuery::CssContainerNotQuery(it) => std::fmt::Debug::fmt(it, f),
            AnyCssContainerQuery::CssContainerOrQuery(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssContainerQuery> for SyntaxNode {
    fn from(n: AnyCssContainerQuery) -> SyntaxNode {
        match n {
            AnyCssContainerQuery::AnyCssContainerQueryInParens(it) => it.into(),
            AnyCssContainerQuery::CssContainerAndQuery(it) => it.into(),
            AnyCssContainerQuery::CssContainerNotQuery(it) => it.into(),
            AnyCssContainerQuery::CssContainerOrQuery(it) => it.into(),
        }
    }
}
impl From<AnyCssContainerQuery> for SyntaxElement {
    fn from(n: AnyCssContainerQuery) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssContainerQueryInParens> for AnyCssContainerQueryInParens {
    fn from(node: CssContainerQueryInParens) -> AnyCssContainerQueryInParens {
        AnyCssContainerQueryInParens::CssContainerQueryInParens(node)
    }
}
impl From<CssContainerSizeFeatureInParens> for AnyCssContainerQueryInParens {
    fn from(node: CssContainerSizeFeatureInParens) -> AnyCssContainerQueryInParens {
        AnyCssContainerQueryInParens::CssContainerSizeFeatureInParens(node)
    }
}
impl From<CssContainerStyleQueryInParens> for AnyCssContainerQueryInParens {
    fn from(node: CssContainerStyleQueryInParens) -> AnyCssContainerQueryInParens {
        AnyCssContainerQueryInParens::CssContainerStyleQueryInParens(node)
    }
}
impl AstNode for AnyCssContainerQueryInParens {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssContainerQueryInParens::KIND_SET
        .union(CssContainerSizeFeatureInParens::KIND_SET)
        .union(CssContainerStyleQueryInParens::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_CONTAINER_QUERY_IN_PARENS
                | CSS_CONTAINER_SIZE_FEATURE_IN_PARENS
                | CSS_CONTAINER_STYLE_QUERY_IN_PARENS
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_CONTAINER_QUERY_IN_PARENS => {
                AnyCssContainerQueryInParens::CssContainerQueryInParens(CssContainerQueryInParens {
                    syntax,
                })
            }
            CSS_CONTAINER_SIZE_FEATURE_IN_PARENS => {
                AnyCssContainerQueryInParens::CssContainerSizeFeatureInParens(
                    CssContainerSizeFeatureInParens { syntax },
                )
            }
            CSS_CONTAINER_STYLE_QUERY_IN_PARENS => {
                AnyCssContainerQueryInParens::CssContainerStyleQueryInParens(
                    CssContainerStyleQueryInParens { syntax },
                )
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssContainerQueryInParens::CssContainerQueryInParens(it) => &it.syntax,
            AnyCssContainerQueryInParens::CssContainerSizeFeatureInParens(it) => &it.syntax,
            AnyCssContainerQueryInParens::CssContainerStyleQueryInParens(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssContainerQueryInParens::CssContainerQueryInParens(it) => it.syntax,
            AnyCssContainerQueryInParens::CssContainerSizeFeatureInParens(it) => it.syntax,
            AnyCssContainerQueryInParens::CssContainerStyleQueryInParens(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssContainerQueryInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssContainerQueryInParens::CssContainerQueryInParens(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssContainerQueryInParens::CssContainerSizeFeatureInParens(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssContainerQueryInParens::CssContainerStyleQueryInParens(it) => {
                std::fmt::Debug::fmt(it, f)
            }
        }
    }
}
impl From<AnyCssContainerQueryInParens> for SyntaxNode {
    fn from(n: AnyCssContainerQueryInParens) -> SyntaxNode {
        match n {
            AnyCssContainerQueryInParens::CssContainerQueryInParens(it) => it.into(),
            AnyCssContainerQueryInParens::CssContainerSizeFeatureInParens(it) => it.into(),
            AnyCssContainerQueryInParens::CssContainerStyleQueryInParens(it) => it.into(),
        }
    }
}
impl From<AnyCssContainerQueryInParens> for SyntaxElement {
    fn from(n: AnyCssContainerQueryInParens) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssContainerStyleAndQuery> for AnyCssContainerStyleAndCombinableQuery {
    fn from(node: CssContainerStyleAndQuery) -> AnyCssContainerStyleAndCombinableQuery {
        AnyCssContainerStyleAndCombinableQuery::CssContainerStyleAndQuery(node)
    }
}
impl From<CssContainerStyleInParens> for AnyCssContainerStyleAndCombinableQuery {
    fn from(node: CssContainerStyleInParens) -> AnyCssContainerStyleAndCombinableQuery {
        AnyCssContainerStyleAndCombinableQuery::CssContainerStyleInParens(node)
    }
}
impl AstNode for AnyCssContainerStyleAndCombinableQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssContainerStyleAndQuery::KIND_SET.union(CssContainerStyleInParens::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_CONTAINER_STYLE_AND_QUERY | CSS_CONTAINER_STYLE_IN_PARENS
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_CONTAINER_STYLE_AND_QUERY => {
                AnyCssContainerStyleAndCombinableQuery::CssContainerStyleAndQuery(
                    CssContainerStyleAndQuery { syntax },
                )
            }
            CSS_CONTAINER_STYLE_IN_PARENS => {
                AnyCssContainerStyleAndCombinableQuery::CssContainerStyleInParens(
                    CssContainerStyleInParens { syntax },
                )
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssContainerStyleAndCombinableQuery::CssContainerStyleAndQuery(it) => &it.syntax,
            AnyCssContainerStyleAndCombinableQuery::CssContainerStyleInParens(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssContainerStyleAndCombinableQuery::CssContainerStyleAndQuery(it) => it.syntax,
            AnyCssContainerStyleAndCombinableQuery::CssContainerStyleInParens(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssContainerStyleAndCombinableQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssContainerStyleAndCombinableQuery::CssContainerStyleAndQuery(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssContainerStyleAndCombinableQuery::CssContainerStyleInParens(it) => {
                std::fmt::Debug::fmt(it, f)
            }
        }
    }
}
impl From<AnyCssContainerStyleAndCombinableQuery> for SyntaxNode {
    fn from(n: AnyCssContainerStyleAndCombinableQuery) -> SyntaxNode {
        match n {
            AnyCssContainerStyleAndCombinableQuery::CssContainerStyleAndQuery(it) => it.into(),
            AnyCssContainerStyleAndCombinableQuery::CssContainerStyleInParens(it) => it.into(),
        }
    }
}
impl From<AnyCssContainerStyleAndCombinableQuery> for SyntaxElement {
    fn from(n: AnyCssContainerStyleAndCombinableQuery) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssDeclaration> for AnyCssContainerStyleInParens {
    fn from(node: CssDeclaration) -> AnyCssContainerStyleInParens {
        AnyCssContainerStyleInParens::CssDeclaration(node)
    }
}
impl AstNode for AnyCssContainerStyleInParens {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AnyCssContainerStyleQuery::KIND_SET.union(CssDeclaration::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_DECLARATION => true,
            k if AnyCssContainerStyleQuery::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_DECLARATION => {
                AnyCssContainerStyleInParens::CssDeclaration(CssDeclaration { syntax })
            }
            _ => {
                if let Some(any_css_container_style_query) = AnyCssContainerStyleQuery::cast(syntax)
                {
                    return Some(AnyCssContainerStyleInParens::AnyCssContainerStyleQuery(
                        any_css_container_style_query,
                    ));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssContainerStyleInParens::CssDeclaration(it) => &it.syntax,
            AnyCssContainerStyleInParens::AnyCssContainerStyleQuery(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssContainerStyleInParens::CssDeclaration(it) => it.syntax,
            AnyCssContainerStyleInParens::AnyCssContainerStyleQuery(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssContainerStyleInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssContainerStyleInParens::AnyCssContainerStyleQuery(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssContainerStyleInParens::CssDeclaration(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssContainerStyleInParens> for SyntaxNode {
    fn from(n: AnyCssContainerStyleInParens) -> SyntaxNode {
        match n {
            AnyCssContainerStyleInParens::AnyCssContainerStyleQuery(it) => it.into(),
            AnyCssContainerStyleInParens::CssDeclaration(it) => it.into(),
        }
    }
}
impl From<AnyCssContainerStyleInParens> for SyntaxElement {
    fn from(n: AnyCssContainerStyleInParens) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssContainerStyleInParens> for AnyCssContainerStyleOrCombinableQuery {
    fn from(node: CssContainerStyleInParens) -> AnyCssContainerStyleOrCombinableQuery {
        AnyCssContainerStyleOrCombinableQuery::CssContainerStyleInParens(node)
    }
}
impl From<CssContainerStyleOrQuery> for AnyCssContainerStyleOrCombinableQuery {
    fn from(node: CssContainerStyleOrQuery) -> AnyCssContainerStyleOrCombinableQuery {
        AnyCssContainerStyleOrCombinableQuery::CssContainerStyleOrQuery(node)
    }
}
impl AstNode for AnyCssContainerStyleOrCombinableQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssContainerStyleInParens::KIND_SET.union(CssContainerStyleOrQuery::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_CONTAINER_STYLE_IN_PARENS | CSS_CONTAINER_STYLE_OR_QUERY
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_CONTAINER_STYLE_IN_PARENS => {
                AnyCssContainerStyleOrCombinableQuery::CssContainerStyleInParens(
                    CssContainerStyleInParens { syntax },
                )
            }
            CSS_CONTAINER_STYLE_OR_QUERY => {
                AnyCssContainerStyleOrCombinableQuery::CssContainerStyleOrQuery(
                    CssContainerStyleOrQuery { syntax },
                )
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssContainerStyleOrCombinableQuery::CssContainerStyleInParens(it) => &it.syntax,
            AnyCssContainerStyleOrCombinableQuery::CssContainerStyleOrQuery(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssContainerStyleOrCombinableQuery::CssContainerStyleInParens(it) => it.syntax,
            AnyCssContainerStyleOrCombinableQuery::CssContainerStyleOrQuery(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssContainerStyleOrCombinableQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssContainerStyleOrCombinableQuery::CssContainerStyleInParens(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssContainerStyleOrCombinableQuery::CssContainerStyleOrQuery(it) => {
                std::fmt::Debug::fmt(it, f)
            }
        }
    }
}
impl From<AnyCssContainerStyleOrCombinableQuery> for SyntaxNode {
    fn from(n: AnyCssContainerStyleOrCombinableQuery) -> SyntaxNode {
        match n {
            AnyCssContainerStyleOrCombinableQuery::CssContainerStyleInParens(it) => it.into(),
            AnyCssContainerStyleOrCombinableQuery::CssContainerStyleOrQuery(it) => it.into(),
        }
    }
}
impl From<AnyCssContainerStyleOrCombinableQuery> for SyntaxElement {
    fn from(n: AnyCssContainerStyleOrCombinableQuery) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssContainerStyleAndQuery> for AnyCssContainerStyleQuery {
    fn from(node: CssContainerStyleAndQuery) -> AnyCssContainerStyleQuery {
        AnyCssContainerStyleQuery::CssContainerStyleAndQuery(node)
    }
}
impl From<CssContainerStyleInParens> for AnyCssContainerStyleQuery {
    fn from(node: CssContainerStyleInParens) -> AnyCssContainerStyleQuery {
        AnyCssContainerStyleQuery::CssContainerStyleInParens(node)
    }
}
impl From<CssContainerStyleNotQuery> for AnyCssContainerStyleQuery {
    fn from(node: CssContainerStyleNotQuery) -> AnyCssContainerStyleQuery {
        AnyCssContainerStyleQuery::CssContainerStyleNotQuery(node)
    }
}
impl From<CssContainerStyleOrQuery> for AnyCssContainerStyleQuery {
    fn from(node: CssContainerStyleOrQuery) -> AnyCssContainerStyleQuery {
        AnyCssContainerStyleQuery::CssContainerStyleOrQuery(node)
    }
}
impl From<CssDeclaration> for AnyCssContainerStyleQuery {
    fn from(node: CssDeclaration) -> AnyCssContainerStyleQuery {
        AnyCssContainerStyleQuery::CssDeclaration(node)
    }
}
impl AstNode for AnyCssContainerStyleQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssContainerStyleAndQuery::KIND_SET
        .union(CssContainerStyleInParens::KIND_SET)
        .union(CssContainerStyleNotQuery::KIND_SET)
        .union(CssContainerStyleOrQuery::KIND_SET)
        .union(CssDeclaration::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_CONTAINER_STYLE_AND_QUERY
                | CSS_CONTAINER_STYLE_IN_PARENS
                | CSS_CONTAINER_STYLE_NOT_QUERY
                | CSS_CONTAINER_STYLE_OR_QUERY
                | CSS_DECLARATION
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_CONTAINER_STYLE_AND_QUERY => {
                AnyCssContainerStyleQuery::CssContainerStyleAndQuery(CssContainerStyleAndQuery {
                    syntax,
                })
            }
            CSS_CONTAINER_STYLE_IN_PARENS => {
                AnyCssContainerStyleQuery::CssContainerStyleInParens(CssContainerStyleInParens {
                    syntax,
                })
            }
            CSS_CONTAINER_STYLE_NOT_QUERY => {
                AnyCssContainerStyleQuery::CssContainerStyleNotQuery(CssContainerStyleNotQuery {
                    syntax,
                })
            }
            CSS_CONTAINER_STYLE_OR_QUERY => {
                AnyCssContainerStyleQuery::CssContainerStyleOrQuery(CssContainerStyleOrQuery {
                    syntax,
                })
            }
            CSS_DECLARATION => AnyCssContainerStyleQuery::CssDeclaration(CssDeclaration { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssContainerStyleQuery::CssContainerStyleAndQuery(it) => &it.syntax,
            AnyCssContainerStyleQuery::CssContainerStyleInParens(it) => &it.syntax,
            AnyCssContainerStyleQuery::CssContainerStyleNotQuery(it) => &it.syntax,
            AnyCssContainerStyleQuery::CssContainerStyleOrQuery(it) => &it.syntax,
            AnyCssContainerStyleQuery::CssDeclaration(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssContainerStyleQuery::CssContainerStyleAndQuery(it) => it.syntax,
            AnyCssContainerStyleQuery::CssContainerStyleInParens(it) => it.syntax,
            AnyCssContainerStyleQuery::CssContainerStyleNotQuery(it) => it.syntax,
            AnyCssContainerStyleQuery::CssContainerStyleOrQuery(it) => it.syntax,
            AnyCssContainerStyleQuery::CssDeclaration(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssContainerStyleQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssContainerStyleQuery::CssContainerStyleAndQuery(it) => std::fmt::Debug::fmt(it, f),
            AnyCssContainerStyleQuery::CssContainerStyleInParens(it) => std::fmt::Debug::fmt(it, f),
            AnyCssContainerStyleQuery::CssContainerStyleNotQuery(it) => std::fmt::Debug::fmt(it, f),
            AnyCssContainerStyleQuery::CssContainerStyleOrQuery(it) => std::fmt::Debug::fmt(it, f),
            AnyCssContainerStyleQuery::CssDeclaration(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssContainerStyleQuery> for SyntaxNode {
    fn from(n: AnyCssContainerStyleQuery) -> SyntaxNode {
        match n {
            AnyCssContainerStyleQuery::CssContainerStyleAndQuery(it) => it.into(),
            AnyCssContainerStyleQuery::CssContainerStyleInParens(it) => it.into(),
            AnyCssContainerStyleQuery::CssContainerStyleNotQuery(it) => it.into(),
            AnyCssContainerStyleQuery::CssContainerStyleOrQuery(it) => it.into(),
            AnyCssContainerStyleQuery::CssDeclaration(it) => it.into(),
        }
    }
}
impl From<AnyCssContainerStyleQuery> for SyntaxElement {
    fn from(n: AnyCssContainerStyleQuery) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusBlock> for AnyCssDeclarationListBlock {
    fn from(node: CssBogusBlock) -> AnyCssDeclarationListBlock {
        AnyCssDeclarationListBlock::CssBogusBlock(node)
    }
}
impl From<CssDeclarationListBlock> for AnyCssDeclarationListBlock {
    fn from(node: CssDeclarationListBlock) -> AnyCssDeclarationListBlock {
        AnyCssDeclarationListBlock::CssDeclarationListBlock(node)
    }
}
impl AstNode for AnyCssDeclarationListBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssBogusBlock::KIND_SET.union(CssDeclarationListBlock::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_BOGUS_BLOCK | CSS_DECLARATION_LIST_BLOCK)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_BLOCK => AnyCssDeclarationListBlock::CssBogusBlock(CssBogusBlock { syntax }),
            CSS_DECLARATION_LIST_BLOCK => {
                AnyCssDeclarationListBlock::CssDeclarationListBlock(CssDeclarationListBlock {
                    syntax,
                })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssDeclarationListBlock::CssBogusBlock(it) => &it.syntax,
            AnyCssDeclarationListBlock::CssDeclarationListBlock(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssDeclarationListBlock::CssBogusBlock(it) => it.syntax,
            AnyCssDeclarationListBlock::CssDeclarationListBlock(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssDeclarationListBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssDeclarationListBlock::CssBogusBlock(it) => std::fmt::Debug::fmt(it, f),
            AnyCssDeclarationListBlock::CssDeclarationListBlock(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssDeclarationListBlock> for SyntaxNode {
    fn from(n: AnyCssDeclarationListBlock) -> SyntaxNode {
        match n {
            AnyCssDeclarationListBlock::CssBogusBlock(it) => it.into(),
            AnyCssDeclarationListBlock::CssDeclarationListBlock(it) => it.into(),
        }
    }
}
impl From<AnyCssDeclarationListBlock> for SyntaxElement {
    fn from(n: AnyCssDeclarationListBlock) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssCustomProperty> for AnyCssDeclarationName {
    fn from(node: CssCustomProperty) -> AnyCssDeclarationName {
        AnyCssDeclarationName::CssCustomProperty(node)
    }
}
impl From<CssIdentifier> for AnyCssDeclarationName {
    fn from(node: CssIdentifier) -> AnyCssDeclarationName {
        AnyCssDeclarationName::CssIdentifier(node)
    }
}
impl AstNode for AnyCssDeclarationName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssCustomProperty::KIND_SET.union(CssIdentifier::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_CUSTOM_PROPERTY | CSS_IDENTIFIER)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_CUSTOM_PROPERTY => {
                AnyCssDeclarationName::CssCustomProperty(CssCustomProperty { syntax })
            }
            CSS_IDENTIFIER => AnyCssDeclarationName::CssIdentifier(CssIdentifier { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssDeclarationName::CssCustomProperty(it) => &it.syntax,
            AnyCssDeclarationName::CssIdentifier(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssDeclarationName::CssCustomProperty(it) => it.syntax,
            AnyCssDeclarationName::CssIdentifier(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssDeclarationName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssDeclarationName::CssCustomProperty(it) => std::fmt::Debug::fmt(it, f),
            AnyCssDeclarationName::CssIdentifier(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssDeclarationName> for SyntaxNode {
    fn from(n: AnyCssDeclarationName) -> SyntaxNode {
        match n {
            AnyCssDeclarationName::CssCustomProperty(it) => it.into(),
            AnyCssDeclarationName::CssIdentifier(it) => it.into(),
        }
    }
}
impl From<AnyCssDeclarationName> for SyntaxElement {
    fn from(n: AnyCssDeclarationName) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssPercentage> for AnyCssDimension {
    fn from(node: CssPercentage) -> AnyCssDimension {
        AnyCssDimension::CssPercentage(node)
    }
}
impl From<CssRegularDimension> for AnyCssDimension {
    fn from(node: CssRegularDimension) -> AnyCssDimension {
        AnyCssDimension::CssRegularDimension(node)
    }
}
impl AstNode for AnyCssDimension {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssPercentage::KIND_SET.union(CssRegularDimension::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_PERCENTAGE | CSS_REGULAR_DIMENSION)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_PERCENTAGE => AnyCssDimension::CssPercentage(CssPercentage { syntax }),
            CSS_REGULAR_DIMENSION => {
                AnyCssDimension::CssRegularDimension(CssRegularDimension { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssDimension::CssPercentage(it) => &it.syntax,
            AnyCssDimension::CssRegularDimension(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssDimension::CssPercentage(it) => it.syntax,
            AnyCssDimension::CssRegularDimension(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssDimension::CssPercentage(it) => std::fmt::Debug::fmt(it, f),
            AnyCssDimension::CssRegularDimension(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssDimension> for SyntaxNode {
    fn from(n: AnyCssDimension) -> SyntaxNode {
        match n {
            AnyCssDimension::CssPercentage(it) => it.into(),
            AnyCssDimension::CssRegularDimension(it) => it.into(),
        }
    }
}
impl From<AnyCssDimension> for SyntaxElement {
    fn from(n: AnyCssDimension) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssIdentifier> for AnyCssKeyframeName {
    fn from(node: CssIdentifier) -> AnyCssKeyframeName {
        AnyCssKeyframeName::CssIdentifier(node)
    }
}
impl From<CssString> for AnyCssKeyframeName {
    fn from(node: CssString) -> AnyCssKeyframeName {
        AnyCssKeyframeName::CssString(node)
    }
}
impl AstNode for AnyCssKeyframeName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssIdentifier::KIND_SET.union(CssString::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_IDENTIFIER | CSS_STRING)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_IDENTIFIER => AnyCssKeyframeName::CssIdentifier(CssIdentifier { syntax }),
            CSS_STRING => AnyCssKeyframeName::CssString(CssString { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssKeyframeName::CssIdentifier(it) => &it.syntax,
            AnyCssKeyframeName::CssString(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssKeyframeName::CssIdentifier(it) => it.syntax,
            AnyCssKeyframeName::CssString(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssKeyframeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssKeyframeName::CssIdentifier(it) => std::fmt::Debug::fmt(it, f),
            AnyCssKeyframeName::CssString(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssKeyframeName> for SyntaxNode {
    fn from(n: AnyCssKeyframeName) -> SyntaxNode {
        match n {
            AnyCssKeyframeName::CssIdentifier(it) => it.into(),
            AnyCssKeyframeName::CssString(it) => it.into(),
        }
    }
}
impl From<AnyCssKeyframeName> for SyntaxElement {
    fn from(n: AnyCssKeyframeName) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusBlock> for AnyCssKeyframesBlock {
    fn from(node: CssBogusBlock) -> AnyCssKeyframesBlock {
        AnyCssKeyframesBlock::CssBogusBlock(node)
    }
}
impl From<CssKeyframesBlock> for AnyCssKeyframesBlock {
    fn from(node: CssKeyframesBlock) -> AnyCssKeyframesBlock {
        AnyCssKeyframesBlock::CssKeyframesBlock(node)
    }
}
impl AstNode for AnyCssKeyframesBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssBogusBlock::KIND_SET.union(CssKeyframesBlock::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_BOGUS_BLOCK | CSS_KEYFRAMES_BLOCK)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_BLOCK => AnyCssKeyframesBlock::CssBogusBlock(CssBogusBlock { syntax }),
            CSS_KEYFRAMES_BLOCK => {
                AnyCssKeyframesBlock::CssKeyframesBlock(CssKeyframesBlock { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssKeyframesBlock::CssBogusBlock(it) => &it.syntax,
            AnyCssKeyframesBlock::CssKeyframesBlock(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssKeyframesBlock::CssBogusBlock(it) => it.syntax,
            AnyCssKeyframesBlock::CssKeyframesBlock(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssKeyframesBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssKeyframesBlock::CssBogusBlock(it) => std::fmt::Debug::fmt(it, f),
            AnyCssKeyframesBlock::CssKeyframesBlock(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssKeyframesBlock> for SyntaxNode {
    fn from(n: AnyCssKeyframesBlock) -> SyntaxNode {
        match n {
            AnyCssKeyframesBlock::CssBogusBlock(it) => it.into(),
            AnyCssKeyframesBlock::CssKeyframesBlock(it) => it.into(),
        }
    }
}
impl From<AnyCssKeyframesBlock> for SyntaxElement {
    fn from(n: AnyCssKeyframesBlock) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusKeyframesItem> for AnyCssKeyframesItem {
    fn from(node: CssBogusKeyframesItem) -> AnyCssKeyframesItem {
        AnyCssKeyframesItem::CssBogusKeyframesItem(node)
    }
}
impl From<CssKeyframesItem> for AnyCssKeyframesItem {
    fn from(node: CssKeyframesItem) -> AnyCssKeyframesItem {
        AnyCssKeyframesItem::CssKeyframesItem(node)
    }
}
impl AstNode for AnyCssKeyframesItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssBogusKeyframesItem::KIND_SET.union(CssKeyframesItem::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_BOGUS_KEYFRAMES_ITEM | CSS_KEYFRAMES_ITEM)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_KEYFRAMES_ITEM => {
                AnyCssKeyframesItem::CssBogusKeyframesItem(CssBogusKeyframesItem { syntax })
            }
            CSS_KEYFRAMES_ITEM => {
                AnyCssKeyframesItem::CssKeyframesItem(CssKeyframesItem { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssKeyframesItem::CssBogusKeyframesItem(it) => &it.syntax,
            AnyCssKeyframesItem::CssKeyframesItem(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssKeyframesItem::CssBogusKeyframesItem(it) => it.syntax,
            AnyCssKeyframesItem::CssKeyframesItem(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssKeyframesItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssKeyframesItem::CssBogusKeyframesItem(it) => std::fmt::Debug::fmt(it, f),
            AnyCssKeyframesItem::CssKeyframesItem(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssKeyframesItem> for SyntaxNode {
    fn from(n: AnyCssKeyframesItem) -> SyntaxNode {
        match n {
            AnyCssKeyframesItem::CssBogusKeyframesItem(it) => it.into(),
            AnyCssKeyframesItem::CssKeyframesItem(it) => it.into(),
        }
    }
}
impl From<AnyCssKeyframesItem> for SyntaxElement {
    fn from(n: AnyCssKeyframesItem) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusSelector> for AnyCssKeyframesSelector {
    fn from(node: CssBogusSelector) -> AnyCssKeyframesSelector {
        AnyCssKeyframesSelector::CssBogusSelector(node)
    }
}
impl From<CssKeyframesIdentSelector> for AnyCssKeyframesSelector {
    fn from(node: CssKeyframesIdentSelector) -> AnyCssKeyframesSelector {
        AnyCssKeyframesSelector::CssKeyframesIdentSelector(node)
    }
}
impl From<CssKeyframesPercentageSelector> for AnyCssKeyframesSelector {
    fn from(node: CssKeyframesPercentageSelector) -> AnyCssKeyframesSelector {
        AnyCssKeyframesSelector::CssKeyframesPercentageSelector(node)
    }
}
impl AstNode for AnyCssKeyframesSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssBogusSelector::KIND_SET
        .union(CssKeyframesIdentSelector::KIND_SET)
        .union(CssKeyframesPercentageSelector::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_BOGUS_SELECTOR | CSS_KEYFRAMES_IDENT_SELECTOR | CSS_KEYFRAMES_PERCENTAGE_SELECTOR
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_SELECTOR => {
                AnyCssKeyframesSelector::CssBogusSelector(CssBogusSelector { syntax })
            }
            CSS_KEYFRAMES_IDENT_SELECTOR => {
                AnyCssKeyframesSelector::CssKeyframesIdentSelector(CssKeyframesIdentSelector {
                    syntax,
                })
            }
            CSS_KEYFRAMES_PERCENTAGE_SELECTOR => {
                AnyCssKeyframesSelector::CssKeyframesPercentageSelector(
                    CssKeyframesPercentageSelector { syntax },
                )
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssKeyframesSelector::CssBogusSelector(it) => &it.syntax,
            AnyCssKeyframesSelector::CssKeyframesIdentSelector(it) => &it.syntax,
            AnyCssKeyframesSelector::CssKeyframesPercentageSelector(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssKeyframesSelector::CssBogusSelector(it) => it.syntax,
            AnyCssKeyframesSelector::CssKeyframesIdentSelector(it) => it.syntax,
            AnyCssKeyframesSelector::CssKeyframesPercentageSelector(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssKeyframesSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssKeyframesSelector::CssBogusSelector(it) => std::fmt::Debug::fmt(it, f),
            AnyCssKeyframesSelector::CssKeyframesIdentSelector(it) => std::fmt::Debug::fmt(it, f),
            AnyCssKeyframesSelector::CssKeyframesPercentageSelector(it) => {
                std::fmt::Debug::fmt(it, f)
            }
        }
    }
}
impl From<AnyCssKeyframesSelector> for SyntaxNode {
    fn from(n: AnyCssKeyframesSelector) -> SyntaxNode {
        match n {
            AnyCssKeyframesSelector::CssBogusSelector(it) => it.into(),
            AnyCssKeyframesSelector::CssKeyframesIdentSelector(it) => it.into(),
            AnyCssKeyframesSelector::CssKeyframesPercentageSelector(it) => it.into(),
        }
    }
}
impl From<AnyCssKeyframesSelector> for SyntaxElement {
    fn from(n: AnyCssKeyframesSelector) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssMediaAndCondition> for AnyCssMediaAndCombinableCondition {
    fn from(node: CssMediaAndCondition) -> AnyCssMediaAndCombinableCondition {
        AnyCssMediaAndCombinableCondition::CssMediaAndCondition(node)
    }
}
impl AstNode for AnyCssMediaAndCombinableCondition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AnyCssMediaInParens::KIND_SET.union(CssMediaAndCondition::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_MEDIA_AND_CONDITION => true,
            k if AnyCssMediaInParens::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_MEDIA_AND_CONDITION => {
                AnyCssMediaAndCombinableCondition::CssMediaAndCondition(CssMediaAndCondition {
                    syntax,
                })
            }
            _ => {
                if let Some(any_css_media_in_parens) = AnyCssMediaInParens::cast(syntax) {
                    return Some(AnyCssMediaAndCombinableCondition::AnyCssMediaInParens(
                        any_css_media_in_parens,
                    ));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssMediaAndCombinableCondition::CssMediaAndCondition(it) => &it.syntax,
            AnyCssMediaAndCombinableCondition::AnyCssMediaInParens(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssMediaAndCombinableCondition::CssMediaAndCondition(it) => it.syntax,
            AnyCssMediaAndCombinableCondition::AnyCssMediaInParens(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssMediaAndCombinableCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssMediaAndCombinableCondition::AnyCssMediaInParens(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssMediaAndCombinableCondition::CssMediaAndCondition(it) => {
                std::fmt::Debug::fmt(it, f)
            }
        }
    }
}
impl From<AnyCssMediaAndCombinableCondition> for SyntaxNode {
    fn from(n: AnyCssMediaAndCombinableCondition) -> SyntaxNode {
        match n {
            AnyCssMediaAndCombinableCondition::AnyCssMediaInParens(it) => it.into(),
            AnyCssMediaAndCombinableCondition::CssMediaAndCondition(it) => it.into(),
        }
    }
}
impl From<AnyCssMediaAndCombinableCondition> for SyntaxElement {
    fn from(n: AnyCssMediaAndCombinableCondition) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssMediaAndCondition> for AnyCssMediaCondition {
    fn from(node: CssMediaAndCondition) -> AnyCssMediaCondition {
        AnyCssMediaCondition::CssMediaAndCondition(node)
    }
}
impl From<CssMediaNotCondition> for AnyCssMediaCondition {
    fn from(node: CssMediaNotCondition) -> AnyCssMediaCondition {
        AnyCssMediaCondition::CssMediaNotCondition(node)
    }
}
impl From<CssMediaOrCondition> for AnyCssMediaCondition {
    fn from(node: CssMediaOrCondition) -> AnyCssMediaCondition {
        AnyCssMediaCondition::CssMediaOrCondition(node)
    }
}
impl AstNode for AnyCssMediaCondition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyCssMediaInParens::KIND_SET
        .union(CssMediaAndCondition::KIND_SET)
        .union(CssMediaNotCondition::KIND_SET)
        .union(CssMediaOrCondition::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_MEDIA_AND_CONDITION | CSS_MEDIA_NOT_CONDITION | CSS_MEDIA_OR_CONDITION => true,
            k if AnyCssMediaInParens::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_MEDIA_AND_CONDITION => {
                AnyCssMediaCondition::CssMediaAndCondition(CssMediaAndCondition { syntax })
            }
            CSS_MEDIA_NOT_CONDITION => {
                AnyCssMediaCondition::CssMediaNotCondition(CssMediaNotCondition { syntax })
            }
            CSS_MEDIA_OR_CONDITION => {
                AnyCssMediaCondition::CssMediaOrCondition(CssMediaOrCondition { syntax })
            }
            _ => {
                if let Some(any_css_media_in_parens) = AnyCssMediaInParens::cast(syntax) {
                    return Some(AnyCssMediaCondition::AnyCssMediaInParens(
                        any_css_media_in_parens,
                    ));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssMediaCondition::CssMediaAndCondition(it) => &it.syntax,
            AnyCssMediaCondition::CssMediaNotCondition(it) => &it.syntax,
            AnyCssMediaCondition::CssMediaOrCondition(it) => &it.syntax,
            AnyCssMediaCondition::AnyCssMediaInParens(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssMediaCondition::CssMediaAndCondition(it) => it.syntax,
            AnyCssMediaCondition::CssMediaNotCondition(it) => it.syntax,
            AnyCssMediaCondition::CssMediaOrCondition(it) => it.syntax,
            AnyCssMediaCondition::AnyCssMediaInParens(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssMediaCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssMediaCondition::AnyCssMediaInParens(it) => std::fmt::Debug::fmt(it, f),
            AnyCssMediaCondition::CssMediaAndCondition(it) => std::fmt::Debug::fmt(it, f),
            AnyCssMediaCondition::CssMediaNotCondition(it) => std::fmt::Debug::fmt(it, f),
            AnyCssMediaCondition::CssMediaOrCondition(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssMediaCondition> for SyntaxNode {
    fn from(n: AnyCssMediaCondition) -> SyntaxNode {
        match n {
            AnyCssMediaCondition::AnyCssMediaInParens(it) => it.into(),
            AnyCssMediaCondition::CssMediaAndCondition(it) => it.into(),
            AnyCssMediaCondition::CssMediaNotCondition(it) => it.into(),
            AnyCssMediaCondition::CssMediaOrCondition(it) => it.into(),
        }
    }
}
impl From<AnyCssMediaCondition> for SyntaxElement {
    fn from(n: AnyCssMediaCondition) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssMediaConditionInParens> for AnyCssMediaInParens {
    fn from(node: CssMediaConditionInParens) -> AnyCssMediaInParens {
        AnyCssMediaInParens::CssMediaConditionInParens(node)
    }
}
impl From<CssMediaFeatureInParens> for AnyCssMediaInParens {
    fn from(node: CssMediaFeatureInParens) -> AnyCssMediaInParens {
        AnyCssMediaInParens::CssMediaFeatureInParens(node)
    }
}
impl AstNode for AnyCssMediaInParens {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssMediaConditionInParens::KIND_SET.union(CssMediaFeatureInParens::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_MEDIA_CONDITION_IN_PARENS | CSS_MEDIA_FEATURE_IN_PARENS
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_MEDIA_CONDITION_IN_PARENS => {
                AnyCssMediaInParens::CssMediaConditionInParens(CssMediaConditionInParens { syntax })
            }
            CSS_MEDIA_FEATURE_IN_PARENS => {
                AnyCssMediaInParens::CssMediaFeatureInParens(CssMediaFeatureInParens { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssMediaInParens::CssMediaConditionInParens(it) => &it.syntax,
            AnyCssMediaInParens::CssMediaFeatureInParens(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssMediaInParens::CssMediaConditionInParens(it) => it.syntax,
            AnyCssMediaInParens::CssMediaFeatureInParens(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssMediaInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssMediaInParens::CssMediaConditionInParens(it) => std::fmt::Debug::fmt(it, f),
            AnyCssMediaInParens::CssMediaFeatureInParens(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssMediaInParens> for SyntaxNode {
    fn from(n: AnyCssMediaInParens) -> SyntaxNode {
        match n {
            AnyCssMediaInParens::CssMediaConditionInParens(it) => it.into(),
            AnyCssMediaInParens::CssMediaFeatureInParens(it) => it.into(),
        }
    }
}
impl From<AnyCssMediaInParens> for SyntaxElement {
    fn from(n: AnyCssMediaInParens) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssMediaOrCondition> for AnyCssMediaOrCombinableCondition {
    fn from(node: CssMediaOrCondition) -> AnyCssMediaOrCombinableCondition {
        AnyCssMediaOrCombinableCondition::CssMediaOrCondition(node)
    }
}
impl AstNode for AnyCssMediaOrCombinableCondition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AnyCssMediaInParens::KIND_SET.union(CssMediaOrCondition::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_MEDIA_OR_CONDITION => true,
            k if AnyCssMediaInParens::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_MEDIA_OR_CONDITION => {
                AnyCssMediaOrCombinableCondition::CssMediaOrCondition(CssMediaOrCondition {
                    syntax,
                })
            }
            _ => {
                if let Some(any_css_media_in_parens) = AnyCssMediaInParens::cast(syntax) {
                    return Some(AnyCssMediaOrCombinableCondition::AnyCssMediaInParens(
                        any_css_media_in_parens,
                    ));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssMediaOrCombinableCondition::CssMediaOrCondition(it) => &it.syntax,
            AnyCssMediaOrCombinableCondition::AnyCssMediaInParens(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssMediaOrCombinableCondition::CssMediaOrCondition(it) => it.syntax,
            AnyCssMediaOrCombinableCondition::AnyCssMediaInParens(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssMediaOrCombinableCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssMediaOrCombinableCondition::AnyCssMediaInParens(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssMediaOrCombinableCondition::CssMediaOrCondition(it) => {
                std::fmt::Debug::fmt(it, f)
            }
        }
    }
}
impl From<AnyCssMediaOrCombinableCondition> for SyntaxNode {
    fn from(n: AnyCssMediaOrCombinableCondition) -> SyntaxNode {
        match n {
            AnyCssMediaOrCombinableCondition::AnyCssMediaInParens(it) => it.into(),
            AnyCssMediaOrCombinableCondition::CssMediaOrCondition(it) => it.into(),
        }
    }
}
impl From<AnyCssMediaOrCombinableCondition> for SyntaxElement {
    fn from(n: AnyCssMediaOrCombinableCondition) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusMediaQuery> for AnyCssMediaQuery {
    fn from(node: CssBogusMediaQuery) -> AnyCssMediaQuery {
        AnyCssMediaQuery::CssBogusMediaQuery(node)
    }
}
impl From<CssMediaConditionQuery> for AnyCssMediaQuery {
    fn from(node: CssMediaConditionQuery) -> AnyCssMediaQuery {
        AnyCssMediaQuery::CssMediaConditionQuery(node)
    }
}
impl AstNode for AnyCssMediaQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyCssMediaTypeQuery::KIND_SET
        .union(CssBogusMediaQuery::KIND_SET)
        .union(CssMediaConditionQuery::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_BOGUS_MEDIA_QUERY | CSS_MEDIA_CONDITION_QUERY => true,
            k if AnyCssMediaTypeQuery::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_MEDIA_QUERY => {
                AnyCssMediaQuery::CssBogusMediaQuery(CssBogusMediaQuery { syntax })
            }
            CSS_MEDIA_CONDITION_QUERY => {
                AnyCssMediaQuery::CssMediaConditionQuery(CssMediaConditionQuery { syntax })
            }
            _ => {
                if let Some(any_css_media_type_query) = AnyCssMediaTypeQuery::cast(syntax) {
                    return Some(AnyCssMediaQuery::AnyCssMediaTypeQuery(
                        any_css_media_type_query,
                    ));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssMediaQuery::CssBogusMediaQuery(it) => &it.syntax,
            AnyCssMediaQuery::CssMediaConditionQuery(it) => &it.syntax,
            AnyCssMediaQuery::AnyCssMediaTypeQuery(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssMediaQuery::CssBogusMediaQuery(it) => it.syntax,
            AnyCssMediaQuery::CssMediaConditionQuery(it) => it.syntax,
            AnyCssMediaQuery::AnyCssMediaTypeQuery(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssMediaQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssMediaQuery::AnyCssMediaTypeQuery(it) => std::fmt::Debug::fmt(it, f),
            AnyCssMediaQuery::CssBogusMediaQuery(it) => std::fmt::Debug::fmt(it, f),
            AnyCssMediaQuery::CssMediaConditionQuery(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssMediaQuery> for SyntaxNode {
    fn from(n: AnyCssMediaQuery) -> SyntaxNode {
        match n {
            AnyCssMediaQuery::AnyCssMediaTypeQuery(it) => it.into(),
            AnyCssMediaQuery::CssBogusMediaQuery(it) => it.into(),
            AnyCssMediaQuery::CssMediaConditionQuery(it) => it.into(),
        }
    }
}
impl From<AnyCssMediaQuery> for SyntaxElement {
    fn from(n: AnyCssMediaQuery) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssMediaAndCondition> for AnyCssMediaTypeCondition {
    fn from(node: CssMediaAndCondition) -> AnyCssMediaTypeCondition {
        AnyCssMediaTypeCondition::CssMediaAndCondition(node)
    }
}
impl From<CssMediaNotCondition> for AnyCssMediaTypeCondition {
    fn from(node: CssMediaNotCondition) -> AnyCssMediaTypeCondition {
        AnyCssMediaTypeCondition::CssMediaNotCondition(node)
    }
}
impl AstNode for AnyCssMediaTypeCondition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyCssMediaInParens::KIND_SET
        .union(CssMediaAndCondition::KIND_SET)
        .union(CssMediaNotCondition::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_MEDIA_AND_CONDITION | CSS_MEDIA_NOT_CONDITION => true,
            k if AnyCssMediaInParens::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_MEDIA_AND_CONDITION => {
                AnyCssMediaTypeCondition::CssMediaAndCondition(CssMediaAndCondition { syntax })
            }
            CSS_MEDIA_NOT_CONDITION => {
                AnyCssMediaTypeCondition::CssMediaNotCondition(CssMediaNotCondition { syntax })
            }
            _ => {
                if let Some(any_css_media_in_parens) = AnyCssMediaInParens::cast(syntax) {
                    return Some(AnyCssMediaTypeCondition::AnyCssMediaInParens(
                        any_css_media_in_parens,
                    ));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssMediaTypeCondition::CssMediaAndCondition(it) => &it.syntax,
            AnyCssMediaTypeCondition::CssMediaNotCondition(it) => &it.syntax,
            AnyCssMediaTypeCondition::AnyCssMediaInParens(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssMediaTypeCondition::CssMediaAndCondition(it) => it.syntax,
            AnyCssMediaTypeCondition::CssMediaNotCondition(it) => it.syntax,
            AnyCssMediaTypeCondition::AnyCssMediaInParens(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssMediaTypeCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssMediaTypeCondition::AnyCssMediaInParens(it) => std::fmt::Debug::fmt(it, f),
            AnyCssMediaTypeCondition::CssMediaAndCondition(it) => std::fmt::Debug::fmt(it, f),
            AnyCssMediaTypeCondition::CssMediaNotCondition(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssMediaTypeCondition> for SyntaxNode {
    fn from(n: AnyCssMediaTypeCondition) -> SyntaxNode {
        match n {
            AnyCssMediaTypeCondition::AnyCssMediaInParens(it) => it.into(),
            AnyCssMediaTypeCondition::CssMediaAndCondition(it) => it.into(),
            AnyCssMediaTypeCondition::CssMediaNotCondition(it) => it.into(),
        }
    }
}
impl From<AnyCssMediaTypeCondition> for SyntaxElement {
    fn from(n: AnyCssMediaTypeCondition) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssMediaAndTypeQuery> for AnyCssMediaTypeQuery {
    fn from(node: CssMediaAndTypeQuery) -> AnyCssMediaTypeQuery {
        AnyCssMediaTypeQuery::CssMediaAndTypeQuery(node)
    }
}
impl From<CssMediaTypeQuery> for AnyCssMediaTypeQuery {
    fn from(node: CssMediaTypeQuery) -> AnyCssMediaTypeQuery {
        AnyCssMediaTypeQuery::CssMediaTypeQuery(node)
    }
}
impl AstNode for AnyCssMediaTypeQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssMediaAndTypeQuery::KIND_SET.union(CssMediaTypeQuery::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_MEDIA_AND_TYPE_QUERY | CSS_MEDIA_TYPE_QUERY)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_MEDIA_AND_TYPE_QUERY => {
                AnyCssMediaTypeQuery::CssMediaAndTypeQuery(CssMediaAndTypeQuery { syntax })
            }
            CSS_MEDIA_TYPE_QUERY => {
                AnyCssMediaTypeQuery::CssMediaTypeQuery(CssMediaTypeQuery { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssMediaTypeQuery::CssMediaAndTypeQuery(it) => &it.syntax,
            AnyCssMediaTypeQuery::CssMediaTypeQuery(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssMediaTypeQuery::CssMediaAndTypeQuery(it) => it.syntax,
            AnyCssMediaTypeQuery::CssMediaTypeQuery(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssMediaTypeQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssMediaTypeQuery::CssMediaAndTypeQuery(it) => std::fmt::Debug::fmt(it, f),
            AnyCssMediaTypeQuery::CssMediaTypeQuery(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssMediaTypeQuery> for SyntaxNode {
    fn from(n: AnyCssMediaTypeQuery) -> SyntaxNode {
        match n {
            AnyCssMediaTypeQuery::CssMediaAndTypeQuery(it) => it.into(),
            AnyCssMediaTypeQuery::CssMediaTypeQuery(it) => it.into(),
        }
    }
}
impl From<AnyCssMediaTypeQuery> for SyntaxElement {
    fn from(n: AnyCssMediaTypeQuery) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssNamedNamespacePrefix> for AnyCssNamespacePrefix {
    fn from(node: CssNamedNamespacePrefix) -> AnyCssNamespacePrefix {
        AnyCssNamespacePrefix::CssNamedNamespacePrefix(node)
    }
}
impl From<CssUniversalNamespacePrefix> for AnyCssNamespacePrefix {
    fn from(node: CssUniversalNamespacePrefix) -> AnyCssNamespacePrefix {
        AnyCssNamespacePrefix::CssUniversalNamespacePrefix(node)
    }
}
impl AstNode for AnyCssNamespacePrefix {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssNamedNamespacePrefix::KIND_SET.union(CssUniversalNamespacePrefix::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_NAMED_NAMESPACE_PREFIX | CSS_UNIVERSAL_NAMESPACE_PREFIX
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_NAMED_NAMESPACE_PREFIX => {
                AnyCssNamespacePrefix::CssNamedNamespacePrefix(CssNamedNamespacePrefix { syntax })
            }
            CSS_UNIVERSAL_NAMESPACE_PREFIX => {
                AnyCssNamespacePrefix::CssUniversalNamespacePrefix(CssUniversalNamespacePrefix {
                    syntax,
                })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssNamespacePrefix::CssNamedNamespacePrefix(it) => &it.syntax,
            AnyCssNamespacePrefix::CssUniversalNamespacePrefix(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssNamespacePrefix::CssNamedNamespacePrefix(it) => it.syntax,
            AnyCssNamespacePrefix::CssUniversalNamespacePrefix(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssNamespacePrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssNamespacePrefix::CssNamedNamespacePrefix(it) => std::fmt::Debug::fmt(it, f),
            AnyCssNamespacePrefix::CssUniversalNamespacePrefix(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssNamespacePrefix> for SyntaxNode {
    fn from(n: AnyCssNamespacePrefix) -> SyntaxNode {
        match n {
            AnyCssNamespacePrefix::CssNamedNamespacePrefix(it) => it.into(),
            AnyCssNamespacePrefix::CssUniversalNamespacePrefix(it) => it.into(),
        }
    }
}
impl From<AnyCssNamespacePrefix> for SyntaxElement {
    fn from(n: AnyCssNamespacePrefix) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusPseudoClass> for AnyCssPseudoClass {
    fn from(node: CssBogusPseudoClass) -> AnyCssPseudoClass {
        AnyCssPseudoClass::CssBogusPseudoClass(node)
    }
}
impl From<CssPseudoClassFunctionCompoundSelector> for AnyCssPseudoClass {
    fn from(node: CssPseudoClassFunctionCompoundSelector) -> AnyCssPseudoClass {
        AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelector(node)
    }
}
impl From<CssPseudoClassFunctionCompoundSelectorList> for AnyCssPseudoClass {
    fn from(node: CssPseudoClassFunctionCompoundSelectorList) -> AnyCssPseudoClass {
        AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelectorList(node)
    }
}
impl From<CssPseudoClassFunctionIdentifier> for AnyCssPseudoClass {
    fn from(node: CssPseudoClassFunctionIdentifier) -> AnyCssPseudoClass {
        AnyCssPseudoClass::CssPseudoClassFunctionIdentifier(node)
    }
}
impl From<CssPseudoClassFunctionNth> for AnyCssPseudoClass {
    fn from(node: CssPseudoClassFunctionNth) -> AnyCssPseudoClass {
        AnyCssPseudoClass::CssPseudoClassFunctionNth(node)
    }
}
impl From<CssPseudoClassFunctionRelativeSelectorList> for AnyCssPseudoClass {
    fn from(node: CssPseudoClassFunctionRelativeSelectorList) -> AnyCssPseudoClass {
        AnyCssPseudoClass::CssPseudoClassFunctionRelativeSelectorList(node)
    }
}
impl From<CssPseudoClassFunctionSelector> for AnyCssPseudoClass {
    fn from(node: CssPseudoClassFunctionSelector) -> AnyCssPseudoClass {
        AnyCssPseudoClass::CssPseudoClassFunctionSelector(node)
    }
}
impl From<CssPseudoClassFunctionSelectorList> for AnyCssPseudoClass {
    fn from(node: CssPseudoClassFunctionSelectorList) -> AnyCssPseudoClass {
        AnyCssPseudoClass::CssPseudoClassFunctionSelectorList(node)
    }
}
impl From<CssPseudoClassFunctionValueList> for AnyCssPseudoClass {
    fn from(node: CssPseudoClassFunctionValueList) -> AnyCssPseudoClass {
        AnyCssPseudoClass::CssPseudoClassFunctionValueList(node)
    }
}
impl From<CssPseudoClassIdentifier> for AnyCssPseudoClass {
    fn from(node: CssPseudoClassIdentifier) -> AnyCssPseudoClass {
        AnyCssPseudoClass::CssPseudoClassIdentifier(node)
    }
}
impl AstNode for AnyCssPseudoClass {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssBogusPseudoClass::KIND_SET
        .union(CssPseudoClassFunctionCompoundSelector::KIND_SET)
        .union(CssPseudoClassFunctionCompoundSelectorList::KIND_SET)
        .union(CssPseudoClassFunctionIdentifier::KIND_SET)
        .union(CssPseudoClassFunctionNth::KIND_SET)
        .union(CssPseudoClassFunctionRelativeSelectorList::KIND_SET)
        .union(CssPseudoClassFunctionSelector::KIND_SET)
        .union(CssPseudoClassFunctionSelectorList::KIND_SET)
        .union(CssPseudoClassFunctionValueList::KIND_SET)
        .union(CssPseudoClassIdentifier::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_BOGUS_PSEUDO_CLASS
                | CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR
                | CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_LIST
                | CSS_PSEUDO_CLASS_FUNCTION_IDENTIFIER
                | CSS_PSEUDO_CLASS_FUNCTION_NTH
                | CSS_PSEUDO_CLASS_FUNCTION_RELATIVE_SELECTOR_LIST
                | CSS_PSEUDO_CLASS_FUNCTION_SELECTOR
                | CSS_PSEUDO_CLASS_FUNCTION_SELECTOR_LIST
                | CSS_PSEUDO_CLASS_FUNCTION_VALUE_LIST
                | CSS_PSEUDO_CLASS_IDENTIFIER
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_PSEUDO_CLASS => {
                AnyCssPseudoClass::CssBogusPseudoClass(CssBogusPseudoClass { syntax })
            }
            CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR => {
                AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelector(
                    CssPseudoClassFunctionCompoundSelector { syntax },
                )
            }
            CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_LIST => {
                AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelectorList(
                    CssPseudoClassFunctionCompoundSelectorList { syntax },
                )
            }
            CSS_PSEUDO_CLASS_FUNCTION_IDENTIFIER => {
                AnyCssPseudoClass::CssPseudoClassFunctionIdentifier(
                    CssPseudoClassFunctionIdentifier { syntax },
                )
            }
            CSS_PSEUDO_CLASS_FUNCTION_NTH => {
                AnyCssPseudoClass::CssPseudoClassFunctionNth(CssPseudoClassFunctionNth { syntax })
            }
            CSS_PSEUDO_CLASS_FUNCTION_RELATIVE_SELECTOR_LIST => {
                AnyCssPseudoClass::CssPseudoClassFunctionRelativeSelectorList(
                    CssPseudoClassFunctionRelativeSelectorList { syntax },
                )
            }
            CSS_PSEUDO_CLASS_FUNCTION_SELECTOR => {
                AnyCssPseudoClass::CssPseudoClassFunctionSelector(CssPseudoClassFunctionSelector {
                    syntax,
                })
            }
            CSS_PSEUDO_CLASS_FUNCTION_SELECTOR_LIST => {
                AnyCssPseudoClass::CssPseudoClassFunctionSelectorList(
                    CssPseudoClassFunctionSelectorList { syntax },
                )
            }
            CSS_PSEUDO_CLASS_FUNCTION_VALUE_LIST => {
                AnyCssPseudoClass::CssPseudoClassFunctionValueList(
                    CssPseudoClassFunctionValueList { syntax },
                )
            }
            CSS_PSEUDO_CLASS_IDENTIFIER => {
                AnyCssPseudoClass::CssPseudoClassIdentifier(CssPseudoClassIdentifier { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssPseudoClass::CssBogusPseudoClass(it) => &it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelector(it) => &it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelectorList(it) => &it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionIdentifier(it) => &it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionNth(it) => &it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionRelativeSelectorList(it) => &it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionSelector(it) => &it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionSelectorList(it) => &it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionValueList(it) => &it.syntax,
            AnyCssPseudoClass::CssPseudoClassIdentifier(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssPseudoClass::CssBogusPseudoClass(it) => it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelector(it) => it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelectorList(it) => it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionIdentifier(it) => it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionNth(it) => it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionRelativeSelectorList(it) => it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionSelector(it) => it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionSelectorList(it) => it.syntax,
            AnyCssPseudoClass::CssPseudoClassFunctionValueList(it) => it.syntax,
            AnyCssPseudoClass::CssPseudoClassIdentifier(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssPseudoClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssPseudoClass::CssBogusPseudoClass(it) => std::fmt::Debug::fmt(it, f),
            AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelector(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelectorList(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssPseudoClass::CssPseudoClassFunctionIdentifier(it) => std::fmt::Debug::fmt(it, f),
            AnyCssPseudoClass::CssPseudoClassFunctionNth(it) => std::fmt::Debug::fmt(it, f),
            AnyCssPseudoClass::CssPseudoClassFunctionRelativeSelectorList(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssPseudoClass::CssPseudoClassFunctionSelector(it) => std::fmt::Debug::fmt(it, f),
            AnyCssPseudoClass::CssPseudoClassFunctionSelectorList(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssPseudoClass::CssPseudoClassFunctionValueList(it) => std::fmt::Debug::fmt(it, f),
            AnyCssPseudoClass::CssPseudoClassIdentifier(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssPseudoClass> for SyntaxNode {
    fn from(n: AnyCssPseudoClass) -> SyntaxNode {
        match n {
            AnyCssPseudoClass::CssBogusPseudoClass(it) => it.into(),
            AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelector(it) => it.into(),
            AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelectorList(it) => it.into(),
            AnyCssPseudoClass::CssPseudoClassFunctionIdentifier(it) => it.into(),
            AnyCssPseudoClass::CssPseudoClassFunctionNth(it) => it.into(),
            AnyCssPseudoClass::CssPseudoClassFunctionRelativeSelectorList(it) => it.into(),
            AnyCssPseudoClass::CssPseudoClassFunctionSelector(it) => it.into(),
            AnyCssPseudoClass::CssPseudoClassFunctionSelectorList(it) => it.into(),
            AnyCssPseudoClass::CssPseudoClassFunctionValueList(it) => it.into(),
            AnyCssPseudoClass::CssPseudoClassIdentifier(it) => it.into(),
        }
    }
}
impl From<AnyCssPseudoClass> for SyntaxElement {
    fn from(n: AnyCssPseudoClass) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssPseudoClassNth> for AnyCssPseudoClassNth {
    fn from(node: CssPseudoClassNth) -> AnyCssPseudoClassNth {
        AnyCssPseudoClassNth::CssPseudoClassNth(node)
    }
}
impl From<CssPseudoClassNthIdentifier> for AnyCssPseudoClassNth {
    fn from(node: CssPseudoClassNthIdentifier) -> AnyCssPseudoClassNth {
        AnyCssPseudoClassNth::CssPseudoClassNthIdentifier(node)
    }
}
impl From<CssPseudoClassNthNumber> for AnyCssPseudoClassNth {
    fn from(node: CssPseudoClassNthNumber) -> AnyCssPseudoClassNth {
        AnyCssPseudoClassNth::CssPseudoClassNthNumber(node)
    }
}
impl AstNode for AnyCssPseudoClassNth {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssPseudoClassNth::KIND_SET
        .union(CssPseudoClassNthIdentifier::KIND_SET)
        .union(CssPseudoClassNthNumber::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_PSEUDO_CLASS_NTH | CSS_PSEUDO_CLASS_NTH_IDENTIFIER | CSS_PSEUDO_CLASS_NTH_NUMBER
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_PSEUDO_CLASS_NTH => {
                AnyCssPseudoClassNth::CssPseudoClassNth(CssPseudoClassNth { syntax })
            }
            CSS_PSEUDO_CLASS_NTH_IDENTIFIER => {
                AnyCssPseudoClassNth::CssPseudoClassNthIdentifier(CssPseudoClassNthIdentifier {
                    syntax,
                })
            }
            CSS_PSEUDO_CLASS_NTH_NUMBER => {
                AnyCssPseudoClassNth::CssPseudoClassNthNumber(CssPseudoClassNthNumber { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssPseudoClassNth::CssPseudoClassNth(it) => &it.syntax,
            AnyCssPseudoClassNth::CssPseudoClassNthIdentifier(it) => &it.syntax,
            AnyCssPseudoClassNth::CssPseudoClassNthNumber(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssPseudoClassNth::CssPseudoClassNth(it) => it.syntax,
            AnyCssPseudoClassNth::CssPseudoClassNthIdentifier(it) => it.syntax,
            AnyCssPseudoClassNth::CssPseudoClassNthNumber(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssPseudoClassNth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssPseudoClassNth::CssPseudoClassNth(it) => std::fmt::Debug::fmt(it, f),
            AnyCssPseudoClassNth::CssPseudoClassNthIdentifier(it) => std::fmt::Debug::fmt(it, f),
            AnyCssPseudoClassNth::CssPseudoClassNthNumber(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssPseudoClassNth> for SyntaxNode {
    fn from(n: AnyCssPseudoClassNth) -> SyntaxNode {
        match n {
            AnyCssPseudoClassNth::CssPseudoClassNth(it) => it.into(),
            AnyCssPseudoClassNth::CssPseudoClassNthIdentifier(it) => it.into(),
            AnyCssPseudoClassNth::CssPseudoClassNthNumber(it) => it.into(),
        }
    }
}
impl From<AnyCssPseudoClassNth> for SyntaxElement {
    fn from(n: AnyCssPseudoClassNth) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusSelector> for AnyCssPseudoClassNthSelector {
    fn from(node: CssBogusSelector) -> AnyCssPseudoClassNthSelector {
        AnyCssPseudoClassNthSelector::CssBogusSelector(node)
    }
}
impl From<CssPseudoClassNthSelector> for AnyCssPseudoClassNthSelector {
    fn from(node: CssPseudoClassNthSelector) -> AnyCssPseudoClassNthSelector {
        AnyCssPseudoClassNthSelector::CssPseudoClassNthSelector(node)
    }
}
impl AstNode for AnyCssPseudoClassNthSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssBogusSelector::KIND_SET.union(CssPseudoClassNthSelector::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_BOGUS_SELECTOR | CSS_PSEUDO_CLASS_NTH_SELECTOR)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_SELECTOR => {
                AnyCssPseudoClassNthSelector::CssBogusSelector(CssBogusSelector { syntax })
            }
            CSS_PSEUDO_CLASS_NTH_SELECTOR => {
                AnyCssPseudoClassNthSelector::CssPseudoClassNthSelector(CssPseudoClassNthSelector {
                    syntax,
                })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssPseudoClassNthSelector::CssBogusSelector(it) => &it.syntax,
            AnyCssPseudoClassNthSelector::CssPseudoClassNthSelector(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssPseudoClassNthSelector::CssBogusSelector(it) => it.syntax,
            AnyCssPseudoClassNthSelector::CssPseudoClassNthSelector(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssPseudoClassNthSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssPseudoClassNthSelector::CssBogusSelector(it) => std::fmt::Debug::fmt(it, f),
            AnyCssPseudoClassNthSelector::CssPseudoClassNthSelector(it) => {
                std::fmt::Debug::fmt(it, f)
            }
        }
    }
}
impl From<AnyCssPseudoClassNthSelector> for SyntaxNode {
    fn from(n: AnyCssPseudoClassNthSelector) -> SyntaxNode {
        match n {
            AnyCssPseudoClassNthSelector::CssBogusSelector(it) => it.into(),
            AnyCssPseudoClassNthSelector::CssPseudoClassNthSelector(it) => it.into(),
        }
    }
}
impl From<AnyCssPseudoClassNthSelector> for SyntaxElement {
    fn from(n: AnyCssPseudoClassNthSelector) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusPseudoElement> for AnyCssPseudoElement {
    fn from(node: CssBogusPseudoElement) -> AnyCssPseudoElement {
        AnyCssPseudoElement::CssBogusPseudoElement(node)
    }
}
impl From<CssPseudoElementFunctionIdentifier> for AnyCssPseudoElement {
    fn from(node: CssPseudoElementFunctionIdentifier) -> AnyCssPseudoElement {
        AnyCssPseudoElement::CssPseudoElementFunctionIdentifier(node)
    }
}
impl From<CssPseudoElementFunctionSelector> for AnyCssPseudoElement {
    fn from(node: CssPseudoElementFunctionSelector) -> AnyCssPseudoElement {
        AnyCssPseudoElement::CssPseudoElementFunctionSelector(node)
    }
}
impl From<CssPseudoElementIdentifier> for AnyCssPseudoElement {
    fn from(node: CssPseudoElementIdentifier) -> AnyCssPseudoElement {
        AnyCssPseudoElement::CssPseudoElementIdentifier(node)
    }
}
impl AstNode for AnyCssPseudoElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssBogusPseudoElement::KIND_SET
        .union(CssPseudoElementFunctionIdentifier::KIND_SET)
        .union(CssPseudoElementFunctionSelector::KIND_SET)
        .union(CssPseudoElementIdentifier::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_BOGUS_PSEUDO_ELEMENT
                | CSS_PSEUDO_ELEMENT_FUNCTION_IDENTIFIER
                | CSS_PSEUDO_ELEMENT_FUNCTION_SELECTOR
                | CSS_PSEUDO_ELEMENT_IDENTIFIER
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_PSEUDO_ELEMENT => {
                AnyCssPseudoElement::CssBogusPseudoElement(CssBogusPseudoElement { syntax })
            }
            CSS_PSEUDO_ELEMENT_FUNCTION_IDENTIFIER => {
                AnyCssPseudoElement::CssPseudoElementFunctionIdentifier(
                    CssPseudoElementFunctionIdentifier { syntax },
                )
            }
            CSS_PSEUDO_ELEMENT_FUNCTION_SELECTOR => {
                AnyCssPseudoElement::CssPseudoElementFunctionSelector(
                    CssPseudoElementFunctionSelector { syntax },
                )
            }
            CSS_PSEUDO_ELEMENT_IDENTIFIER => {
                AnyCssPseudoElement::CssPseudoElementIdentifier(CssPseudoElementIdentifier {
                    syntax,
                })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssPseudoElement::CssBogusPseudoElement(it) => &it.syntax,
            AnyCssPseudoElement::CssPseudoElementFunctionIdentifier(it) => &it.syntax,
            AnyCssPseudoElement::CssPseudoElementFunctionSelector(it) => &it.syntax,
            AnyCssPseudoElement::CssPseudoElementIdentifier(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssPseudoElement::CssBogusPseudoElement(it) => it.syntax,
            AnyCssPseudoElement::CssPseudoElementFunctionIdentifier(it) => it.syntax,
            AnyCssPseudoElement::CssPseudoElementFunctionSelector(it) => it.syntax,
            AnyCssPseudoElement::CssPseudoElementIdentifier(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssPseudoElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssPseudoElement::CssBogusPseudoElement(it) => std::fmt::Debug::fmt(it, f),
            AnyCssPseudoElement::CssPseudoElementFunctionIdentifier(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssPseudoElement::CssPseudoElementFunctionSelector(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            AnyCssPseudoElement::CssPseudoElementIdentifier(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssPseudoElement> for SyntaxNode {
    fn from(n: AnyCssPseudoElement) -> SyntaxNode {
        match n {
            AnyCssPseudoElement::CssBogusPseudoElement(it) => it.into(),
            AnyCssPseudoElement::CssPseudoElementFunctionIdentifier(it) => it.into(),
            AnyCssPseudoElement::CssPseudoElementFunctionSelector(it) => it.into(),
            AnyCssPseudoElement::CssPseudoElementIdentifier(it) => it.into(),
        }
    }
}
impl From<AnyCssPseudoElement> for SyntaxElement {
    fn from(n: AnyCssPseudoElement) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssIdentifier> for AnyCssPseudoValue {
    fn from(node: CssIdentifier) -> AnyCssPseudoValue {
        AnyCssPseudoValue::CssIdentifier(node)
    }
}
impl From<CssString> for AnyCssPseudoValue {
    fn from(node: CssString) -> AnyCssPseudoValue {
        AnyCssPseudoValue::CssString(node)
    }
}
impl AstNode for AnyCssPseudoValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssIdentifier::KIND_SET.union(CssString::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_IDENTIFIER | CSS_STRING)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_IDENTIFIER => AnyCssPseudoValue::CssIdentifier(CssIdentifier { syntax }),
            CSS_STRING => AnyCssPseudoValue::CssString(CssString { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssPseudoValue::CssIdentifier(it) => &it.syntax,
            AnyCssPseudoValue::CssString(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssPseudoValue::CssIdentifier(it) => it.syntax,
            AnyCssPseudoValue::CssString(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssPseudoValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssPseudoValue::CssIdentifier(it) => std::fmt::Debug::fmt(it, f),
            AnyCssPseudoValue::CssString(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssPseudoValue> for SyntaxNode {
    fn from(n: AnyCssPseudoValue) -> SyntaxNode {
        match n {
            AnyCssPseudoValue::CssIdentifier(it) => it.into(),
            AnyCssPseudoValue::CssString(it) => it.into(),
        }
    }
}
impl From<AnyCssPseudoValue> for SyntaxElement {
    fn from(n: AnyCssPseudoValue) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssQueryFeatureBoolean> for AnyCssQueryFeature {
    fn from(node: CssQueryFeatureBoolean) -> AnyCssQueryFeature {
        AnyCssQueryFeature::CssQueryFeatureBoolean(node)
    }
}
impl From<CssQueryFeaturePlain> for AnyCssQueryFeature {
    fn from(node: CssQueryFeaturePlain) -> AnyCssQueryFeature {
        AnyCssQueryFeature::CssQueryFeaturePlain(node)
    }
}
impl From<CssQueryFeatureRange> for AnyCssQueryFeature {
    fn from(node: CssQueryFeatureRange) -> AnyCssQueryFeature {
        AnyCssQueryFeature::CssQueryFeatureRange(node)
    }
}
impl From<CssQueryFeatureRangeInterval> for AnyCssQueryFeature {
    fn from(node: CssQueryFeatureRangeInterval) -> AnyCssQueryFeature {
        AnyCssQueryFeature::CssQueryFeatureRangeInterval(node)
    }
}
impl From<CssQueryFeatureReverseRange> for AnyCssQueryFeature {
    fn from(node: CssQueryFeatureReverseRange) -> AnyCssQueryFeature {
        AnyCssQueryFeature::CssQueryFeatureReverseRange(node)
    }
}
impl AstNode for AnyCssQueryFeature {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssQueryFeatureBoolean::KIND_SET
        .union(CssQueryFeaturePlain::KIND_SET)
        .union(CssQueryFeatureRange::KIND_SET)
        .union(CssQueryFeatureRangeInterval::KIND_SET)
        .union(CssQueryFeatureReverseRange::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_QUERY_FEATURE_BOOLEAN
                | CSS_QUERY_FEATURE_PLAIN
                | CSS_QUERY_FEATURE_RANGE
                | CSS_QUERY_FEATURE_RANGE_INTERVAL
                | CSS_QUERY_FEATURE_REVERSE_RANGE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_QUERY_FEATURE_BOOLEAN => {
                AnyCssQueryFeature::CssQueryFeatureBoolean(CssQueryFeatureBoolean { syntax })
            }
            CSS_QUERY_FEATURE_PLAIN => {
                AnyCssQueryFeature::CssQueryFeaturePlain(CssQueryFeaturePlain { syntax })
            }
            CSS_QUERY_FEATURE_RANGE => {
                AnyCssQueryFeature::CssQueryFeatureRange(CssQueryFeatureRange { syntax })
            }
            CSS_QUERY_FEATURE_RANGE_INTERVAL => {
                AnyCssQueryFeature::CssQueryFeatureRangeInterval(CssQueryFeatureRangeInterval {
                    syntax,
                })
            }
            CSS_QUERY_FEATURE_REVERSE_RANGE => {
                AnyCssQueryFeature::CssQueryFeatureReverseRange(CssQueryFeatureReverseRange {
                    syntax,
                })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssQueryFeature::CssQueryFeatureBoolean(it) => &it.syntax,
            AnyCssQueryFeature::CssQueryFeaturePlain(it) => &it.syntax,
            AnyCssQueryFeature::CssQueryFeatureRange(it) => &it.syntax,
            AnyCssQueryFeature::CssQueryFeatureRangeInterval(it) => &it.syntax,
            AnyCssQueryFeature::CssQueryFeatureReverseRange(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssQueryFeature::CssQueryFeatureBoolean(it) => it.syntax,
            AnyCssQueryFeature::CssQueryFeaturePlain(it) => it.syntax,
            AnyCssQueryFeature::CssQueryFeatureRange(it) => it.syntax,
            AnyCssQueryFeature::CssQueryFeatureRangeInterval(it) => it.syntax,
            AnyCssQueryFeature::CssQueryFeatureReverseRange(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssQueryFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssQueryFeature::CssQueryFeatureBoolean(it) => std::fmt::Debug::fmt(it, f),
            AnyCssQueryFeature::CssQueryFeaturePlain(it) => std::fmt::Debug::fmt(it, f),
            AnyCssQueryFeature::CssQueryFeatureRange(it) => std::fmt::Debug::fmt(it, f),
            AnyCssQueryFeature::CssQueryFeatureRangeInterval(it) => std::fmt::Debug::fmt(it, f),
            AnyCssQueryFeature::CssQueryFeatureReverseRange(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssQueryFeature> for SyntaxNode {
    fn from(n: AnyCssQueryFeature) -> SyntaxNode {
        match n {
            AnyCssQueryFeature::CssQueryFeatureBoolean(it) => it.into(),
            AnyCssQueryFeature::CssQueryFeaturePlain(it) => it.into(),
            AnyCssQueryFeature::CssQueryFeatureRange(it) => it.into(),
            AnyCssQueryFeature::CssQueryFeatureRangeInterval(it) => it.into(),
            AnyCssQueryFeature::CssQueryFeatureReverseRange(it) => it.into(),
        }
    }
}
impl From<AnyCssQueryFeature> for SyntaxElement {
    fn from(n: AnyCssQueryFeature) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssAnyFunction> for AnyCssQueryFeatureValue {
    fn from(node: CssAnyFunction) -> AnyCssQueryFeatureValue {
        AnyCssQueryFeatureValue::CssAnyFunction(node)
    }
}
impl From<CssIdentifier> for AnyCssQueryFeatureValue {
    fn from(node: CssIdentifier) -> AnyCssQueryFeatureValue {
        AnyCssQueryFeatureValue::CssIdentifier(node)
    }
}
impl From<CssNumber> for AnyCssQueryFeatureValue {
    fn from(node: CssNumber) -> AnyCssQueryFeatureValue {
        AnyCssQueryFeatureValue::CssNumber(node)
    }
}
impl From<CssRatio> for AnyCssQueryFeatureValue {
    fn from(node: CssRatio) -> AnyCssQueryFeatureValue {
        AnyCssQueryFeatureValue::CssRatio(node)
    }
}
impl AstNode for AnyCssQueryFeatureValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyCssDimension::KIND_SET
        .union(CssAnyFunction::KIND_SET)
        .union(CssIdentifier::KIND_SET)
        .union(CssNumber::KIND_SET)
        .union(CssRatio::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_ANY_FUNCTION | CSS_IDENTIFIER | CSS_NUMBER | CSS_RATIO => true,
            k if AnyCssDimension::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_ANY_FUNCTION => AnyCssQueryFeatureValue::CssAnyFunction(CssAnyFunction { syntax }),
            CSS_IDENTIFIER => AnyCssQueryFeatureValue::CssIdentifier(CssIdentifier { syntax }),
            CSS_NUMBER => AnyCssQueryFeatureValue::CssNumber(CssNumber { syntax }),
            CSS_RATIO => AnyCssQueryFeatureValue::CssRatio(CssRatio { syntax }),
            _ => {
                if let Some(any_css_dimension) = AnyCssDimension::cast(syntax) {
                    return Some(AnyCssQueryFeatureValue::AnyCssDimension(any_css_dimension));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssQueryFeatureValue::CssAnyFunction(it) => &it.syntax,
            AnyCssQueryFeatureValue::CssIdentifier(it) => &it.syntax,
            AnyCssQueryFeatureValue::CssNumber(it) => &it.syntax,
            AnyCssQueryFeatureValue::CssRatio(it) => &it.syntax,
            AnyCssQueryFeatureValue::AnyCssDimension(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssQueryFeatureValue::CssAnyFunction(it) => it.syntax,
            AnyCssQueryFeatureValue::CssIdentifier(it) => it.syntax,
            AnyCssQueryFeatureValue::CssNumber(it) => it.syntax,
            AnyCssQueryFeatureValue::CssRatio(it) => it.syntax,
            AnyCssQueryFeatureValue::AnyCssDimension(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssQueryFeatureValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssQueryFeatureValue::AnyCssDimension(it) => std::fmt::Debug::fmt(it, f),
            AnyCssQueryFeatureValue::CssAnyFunction(it) => std::fmt::Debug::fmt(it, f),
            AnyCssQueryFeatureValue::CssIdentifier(it) => std::fmt::Debug::fmt(it, f),
            AnyCssQueryFeatureValue::CssNumber(it) => std::fmt::Debug::fmt(it, f),
            AnyCssQueryFeatureValue::CssRatio(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssQueryFeatureValue> for SyntaxNode {
    fn from(n: AnyCssQueryFeatureValue) -> SyntaxNode {
        match n {
            AnyCssQueryFeatureValue::AnyCssDimension(it) => it.into(),
            AnyCssQueryFeatureValue::CssAnyFunction(it) => it.into(),
            AnyCssQueryFeatureValue::CssIdentifier(it) => it.into(),
            AnyCssQueryFeatureValue::CssNumber(it) => it.into(),
            AnyCssQueryFeatureValue::CssRatio(it) => it.into(),
        }
    }
}
impl From<AnyCssQueryFeatureValue> for SyntaxElement {
    fn from(n: AnyCssQueryFeatureValue) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusSelector> for AnyCssRelativeSelector {
    fn from(node: CssBogusSelector) -> AnyCssRelativeSelector {
        AnyCssRelativeSelector::CssBogusSelector(node)
    }
}
impl From<CssRelativeSelector> for AnyCssRelativeSelector {
    fn from(node: CssRelativeSelector) -> AnyCssRelativeSelector {
        AnyCssRelativeSelector::CssRelativeSelector(node)
    }
}
impl AstNode for AnyCssRelativeSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssBogusSelector::KIND_SET.union(CssRelativeSelector::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_BOGUS_SELECTOR | CSS_RELATIVE_SELECTOR)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_SELECTOR => {
                AnyCssRelativeSelector::CssBogusSelector(CssBogusSelector { syntax })
            }
            CSS_RELATIVE_SELECTOR => {
                AnyCssRelativeSelector::CssRelativeSelector(CssRelativeSelector { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssRelativeSelector::CssBogusSelector(it) => &it.syntax,
            AnyCssRelativeSelector::CssRelativeSelector(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssRelativeSelector::CssBogusSelector(it) => it.syntax,
            AnyCssRelativeSelector::CssRelativeSelector(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssRelativeSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssRelativeSelector::CssBogusSelector(it) => std::fmt::Debug::fmt(it, f),
            AnyCssRelativeSelector::CssRelativeSelector(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssRelativeSelector> for SyntaxNode {
    fn from(n: AnyCssRelativeSelector) -> SyntaxNode {
        match n {
            AnyCssRelativeSelector::CssBogusSelector(it) => it.into(),
            AnyCssRelativeSelector::CssRelativeSelector(it) => it.into(),
        }
    }
}
impl From<AnyCssRelativeSelector> for SyntaxElement {
    fn from(n: AnyCssRelativeSelector) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssAtRule> for AnyCssRule {
    fn from(node: CssAtRule) -> AnyCssRule {
        AnyCssRule::CssAtRule(node)
    }
}
impl From<CssBogusRule> for AnyCssRule {
    fn from(node: CssBogusRule) -> AnyCssRule {
        AnyCssRule::CssBogusRule(node)
    }
}
impl From<CssRule> for AnyCssRule {
    fn from(node: CssRule) -> AnyCssRule {
        AnyCssRule::CssRule(node)
    }
}
impl AstNode for AnyCssRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssAtRule::KIND_SET
        .union(CssBogusRule::KIND_SET)
        .union(CssRule::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_AT_RULE | CSS_BOGUS_RULE | CSS_RULE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_AT_RULE => AnyCssRule::CssAtRule(CssAtRule { syntax }),
            CSS_BOGUS_RULE => AnyCssRule::CssBogusRule(CssBogusRule { syntax }),
            CSS_RULE => AnyCssRule::CssRule(CssRule { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssRule::CssAtRule(it) => &it.syntax,
            AnyCssRule::CssBogusRule(it) => &it.syntax,
            AnyCssRule::CssRule(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssRule::CssAtRule(it) => it.syntax,
            AnyCssRule::CssBogusRule(it) => it.syntax,
            AnyCssRule::CssRule(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssRule::CssAtRule(it) => std::fmt::Debug::fmt(it, f),
            AnyCssRule::CssBogusRule(it) => std::fmt::Debug::fmt(it, f),
            AnyCssRule::CssRule(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssRule> for SyntaxNode {
    fn from(n: AnyCssRule) -> SyntaxNode {
        match n {
            AnyCssRule::CssAtRule(it) => it.into(),
            AnyCssRule::CssBogusRule(it) => it.into(),
            AnyCssRule::CssRule(it) => it.into(),
        }
    }
}
impl From<AnyCssRule> for SyntaxElement {
    fn from(n: AnyCssRule) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusBlock> for AnyCssRuleListBlock {
    fn from(node: CssBogusBlock) -> AnyCssRuleListBlock {
        AnyCssRuleListBlock::CssBogusBlock(node)
    }
}
impl From<CssRuleListBlock> for AnyCssRuleListBlock {
    fn from(node: CssRuleListBlock) -> AnyCssRuleListBlock {
        AnyCssRuleListBlock::CssRuleListBlock(node)
    }
}
impl AstNode for AnyCssRuleListBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssBogusBlock::KIND_SET.union(CssRuleListBlock::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_BOGUS_BLOCK | CSS_RULE_LIST_BLOCK)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_BLOCK => AnyCssRuleListBlock::CssBogusBlock(CssBogusBlock { syntax }),
            CSS_RULE_LIST_BLOCK => {
                AnyCssRuleListBlock::CssRuleListBlock(CssRuleListBlock { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssRuleListBlock::CssBogusBlock(it) => &it.syntax,
            AnyCssRuleListBlock::CssRuleListBlock(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssRuleListBlock::CssBogusBlock(it) => it.syntax,
            AnyCssRuleListBlock::CssRuleListBlock(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssRuleListBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssRuleListBlock::CssBogusBlock(it) => std::fmt::Debug::fmt(it, f),
            AnyCssRuleListBlock::CssRuleListBlock(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssRuleListBlock> for SyntaxNode {
    fn from(n: AnyCssRuleListBlock) -> SyntaxNode {
        match n {
            AnyCssRuleListBlock::CssBogusBlock(it) => it.into(),
            AnyCssRuleListBlock::CssRuleListBlock(it) => it.into(),
        }
    }
}
impl From<AnyCssRuleListBlock> for SyntaxElement {
    fn from(n: AnyCssRuleListBlock) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssBogusSelector> for AnyCssSelector {
    fn from(node: CssBogusSelector) -> AnyCssSelector {
        AnyCssSelector::CssBogusSelector(node)
    }
}
impl From<CssComplexSelector> for AnyCssSelector {
    fn from(node: CssComplexSelector) -> AnyCssSelector {
        AnyCssSelector::CssComplexSelector(node)
    }
}
impl From<CssCompoundSelector> for AnyCssSelector {
    fn from(node: CssCompoundSelector) -> AnyCssSelector {
        AnyCssSelector::CssCompoundSelector(node)
    }
}
impl AstNode for AnyCssSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssBogusSelector::KIND_SET
        .union(CssComplexSelector::KIND_SET)
        .union(CssCompoundSelector::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_BOGUS_SELECTOR | CSS_COMPLEX_SELECTOR | CSS_COMPOUND_SELECTOR
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_BOGUS_SELECTOR => AnyCssSelector::CssBogusSelector(CssBogusSelector { syntax }),
            CSS_COMPLEX_SELECTOR => {
                AnyCssSelector::CssComplexSelector(CssComplexSelector { syntax })
            }
            CSS_COMPOUND_SELECTOR => {
                AnyCssSelector::CssCompoundSelector(CssCompoundSelector { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssSelector::CssBogusSelector(it) => &it.syntax,
            AnyCssSelector::CssComplexSelector(it) => &it.syntax,
            AnyCssSelector::CssCompoundSelector(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssSelector::CssBogusSelector(it) => it.syntax,
            AnyCssSelector::CssComplexSelector(it) => it.syntax,
            AnyCssSelector::CssCompoundSelector(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssSelector::CssBogusSelector(it) => std::fmt::Debug::fmt(it, f),
            AnyCssSelector::CssComplexSelector(it) => std::fmt::Debug::fmt(it, f),
            AnyCssSelector::CssCompoundSelector(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssSelector> for SyntaxNode {
    fn from(n: AnyCssSelector) -> SyntaxNode {
        match n {
            AnyCssSelector::CssBogusSelector(it) => it.into(),
            AnyCssSelector::CssComplexSelector(it) => it.into(),
            AnyCssSelector::CssCompoundSelector(it) => it.into(),
        }
    }
}
impl From<AnyCssSelector> for SyntaxElement {
    fn from(n: AnyCssSelector) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssTypeSelector> for AnyCssSimpleSelector {
    fn from(node: CssTypeSelector) -> AnyCssSimpleSelector {
        AnyCssSimpleSelector::CssTypeSelector(node)
    }
}
impl From<CssUniversalSelector> for AnyCssSimpleSelector {
    fn from(node: CssUniversalSelector) -> AnyCssSimpleSelector {
        AnyCssSimpleSelector::CssUniversalSelector(node)
    }
}
impl AstNode for AnyCssSimpleSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssTypeSelector::KIND_SET.union(CssUniversalSelector::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_TYPE_SELECTOR | CSS_UNIVERSAL_SELECTOR)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_TYPE_SELECTOR => AnyCssSimpleSelector::CssTypeSelector(CssTypeSelector { syntax }),
            CSS_UNIVERSAL_SELECTOR => {
                AnyCssSimpleSelector::CssUniversalSelector(CssUniversalSelector { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssSimpleSelector::CssTypeSelector(it) => &it.syntax,
            AnyCssSimpleSelector::CssUniversalSelector(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssSimpleSelector::CssTypeSelector(it) => it.syntax,
            AnyCssSimpleSelector::CssUniversalSelector(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssSimpleSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssSimpleSelector::CssTypeSelector(it) => std::fmt::Debug::fmt(it, f),
            AnyCssSimpleSelector::CssUniversalSelector(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssSimpleSelector> for SyntaxNode {
    fn from(n: AnyCssSimpleSelector) -> SyntaxNode {
        match n {
            AnyCssSimpleSelector::CssTypeSelector(it) => it.into(),
            AnyCssSimpleSelector::CssUniversalSelector(it) => it.into(),
        }
    }
}
impl From<AnyCssSimpleSelector> for SyntaxElement {
    fn from(n: AnyCssSimpleSelector) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssAttributeSelector> for AnyCssSubSelector {
    fn from(node: CssAttributeSelector) -> AnyCssSubSelector {
        AnyCssSubSelector::CssAttributeSelector(node)
    }
}
impl From<CssBogusSubSelector> for AnyCssSubSelector {
    fn from(node: CssBogusSubSelector) -> AnyCssSubSelector {
        AnyCssSubSelector::CssBogusSubSelector(node)
    }
}
impl From<CssClassSelector> for AnyCssSubSelector {
    fn from(node: CssClassSelector) -> AnyCssSubSelector {
        AnyCssSubSelector::CssClassSelector(node)
    }
}
impl From<CssIdSelector> for AnyCssSubSelector {
    fn from(node: CssIdSelector) -> AnyCssSubSelector {
        AnyCssSubSelector::CssIdSelector(node)
    }
}
impl From<CssPseudoClassSelector> for AnyCssSubSelector {
    fn from(node: CssPseudoClassSelector) -> AnyCssSubSelector {
        AnyCssSubSelector::CssPseudoClassSelector(node)
    }
}
impl From<CssPseudoElementSelector> for AnyCssSubSelector {
    fn from(node: CssPseudoElementSelector) -> AnyCssSubSelector {
        AnyCssSubSelector::CssPseudoElementSelector(node)
    }
}
impl AstNode for AnyCssSubSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssAttributeSelector::KIND_SET
        .union(CssBogusSubSelector::KIND_SET)
        .union(CssClassSelector::KIND_SET)
        .union(CssIdSelector::KIND_SET)
        .union(CssPseudoClassSelector::KIND_SET)
        .union(CssPseudoElementSelector::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_ATTRIBUTE_SELECTOR
                | CSS_BOGUS_SUB_SELECTOR
                | CSS_CLASS_SELECTOR
                | CSS_ID_SELECTOR
                | CSS_PSEUDO_CLASS_SELECTOR
                | CSS_PSEUDO_ELEMENT_SELECTOR
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_ATTRIBUTE_SELECTOR => {
                AnyCssSubSelector::CssAttributeSelector(CssAttributeSelector { syntax })
            }
            CSS_BOGUS_SUB_SELECTOR => {
                AnyCssSubSelector::CssBogusSubSelector(CssBogusSubSelector { syntax })
            }
            CSS_CLASS_SELECTOR => AnyCssSubSelector::CssClassSelector(CssClassSelector { syntax }),
            CSS_ID_SELECTOR => AnyCssSubSelector::CssIdSelector(CssIdSelector { syntax }),
            CSS_PSEUDO_CLASS_SELECTOR => {
                AnyCssSubSelector::CssPseudoClassSelector(CssPseudoClassSelector { syntax })
            }
            CSS_PSEUDO_ELEMENT_SELECTOR => {
                AnyCssSubSelector::CssPseudoElementSelector(CssPseudoElementSelector { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssSubSelector::CssAttributeSelector(it) => &it.syntax,
            AnyCssSubSelector::CssBogusSubSelector(it) => &it.syntax,
            AnyCssSubSelector::CssClassSelector(it) => &it.syntax,
            AnyCssSubSelector::CssIdSelector(it) => &it.syntax,
            AnyCssSubSelector::CssPseudoClassSelector(it) => &it.syntax,
            AnyCssSubSelector::CssPseudoElementSelector(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssSubSelector::CssAttributeSelector(it) => it.syntax,
            AnyCssSubSelector::CssBogusSubSelector(it) => it.syntax,
            AnyCssSubSelector::CssClassSelector(it) => it.syntax,
            AnyCssSubSelector::CssIdSelector(it) => it.syntax,
            AnyCssSubSelector::CssPseudoClassSelector(it) => it.syntax,
            AnyCssSubSelector::CssPseudoElementSelector(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyCssSubSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssSubSelector::CssAttributeSelector(it) => std::fmt::Debug::fmt(it, f),
            AnyCssSubSelector::CssBogusSubSelector(it) => std::fmt::Debug::fmt(it, f),
            AnyCssSubSelector::CssClassSelector(it) => std::fmt::Debug::fmt(it, f),
            AnyCssSubSelector::CssIdSelector(it) => std::fmt::Debug::fmt(it, f),
            AnyCssSubSelector::CssPseudoClassSelector(it) => std::fmt::Debug::fmt(it, f),
            AnyCssSubSelector::CssPseudoElementSelector(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssSubSelector> for SyntaxNode {
    fn from(n: AnyCssSubSelector) -> SyntaxNode {
        match n {
            AnyCssSubSelector::CssAttributeSelector(it) => it.into(),
            AnyCssSubSelector::CssBogusSubSelector(it) => it.into(),
            AnyCssSubSelector::CssClassSelector(it) => it.into(),
            AnyCssSubSelector::CssIdSelector(it) => it.into(),
            AnyCssSubSelector::CssPseudoClassSelector(it) => it.into(),
            AnyCssSubSelector::CssPseudoElementSelector(it) => it.into(),
        }
    }
}
impl From<AnyCssSubSelector> for SyntaxElement {
    fn from(n: AnyCssSubSelector) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssAnyFunction> for AnyCssValue {
    fn from(node: CssAnyFunction) -> AnyCssValue {
        AnyCssValue::CssAnyFunction(node)
    }
}
impl From<CssCustomProperty> for AnyCssValue {
    fn from(node: CssCustomProperty) -> AnyCssValue {
        AnyCssValue::CssCustomProperty(node)
    }
}
impl From<CssIdentifier> for AnyCssValue {
    fn from(node: CssIdentifier) -> AnyCssValue {
        AnyCssValue::CssIdentifier(node)
    }
}
impl From<CssNumber> for AnyCssValue {
    fn from(node: CssNumber) -> AnyCssValue {
        AnyCssValue::CssNumber(node)
    }
}
impl From<CssRatio> for AnyCssValue {
    fn from(node: CssRatio) -> AnyCssValue {
        AnyCssValue::CssRatio(node)
    }
}
impl From<CssString> for AnyCssValue {
    fn from(node: CssString) -> AnyCssValue {
        AnyCssValue::CssString(node)
    }
}
impl AstNode for AnyCssValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyCssDimension::KIND_SET
        .union(CssAnyFunction::KIND_SET)
        .union(CssCustomProperty::KIND_SET)
        .union(CssIdentifier::KIND_SET)
        .union(CssNumber::KIND_SET)
        .union(CssRatio::KIND_SET)
        .union(CssString::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_ANY_FUNCTION | CSS_CUSTOM_PROPERTY | CSS_IDENTIFIER | CSS_NUMBER | CSS_RATIO
            | CSS_STRING => true,
            k if AnyCssDimension::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_ANY_FUNCTION => AnyCssValue::CssAnyFunction(CssAnyFunction { syntax }),
            CSS_CUSTOM_PROPERTY => AnyCssValue::CssCustomProperty(CssCustomProperty { syntax }),
            CSS_IDENTIFIER => AnyCssValue::CssIdentifier(CssIdentifier { syntax }),
            CSS_NUMBER => AnyCssValue::CssNumber(CssNumber { syntax }),
            CSS_RATIO => AnyCssValue::CssRatio(CssRatio { syntax }),
            CSS_STRING => AnyCssValue::CssString(CssString { syntax }),
            _ => {
                if let Some(any_css_dimension) = AnyCssDimension::cast(syntax) {
                    return Some(AnyCssValue::AnyCssDimension(any_css_dimension));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyCssValue::CssAnyFunction(it) => &it.syntax,
            AnyCssValue::CssCustomProperty(it) => &it.syntax,
            AnyCssValue::CssIdentifier(it) => &it.syntax,
            AnyCssValue::CssNumber(it) => &it.syntax,
            AnyCssValue::CssRatio(it) => &it.syntax,
            AnyCssValue::CssString(it) => &it.syntax,
            AnyCssValue::AnyCssDimension(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyCssValue::CssAnyFunction(it) => it.syntax,
            AnyCssValue::CssCustomProperty(it) => it.syntax,
            AnyCssValue::CssIdentifier(it) => it.syntax,
            AnyCssValue::CssNumber(it) => it.syntax,
            AnyCssValue::CssRatio(it) => it.syntax,
            AnyCssValue::CssString(it) => it.syntax,
            AnyCssValue::AnyCssDimension(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyCssValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyCssValue::AnyCssDimension(it) => std::fmt::Debug::fmt(it, f),
            AnyCssValue::CssAnyFunction(it) => std::fmt::Debug::fmt(it, f),
            AnyCssValue::CssCustomProperty(it) => std::fmt::Debug::fmt(it, f),
            AnyCssValue::CssIdentifier(it) => std::fmt::Debug::fmt(it, f),
            AnyCssValue::CssNumber(it) => std::fmt::Debug::fmt(it, f),
            AnyCssValue::CssRatio(it) => std::fmt::Debug::fmt(it, f),
            AnyCssValue::CssString(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyCssValue> for SyntaxNode {
    fn from(n: AnyCssValue) -> SyntaxNode {
        match n {
            AnyCssValue::AnyCssDimension(it) => it.into(),
            AnyCssValue::CssAnyFunction(it) => it.into(),
            AnyCssValue::CssCustomProperty(it) => it.into(),
            AnyCssValue::CssIdentifier(it) => it.into(),
            AnyCssValue::CssNumber(it) => it.into(),
            AnyCssValue::CssRatio(it) => it.into(),
            AnyCssValue::CssString(it) => it.into(),
        }
    }
}
impl From<AnyCssValue> for SyntaxElement {
    fn from(n: AnyCssValue) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnyCssAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssAttributeMatcherValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssCompoundSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssContainerAndCombinableQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssContainerOrCombinableQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssContainerQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssContainerQueryInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssContainerStyleAndCombinableQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssContainerStyleInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssContainerStyleOrCombinableQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssContainerStyleQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssDeclarationListBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssDeclarationName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssKeyframeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssKeyframesBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssKeyframesItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssKeyframesSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssMediaAndCombinableCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssMediaCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssMediaInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssMediaOrCombinableCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssMediaQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssMediaTypeCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssMediaTypeQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssNamespacePrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssPseudoClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssPseudoClassNth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssPseudoClassNthSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssPseudoElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssPseudoValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssQueryFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssQueryFeatureValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssRelativeSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssRuleListBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssSimpleSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssSubSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyCssValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAnyFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAttributeMatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAttributeMatcherValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAttributeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAttributeSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssCharsetAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssClassSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssColorProfileAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssComplexSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssCompoundSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssContainerAndQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssContainerAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssContainerNotQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssContainerOrQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssContainerQueryInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssContainerSizeFeatureInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssContainerStyleAndQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssContainerStyleInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssContainerStyleNotQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssContainerStyleOrQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssContainerStyleQueryInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssCounterStyleAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssCustomProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssDeclarationImportant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssDeclarationListBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssFontFaceAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssFontPaletteValuesAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssIdSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssKeyframesAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssKeyframesBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssKeyframesIdentSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssKeyframesItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssKeyframesPercentageSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMediaAndCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMediaAndTypeQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMediaAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMediaConditionInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMediaConditionQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMediaFeatureInParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMediaNotCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMediaOrCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssMediaTypeQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssNamedNamespacePrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssNamespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssNthOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPercentDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPercentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassFunctionCompoundSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassFunctionCompoundSelectorList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassFunctionIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassFunctionNth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassFunctionRelativeSelectorList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassFunctionSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassFunctionSelectorList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassFunctionValueList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassNth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassNthIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassNthNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassNthSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassOfNthSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoElementFunctionIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoElementFunctionSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoElementIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoElementSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssQueryFeatureBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssQueryFeaturePlain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssQueryFeatureRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssQueryFeatureRangeComparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssQueryFeatureRangeInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssQueryFeatureReverseRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssRatio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssRegularDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssRelativeSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssRuleListBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssSimpleFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssTypeSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssUniversalNamespacePrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssUniversalSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssVarFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssVarFunctionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssBogus {
    syntax: SyntaxNode,
}
impl CssBogus {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for CssBogus {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogus")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogus> for SyntaxNode {
    fn from(n: CssBogus) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssBogus> for SyntaxElement {
    fn from(n: CssBogus) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssBogusAtRule {
    syntax: SyntaxNode,
}
impl CssBogusAtRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for CssBogusAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_AT_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_AT_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusAtRule")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusAtRule> for SyntaxNode {
    fn from(n: CssBogusAtRule) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssBogusAtRule> for SyntaxElement {
    fn from(n: CssBogusAtRule) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssBogusBlock {
    syntax: SyntaxNode,
}
impl CssBogusBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for CssBogusBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusBlock")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusBlock> for SyntaxNode {
    fn from(n: CssBogusBlock) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssBogusBlock> for SyntaxElement {
    fn from(n: CssBogusBlock) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssBogusComponentValue {
    syntax: SyntaxNode,
}
impl CssBogusComponentValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for CssBogusComponentValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_COMPONENT_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_COMPONENT_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusComponentValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusComponentValue")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusComponentValue> for SyntaxNode {
    fn from(n: CssBogusComponentValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssBogusComponentValue> for SyntaxElement {
    fn from(n: CssBogusComponentValue) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssBogusDeclarationItem {
    syntax: SyntaxNode,
}
impl CssBogusDeclarationItem {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for CssBogusDeclarationItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_DECLARATION_ITEM as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_DECLARATION_ITEM
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusDeclarationItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusDeclarationItem")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusDeclarationItem> for SyntaxNode {
    fn from(n: CssBogusDeclarationItem) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssBogusDeclarationItem> for SyntaxElement {
    fn from(n: CssBogusDeclarationItem) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssBogusKeyframesItem {
    syntax: SyntaxNode,
}
impl CssBogusKeyframesItem {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for CssBogusKeyframesItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_KEYFRAMES_ITEM as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_KEYFRAMES_ITEM
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusKeyframesItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusKeyframesItem")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusKeyframesItem> for SyntaxNode {
    fn from(n: CssBogusKeyframesItem) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssBogusKeyframesItem> for SyntaxElement {
    fn from(n: CssBogusKeyframesItem) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssBogusMediaQuery {
    syntax: SyntaxNode,
}
impl CssBogusMediaQuery {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for CssBogusMediaQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_MEDIA_QUERY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_MEDIA_QUERY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusMediaQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusMediaQuery")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusMediaQuery> for SyntaxNode {
    fn from(n: CssBogusMediaQuery) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssBogusMediaQuery> for SyntaxElement {
    fn from(n: CssBogusMediaQuery) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssBogusParameter {
    syntax: SyntaxNode,
}
impl CssBogusParameter {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for CssBogusParameter {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_PARAMETER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_PARAMETER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusParameter")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusParameter> for SyntaxNode {
    fn from(n: CssBogusParameter) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssBogusParameter> for SyntaxElement {
    fn from(n: CssBogusParameter) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssBogusPseudoClass {
    syntax: SyntaxNode,
}
impl CssBogusPseudoClass {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for CssBogusPseudoClass {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_PSEUDO_CLASS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_PSEUDO_CLASS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusPseudoClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusPseudoClass")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusPseudoClass> for SyntaxNode {
    fn from(n: CssBogusPseudoClass) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssBogusPseudoClass> for SyntaxElement {
    fn from(n: CssBogusPseudoClass) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssBogusPseudoElement {
    syntax: SyntaxNode,
}
impl CssBogusPseudoElement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for CssBogusPseudoElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_PSEUDO_ELEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_PSEUDO_ELEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusPseudoElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusPseudoElement")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusPseudoElement> for SyntaxNode {
    fn from(n: CssBogusPseudoElement) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssBogusPseudoElement> for SyntaxElement {
    fn from(n: CssBogusPseudoElement) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssBogusRule {
    syntax: SyntaxNode,
}
impl CssBogusRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for CssBogusRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_RULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusRule")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusRule> for SyntaxNode {
    fn from(n: CssBogusRule) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssBogusRule> for SyntaxElement {
    fn from(n: CssBogusRule) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssBogusSelector {
    syntax: SyntaxNode,
}
impl CssBogusSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for CssBogusSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_SELECTOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusSelector")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusSelector> for SyntaxNode {
    fn from(n: CssBogusSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssBogusSelector> for SyntaxElement {
    fn from(n: CssBogusSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssBogusSubSelector {
    syntax: SyntaxNode,
}
impl CssBogusSubSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for CssBogusSubSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS_SUB_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_BOGUS_SUB_SELECTOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CssBogusSubSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogusSubSelector")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogusSubSelector> for SyntaxNode {
    fn from(n: CssBogusSubSelector) -> SyntaxNode {
        n.syntax
    }
}
impl From<CssBogusSubSelector> for SyntaxElement {
    fn from(n: CssBogusSubSelector) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssComponentValueList {
    syntax_list: SyntaxList,
}
impl CssComponentValueList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for CssComponentValueList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_COMPONENT_VALUE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_COMPONENT_VALUE_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<CssComponentValueList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssComponentValueList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssComponentValueList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstNodeList for CssComponentValueList {
    type Language = Language;
    type Node = AnyCssValue;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssComponentValueList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssComponentValueList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssComponentValueList {
    type Item = AnyCssValue;
    type IntoIter = AstNodeListIterator<Language, AnyCssValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssComponentValueList {
    type Item = AnyCssValue;
    type IntoIter = AstNodeListIterator<Language, AnyCssValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssCompoundSelectorList {
    syntax_list: SyntaxList,
}
impl CssCompoundSelectorList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for CssCompoundSelectorList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_COMPOUND_SELECTOR_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_COMPOUND_SELECTOR_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<CssCompoundSelectorList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssCompoundSelectorList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssCompoundSelectorList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for CssCompoundSelectorList {
    type Language = Language;
    type Node = AnyCssCompoundSelector;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssCompoundSelectorList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssCompoundSelectorList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for CssCompoundSelectorList {
    type Item = SyntaxResult<AnyCssCompoundSelector>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssCompoundSelector>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &CssCompoundSelectorList {
    type Item = SyntaxResult<AnyCssCompoundSelector>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssCompoundSelector>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssDeclarationList {
    syntax_list: SyntaxList,
}
impl CssDeclarationList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for CssDeclarationList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_DECLARATION_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_DECLARATION_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<CssDeclarationList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssDeclarationList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssDeclarationList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for CssDeclarationList {
    type Language = Language;
    type Node = CssDeclaration;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssDeclarationList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssDeclarationList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for CssDeclarationList {
    type Item = SyntaxResult<CssDeclaration>;
    type IntoIter = AstSeparatedListNodesIterator<Language, CssDeclaration>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &CssDeclarationList {
    type Item = SyntaxResult<CssDeclaration>;
    type IntoIter = AstSeparatedListNodesIterator<Language, CssDeclaration>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssKeyframesItemList {
    syntax_list: SyntaxList,
}
impl CssKeyframesItemList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for CssKeyframesItemList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_KEYFRAMES_ITEM_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_KEYFRAMES_ITEM_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<CssKeyframesItemList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssKeyframesItemList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssKeyframesItemList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstNodeList for CssKeyframesItemList {
    type Language = Language;
    type Node = AnyCssKeyframesItem;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssKeyframesItemList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssKeyframesItemList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssKeyframesItemList {
    type Item = AnyCssKeyframesItem;
    type IntoIter = AstNodeListIterator<Language, AnyCssKeyframesItem>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssKeyframesItemList {
    type Item = AnyCssKeyframesItem;
    type IntoIter = AstNodeListIterator<Language, AnyCssKeyframesItem>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssKeyframesSelectorList {
    syntax_list: SyntaxList,
}
impl CssKeyframesSelectorList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for CssKeyframesSelectorList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_KEYFRAMES_SELECTOR_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_KEYFRAMES_SELECTOR_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<CssKeyframesSelectorList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssKeyframesSelectorList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssKeyframesSelectorList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for CssKeyframesSelectorList {
    type Language = Language;
    type Node = AnyCssKeyframesSelector;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssKeyframesSelectorList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssKeyframesSelectorList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for CssKeyframesSelectorList {
    type Item = SyntaxResult<AnyCssKeyframesSelector>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssKeyframesSelector>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &CssKeyframesSelectorList {
    type Item = SyntaxResult<AnyCssKeyframesSelector>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssKeyframesSelector>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssMediaQueryList {
    syntax_list: SyntaxList,
}
impl CssMediaQueryList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for CssMediaQueryList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_MEDIA_QUERY_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_MEDIA_QUERY_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<CssMediaQueryList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssMediaQueryList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssMediaQueryList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for CssMediaQueryList {
    type Language = Language;
    type Node = AnyCssMediaQuery;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssMediaQueryList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssMediaQueryList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for CssMediaQueryList {
    type Item = SyntaxResult<AnyCssMediaQuery>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssMediaQuery>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &CssMediaQueryList {
    type Item = SyntaxResult<AnyCssMediaQuery>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssMediaQuery>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssParameterList {
    syntax_list: SyntaxList,
}
impl CssParameterList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for CssParameterList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PARAMETER_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PARAMETER_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<CssParameterList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssParameterList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssParameterList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for CssParameterList {
    type Language = Language;
    type Node = CssParameter;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssParameterList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssParameterList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for CssParameterList {
    type Item = SyntaxResult<CssParameter>;
    type IntoIter = AstSeparatedListNodesIterator<Language, CssParameter>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &CssParameterList {
    type Item = SyntaxResult<CssParameter>;
    type IntoIter = AstSeparatedListNodesIterator<Language, CssParameter>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssPseudoValueList {
    syntax_list: SyntaxList,
}
impl CssPseudoValueList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for CssPseudoValueList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_VALUE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_PSEUDO_VALUE_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<CssPseudoValueList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssPseudoValueList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoValueList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for CssPseudoValueList {
    type Language = Language;
    type Node = AnyCssPseudoValue;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssPseudoValueList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssPseudoValueList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for CssPseudoValueList {
    type Item = SyntaxResult<AnyCssPseudoValue>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssPseudoValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &CssPseudoValueList {
    type Item = SyntaxResult<AnyCssPseudoValue>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssPseudoValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssRelativeSelectorList {
    syntax_list: SyntaxList,
}
impl CssRelativeSelectorList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for CssRelativeSelectorList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_RELATIVE_SELECTOR_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_RELATIVE_SELECTOR_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<CssRelativeSelectorList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssRelativeSelectorList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssRelativeSelectorList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for CssRelativeSelectorList {
    type Language = Language;
    type Node = AnyCssRelativeSelector;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssRelativeSelectorList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssRelativeSelectorList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for CssRelativeSelectorList {
    type Item = SyntaxResult<AnyCssRelativeSelector>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssRelativeSelector>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &CssRelativeSelectorList {
    type Item = SyntaxResult<AnyCssRelativeSelector>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssRelativeSelector>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssRuleList {
    syntax_list: SyntaxList,
}
impl CssRuleList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for CssRuleList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_RULE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_RULE_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<CssRuleList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssRuleList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssRuleList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstNodeList for CssRuleList {
    type Language = Language;
    type Node = AnyCssRule;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssRuleList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssRuleList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssRuleList {
    type Item = AnyCssRule;
    type IntoIter = AstNodeListIterator<Language, AnyCssRule>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssRuleList {
    type Item = AnyCssRule;
    type IntoIter = AstNodeListIterator<Language, AnyCssRule>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssSelectorList {
    syntax_list: SyntaxList,
}
impl CssSelectorList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for CssSelectorList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SELECTOR_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_SELECTOR_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<CssSelectorList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssSelectorList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssSelectorList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for CssSelectorList {
    type Language = Language;
    type Node = AnyCssSelector;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssSelectorList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssSelectorList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for CssSelectorList {
    type Item = SyntaxResult<AnyCssSelector>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssSelector>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &CssSelectorList {
    type Item = SyntaxResult<AnyCssSelector>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyCssSelector>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssSubSelectorList {
    syntax_list: SyntaxList,
}
impl CssSubSelectorList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for CssSubSelectorList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SUB_SELECTOR_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CSS_SUB_SELECTOR_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<CssSubSelectorList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssSubSelectorList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssSubSelectorList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstNodeList for CssSubSelectorList {
    type Language = Language;
    type Node = AnyCssSubSelector;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for CssSubSelectorList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssSubSelectorList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssSubSelectorList {
    type Item = AnyCssSubSelector;
    type IntoIter = AstNodeListIterator<Language, AnyCssSubSelector>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for CssSubSelectorList {
    type Item = AnyCssSubSelector;
    type IntoIter = AstNodeListIterator<Language, AnyCssSubSelector>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone)]
pub struct DebugSyntaxElementChildren(pub SyntaxElementChildren);
impl Debug for DebugSyntaxElementChildren {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.clone().0.map(DebugSyntaxElement))
            .finish()
    }
}
struct DebugSyntaxElement(SyntaxElement);
impl Debug for DebugSyntaxElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            SyntaxElement::Node(node) => {
                map_syntax_node ! (node . clone () , node => std :: fmt :: Debug :: fmt (& node , f))
            }
            SyntaxElement::Token(token) => Debug::fmt(token, f),
        }
    }
}
