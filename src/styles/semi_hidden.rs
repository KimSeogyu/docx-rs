use hard_xml::{XmlRead, XmlWrite};

/// WidowControl
///
/// ```rust
/// use rs_docx::formatting::*;
///
/// let WidowControl = WidowControl::from(false);
/// let WidowControl = WidowControl::from(true);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:semiHidden")]
pub struct SemiHidden {
    #[xml(attr = "w:val")]
    pub value: Option<bool>,
}
