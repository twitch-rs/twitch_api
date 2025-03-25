(function() {
    var type_impls = Object.fromEntries([["twitch_api",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ChannelEmote\" class=\"impl\"><a class=\"src rightside\" href=\"src/twitch_api/helix/endpoints/chat/mod.rs.html#164-182\">Source</a><a href=\"#impl-ChannelEmote\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"struct\" href=\"twitch_api/helix/chat/struct.ChannelEmote.html\" title=\"struct twitch_api::helix::chat::ChannelEmote\">ChannelEmote</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.url\" class=\"method\"><a class=\"src rightside\" href=\"src/twitch_api/helix/endpoints/chat/mod.rs.html#181\">Source</a><h4 class=\"code-header\">pub fn <a href=\"twitch_api/helix/chat/struct.ChannelEmote.html#tymethod.url\" class=\"fn\">url</a>(&amp;self) -&gt; <a class=\"struct\" href=\"twitch_types/emote/struct.EmoteUrlBuilder.html\" title=\"struct twitch_types::emote::EmoteUrlBuilder\">EmoteUrlBuilder</a>&lt;'_&gt;</h4></section></summary><div class=\"docblock\"><p>Create an emote builder for this emote.</p>\n<h5 id=\"examples\"><a class=\"doc-anchor\" href=\"#examples\">§</a>Examples</h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">let </span>emotes = client.get_channel_emotes_from_login(<span class=\"string\">\"twitchdev\"</span>, <span class=\"kw-2\">&amp;</span>token).<span class=\"kw\">await</span><span class=\"question-mark\">?</span>.expect(<span class=\"string\">\"user not found\"</span>);\n<span class=\"macro\">assert_eq!</span>(emotes[<span class=\"number\">0</span>].url().size_3x().dark_mode().render(), <span class=\"string\">\"https://static-cdn.jtvnw.net/emoticons/v2/emotesv2_dc24652ada1e4c84a5e3ceebae4de709/default/dark/3.0\"</span>);</code></pre></div>\n</div></details></div></details>",0,"twitch_api::helix::endpoints::chat::get_channel_emotes::GetChannelEmotesResponse"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-ChannelEmote\" class=\"impl\"><a class=\"src rightside\" href=\"src/twitch_api/helix/endpoints/chat/mod.rs.html#127\">Source</a><a href=\"#impl-Clone-for-ChannelEmote\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"twitch_api/helix/chat/struct.ChannelEmote.html\" title=\"struct twitch_api::helix::chat::ChannelEmote\">ChannelEmote</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/twitch_api/helix/endpoints/chat/mod.rs.html#127\">Source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"twitch_api/helix/chat/struct.ChannelEmote.html\" title=\"struct twitch_api::helix::chat::ChannelEmote\">ChannelEmote</a></h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/clone.rs.html#174\">Source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: &amp;Self)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","twitch_api::helix::endpoints::chat::get_channel_emotes::GetChannelEmotesResponse"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-ChannelEmote\" class=\"impl\"><a class=\"src rightside\" href=\"src/twitch_api/helix/endpoints/chat/mod.rs.html#127\">Source</a><a href=\"#impl-Debug-for-ChannelEmote\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"twitch_api/helix/chat/struct.ChannelEmote.html\" title=\"struct twitch_api::helix::chat::ChannelEmote\">ChannelEmote</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/twitch_api/helix/endpoints/chat/mod.rs.html#127\">Source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","twitch_api::helix::endpoints::chat::get_channel_emotes::GetChannelEmotesResponse"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Deserialize%3C'de%3E-for-ChannelEmote\" class=\"impl\"><a class=\"src rightside\" href=\"src/twitch_api/helix/endpoints/chat/mod.rs.html#127\">Source</a><a href=\"#impl-Deserialize%3C'de%3E-for-ChannelEmote\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.217/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"twitch_api/helix/chat/struct.ChannelEmote.html\" title=\"struct twitch_api::helix::chat::ChannelEmote\">ChannelEmote</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.deserialize\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/twitch_api/helix/endpoints/chat/mod.rs.html#127\">Source</a><a href=\"#method.deserialize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://docs.rs/serde/1.0.217/serde/de/trait.Deserialize.html#tymethod.deserialize\" class=\"fn\">deserialize</a>&lt;__D&gt;(__deserializer: __D) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;Self, __D::<a class=\"associatedtype\" href=\"https://docs.rs/serde/1.0.217/serde/de/trait.Deserializer.html#associatedtype.Error\" title=\"type serde::de::Deserializer::Error\">Error</a>&gt;<div class=\"where\">where\n    __D: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.217/serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt;,</div></h4></section></summary><div class='docblock'>Deserialize this value from the given Serde deserializer. <a href=\"https://docs.rs/serde/1.0.217/serde/de/trait.Deserialize.html#tymethod.deserialize\">Read more</a></div></details></div></details>","Deserialize<'de>","twitch_api::helix::endpoints::chat::get_channel_emotes::GetChannelEmotesResponse"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-ChannelEmote\" class=\"impl\"><a class=\"src rightside\" href=\"src/twitch_api/helix/endpoints/chat/mod.rs.html#127\">Source</a><a href=\"#impl-PartialEq-for-ChannelEmote\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for <a class=\"struct\" href=\"twitch_api/helix/chat/struct.ChannelEmote.html\" title=\"struct twitch_api::helix::chat::ChannelEmote\">ChannelEmote</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/twitch_api/helix/endpoints/chat/mod.rs.html#127\">Source</a><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;<a class=\"struct\" href=\"twitch_api/helix/chat/struct.ChannelEmote.html\" title=\"struct twitch_api::helix::chat::ChannelEmote\">ChannelEmote</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>self</code> and <code>other</code> values to be equal, and is used by <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#262\">Source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>!=</code>. The default implementation is almost always sufficient,\nand should not be overridden without very good reason.</div></details></div></details>","PartialEq","twitch_api::helix::endpoints::chat::get_channel_emotes::GetChannelEmotesResponse"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Serialize-for-ChannelEmote\" class=\"impl\"><a class=\"src rightside\" href=\"src/twitch_api/helix/endpoints/chat/mod.rs.html#127\">Source</a><a href=\"#impl-Serialize-for-ChannelEmote\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://docs.rs/serde/1.0.217/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"twitch_api/helix/chat/struct.ChannelEmote.html\" title=\"struct twitch_api::helix::chat::ChannelEmote\">ChannelEmote</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.serialize\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/twitch_api/helix/endpoints/chat/mod.rs.html#127\">Source</a><a href=\"#method.serialize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://docs.rs/serde/1.0.217/serde/ser/trait.Serialize.html#tymethod.serialize\" class=\"fn\">serialize</a>&lt;__S&gt;(&amp;self, __serializer: __S) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;__S::<a class=\"associatedtype\" href=\"https://docs.rs/serde/1.0.217/serde/ser/trait.Serializer.html#associatedtype.Ok\" title=\"type serde::ser::Serializer::Ok\">Ok</a>, __S::<a class=\"associatedtype\" href=\"https://docs.rs/serde/1.0.217/serde/ser/trait.Serializer.html#associatedtype.Error\" title=\"type serde::ser::Serializer::Error\">Error</a>&gt;<div class=\"where\">where\n    __S: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.217/serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a>,</div></h4></section></summary><div class='docblock'>Serialize this value into the given Serde serializer. <a href=\"https://docs.rs/serde/1.0.217/serde/ser/trait.Serialize.html#tymethod.serialize\">Read more</a></div></details></div></details>","Serialize","twitch_api::helix::endpoints::chat::get_channel_emotes::GetChannelEmotesResponse"],["<section id=\"impl-Eq-for-ChannelEmote\" class=\"impl\"><a class=\"src rightside\" href=\"src/twitch_api/helix/endpoints/chat/mod.rs.html#127\">Source</a><a href=\"#impl-Eq-for-ChannelEmote\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> for <a class=\"struct\" href=\"twitch_api/helix/chat/struct.ChannelEmote.html\" title=\"struct twitch_api::helix::chat::ChannelEmote\">ChannelEmote</a></h3></section>","Eq","twitch_api::helix::endpoints::chat::get_channel_emotes::GetChannelEmotesResponse"],["<section id=\"impl-StructuralPartialEq-for-ChannelEmote\" class=\"impl\"><a class=\"src rightside\" href=\"src/twitch_api/helix/endpoints/chat/mod.rs.html#127\">Source</a><a href=\"#impl-StructuralPartialEq-for-ChannelEmote\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html\" title=\"trait core::marker::StructuralPartialEq\">StructuralPartialEq</a> for <a class=\"struct\" href=\"twitch_api/helix/chat/struct.ChannelEmote.html\" title=\"struct twitch_api::helix::chat::ChannelEmote\">ChannelEmote</a></h3></section>","StructuralPartialEq","twitch_api::helix::endpoints::chat::get_channel_emotes::GetChannelEmotesResponse"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[14022]}