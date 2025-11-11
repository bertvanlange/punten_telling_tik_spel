# Punten Telling Tik Spel

Rust + WebAssembly app that parses Google Sheets CSVs for a “Tik” game and shows the results in a small web page.
Web page: https://bertvanlange.github.io/punten_telling_tik_spel/dashboard.html 

## Overview

This project reads three CSVs published from Google Sheets:
- Tikkers (participants): name/password with timestamps
- Getikt (ticks per team): team index + password with timestamps
- Config (settings): relevance window and default team metadata

The browser fetches the CSVs, then calls into a Rust/WASM module to parse and return a JSON-like structure with teams and tikkers. All timestamps are handled with chrono’s `NaiveDateTime`.

## Data flow

1. `index.html` initializes the WASM module.
2. JS fetches the three CSV URLs provided by the WASM exports.
3. JS passes the CSV strings into `parse_game_data(...)`.
4. Rust parses the config (computes a cutoff timestamp), then parses tikkers and getikt rows and filters by relevance.
5. JS renders the returned `{ teams, tikkers }` object.

## Code structure

- `punten_telling_tik_spel/Cargo.toml` — WASM-ready crate config
- `punten_telling_tik_spel/src/lib.rs`
  - Exports to JS: `get_tikkers_url()`, `get_getikt_url()`, `get_config_url()`, `parse_game_data(...)`
  - Core function: `parse_game_data_core(...) -> Result<GameData, String>`
- `punten_telling_tik_spel/src/config.rs`
  - `Config`, `Relevance`, `DefaultTeam`, etc.
  - `config_from_csv(&str) -> Result<Config, Box<dyn Error>>`
  - Precomputes `cutoff_timestamp: Option<Timestamp>` based on `from` or relative `days`/`hours`.
  - `is_relevent_time_stamp(&self, tijdstempel: &str) -> Option<Timestamp>` returns Some(Timestamp) if valid and within relevance.
- `punten_telling_tik_spel/src/tikker.rs`
  - `Tikker`, `Tikkers` + parsing: `get_tikkers_from_google_sheet(csv, &Config)`
- `punten_telling_tik_spel/src/team.rs`
  - `Team`, `Teams` + parsing: `populate_teams_from_google_sheet(csv, &mut Teams, &mut Tikkers, &Config)`
- `punten_telling_tik_spel/src/location_date.rs`
  - `pub type Timestamp = chrono::NaiveDateTime`
  - `parse_tijdstempel("%d-%m-%Y %H:%M:%S") -> Option<Timestamp>`
- `punten_telling_tik_spel/src/inport_info.rs`
  - CSV loader helpers and Google Sheet URLs
- `index.html`
  - Minimal UI that imports the WASM JS glue, fetches CSVs, calls into `parse_game_data`, and renders.

## CSV formats

- Tikkers sheet (headers):
  - Columns: `Tijdstempel`, `Naam`, `Wachtwoord`
  - Timestamp format: `DD-MM-YYYY HH:MM:SS` (e.g., `19-10-2025 14:12:22`)
- Getikt sheet (headers):
  - Columns: `Tijdstempel`, `Team index`, `Wachtwoord`
- Config sheet (no headers, row-based):
  - Relevance examples:
    - `relevance_time,hour:,24`
    - `relevance,days:,7`
    - `relevance,from:,20-10-2025 13:00:00`
  - Team config example:
    - `team_config,Name:,Aligatoren,ID,416C,image,Aligatoren.png`
  - Tokens are case-insensitive; recognized for relevance: `hour[s]`, `day[s]`, `from`.

## Build (WASM)

Prerequisites:
- Rust toolchain
- wasm32 target and wasm-pack

Install once:

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

Build the crate for the web:

```bash
cd punten_telling_tik_spel
wasm-pack build --target web
```

This produces `punten_telling_tik_spel/pkg/` with the `.wasm` and JS bindings used by `index.html`.

## Tests (Rust)

Some tests fetch the live Google Sheet CSVs (network required).

```bash
cd punten_telling_tik_spel
cargo test
```

## Run locally (static hosting)

Serve the repository root via HTTP so the browser can load ES modules and WASM:

```bash
cd /root/punten_telling_tik_spel
python3 -m http.server 8080
```

Open http://localhost:8080 and the page will fetch the CSVs, parse them in WASM, and display the results.

If you hit errors:
- Ensure `punten_telling_tik_spel/pkg/` exists (build step ran).
- Ensure the Google Sheets are published and reachable.
- Check the browser console for detailed logs.

## Using from JavaScript

```js
import init, { get_tikkers_url, get_getikt_url, get_config_url, parse_game_data } from './punten_telling_tik_spel/pkg/punten_telling_tik_spel.js';

await init();

const tikkersCsv = await (await fetch(get_tikkers_url())).text();
const getiktCsv  = await (await fetch(get_getikt_url())).text();
const configCsv  = await (await fetch(get_config_url())).text();

const game = parse_game_data(tikkersCsv, getiktCsv, configCsv);
console.log(game.teams.team_list);
console.log(game.tikkers.tikker_list);
```

## Relevance logic

- `Config` precomputes `cutoff_timestamp` when parsing configuration:
  - If `from` provided: exact datetime is used.
  - Otherwise: current UTC time minus `days` and/or `hours` (via chrono `Duration`).
- A CSV timestamp is relevant if it parses and is >= `cutoff_timestamp` (or if no cutoff is set).

## Extending

- Add fields to `DefaultTeam`, `UiConfig`, etc. with serde attributes.
- Extend relevance (e.g., `until`) by updating `Relevance` and the cutoff computation.
- Add more parsing or additional sheets in `tikker.rs` and `team.rs` as needed.

## License

Add your license of choice here (e.g., MIT/Apache-2.0).
