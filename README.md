# Block Gauge Finder

I had develop this project to solve a problem i often find myself into. Sometimes when i have to make a measure some blocks are missing and i takes long time to calculate other combination by hand so i created this solution.

Currently only inches is supported and by default the 81 pieces mitutoyo block gauge is setted.

## Usage

Compile the project

```bash
gcc main.c -o bkg
```

Run

```bash
./bkg <measure-in-inches> -<flags>
```

"-ex <measure>" flag can be used to exclude missing blocks
"-max <int>" flag can be used to change the maximum number of combination to find

## How it work

Recursively find possible combination for a given measure by default 2 each combination found will exclude the blocks used in the previous ones. There is an upper bound if the recursion is going to far.

## Future improvements

- Add Metric support;
- Add different block gauges sets support;

## Implementations

- `main.c` and `bkg.h` are the original C/native implementation.
- `src/` contains the Rust implementation used by the CLI, tests, benchmarks,
  and browser build.
- `web/` contains a small GitHub Pages frontend. The generated `web/pkg/`
  WebAssembly package is built by CI with `wasm-pack`.

## WebAssembly

Build the browser package locally:

```bash
wasm-pack build --target web --out-dir web/pkg --release
```

Then serve `web/` with any static file server:

```bash
cd web
python3 -m http.server 4176 --bind 127.0.0.1
```

Open `http://127.0.0.1:4176/`. The web app includes a PWA manifest and service
worker, so supported mobile browsers can install it after the first load.
Pushing to `main` runs `.github/workflows/pages.yml` and deploys the same static
app to GitHub Pages.
