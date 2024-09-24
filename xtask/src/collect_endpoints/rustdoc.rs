use color_eyre::eyre::{OptionExt, Result};
use rustdoc_types::{Crate, Id, ItemEnum};
use std::collections::{HashMap, HashSet};

pub struct ParsedRustdoc<'a> {
    /// Helper methods of HelixClient
    pub helix_methods: HashSet<&'a str>,
    /// Endpoints (helix::endpoints) mapped to their members
    pub helix_mods: HashMap<&'a str, HashSet<&'a str>>,
    /// Event categories mapped to their types
    pub eventsub_types: HashMap<&'a str, HashSet<&'a str>>,
}

pub fn parse(rustdoc: &Crate) -> Result<ParsedRustdoc<'_>> {
    let get_item = |id: &Id| get_doc_item(rustdoc, id);

    let root = get_item(&rustdoc.root);
    let helix = root
        .links
        .get("helix")
        .map(get_item)
        .and_then(as_module)
        .ok_or_eyre("failed to find 'helix' link")?;
    let eventsub = root
        .links
        .get("eventsub")
        .map(get_item)
        .and_then(as_module)
        .ok_or_eyre("failed to find 'eventsub' link")?;

    Ok(ParsedRustdoc {
        helix_methods: parse_helix_methods(rustdoc, helix)?,
        helix_mods: parse_helix_endpoints(rustdoc, helix)?,
        eventsub_types: parse_eventsub_types(rustdoc, eventsub),
    })
}

fn parse_helix_methods<'a>(
    rustdoc: &'a Crate,
    helix: &'a rustdoc_types::Module,
) -> Result<HashSet<&'a str>> {
    let get_item = |id: &Id| get_doc_item(rustdoc, id);

    let client = find_item(rustdoc, &helix.items, "client")
        .and_then(as_module)
        .ok_or_eyre("failed to find helix::client")?;
    let helix_client = find_item(rustdoc, &client.items, "HelixClient")
        .and_then(as_struct)
        .ok_or_eyre("failed to find helix::client::HelixClient")?;

    let mut methods = HashSet::new();
    for imp in helix_client
        .impls
        .iter()
        .map(get_item)
        .filter_map(as_impl)
        .filter(|i| i.trait_.is_none())
    {
        methods.extend(
            imp.items
                .iter()
                .map(get_item)
                .filter_map(|it| it.name.as_deref()),
        );
    }

    Ok(methods)
}

fn parse_helix_endpoints<'a>(
    rustdoc: &'a Crate,
    helix: &'a rustdoc_types::Module,
) -> Result<HashMap<&'a str, HashSet<&'a str>>> {
    let get_item = |id: &Id| get_doc_item(rustdoc, id);

    let endpoints = find_resolve_use(rustdoc, &helix.items, "endpoints")
        .and_then(as_module)
        .ok_or_eyre("failed to resolve helix::endpoints")?;

    Ok(endpoints
        .items
        .iter()
        .map(get_item)
        .filter_map(|it| both(it.name.as_deref(), as_module(it)))
        .map(|(name, m)| {
            (
                name,
                m.items
                    .iter()
                    .map(get_item)
                    .filter(|it| matches!(it.inner, ItemEnum::Module(_)))
                    .filter_map(|it| it.name.as_deref())
                    .collect::<HashSet<_>>(),
            )
        })
        .collect())
}

fn both<T, U>(t: Option<T>, u: Option<U>) -> Option<(T, U)> {
    match (t, u) {
        (Some(t), Some(u)) => Some((t, u)),
        _ => None,
    }
}

fn parse_eventsub_types<'a>(
    rustdoc: &'a Crate,
    eventsub: &'a rustdoc_types::Module,
) -> HashMap<&'a str, HashSet<&'a str>> {
    let get_item = |id: &Id| get_doc_item(rustdoc, id);

    eventsub
        .items
        .iter()
        .map(get_item)
        .filter_map(|it| both(it.name.as_deref(), as_module(it)))
        .map(|(name, m)| {
            (
                name,
                m.items
                    .iter()
                    .map(get_item)
                    .filter_map(as_use)
                    .map(|u| u.name.as_str())
                    .collect::<HashSet<_>>(),
            )
        })
        .collect()
}

fn get_doc_item<'a>(rustdoc: &'a Crate, id: &Id) -> &'a rustdoc_types::Item {
    rustdoc.index.get(id).expect("failed to resolve ID")
}

fn find_item<'a>(rustdoc: &'a Crate, ids: &[Id], name: &str) -> Option<&'a rustdoc_types::Item> {
    ids.iter()
        .map(|id| get_doc_item(rustdoc, id))
        .find(|it| it.name.as_deref() == Some(name))
}

fn find_resolve_use<'a>(
    rustdoc: &'a Crate,
    ids: &[Id],
    name: &str,
) -> Option<&'a rustdoc_types::Item> {
    ids.iter()
        .filter_map(|id| as_use(get_doc_item(rustdoc, id)))
        .find(|it| it.name == name)
        .and_then(|it| it.id.as_ref())
        .map(|id| get_doc_item(rustdoc, id))
}

fn as_module(item: &rustdoc_types::Item) -> Option<&rustdoc_types::Module> {
    if let ItemEnum::Module(ref m) = item.inner {
        Some(m)
    } else {
        None
    }
}

fn as_impl(item: &rustdoc_types::Item) -> Option<&rustdoc_types::Impl> {
    if let ItemEnum::Impl(ref i) = item.inner {
        Some(i)
    } else {
        None
    }
}

fn as_struct(item: &rustdoc_types::Item) -> Option<&rustdoc_types::Struct> {
    if let ItemEnum::Struct(ref s) = item.inner {
        Some(s)
    } else {
        None
    }
}

fn as_use(item: &rustdoc_types::Item) -> Option<&rustdoc_types::Use> {
    if let ItemEnum::Use(ref u) = item.inner {
        Some(u)
    } else {
        None
    }
}
