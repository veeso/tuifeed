# Changelog

- [Changelog](#changelog)
  - [0.2.0](#020)
  - [0.1.1](#011)
  - [0.1.0](#010)

---

## 0.2.0

Released on ??

- If `content` in RSS feed is populated, use content instead of `summary`
  - This should be preferred since contains much more information than just the summary. Unfortunately sometimes the content is empty, so summary should be used as fallback
- Changed stdlib `Textarea` to `tui-realm-textarea` component to improve readability of summary
- Migrated UI to tui-realm 1.7.0
- Updated dependencies
  - argh `0.1.8`
  - crossterm `0.24`
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
