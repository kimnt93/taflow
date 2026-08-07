# taflow verification project

Standalone [uv](https://docs.astral.sh/uv/) project that checks every taflow
function against a reference implementation and writes a Markdown report.

## Protocol (per function)

With 10,000 seeded bars:

1. **Oracle full pass** — reference library over all 10k bars
   (TA-Lib for TA-Lib-named functions; pandas for `rolling_*`/`ewm_*`;
   self-oracle when no reference exists).
2. **Warm-up / continue** — feed the first **9,000** bars into the
   persistent state (`extend`), then continue with the last **1,000** bars
   through scalar `append` calls (the live-update path). The concatenated
   9k+1k output is compared to:
   - the oracle 10k result.

TA-Lib one-shot migration checks are intentionally disabled. TAFlow has no
TA-Lib compatibility package; canonical users import native-backed CamelCase
classes from `taflow`.

Verdict is `MATCH` when NaN placement is identical and values agree within
`rtol=1e-8, atol=1e-10`; the report also records the max absolute error so
tolerance-scale drift is visible even on matches.

## Run

```bash
cd verify
uv sync              # builds taflow from the repo root via maturin
uv run python verify.py            # all functions -> REPORT.md
uv run python verify.py EMA ATR    # subset
uv run python verify.py --bars 10000 --warmup-split 9000
```

Output: `verify/REPORT.md` (summary + one row per function/output) and
`verify/report.json` (machine-readable detail).

Optional extra oracles (pandas-ta-classic, smartmoneyconcepts):

```bash
uv sync --extra extra-oracles
```
