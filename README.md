# tuifeed

<p align="center">
  <img src="/docs/images/tuifeed.svg" width="256" height="256" />
</p>

<p align="center">~ A terminal news feed reader with a fancy ui ~</p>

<p align="center">Developed by <a href="https://veeso.github.io/" target="_blank">@veeso</a></p>
<p align="center">Current version: 0.2.0 (17/11/2021)</p>

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
  <a href="https://ko-fi.com/veeso">
    <img
      src="https://img.shields.io/badge/donate-ko--fi-red"
      alt="Ko-fi"
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
</p>

---

- [tuifeed](#tuifeed)
  - [About tuifeed 📰](#about-tuifeed-)
    - [Features 🎁](#features-)
  - [Get started 🏁](#get-started-)
    - [Installation 🚀](#installation-)
      - [Shell script 🖥️](#shell-script-️)
      - [Windows users 🍫](#windows-users-)
      - [Cargo installation 📦](#cargo-installation-)
    - [Configure your news feed](#configure-your-news-feed)
  - [Keybindings ⌨️](#keybindings-️)
  - [Support the developer ☕](#support-the-developer-)
  - [Powered by 💪](#powered-by-)
  - [Contributing and issues 🤝🏻](#contributing-and-issues-)
  - [Changelog ⏳](#changelog-)
  - [License 📃](#license-)

---

## About tuifeed 📰

tuifeed is a news feed reader with a fancy terminal user interface. It allows you read news from your favourite RSS and Atom sources, which can be easily configured in a TOML file.

![Demo](/docs/images/demo.gif)

### Features 🎁

- 📰 Read feed from any RSS/Atom source
- 🌐 Open any article in your favourite browser in one-click
- 🐧 Compatible with Windows, Linux, *BSD and MacOS
- 🤖 Easy setup
- 🦀 Rust-powered

---

## Get started 🏁

### Installation 🚀

#### Shell script 🖥️

You can directly install tuifeed with this simple shell script:

```sh
curl --proto '=https' --tlsv1.2 -sSLf "https://git.io/J1O0Z" | sh
```

⚠️ this method is supported for the following operating systems:

- 🐧 **GNU/Linux**
- 🍎 **MacOS**
- 🚩 **NetBSD**

> ❗ If you don't trust the script, you can view its content [on Github](https://github.com/veeso/tuifeed/blob/main/install.sh)

#### Windows users 🍫

You can install tuifeed on Windows with [Chocolatey](https://chocolatey.org/):

```ps1
choco install tuifeed
```

#### Cargo installation 📦

You can opt to install tuifeed with Cargo:

```sh
cargo install --locked tuifeed
```

---

### Configure your news feed

Once installed, in order to start using tuifeed, the first thing you need to do is to configure the feeds you want to read from.

To open the configuration file you can run

```sh
tuifeed -c
```

this will open the configuration file in your favourite editor.

> ❗ If this option doesn't work for your, you can edit the file manually at:
>
> - `~/.config/tuifeed/config.toml` on Linux
> - `/Users/$USER/Library/Application\ Support/tuifeed/config.toml` on MacOS

then you can add new sources as follows:

```toml
[sources]
"Corriere Della Sera" = "http://xml2.corriereobjects.it/rss/homepage.xml"
"Dev.to" = "https://dev.to/feed/"
"Hacker News" = "https://hnrss.org/newest"
"Il Piccolo" = "https://ilpiccolo.gelocal.it/rss/finegil/ilpiccolo/feed.rss"
"Il Post (Mondo)" = "https://www.ilpost.it/mondo/feed/"
"Il Post (Italia)" = "https://www.ilpost.it/italia/feed/"
"Messaggero Veneto" = "https://messaggeroveneto.gelocal.it/rss/finegil/messaggeroveneto/feed.rss"
"New York Times" = "https://rss.nytimes.com/services/xml/rss/nyt/World.xml"
```

so for each feed you want to read from, you must put an entry with a key, which identifies the **Name** of the source as it'll be displayed in the UI associated to the URL of the feed.
  
> 🪄 If you want to use special characters in toml you can quote the key name:
> `"Il Post (Mondo)" = "https://www.ilpost.it/mondo/feed/"`

Once you're done with configuration, save, close and enjoy tuifeed 😄

## Keybindings ⌨️

| Key                              | Where                           | Description                                         |
|----------------------------------|---------------------------------|-----------------------------------------------------|
| Tab, Right                       | Feed list                       | Move to article list                                |
| Up, Down, PageUp, PageDown       | Feed list, article list         | Scroll up/down in list                              |
| Home, End                        | Feed list, article list         | Go to the beginning/end of the list                 |
| R                                | Feed list                       | Reload selected source                              |
| CTRL+R                           | Feed list                       | Reload all sources                                  |
| Tab, Left                        | Article list                    | Move to feed list                                   |
| Right                            | Article list                    | Move to article summary                             |
| Left                             | Article summary                 | Move to article list                                |
| Up, Down, PageUp, PageDown       | Article summary                 | Scroll up/down in summary                           |
| Home, End                        | Article summary                 | Go to the beginning/end of summary                  |
| Enter                            | Article summary, article link   | Open selected article url in your favourite browser |
| Esc                              | *                               | Quit tuifeed                                        |

---

## Support the developer ☕

If you like tuifeed and you're grateful for the work I've done, please consider a little donation 🥳

You can make a donation with one of these platforms:

[![ko-fi](https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/veeso)
[![PayPal](https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white)](https://www.paypal.me/chrisintin)

---

## Powered by 💪

- [crossterm](https://github.com/crossterm-rs/crossterm)
- [feed-rs](https://github.com/feed-rs/feed-rs)
- [tui-realm](https://github.com/veeso/tui-realm)
- [tui-rs](https://github.com/fdehau/tui-rs)
- [ureq](https://github.com/algesten/ureq)

---

## Contributing and issues 🤝🏻

Contributions, bug reports, new features and questions are welcome! 😉
If you have any question or concern, or you want to suggest a new feature, or you want just want to improve tuifeed, feel free to open an issue or a PR.

Please follow [our contributing guidelines](CONTRIBUTING.md)

---

## Changelog ⏳

View tuifeed's changelog [HERE](CHANGELOG.md)

---

## License 📃

tuifeed is licensed under the MIT license.

You can read the entire license [HERE](LICENSE)
