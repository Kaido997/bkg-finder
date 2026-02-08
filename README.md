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
