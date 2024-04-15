//! Generate overview for implemented helix endpoint with links
//!
//! Requires Rust nightly and Deno
//!
//! 1. Generate documentation
//!   $ RUSTDOCFLAGS='-Zunstable-options --output-format json' cargo doc --no-deps -F _all
//! 2. Run collector
//!   $ deno run --allow-net --allow-read tools/collect-helix-endpoints.ts

import {
  DOMParser,
  Element,
  Node,
} from "https://deno.land/x/deno_dom@v0.1.45/deno-dom-wasm.ts";

interface Rustdoc {
  helixMethods: Set<string>;
  helixMods: Map<string, Set<string>>;
  eventsubTypes: Map<string, Set<string>>;
}

async function readRustdoc(): Promise<Rustdoc> {
  interface RootItem {
    root: string;
    index: Record<string, DocItem>;
  }

  interface DocItem {
    name: string | null;
    links: Record<string, string>;
    inner: InnerDoc;
  }

  type InnerDoc =
    | { module: ModDoc }
    | { import: ImportDoc }
    | { struct: StructDoc }
    | { impl: ImplDoc };

  interface ModDoc {
    items: string[];
  }

  interface ImportDoc {
    name: string;
    id: string;
  }

  interface StructDoc {
    impls: string[];
  }

  interface ImplDoc {
    items: string[];
    trait: TraitRef | null;
  }

  interface TraitRef {
    name: string;
    id: string;
  }

  function isMod(inner: InnerDoc): inner is { module: ModDoc } {
    return "module" in inner;
  }

  function isImport(inner: InnerDoc): inner is { import: ImportDoc } {
    return "import" in inner;
  }

  function isStruct(inner: InnerDoc): inner is { struct: StructDoc } {
    return "struct" in inner;
  }

  function isImpl(inner: InnerDoc): inner is { impl: ImplDoc } {
    return "impl" in inner;
  }

  const decoder = new TextDecoder("utf-8");
  const data = await Deno.readFile(
    `${import.meta.dirname ?? "."}/../target/doc/twitch_api.json`
  ).catch((e) => {
    console.warn("Failed to read twitch_api.json");
    console.warn("Forgot to generate documentation?");
    console.warn(
      "Generate with: RUSTDOCFLAGS='-Zunstable-options --output-format json' cargo doc --no-deps -F _all"
    );
    throw e;
  });
  const doc: RootItem = JSON.parse(decoder.decode(data));

  const getItem = (id: string) => doc.index[id];
  const resolveImport = (ids: string[], name: string) => {
    for (const id of ids) {
      const item = getItem(id);
      if (isImport(item.inner) && item.inner.import.name === name) {
        return getItem(item.inner.import.id);
      }
    }
    return null;
  };
  const allItems = (ids: string[]) => {
    return (function* () {
      for (const id of ids) {
        const item = getItem(id);
        yield item;
      }
    })();
  };
  const findItem = (ids: string[], name: string) => {
    for (const id of ids) {
      const item = getItem(id);
      if (item.name === name) {
        return item;
      }
    }
    return null;
  };
  const iterNames = (
    ids: string[],
    filter: (inner: InnerDoc) => boolean = () => true
  ) => {
    return (function* () {
      for (const id of ids) {
        const item = getItem(id);
        if (!item?.name || !filter(item.inner)) continue;
        yield item.name;
      }
    })();
  };

  const root = getItem(doc.root);

  const helix = getItem(root.links.helix);

  const findHelixMethods = () => {
    if (!isMod(helix.inner)) throw new Error("helix isn't a module");

    const client = resolveImport(helix.inner.module.items, "client");

    if (!client || !isMod(client.inner))
      throw new Error("Failed to find helix::client");

    const helixClient = findItem(client.inner.module.items, "HelixClient");

    if (!helixClient || !isStruct(helixClient.inner))
      throw new Error("Failed to find helix::client::HelixClient");

    const methods = new Set<string>();
    for (const id of helixClient.inner.struct.impls) {
      const impl = getItem(id);
      if (!isImpl(impl.inner)) {
        throw new Error(`Not an impl: ${impl.name}`);
      }
      if (impl.inner.impl.trait) continue;

      for (const name of iterNames(impl.inner.impl.items)) {
        methods.add(name);
      }
    }
    return methods;
  };

  const findHelixEndpoints = () => {
    if (!isMod(helix.inner)) throw new Error("helix isn't a module");

    const endpoints = resolveImport(helix.inner.module.items, "endpoints");

    if (!endpoints || !isMod(endpoints.inner))
      throw new Error("Failed to find helix::endpoints");

    const mods = new Map<string, Set<string>>();
    for (const modId of endpoints.inner.module.items) {
      const mod = getItem(modId);
      if (!mod || !isMod(mod.inner) || !mod.name) {
        throw new Error(`Not a mod: ${mod.name}`);
      }
      mods.set(mod.name, new Set(iterNames(mod.inner.module.items, isMod)));
    }

    return mods;
  };

  const findEventsubTypes = () => {
    const eventsub = getItem(root.links.eventsub);
    if (!isMod(eventsub.inner)) throw new Error("eventsub isn't a module");

    const types = new Map<string, Set<string>>();
    for (const modItem of allItems(eventsub.inner.module.items)) {
      if (!isMod(modItem.inner) || !modItem.name) continue;
      const items = modItem.inner.module.items;
      types.set(
        modItem.name,
        new Set(
          (function* () {
            for (const innerItem of allItems(items)) {
              if (isImport(innerItem.inner)) {
                yield innerItem.inner.import.name;
              }
            }
          })()
        )
      );
    }
    return types;
  };

  return {
    helixMethods: findHelixMethods(),
    helixMods: findHelixEndpoints(),
    eventsubTypes: findEventsubTypes(),
  };
}

function isElement(x: Node): x is Element {
  return x instanceof Element;
}

function indicatorFor(actual: number, max: number) {
  if (actual === 0) return "🔴";
  if (actual === max) return "🟢";

  return "🟡";
}

function makeHelixOverview(tbody: Element, rustdoc: Rustdoc) {
  const categories = new Map<string, Array<{ name: string; link: string }>>();
  for (const row of tbody.children) {
    const category = row.children[0].textContent;
    const endpoint = row.children[1].firstChild;
    if (!isElement(endpoint)) {
      throw new Error("endpoint not a node");
    }
    if (!categories.has(category)) {
      categories.set(category, []);
    }
    categories.get(category)!.push({
      name: endpoint.textContent,
      link: new URL(
        endpoint.attributes[0].value,
        "https://dev.twitch.tv/docs/api/reference"
      ).toString(),
    });
  }
  const toSnake = (s: string) => s.toLowerCase().replaceAll(" ", "_");

  const categoryOverrides = new Map(
    Object.entries({
      ads: "channels",
      channel_points: "points",
      conduits: "eventsub",
      hype_train: "hypetrain",
    })
  );
  const itemOverrides = new Map(
    Object.entries({
      create_conduits: "create_conduit",
    })
  );
  const moduleOverrides = new Map(
    Object.entries({
      add_channel_vip: "channels",
      get_vips: "channels",
      remove_channel_vips: "channels",
    })
  );

  const moduleForCategory = (cat: string) => {
    const moduleName = toSnake(cat);
    return categoryOverrides.get(moduleName) ?? moduleName;
  };

  const methodLink = (itemName: string) => {
    if (!rustdoc.helixMethods.has(itemName)) {
      return null;
    }
    return `[\`HelixClient::${itemName}\`]`;
  };
  const moduleLink = (categoryModule: string, itemName: string) => {
    const mod = rustdoc.helixMods.get(categoryModule);
    if (!mod || !mod.has(itemName)) {
      return null;
    }
    return `[\`${categoryModule}::${itemName}\`]`;
  };

  let doc = "";
  for (const [catName, endpoints] of categories) {
    const moduleName = moduleForCategory(catName);

    const resolved = endpoints.map(({ name, link }) => {
      let itemName = toSnake(name);
      itemName = itemOverrides.get(itemName) ?? itemName;
      const actualModule = moduleOverrides.get(itemName) ?? moduleName;
      const module = moduleLink(actualModule, itemName);
      if (!module) {
        console.warn(`[Helix]: missing ${actualModule}::${itemName}`);
      }
      return {
        endpoint: `[${name}](${link})`,
        helper: methodLink(itemName),
        module,
      };
    });
    const nImplemented = resolved.reduce(
      (acc, { helper, module }) => acc + (helper || module ? 1 : 0),
      0
    );
    const indicator = indicatorFor(nImplemented, endpoints.length);

    doc += `//! <details><summary style="cursor: pointer">${catName} ${indicator} ${nImplemented}/${endpoints.length}</summary>\n`;
    doc += "//!\n";
    doc += "//! | Endpoint | Helper | Module |\n";
    doc += "//! |---|---|---|\n";
    for (const { endpoint, helper, module } of resolved) {
      doc += `//! | ${endpoint} | ${helper ?? "-"} | ${module ?? "-"} |\n`;
    }
    doc += "//!\n";
    doc += "//! </details>\n";
    doc += "//!\n";
  }
  return doc;
}

function makeEventsubOverview(tbody: Element, rustdoc: Rustdoc) {
  interface EventSubType {
    subType: string;
    link: string;
    name: string;
    version: string;
  }

  const categories = new Map<string, EventSubType[]>();
  for (const row of tbody.children) {
    const subTypeEl = row.children[0].querySelector("a");
    const name = row.children[1].querySelector("code")?.textContent;
    const version = row.children[2].querySelector("code")?.textContent;
    if (!subTypeEl || !name || !version) {
      throw new Error("Invalid row");
    }
    const category = name.split(".")[0];
    if (!categories.has(category)) {
      categories.set(category, []);
    }
    categories.get(category)!.push({
      subType: subTypeEl.textContent,
      link: new URL(
        subTypeEl.attributes[0].value,
        "https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types"
      ).toString(),
      name,
      version,
    });
  }

  const toPascal = (s: string) =>
    s
      .split("_")
      .map((word) => word[0].toUpperCase() + word.slice(1))
      .join("");

  const subTypeToStruct = (ty: EventSubType) => {
    const parts = ty.name.split(".");
    const base = parts
      .slice(parts[1].startsWith(parts[0]) ? 1 : 0)
      .map(toPascal)
      .join("");
    if (/^\d+$/.test(ty.version)) {
      return base + `V${ty.version}`;
    }

    return base + toPascal(ty.version);
  };

  let doc = "";
  for (const [category, subscriptions] of categories) {
    const mod = rustdoc.eventsubTypes.get(category) ?? new Set();
    const resolved = subscriptions.map((sub) => {
      const subName = subTypeToStruct(sub);
      const payloadName = subName + "Payload";
      const fmtName = (name: string) => {
        if (name.length < 40) {
          return `\`${name}\``;
        }
        return `<span style="font-size: 0.9em">\`${name}\`</span>`;
      };
      const name = `[${fmtName(sub.name)}](${sub.link})`;

      if (!mod.has(subName) || !mod.has(payloadName)) {
        console.log(
          `[EventSub]: missing ${sub.name} (${subName}/${payloadName})`
        );
        return { name, ty: sub.name };
      }

      return {
        name,
        subscription: `[${subName}](${category}::${subName})`,
        payload: `[${payloadName}](${category}::${payloadName})`,
        ty: sub.name,
      };
    });
    resolved.sort((a, b) => a.ty.localeCompare(b.ty));

    const nImplemented = resolved.reduce(
      (acc, { subscription, payload }) =>
        acc + (subscription && payload ? 1 : 0),
      0
    );
    const indicator = indicatorFor(nImplemented, subscriptions.length);

    doc += `//! <details><summary style="cursor: pointer"><code style="color: var(--link-color)">${category}.*</code> ${indicator} ${nImplemented}/${subscriptions.length}</summary>\n`;
    doc += "//!\n";
    doc += "//! | Name | Subscription<br>Payload |\n";
    doc += "//! |---|:---|\n";
    for (const { name, subscription, payload } of resolved) {
      doc += `//! | ${name} | ${subscription ?? "-"}<br>${payload ?? "-"} |\n`;
    }
    doc += "//!\n";
    doc += "//! </details>\n";
    doc += "//!\n";
  }

  return doc;
}

async function pasteInFile(path: string, content: string) {
  const BEGIN_MARKER = "//! <!-- BEGIN-OVERVIEW -->";
  const END_MARKER = "//! <!-- END-OVERVIEW -->";

  path = `${import.meta.dirname ?? "."}/../${path}`;

  const decoder = new TextDecoder("utf-8");
  const fileBytes = await Deno.readFile(path);
  const fileContent = decoder.decode(fileBytes);

  const startIndex = fileContent.indexOf(BEGIN_MARKER);
  if (startIndex < 0) {
    throw new Error(`Failed to find '${BEGIN_MARKER}' in ${path}`);
  }
  const endIndex = fileContent.indexOf(END_MARKER, startIndex);
  if (endIndex < 0) {
    throw new Error(`Failed to find '${END_MARKER}' in ${path}`);
  }

  const encoder = new TextEncoder();
  await Deno.writeFile(
    path,
    encoder.encode(
      fileContent.slice(0, startIndex + BEGIN_MARKER.length) +
        "\n" +
        content +
        fileContent.slice(endIndex)
    )
  );
}

async function writeHelixOverview(html: string, rustdoc: Rustdoc) {
  const doc = new DOMParser().parseFromString(html, "text/html");
  const table = doc
    ?.getElementById("twitch-api-reference")
    ?.parentElement?.querySelector("tbody");
  if (!table) throw new Error("failed to find table");

  await pasteInFile("src/helix/mod.rs", makeHelixOverview(table, rustdoc));
}

async function writeEventsubOverview(html: string, rustdoc: Rustdoc) {
  const doc = new DOMParser().parseFromString(html, "text/html");
  const table = doc
    ?.getElementById("subscription-types")
    ?.parentElement?.querySelector("tbody");
  if (!table) throw new Error("failed to find table");

  await pasteInFile(
    "src/eventsub/mod.rs",
    makeEventsubOverview(table, rustdoc)
  );
}

const [helixHtml, eventsubHtml, rustdoc] = await Promise.all([
  fetch("https://dev.twitch.tv/docs/api/reference").then((x) => x.text()),
  fetch("https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types").then(
    (x) => x.text()
  ),
  readRustdoc(),
]);

await writeEventsubOverview(eventsubHtml, rustdoc);
await writeHelixOverview(helixHtml, rustdoc);
