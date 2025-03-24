//! # Client
//!
//! RSS/Atom client

mod file;
mod http;

use std::io::Read;

use feed_rs::parser as feed_parser;

use super::{Feed, FeedError, FeedResult, FeedSource};

/// RSS client. Fetches its sources to retrieve all the required Feeds
#[derive(Default)]
pub struct Client;

impl Client {
    /// Fetch a single source from remote
    pub fn fetch(&self, source: &FeedSource) -> FeedResult<Feed> {
        match source {
            FeedSource::File(p) => self.parse_feed(file::FileClient.fetch(p)?),
            FeedSource::Http(s) => self.parse_feed(http::HttpClient.fetch(s)?),
        }
    }

    /// Parse feed from HTTP response
    fn parse_feed<R: Read>(&self, response: R) -> FeedResult<Feed> {
        feed_parser::parse(response)
            .map(Feed::from)
            .map_err(FeedError::from)
    }
}

/// The fetch trait is used to get a [`Feed`] from a given [`FeedSource`]
pub trait Fetch {
    type Source;

    fn fetch(&self, source: &Self::Source) -> FeedResult<impl Read>;
}

#[cfg(test)]
mod test {

    use std::io::Write as _;

    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn should_get_source() {
        let client = Client::default();
        assert!(
            client
                .fetch(
                    &(String::from("https://rss.nytimes.com/services/xml/rss/nyt/World.xml")
                        .try_into()
                        .unwrap())
                )
                .is_ok()
        );
    }

    #[test]
    fn should_fail_getting_source() {
        let client = Client::default();
        assert!(
            client
                .fetch(
                    &(String::from(
                        "https://rss.nytimes.com/services/xml/rss/nyt/pippopippopippo.xml"
                    )
                    .try_into()
                    .unwrap())
                )
                .is_err()
        );
    }

    #[test]
    fn should_fetch_source() {
        assert!(
            Client
                .fetch(
                    &("https://rss.nytimes.com/services/xml/rss/nyt/World.xml"
                        .to_string()
                        .try_into()
                        .unwrap())
                )
                .is_ok()
        );
        assert!(
            Client
                .fetch(
                    &("https://www.lefigaro.fr/rss/figaro_actualites.xml"
                        .to_string()
                        .try_into()
                        .unwrap())
                )
                .is_ok()
        );
    }

    #[test]
    fn test_should_fetch_source_from_file() {
        let f = NamedTempFile::new().unwrap();

        // write to f
        let mut file = f.reopen().unwrap();
        file.write_all(FEED.as_bytes()).unwrap();
        drop(file);

        assert!(
            Client
                .fetch(&FeedSource::File(f.path().to_path_buf()))
                .is_ok()
        );
    }

    const FEED: &str = r#"
<?xml version="1.0" encoding="UTF-8"?><rss xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:content="http://purl.org/rss/1.0/modules/content/" xmlns:atom="http://www.w3.org/2005/Atom" version="2.0"><channel><title><![CDATA[Christian Visintin Blog]]></title><description><![CDATA[Rust tech blogger, software engineer, and open-source enthusiast. I write about Rust, Web Development. Seen on this week in Rust]]></description><link>https://blog.veeso.dev</link><generator>GatsbyJS</generator><lastBuildDate>Mon, 24 Mar 2025 08:27:52 GMT</lastBuildDate><item><title><![CDATA[Embedding shared objects in Rust]]></title><description><![CDATA[But why? So recently I've covered the topic of Vendoring C Dependencies in Rust and I've shown how to build a static library and link it to a Rust library.

In the same article I've also covered how hard it was to get a static library for smbclient and so…]]></description><link>https://blog.veeso.dev/blog/en/embedding-shared-objects-in-rust/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/embedding-shared-objects-in-rust/</guid><pubDate>Fri, 21 Mar 2025 16:00:00 GMT</pubDate><preview>https://blog.veeso.dev/static/46ed2c88df4a69901cb0f9510710d35c/47498/featured.jpg</preview></item><item><title><![CDATA[Vendoring C/C++ dependencies in Rust]]></title><description><![CDATA[What is vendoring about? I'm writing this post because basically I haven't found any guide about this, and for sure with this level of completeness and masochism.

So what are we even talking about? Have you ever used the native-tls crate?

Well it is an…]]></description><link>https://blog.veeso.dev/blog/en/vendoring-c-cpp-dependencies-in-rust/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/vendoring-c-cpp-dependencies-in-rust/</guid><pubDate>Thu, 20 Mar 2025 17:00:00 GMT</pubDate><preview>https://blog.veeso.dev/static/11e95ba2de0102bb932173565f537e37/47498/featured.jpg</preview></item><item><title><![CDATA[Extending Future in Rust]]></title><description><![CDATA[Introduction As part of my experiments with Async Rust (cuz I'm working on something big 🤞) and while writing my last article Async Rust for Dummies a weird and sick thought came to my mind: can we actually extend Future?

Extending a Trait

This may not be…]]></description><link>https://blog.veeso.dev/blog/en/extending-future-in-rust/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/extending-future-in-rust/</guid><pubDate>Fri, 14 Mar 2025 18:00:00 GMT</pubDate><preview>https://blog.veeso.dev/static/366aeeb2aa1a8ec5939b7bb40c8b8721/47498/featured.jpg</preview></item><item><title><![CDATA[Async Rust for Dummies]]></title><description><![CDATA[Introduction Hello, Rustaceans! I'm quite sure many of you use async Rust every day in your projects, but do you actually know how it works under the hood? In this article I'm going to show you how async Rust works, explaining the Future trait, the Pin…]]></description><link>https://blog.veeso.dev/blog/en/async-rust-for-dummies/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/async-rust-for-dummies/</guid><pubDate>Tue, 11 Mar 2025 18:00:00 GMT</pubDate><preview>https://blog.veeso.dev/static/9775a6d3f4fbc9b6343e59c39c25c91d/47498/featured.jpg</preview></item><item><title><![CDATA[How to setup a Bitcoin Solo Mining Pool]]></title><description><![CDATA[About lottery mining This is an expansion of my previous article "How to get started with Bitcoin Lottery Mining".

So if you're just interested in lottery mining on public pools, you can stop reading here and go to the previous article.

Why solo mining on…]]></description><link>https://blog.veeso.dev/blog/en/how-to-setup-a-bitcoin-solo-mining-pool/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/how-to-setup-a-bitcoin-solo-mining-pool/</guid><pubDate>Fri, 21 Feb 2025 15:15:00 GMT</pubDate><preview>https://blog.veeso.dev/static/582e3a869e5b1a8df588363679de0074/47498/featured.jpg</preview></item><item><title><![CDATA[A journey into File Transfer Protocols in Rust]]></title><description><![CDATA[How it started I can for sure affirm that you've used File transfer protocols before. Let's exclude HTTP from here, because, of course it is currently used also to transfer files, but it's not bi-directional and it mostly a workaround added at a certain…]]></description><link>https://blog.veeso.dev/blog/en/a-journey-into-file-transfer-protocols-in-rust/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/a-journey-into-file-transfer-protocols-in-rust/</guid><pubDate>Mon, 06 Jan 2025 13:15:00 GMT</pubDate><preview>https://blog.veeso.dev/static/062e7ed6e8da51775a22e6b1f1c69ded/47498/featured.jpg</preview></item><item><title><![CDATA[Why Bitcoin is not decentralized - in any way]]></title><description><![CDATA[Bitcoin is decentralized, right? So yesterday I was scrolling my feed on LinkedIn, a thing I usually do once a month, and I found this:

So in the image above you can see a chart showing the Bitcoin distribution over different entities.

The chart is…]]></description><link>https://blog.veeso.dev/blog/en/why-bitcoin-is-not-decentralized--in-any-way/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/why-bitcoin-is-not-decentralized--in-any-way/</guid><pubDate>Thu, 19 Dec 2024 15:00:00 GMT</pubDate><preview>https://blog.veeso.dev/static/17bd4aade9565ec8bdd2c6479e932749/47498/featured.jpg</preview></item><item><title><![CDATA[Revolutionizing the real estate market with EKOKE DAO]]></title><description><![CDATA[Introduction So in the last year I've been working a lot on the development of the EKOKE DAO project. I am the main software engineer of the software and I've personally designed the project architecture and implemented most of the codebase.

Now we're…]]></description><link>https://blog.veeso.dev/blog/en/revolutioninzing-the-real-estate-market-with-ekoke-dao/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/revolutioninzing-the-real-estate-market-with-ekoke-dao/</guid><pubDate>Wed, 18 Dec 2024 13:00:00 GMT</pubDate><preview>https://blog.veeso.dev/static/d154dca9e0657a4169c52788f8725527/47498/featured.jpg</preview></item><item><title><![CDATA[How to make react-markdown work with Parcel?]]></title><description><![CDATA[Audience If you encounter an error like this while trying to render a markdown with react-markdown and you're using parcel

Uncaught TypeError: Cannot convert undefined or null to object
Cannot read properties of undefined (reading 'src')


this article is…]]></description><link>https://blog.veeso.dev/blog/en/how-to-make-react--markdown-work-with-parcel/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/how-to-make-react--markdown-work-with-parcel/</guid><pubDate>Mon, 16 Dec 2024 10:45:00 GMT</pubDate><preview>https://blog.veeso.dev/static/1e68910d496b4f75d0733c4335db1796/47498/featured.jpg</preview></item><item><title><![CDATA[The Ethereum MEV Bots are a scam]]></title><description><![CDATA[Several months ago, I wrote this article: The Fascinating Ethereum MEV Bot Scam, where I dived deep into the technical aspects of a scam that has been circulating for years on platforms like X.com and YouTube. These scams revolve around deploying smart…]]></description><link>https://blog.veeso.dev/blog/en/ethereum-mev-bots-are-a-scam/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/ethereum-mev-bots-are-a-scam/</guid><pubDate>Tue, 19 Nov 2024 19:00:00 GMT</pubDate><preview>https://blog.veeso.dev/static/ec601aa27d1149167054ea690ddca9e7/47498/featured.jpg</preview></item><item><title><![CDATA[You don't (*always*) need async]]></title><description><![CDATA[We are too much into async, but we can't go back You know what? Async Rust is great, but I also was relatively a late adopter of it, mainly because I've worked a lot on CLI tools before starting to work on server applications and web service, so I hadn't…]]></description><link>https://blog.veeso.dev/blog/en/you-dont-always-need-async/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/you-dont-always-need-async/</guid><pubDate>Mon, 18 Nov 2024 15:00:00 GMT</pubDate><preview>https://blog.veeso.dev/static/461c79482a5c751b5c2531eee45f8aa4/47498/featured.jpg</preview></item><item><title><![CDATA[Reached the recursion limit... at build time?]]></title><description><![CDATA[A bit of context So on the other day I was implementing Dokany for my Remotefs-fuse driver (coming soon btw). For those who doesn't know, Dokany provides something similiar to FUSE, but on Windows Systems, and for those who don't know what FUSE is, it is a…]]></description><link>https://blog.veeso.dev/blog/en/reached-the-recursion-limit-at-build-time/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/reached-the-recursion-limit-at-build-time/</guid><pubDate>Sun, 03 Nov 2024 17:00:00 GMT</pubDate><preview>https://blog.veeso.dev/static/0e4edd30b5fb6cecaa4515fe6f0bbc52/47498/featured.jpg</preview></item><item><title><![CDATA[Dyn Box Vs. Generics]]></title><description><![CDATA[Case scenario Have you ever done something similiar?

trait Greet {

  fn greet(&self) -> String;

}

struct Alice;

impl Greet for Alice {
  fn greet(&self) -> String {
    "Hello".to_string()
  }
}

struct Carlo;

impl Greet for Carlo {
  fn greet(&self)…]]></description><link>https://blog.veeso.dev/blog/en/dyn-box-vs-generics-in-rust/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/dyn-box-vs-generics-in-rust/</guid><pubDate>Mon, 28 Oct 2024 17:00:00 GMT</pubDate><preview>https://blog.veeso.dev/static/f0ec919322601feaf2ef2ec4bcb5c612/47498/featured.jpg</preview></item><item><title><![CDATA[Announcing termscp 0.16.0]]></title><description><![CDATA[What's new So let's see what's new in this version 0.16 of termscp.

Don't you know termscp yet? Come discover this amazing project on Github.

Multi host transfers
What is multi-host

So currently termscp could only work with a remote in the following way

|-…]]></description><link>https://blog.veeso.dev/blog/en/announcing-termscp-016/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/announcing-termscp-016/</guid><pubDate>Mon, 14 Oct 2024 18:15:00 GMT</pubDate><preview>https://blog.veeso.dev/static/cdf748f0da28f4785559dfbf33d690b8/47498/featured.jpg</preview></item><item><title><![CDATA[Announcing termscp 0.15.0]]></title><description><![CDATA[What's new So let's see what's new in this version 0.15 of termscp.

Don't you know termscp yet? Come discover this amazing project on Github.

Fuzzy Search

The find command was a little bit rudimental; it had been introduced in one of the first versions of…]]></description><link>https://blog.veeso.dev/blog/en/announcing-termscp-015/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/announcing-termscp-015/</guid><pubDate>Thu, 03 Oct 2024 18:15:00 GMT</pubDate><preview>https://blog.veeso.dev/static/53919b23e1ff1888871d834e31be1a19/47498/featured.jpg</preview></item><item><title><![CDATA[Hey Rustaceans: Rust freelancers do exist!]]></title><description><![CDATA[In 2023 I've decided to resign from my job at Prima to start working as a freelancer, with a particular focus on Rust, along with a minor web development. Currently I can affirm that for 90/95% of the time I work on Rust projects as a freelancer and I've…]]></description><link>https://blog.veeso.dev/blog/en/hey-rustaceans-rust-freelancers-do-exist/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/hey-rustaceans-rust-freelancers-do-exist/</guid><pubDate>Wed, 04 Sep 2024 12:00:00 GMT</pubDate><preview>https://blog.veeso.dev/static/bd89e2bb423e78c486df49f8cb866575/47498/featured.jpg</preview></item><item><title><![CDATA[termscp 0.14 has been released]]></title><description><![CDATA[It's been a long time It's been 5 months since the last release of termscp came out, so yeah let's party for this new termscp release 🎉.

what's new

So let's see what's new in this version 0.14 of termscp

Ssh agent

I've received many, many and many others…]]></description><link>https://blog.veeso.dev/blog/en/termscp-014-released/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/termscp-014-released/</guid><pubDate>Wed, 17 Jul 2024 17:30:00 GMT</pubDate><preview>https://blog.veeso.dev/static/faf0e55c67baa787ebc1f37da89b5c20/47498/featured.jpg</preview></item><item><title><![CDATA[Implementing a generic range parser in Rust]]></title><description><![CDATA[Simple problems that require complex solutions A few days ago I was implementing a really simple function which would given a string representation of a range it returns a Vec of the items to include for that range.

I wanted to make it generic for any kind…]]></description><link>https://blog.veeso.dev/blog/en/implementing-a-generic-range-parser-in-rust/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/implementing-a-generic-range-parser-in-rust/</guid><pubDate>Fri, 12 Jul 2024 16:00:00 GMT</pubDate><preview>https://blog.veeso.dev/static/12cba2c3131765ea9bfb2d86b12db4e9/47498/featured.jpg</preview></item><item><title><![CDATA[How to configure CPU cores to be used in a Tokio application with core_affinity]]></title><description><![CDATA[There are several cases where applications are developed for extreme performance and choosing the cores to be used for an application is required. For instance we may want to reserve certain cores for some specific process, or we may want to chunk the…]]></description><link>https://blog.veeso.dev/blog/en/how-to-configure-cpu-cores-to-be-used-on-a-tokio-with-core--affinity/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/how-to-configure-cpu-cores-to-be-used-on-a-tokio-with-core--affinity/</guid><pubDate>Sat, 06 Jul 2024 15:00:00 GMT</pubDate><preview>https://blog.veeso.dev/static/9069b016a6e8a23d4d53ac68b79b054e/47498/featured.jpg</preview></item><item><title><![CDATA[Let me introduce you ShitAI]]></title><description><![CDATA[A brief resume of how AI has taken over If we went back to five years ago and you would have said "AI" in front of somebody, probably the first thought to come to minds would have been sci-fi, stupid NPCs in videogames or a bunch of ridicolous videos of…]]></description><link>https://blog.veeso.dev/blog/en/let-me-introduce-you-shit-ai/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/let-me-introduce-you-shit-ai/</guid><pubDate>Tue, 28 May 2024 15:30:00 GMT</pubDate><preview>https://blog.veeso.dev/static/635ebce0285e50e5205871ae9cf067c4/47498/featured.jpg</preview></item><item><title><![CDATA[The Bitcoin Utopia is a Lie]]></title><description><![CDATA[Preamble I want to be clear on some aspects before starting this article:

This is not going to be a Fiat Vs. Bitcoin argumentation: I both believe both of them are equally bad There's not any alternative: I'm not here to tell you "you shouldn't use…]]></description><link>https://blog.veeso.dev/blog/en/the-bitcoin-utopia-is-a-lie/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/the-bitcoin-utopia-is-a-lie/</guid><pubDate>Fri, 12 Apr 2024 12:30:00 GMT</pubDate><preview>https://blog.veeso.dev/static/410eb9dbe9a3dd79309a81870aa13e86/47498/featured.jpg</preview></item><item><title><![CDATA[How to get started with Bitcoin Lottery Mining?]]></title><description><![CDATA[What is the Lottery Mining Have you ever heard about Lottery Mining? Lottery Mining is a low-probability mining strategy, and with low I mean VERY low, to try to mine a block on PoW blockchains, such as Bitcoin.

To understand that, we first need to…]]></description><link>https://blog.veeso.dev/blog/en/how-to-get-started-with-bitcoin-lottery-mining/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/how-to-get-started-with-bitcoin-lottery-mining/</guid><pubDate>Thu, 15 Feb 2024 15:30:00 GMT</pubDate><preview>https://blog.veeso.dev/static/70ffe8658cdb0fb349e0f2df912a8907/47498/featured.jpg</preview></item><item><title><![CDATA[The fascinating Ethereum MEV Bot scam]]></title><description><![CDATA[What is a MEV Bot For those who don't know:

A MEV bot, short for "Maximal Extractable Value" bot, is a sophisticated program that operates in the cryptocurrency world. It scans the blockchain, which is like a digital ledger of all crypto transactions…]]></description><link>https://blog.veeso.dev/blog/en/the-fascinating-ethereum-mev-bot-scam/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/the-fascinating-ethereum-mev-bot-scam/</guid><pubDate>Wed, 31 Jan 2024 12:40:00 GMT</pubDate><preview>https://blog.veeso.dev/static/c99c4be07f5601634209d1a5b5e3fcb4/47498/featured.jpg</preview></item><item><title><![CDATA[I've replaced Google Analytics with Umami]]></title><description><![CDATA[Google Analytics is mostly overrated I've used GA4 for my website in the past few months since it was released at the beginning of 2023, but to be honest, I couldn't really enjoy it, and I actually found several pain points in its usage:

Poor user…]]></description><link>https://blog.veeso.dev/blog/en/ive-replaced-google-analytics-with-umami/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/ive-replaced-google-analytics-with-umami/</guid><pubDate>Wed, 04 Oct 2023 17:00:00 GMT</pubDate><preview>https://blog.veeso.dev/static/92f71812e3470e3b7a17497eac3b070d/47498/featured.jpg</preview></item><item><title><![CDATA[How to migrate from ReactJS to Gatsby]]></title><description><![CDATA[Why to migrate your ReactJS frontend to Gatsby Do you have a website entirely developed with React without any Server-Side rendering? And maybe you're website is mostly a static one. Well, in that case, you should really consider migrating it to Gatsby.

Bu…]]></description><link>https://blog.veeso.dev/blog/en/how-to-migrate-from-reactjs-to-gatsby/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/how-to-migrate-from-reactjs-to-gatsby/</guid><pubDate>Thu, 21 Sep 2023 12:00:00 GMT</pubDate><preview>https://blog.veeso.dev/static/17266f750567dc3daab186a22b7a39ce/47498/featured.jpg</preview></item><item><title><![CDATA[Why you should (and should not) do open-source]]></title><description><![CDATA[Preamble I started my journey into open-source almost 6 years ago and I've worked on several projects. Currently, I have 90 public repos on my GitHub and at least 5 of them have become quite successful over the years. It started as a hobby but it's now…]]></description><link>https://blog.veeso.dev/blog/en/why-you-should-and-should-not-do-open-source/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/why-you-should-and-should-not-do-open-source/</guid><pubDate>Mon, 17 Jul 2023 12:00:00 GMT</pubDate><preview>https://blog.veeso.dev/static/642c04a53393de43034a36f1e03c1f6d/47498/featured.jpg</preview></item><item><title><![CDATA[How to access an SMB share with Rust (on Windows)]]></title><description><![CDATA[Introduction I’ve developed termscp, which is basically a feature-rich terminal file transfer and explorer, with support for SCP/SFTP/FTP/S3 and now also SMB. But implementing SMB wasn’t that simple.

Even if I managed to get a rust interface for…]]></description><link>https://blog.veeso.dev/blog/en/how-to-access-an-smb-share-with-rust-on-windows/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/how-to-access-an-smb-share-with-rust-on-windows/</guid><pubDate>Sat, 13 May 2023 00:00:00 GMT</pubDate><preview>https://blog.veeso.dev/static/6d64c93f14ff88b699fe61584b92d30b/7d509/featured.jpg</preview></item><item><title><![CDATA[For a sustainable open-source community, start pointing the finger at the mirror]]></title><description><![CDATA[Preamble It’s November 2020, and several NodeJs services suddenly stopped working. The panic spreads across the community, immediately someone points the finger to a possible wide-spread NPM package which is probably used by most node applications. They…]]></description><link>https://blog.veeso.dev/blog/en/for-a-sustainable-open-source-community-start-pointing-the-finger-at-the-mirror/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/for-a-sustainable-open-source-community-start-pointing-the-finger-at-the-mirror/</guid><pubDate>Mon, 20 Mar 2023 00:00:00 GMT</pubDate><preview>https://blog.veeso.dev/static/fb0b9d06214fa55767b235daff2777f1/47498/featured.jpg</preview></item><item><title><![CDATA[How Rust Nation ‘23 changed my POV on the Rust community]]></title><description><![CDATA[About Rust Nation '23 Between the 16th and the 17th of February 2023, at the Brewery in London took place the Rust Nation ’23 conference, which is the first UK conference dedicated to the Rust language. Long the two days took place three level workshops…]]></description><link>https://blog.veeso.dev/blog/en/how-rust-nation-23-changed-my-pov-on-the-rust-community/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/how-rust-nation-23-changed-my-pov-on-the-rust-community/</guid><pubDate>Mon, 20 Feb 2023 00:00:00 GMT</pubDate><preview>https://blog.veeso.dev/static/466ee61afc0822204e034747e633a316/47498/featured.jpg</preview></item><item><title><![CDATA[Dev Horror Story #1 — An Android Nightmare]]></title><description><![CDATA[Preamble Reflecting on our Android perception

Let's jump back to the 2010s, I was still a student at the high school and there was a feud between iOS and Android users. Everybody used to point out what made one of the two operating systems better than the…]]></description><link>https://blog.veeso.dev/blog/en/dev-horror-story-1-an-android-nightmare/</link><guid isPermaLink="false">https://blog.veeso.dev/blog/en/dev-horror-story-1-an-android-nightmare/</guid><pubDate>Wed, 25 Jan 2023 00:00:00 GMT</pubDate><preview>https://blog.veeso.dev/static/f449527f9f6fe5e111f39bc9e17d33ee/47498/featured.jpg</preview></item></channel></rss>
    "#;
}
