use rustc_ast::{LitIntType, LitKind};
use rustc_hir::attrs::UnrollAttr;
use rustc_lint_defs::builtin::ILL_FORMED_ATTRIBUTE_INPUT;

use super::prelude::*;

pub(crate) struct UnrollParser;
impl SingleAttributeParser for UnrollParser {
    const PATH: &[Symbol] = &[sym::unroll];
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[
        Allow(Target::Loop),
        Allow(Target::ForLoop),
        Allow(Target::While),
    ]);
    const TEMPLATE: AttributeTemplate = template!(
        Word,
        List: &["full", "never", "<integer>"]
    );

    fn convert(cx: &mut AcceptContext<'_, '_>, args: &ArgParser) -> Option<AttributeKind> {
        match args {
            ArgParser::NoArgs => Some(AttributeKind::Unroll(UnrollAttr::Hint)),
            ArgParser::List(list) => {
                let l = cx.expect_single(list)?;

                if let Some(lit) = l.as_lit()
                    && let LitKind::Int(val, LitIntType::Unsuffixed) = lit.kind
                {
                    if let Ok(val) = u32::try_from(val.get()) {
                        return Some(AttributeKind::Unroll(UnrollAttr::Count(val)));
                    } else {
                        cx.adcx().expected_integer_literal_in_range(l.span(), 0, u32::MAX as isize);
                        return None;
                    }
                }

                match l.meta_item().and_then(|i| i.path().word_sym()) {
                    Some(sym::full) => Some(AttributeKind::Unroll(UnrollAttr::Full)),
                    Some(sym::never) => Some(AttributeKind::Unroll(UnrollAttr::Never)),
                    _ => {
                        cx.adcx().expected_specific_argument(l.span(), &[sym::full, sym::never]);
                        None
                    }
                }
            }
            ArgParser::NameValue(_) => {
                cx.adcx().warn_ill_formed_attribute_input(ILL_FORMED_ATTRIBUTE_INPUT);
                return None;
            }
        }
    }
}
