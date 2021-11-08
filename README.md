# tuifeed

<p align="center">
  <img src="/docs/images/tuifeed.svg" width="256" height="256" />
</p>

<p align="center">~ A terminal news feed reader with a fancy ui ~</p>

<p align="center">Developed by <a href="https://veeso.github.io/" target="_blank">@veeso</a></p>
<p align="center">Current version: 0.1.0 (FIXME:/10/2021)</p>

<p align="center">
  <a href="https://opensource.org/licenses/MIT"
    ><img
      src="https://img.shields.io/badge/License-MIT-teal.svg"
      alt="License-MIT"
  /></a>
  <a href="https://github.com/veeso/tuifeed/stargazers"
    ><img
      src="https://img.shields.io/github/stars/veeso/tuifeed.svg"
      alt="Repo stars"
  /></a>
  <a href="https://crates.io/crates/tuifeed"
    ><img
      src="https://img.shields.io/crates/d/tuifeed.svg"
      alt="Downloads counter"
  /></a>
  <a href="https://crates.io/crates/tuifeed"
    ><img
      src="https://img.shields.io/crates/v/tuifeed.svg"
      alt="Latest version"
  /></a>
  <a href="https://www.buymeacoffee.com/veeso">
    <img
      src="https://img.shields.io/badge/Donate-BuyMeACoffee-yellow.svg"
      alt="Buy me a coffee"
  /></a>
</p>
<p align="center">
  <a href="https://github.com/veeso/tuifeed/actions"
    ><img
      src="https://github.com/veeso/tuifeed/workflows/Linux/badge.svg"
      alt="Linux CI"
  /></a>
  <a href="https://github.com/veeso/tuifeed/actions"
    ><img
      src="https://github.com/veeso/tuifeed/workflows/MacOS/badge.svg"
      alt="MacOS"
  /></a>
  <a href="https://github.com/veeso/tuifeed/actions"
    ><img
      src="https://github.com/veeso/tuifeed/workflows/Windows/badge.svg"
      alt="Windows"
  /></a>
  <a href="https://coveralls.io/github/veeso/tuifeed"
    ><img
      src="https://coveralls.io/repos/github/veeso/tuifeed/badge.svg"
      alt="Coveralls"
  /></a>
  <a href="https://docs.rs/tuifeed"
    ><img
      src="https://docs.rs/tuifeed/badge.svg"
      alt="Docs"
  /></a>
</p>

---

- [tuifeed](#tuifeed)
  - [About tuifeed 📰](#about-tuifeed-)
  - [Get started 🏁](#get-started-)
    - [Configure your news feed](#configure-your-news-feed)
  - [Keybindings ⌨️](#keybindings-️)
  - [Support the developer ☕](#support-the-developer-)
  - [Contributing and issues 🤝🏻](#contributing-and-issues-)
  - [Changelog ⏳](#changelog-)
  - [License 📃](#license-)

---

## About tuifeed 📰

tuifeed is a news feed reader with a fancy terminal user interface. It allows you read news from your favourite RSS and Atom sources, which can be easily configured in a TOML file.

![Demo](/docs/images/demo.gif)

---

## Get started 🏁

> ⚠️ I'M STILL WORKING ON IT, DON'T DOWNLOAD IT YET

Install with cargo

```sh
cargo install tuifeed
```

### Configure your news feed

In order to start using tuifeed, the first thing you need to do is to configure the feeds you want to read from.

To open the configuration file you can run

```sh
tuifeed -c
```

this will open the configuration file in your favourite editor.

> ❗ If you don't have a GUI you can edit your configuration file at:
>
> - `~/.config/tuifeed/config.toml` on Linux
> - `/Users/$USER/Library/Application\ Support/tuifeed/config.toml` on MacOS

then you can add new sources as follows:

```toml
[sources]
Il_Post_Mondo = "https://www.ilpost.it/mondo/feed/"
Il_Post_Italia = "https://www.ilpost.it/italia/feed/"
New_York_Times = "https://rss.nytimes.com/services/xml/rss/nyt/World.xml"
```

so for each feed you want to read from, you must put an entry with a key, which identifies the **Name** of the source as it'll be displayed in the UI associated to the URL of the feed.
  
> 🪄 If you want to use special characters in toml you can quote the key name:
> `"Il Post (Mondo)" = "https://www.ilpost.it/mondo/feed/"`

Once you're done with configuration, save, close and enjoy tuifeed 😄

## Keybindings ⌨️

TBD

---

## Support the developer ☕

If you like tuifeed and you're grateful for the work I've done, please consider a little donation 🥳

You can make a donation with one of these platforms:

[![Buy-me-a-coffee](https://img.shields.io/badge/-buy_me_a%C2%A0coffee-gray?style=for-the-badge&logo=buy-me-a-coffee)](https://www.buymeacoffee.com/veeso)
[![PayPal](https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white)](https://www.paypal.me/chrisintin)

---

## Contributing and issues 🤝🏻

Contributions, bug reports, new features and questions are welcome! 😉
If you have any question or concern, or you want to suggest a new feature, or you want just want to improve tuifeed, feel free to open an issue or a PR.

TODO: contributing
Please follow [our contributing guidelines](CONTRIBUTING.md)

---

## Changelog ⏳

View tuifeed's changelog [HERE](CHANGELOG.md)

---

## License 📃

tuifeed is licensed under the MIT license.

You can read the entire license [HERE](LICENSE)
