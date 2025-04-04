# Changelog

- [Changelog](#changelog)
  - [0.4.2](#042)
  - [0.4.1](#041)
  - [0.4.0](#040)
  - [0.3.2](#032)
  - [0.3.1 (yanked)](#031-yanked)
  - [0.3.0](#030)
  - [0.2.1](#021)
  - [0.2.0](#020)
  - [0.1.1](#011)
  - [0.1.0](#010)

---

## 0.4.2

Released on 30/03/2025

- [issue 16](https://github.com/veeso/tuifeed/issues/16): filter out old articles from the history.
- [issue 17](https://github.com/veeso/tuifeed/issues/17): mark as all read feature. Use `V` to mark all articles from a **source** as read. Use `CTRL+V` to mark all articles from **all sources** as read.
- [issue 18](https://github.com/veeso/tuifeed/issues/18): first article is not marked as read when first loaded. Also mark source as read if all articles are read

## 0.4.1

Released on 24/03/2025

- fixed a panic occurring on release versions due to ugly unwrap

## 0.4.0

Released on 24/03/2025

- [issue 4](https://github.com/veeso/tuifeed/issues/4): highlight new articles
- [issue 14](https://github.com/veeso/tuifeed/issues/14) fix: create config dir at first run
- fix: first article has width 1
- [issue 11](https://github.com/veeso/tuifeed/issues/11): add support for local file. Specify local file with `file:///path/to/rss.xml` in the URL field

## 0.3.2

Released on 30/08/2022

- Fixed articles sorted by date

## 0.3.1 (yanked)

Released on 30/08/2022

- Sort articles by date

## 0.3.0

Released on 29/08/2022

- Show timestamp next to articles (can be configured)
- Added new key `article-title` to configuration (optional)

    ```toml
    [article-title]
    show-author = false
    show-timestamp = true
    ```

## 0.2.1

Released on 19/08/2022

- If `published` is `None`, use `updated` from feed (see [issue 5](https://github.com/veeso/tuifeed/issues/5))

## 0.2.0

Released on 05/08/2022

- If `content` in RSS feed is populated, use content instead of `summary`
  - This should be preferred since contains much more information than just the summary. Unfortunately sometimes the content is empty, so summary should be used as fallback
- Changed stdlib `Textarea` to `tui-realm-textarea` component to improve readability of summary
- Fixed the article date shown: was `updated` but should be `published`
- Migrated UI to tui-realm 1.7.1
- Updated dependencies
  - argh `0.1.8`
  - feed-rs `1.1.0`
  - lazy-regex `2.3.0`
  - lazy_static `1.4.0`
  - open `3.0.2`
  - toml `0.5.9`
  - ureq `2.5.0`

## 0.1.1

Released on 17/11/2021

- Escape HTML entities from text (e.g. `&amp;` or `&#8220;`)

## 0.1.0

Released on 15/11/2021

- First release
