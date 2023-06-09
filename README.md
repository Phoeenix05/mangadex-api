# Mangadex API

This project offers an API wrapper and Rust types for [MangaDex API (5.9)](https://api.mangadex.org/docs/redoc.html).

**NOTE: At This point in the development of this library I'm trying to create features / make the library work so
that it is as easy as possible to integrate with my MangaDex Reader project**

<br />

## Installation / How to use

1. Add to project dependencies
```toml
mangadex-api = { git = "https://github.com/Phoeenix05/mangadex-api" }
```

2. Use
```rust
use mangadex_api::prelude::*;

let data = Client::<Manga>::new(uuid::uuid!("77bee52c-d2d6-44ad-a33a-1734c1fe696a")).get().unwrap();
```

<br />

## Features

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
| manga tag         | X   | -    | -   | -      |
| rating            |     |      |     |        |
| readmarker        |     |      |     |        |
| report            |     |      |     |        |
| scanlationgroup   |     |      |     |        |
| settings          |     |      |     |        |
| statistics        | WIP | -    | -   | -      |
| upload            |     |      |     |        |
| user              |     |      |     |        |

### API Endpoint that requires authentication
| Wrapper Endpoints | GET | POST | PUT | DELETE |
| ----------------- | --- | ---- | --- | ------ |
| author            | -   |      |     |        |
| chapter           | -   | -    |     |        |
| cover             | -   |      |     |        |
| customlist        |     |      |     |        |
| feed              |     | -    | -   | -      |
| follows           |     | -    | -   | -      |
| forums            | -   |      | -   | -      |
| manga             |     |      |     |        |
| rating            |     |      | -   |        |
| readmarker        |     |      | -   | -      |
| report            |     |      | -   | -      |
| scanlationgroup   | -   |      |     |        |
| settings          |     |      |     |        |
| upload            |     |      |     |        |
| user              |     | -    | -   | -      |

<br/>

## Todo list

- [ ] Chapter / Manga downloader
