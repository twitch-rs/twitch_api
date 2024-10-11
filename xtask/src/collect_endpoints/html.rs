use color_eyre::eyre::{eyre, OptionExt, Result};

/// Finds a `tag` that's a direct child of this `node`
pub fn find_by_tag<'p, 'buf>(
    node: &'p tl::HTMLTag<'buf>,
    parser: &'p tl::Parser<'buf>,
    tag: &str,
) -> Option<&'p tl::HTMLTag<'buf>> {
    let children = node.children();
    let x = tags(&children, parser).find(|x| x.name() == tag);
    x
}

/// An iterator of `HTMLTag`s that are direct children.
///
/// This ignores whitespace.
pub fn tags<'a, 'p, 'buf>(
    children: &'a tl::Children<'buf, 'p>,
    parser: &'p tl::Parser<'buf>,
) -> impl Iterator<Item = &'p tl::HTMLTag<'buf>> + 'a {
    children
        .top()
        .iter()
        .filter_map(|it| it.get(parser).and_then(|n| n.as_tag()))
}

/// Finds a <tbody> after some `id`
///
/// **Example**:
/// ```html
/// <section>
///     <h1 id="my-target-id">Foo</h1>
///     <table>
///         <thead>...</thead>
///         <tbody> <!-- <- this will be returned -->
///         </tbody>
///     </table>
/// </section>
/// ```
pub fn tbody_below_id<'p, 'v>(dom: &'p tl::VDom<'v>, id: &str) -> Result<&'p tl::HTMLTag<'v>> {
    for section in dom
        .query_selector("section")
        .ok_or_eyre("failed to parse selector")?
        .filter_map(|it| it.get(dom.parser()).and_then(|n| n.children()))
    {
        let mut tags = tags(&section, dom.parser());
        let (Some(first), Some(second)) = (tags.next(), tags.next()) else {
            continue;
        };

        if first.name() != "h1" || !first.attributes().id().is_some_and(|a| a == id) {
            continue;
        }
        if second.name() != "table" {
            continue;
        }
        if let Some(tbody) = second
            .children()
            .top()
            .iter()
            .filter_map(|it| it.get(dom.parser()).and_then(|n| n.as_tag()))
            .find(|n| n.name() == "tbody")
        {
            return Ok(tbody);
        }
    }
    Err(eyre!(
        "Failed to find [table > tbody] below [section > section > h1#{id}]"
    ))
}
