# Changelog

- [Changelog](#changelog)
  - [0.3.0](#030)
  - [0.2.1](#021)
  - [0.2.0](#020)
  - [0.1.1](#011)
  - [0.1.0](#010)

---

## 0.3.0

Released on 29/08/2022

- Show timestamp next to articles (can be configured)

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
