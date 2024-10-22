use std::{borrow::Cow, collections::BTreeMap, fmt::Write, ops::Deref};

use super::html;
use super::rustdoc::ParsedHelixRustdoc;
use color_eyre::eyre::{bail, Result};
use url::Url;

pub fn make_overview(
    base_url: &Url,
    raw: &str,
    rustdoc: &mut ParsedHelixRustdoc,
) -> Result<String> {
    let parser = tl::parse(raw, tl::ParserOptions::new())?;
    let table = html::tbody_below_id(&parser, "twitch-api-reference")?;
    let children = table.children();

    // the categories should be sorted (thus a BTreeMap)
    let mut categories = BTreeMap::<String, Vec<HelixRow>>::new();
    for row in children
        .top()
        .iter()
        .filter_map(|it| it.get(parser.parser()).and_then(|n| n.as_tag()))
    {
        let (category, row) = HelixRow::parse(row, parser.parser(), base_url)?;
        if let Some(cat) = categories.get_mut(category.deref()) {
            cat.push(row);
        } else {
            categories.insert(category.into_owned(), vec![row]);
        }
    }

    struct HelixEntry {
        endpoint_link: String,
        helper_links: Vec<String>,
        module_link: Option<String>,
        item_name: String,
    }

    let mut doc = String::new();
    for (cat_name, endpoints) in categories.into_iter() {
        let module_name = category_override(to_snake(&cat_name));

        let mut resolved = endpoints
            .into_iter()
            .map(|endpoint| {
                let item_name = item_override(to_snake(&endpoint.name));
                let actual_module = module_override(&module_name, &item_name);
                let module_link = if rustdoc
                    .mods
                    .get(&actual_module)
                    .is_some_and(|m| m.contains(item_name.as_str()))
                {
                    Some(format!("[`{actual_module}::{item_name}`]"))
                } else {
                    println!("[Helix]: missing {actual_module}::{item_name}");
                    None
                };
                let helper_links = if rustdoc.methods.remove(&*item_name) {
                    vec![format!("[`HelixClient::{item_name}`]")]
                } else {
                    vec![]
                };

                HelixEntry {
                    endpoint_link: format!("[{}]({})", endpoint.name, endpoint.link),
                    helper_links,
                    module_link,
                    item_name,
                }
            })
            .collect::<Vec<_>>();
        // helper, (distance, endpoint link)
        let mut distances: BTreeMap<String, (usize, String)> = BTreeMap::new();

        // second iteration to find distance-matches
        for entry in &mut resolved {
            for m in &rustdoc.methods {
                let current_distance = distance(&entry.item_name, m);

                // Only consider distances less than 3
                if current_distance < 3 {
                    distances
                        .entry((*m).to_owned())
                        .and_modify(|e| {
                            let new_distance = std::cmp::min(e.0, current_distance);
                            if new_distance < e.0 {
                                let _ = std::mem::replace(
                                    e,
                                    (new_distance, entry.endpoint_link.clone()),
                                );
                            }
                        })
                        .or_insert((current_distance, entry.endpoint_link.clone()));
                }
            }
        }
        for entry in &mut resolved {
            for (item, (_, link)) in distances.iter() {
                if *link == entry.endpoint_link {
                    entry.helper_links.push(format!("[`HelixClient::{item}`]"));
                }
            }
        }

        let n_implemented = resolved
            .iter()
            .filter(|it| it.module_link.is_some())
            .count();
        let n_items = resolved.len();
        let indicator = super::indicator_for(n_implemented, n_items);

        write!(
            &mut doc,
            r#"//! <details><summary style="cursor: pointer">{cat_name} {indicator} {n_implemented}/{n_items}</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
"#
        )?;
        let mut helper: String;
        for HelixEntry {
            endpoint_link,
            helper_links,
            module_link,
            ..
        } in resolved
        {
            helper = if helper_links.is_empty() {
                "-".to_owned()
            } else {
                helper_links.join(", ")
            };
            writeln!(
                &mut doc,
                "//! | {endpoint_link} | {} | {} |",
                helper,
                module_link.as_deref().unwrap_or("-")
            )?;
        }
        doc.push_str("//!\n//! </details>\n//!\n");
    }

    Ok(doc)
}

fn to_snake(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c == ' ' {
                '_'
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect()
}

fn category_override(c: String) -> String {
    match c.as_str() {
        "ads" => "channels".to_owned(),
        "channel_points" => "points".to_owned(),
        "conduits" => "eventsub".to_owned(),
        "hype_train" => "hypetrain".to_owned(),
        _ => c,
    }
}

fn item_override(i: String) -> String {
    match i.as_str() {
        "create_conduits" => "create_conduit".to_owned(),
        "resolve_unban_requests" => "resolve_unban_request".to_owned(),
        "get_hype_train_events" => "get_hypetrain_events".to_owned(),
        _ => i,
    }
}

fn module_override<'a>(module: &'a str, item: &'a str) -> &'a str {
    match item {
        "add_channel_vip" | "get_vips" | "remove_channel_vip" => "channels",
        "get_stream_tags" => "streams",
        _ => module,
    }
}

struct HelixRow<'a> {
    /// e.g. "Start Commercial"
    name: Cow<'a, str>,
    /// e.g. "#start-commercial"
    link: Url,
}

impl<'a> HelixRow<'a> {
    pub fn parse(
        row: &'a tl::HTMLTag,
        parser: &'a tl::Parser,
        base_url: &Url,
    ) -> Result<(Cow<'a, str>, Self)> {
        let children = row.children();
        let mut tags = html::tags(&children, parser);

        let (Some(category_el), Some(name_el)) = (
            tags.next(),
            tags.next().and_then(|n| html::find_by_tag(n, parser, "a")),
        ) else {
            bail!("Failed to get <a>/<code> of <tr>")
        };

        let Some(link) = name_el
            .attributes()
            .get("href")
            .flatten()
            .map(|attr| attr.as_utf8_str())
        else {
            bail!("Failed to get href from <a>")
        };

        Ok((
            category_el.inner_text(parser),
            Self {
                name: name_el.inner_text(parser),
                link: base_url.join(&link)?,
            },
        ))
    }
}

pub fn distance(src: &str, tar: &str) -> usize {
    let ignore_phrases = vec![
        "in_channel",
        "by_id",
        "from_ids",
        "from_id",
        "from_logins",
        "from_login",
    ];

    fn remove_ignored_phrases(s: &str, ignore_phrases: &[&str]) -> String {
        let mut result = s.to_string();
        for phrase in ignore_phrases {
            result = result.replace(phrase, "_");
        }
        result
    }

    fn depluralize(token: &str) -> &str {
        if token.ends_with('s') && token.len() > 1 {
            &token[..token.len() - 1]
        } else {
            token
        }
    }

    let src_cleaned = remove_ignored_phrases(src, &ignore_phrases);
    let tar_cleaned = remove_ignored_phrases(tar, &ignore_phrases);

    let src_tokens: Vec<&str> = src_cleaned
        .split('_')
        .filter(|token| !token.is_empty())
        .map(depluralize)
        .collect();

    let tar_tokens: Vec<&str> = tar_cleaned
        .split('_')
        .filter(|token| !token.is_empty())
        .map(depluralize)
        .collect();

    let min_len = usize::min(src_tokens.len(), tar_tokens.len());
    let mut distance = 0;

    // Compare tokens like `editor` and `emote`
    for i in 0..min_len {
        // XXX: could make early tokens penalize heavier
        let weight = 2;

        if src_tokens[i] == tar_tokens[i] {
            continue;
        } else {
            let token_distance = super::levenshtein(src_tokens[i], tar_tokens[i]);
            distance += weight * token_distance;
        }
    }

    // penalize extra tokens
    // for example, `get_channel_editors`, `get_channels_from_ids` == 24
    if src_tokens.len() > min_len {
        for token in &src_tokens[min_len..] {
            let weight = 2; // higher penalty for extra tokens
            let token_len = token.chars().count();
            distance += weight * token_len;
        }
    } else if tar_tokens.len() > min_len {
        for token in &tar_tokens[min_len..] {
            let weight = 2;
            let token_len = token.chars().count();
            distance += weight * token_len;
        }
    }

    distance
}

#[cfg(test)]
#[test]
fn distance_test() {
    use std::cmp::Ordering::*;
    let examples = vec![
        ("get_channel_emotes_from_id", "get_channel_emotes", Less),
        ("get_users_chat_colors", "get_user_chat_color", Less),
        ("get_games_by_id", "get_games", Less),
        (
            "get_banned_users_in_channel_from_id",
            "get_banned_users",
            Less,
        ),
        ("get_channel_schedule", "get_ad_schedule", Greater),
        ("get_game_analytics", "get_games_by_id", Greater),
        ("get_channel_editors", "get_channel_emotes_from_id", Greater),
        ("get_channel_editors", "get_channel_schedule", Greater),
        ("get_channel_editors", "get_channels_from_ids", Greater),
        ("get_clips", "get_vips_in_channel", Greater),
        ("get_teams", "get_streams_from_ids", Greater),
        ("get_team_a", "get_teams", Less),
    ];

    for (src, tar, ord) in examples {
        let dist = distance(src, tar);
        println!("Distance between '{}' and '{}' is: {}", src, tar, dist);
        assert!(dist.cmp(&3) == ord)
    }
}
