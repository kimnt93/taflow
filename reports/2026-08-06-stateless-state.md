# Stateless state API validation — 2026-08-06

This report covers TA-Lib functions whose output depends only on the current
bar. Every function has a Rust state type and a Python class with `append`,
`extend`, `value`, and `reset`. There is no warm-up period.

## Verification

| Scope | Method | Series size | Result |
|---|---|---:|---|
| 15 math transforms | exact Python array equality with batch API; append replay | 128 each | pass |
| ADD, SUB, MULT, DIV | exact Python array equality with batch API; append replay | 128 each | pass |
| four price transforms | exact Python array equality with batch API; append replay | 128 each | pass |
| Rust state layer | per-bar equality with Rust batch functions | 40 each | pass |
| Complete batch oracle plus state suite | pytest | 288 cases | pass |

The Rust test deliberately checks exact floating-point equality. It detected
and prevented expression-order drift in TYPPRICE and WCLPRICE.

## One-million-update benchmark

Criterion `--quick`, release build, after a 10,000-value initialization pass.
Times include initialization. Values below are the observed total ranges.

| Function | Computation | Total time |
|---|---|---:|
| ACOS | `acos(x)` | 6.15–6.16 ms |
| ASIN | `asin(x)` | 6.39–6.53 ms |
| ATAN | `atan(x)` | 5.97–6.19 ms |
| CEIL | `ceil(x)` | 2.09–2.10 ms |
| COS | `cos(x)` | 10.32–10.53 ms |
| COSH | `cosh(x)` | 6.02–6.05 ms |
| EXP | `exp(x)` | 5.72–5.78 ms |
| FLOOR | `floor(x)` | 2.09–2.12 ms |
| LN | `ln(x)` | 3.99–4.04 ms |
| LOG10 | `log10(x)` | 8.13–8.18 ms |
| SIN | `sin(x)` | 9.87–9.89 ms |
| SINH | `sinh(x)` | 7.33–7.42 ms |
| SQRT | `sqrt(x)` | 1.39–1.42 ms |
| TAN | `tan(x)` | 13.35–13.46 ms |
| TANH | `tanh(x)` | 2.37–2.38 ms |
| ADD | `a + b` | 453–477 µs |
| SUB | `a - b` | 462–472 µs |
| MULT | `a * b` | 476–487 µs |
| DIV | `a / b` | 0.995–1.022 ms |
| AVGPRICE | `(O+H+L+C)/4` | 862–881 µs |
| MEDPRICE | `(H+L)/2` | 489–494 µs |
| TYPPRICE | `(H+L+C)/3` | 719–724 µs |
| WCLPRICE | `(H+L+C+C)/4` | 857–864 µs |

These are Rust-core throughput measurements. They do not claim that one Python
call per bar has the same cost; `extend` is the intended Python backfill path.
