# Mangadex API

This project offers an API wrapper and Rust types for [MangaDex API (5.9)](https://api.mangadex.org/docs/redoc.html).

**NOTE: At This point in the development of this library I'm trying to create features / make the library work so
that it is as easy as possible to integrate with my MangaDex Reader project that uses Tauri**

## Feature / Todo list

> `X` done, `-` no methods of this type for this API endpoint

| Wrapper Endpoints | GET | POST | PUT | DELETE |
| ----------------- | --- | ---- | --- | ------ |
| at_home           | X   | -    | -   | -      |
| auth              |     |      |     |        |
| author            | X   |      |     |        |
| captcha           |     |      |     |        |
| chapter           | X   |      |     |        |
| cover             | X   |      |     |        |
| customlist        |     |      |     |        |
| feed              |     |      |     |        |
| follows           |     |      |     |        |
| forums            |     |      |     |        |
| infrastructure    |     |      |     |        |
| legacy            |     |      |     |        |
| manga             | WIP |      |     |        |
| rating            |     |      |     |        |
| readmarker        |     |      |     |        |
| report            |     |      |     |        |
| scanlationgroup   |     |      |     |        |
| settings          |     |      |     |        |
| upload            |     |      |     |        |
| user              |     |      |     |        |

<br/>

### Todo

- [ ] Chapter / Manga downloader
