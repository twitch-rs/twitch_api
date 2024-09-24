use std::{borrow::Cow, collections::BTreeMap, fmt::Write};

use super::html;
use super::rustdoc::ParsedRustdoc;
use color_eyre::eyre::{bail, Result};
use url::Url;

pub fn make_overview(base_url: &Url, raw: &str, rustdoc: &ParsedRustdoc) -> Result<String> {
    let parser = tl::parse(raw, tl::ParserOptions::new())?;
    let table = html::tbody_below_id(&parser, "subscription-types")?;
    let children = table.children();

    // the categories should be sorted (thus a BTreeMap)
    let mut categories = BTreeMap::<String, Vec<EventSubRow>>::new();
    for row in children
        .top()
        .iter()
        .filter_map(|it| it.get(parser.parser()).and_then(|n| n.as_tag()))
    {
        let row = EventSubRow::parse(row, parser.parser(), base_url)?;
        let Some((category, _)) = row.name.split_once('.') else {
            bail!("Invalid category name: {}", row.name);
        };
        if let Some(cat) = categories.get_mut(category) {
            cat.push(row);
        } else {
            categories.insert(category.to_owned(), vec![row]);
        }
    }

    struct EventSubEntry<'a> {
        name: String,
        subscription_link: Option<String>,
        payload_link: Option<String>,
        ty: Cow<'a, str>,
    }

    let mut doc = String::new();
    for (category, subscriptions) in categories.into_iter() {
        let implemented = rustdoc.eventsub_types.get(category.as_str());
        let is_implemented = |sub: &str, payload: &str| {
            if let Some(types) = implemented {
                types.contains(sub) && types.contains(payload)
            } else {
                false
            }
        };

        let mut resolved = subscriptions
            .into_iter()
            .map(|sub| {
                let sub_name = sub.struct_name();
                let payload_name = format!("{sub_name}Payload");
                let fmt_name = |n: &str| {
                    if n.len() < 40 {
                        format!("`{n}`")
                    } else {
                        format!("<span style=\"font-size: 0.9em\">`{n}`</span>")
                    }
                };
                let name = format!("[{}]({})", fmt_name(&sub.name), sub.link);
                if is_implemented(&sub_name, &payload_name) {
                    EventSubEntry {
                        name,
                        subscription_link: Some(format!("[{sub_name}]({category}::{sub_name})")),
                        payload_link: Some(format!("[{payload_name}]({category}::{payload_name})")),
                        ty: sub.name,
                    }
                } else {
                    println!(
                        "[EventSub]: missing {} ({sub_name}/{payload_name})",
                        sub.name
                    );
                    EventSubEntry {
                        name,
                        subscription_link: None,
                        payload_link: None,
                        ty: sub.name,
                    }
                }
            })
            .collect::<Vec<_>>();
        resolved.sort_by(|a, b| a.ty.cmp(&b.ty));

        let n_implemented = resolved
            .iter()
            .filter(|it| it.payload_link.is_some() && it.subscription_link.is_some())
            .count();
        let n_items = resolved.len();
        let indicator = super::indicator_for(n_implemented, n_items);

        write!(
            &mut doc,
            r#"//! <details><summary style="cursor: pointer"><code style="color: var(--link-color)">{category}.*</code> {indicator} {n_implemented}/{n_items}</summary>
//!
//! | Name | Subscription<br>Payload |
//! |---|:---|
"#
        )?;
        for EventSubEntry {
            name,
            subscription_link,
            payload_link,
            ..
        } in resolved
        {
            writeln!(
                &mut doc,
                "//! | {name} | {}<br>{} |",
                subscription_link.as_deref().unwrap_or("-"),
                payload_link.as_deref().unwrap_or("-")
            )?;
        }
        doc.push_str("//!\n//! </details>\n//!\n");
    }

    Ok(doc)
}

fn append_pascal(pascal: &mut String, s: &str) {
    for part in s.split('_') {
        pascal.push(part.chars().next().unwrap_or('?').to_ascii_uppercase());
        pascal.push_str(&part[1..]);
    }
}

struct EventSubRow<'a> {
    /// e.g. "#automodmessagehold"
    link: Url,
    /// e.g. "automod.message.hold"
    name: Cow<'a, str>,
    /// e.g. "1"
    version: Cow<'a, str>,
}

impl<'a> EventSubRow<'a> {
    pub fn parse(row: &'a tl::HTMLTag, parser: &'a tl::Parser, base_url: &Url) -> Result<Self> {
        let children = row.children();
        let mut tags = html::tags(&children, parser);

        let (Some(sub_type_el), Some(name_el), Some(version_el)) = (
            tags.next().and_then(|n| html::find_by_tag(n, parser, "a")),
            tags.next()
                .and_then(|n| html::find_by_tag(n, parser, "code")),
            tags.next()
                .and_then(|n| html::find_by_tag(n, parser, "code")),
        ) else {
            bail!("Failed to get <a>/<code> of <tr>")
        };

        let Some(sub_link) = sub_type_el
            .attributes()
            .get("href")
            .flatten()
            .map(|attr| attr.as_utf8_str())
        else {
            bail!("Failed to get href from <a>")
        };

        Ok(Self {
            link: base_url.join(&sub_link)?,
            name: name_el.inner_text(parser),
            version: version_el.inner_text(parser),
        })
    }

    pub fn struct_name(&self) -> String {
        let mut parts = self.name.split('.');
        let mut buf = String::new();

        let first = parts.next().expect("EventSub name");
        let second = parts.next().expect("EventSub topic without '.'");

        if !second.starts_with(first) {
            append_pascal(&mut buf, first);
        }
        append_pascal(&mut buf, second);
        for part in parts {
            append_pascal(&mut buf, part);
        }

        if self.version.chars().all(|it| it.is_numeric()) {
            buf.push('V');
            buf.push_str(&self.version);
        } else {
            append_pascal(&mut buf, &self.version); // most likely 'beta'
        }

        buf
    }
}
