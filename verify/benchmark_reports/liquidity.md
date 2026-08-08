# Liquidity benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.184 | 5.42M | 0.185 | 5.41M | nan | — | — |
| 10,000 | 2.306 | 4.34M | 2.310 | 4.33M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.301 ms**; native kernel **0.287 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.544 | 0.445 | 2.25M | nan | — | — |
| 1,500 | 10 | 3.667 | 2.790 | 3.58M | nan | — | — |
| 1,500 | 100 | 23.850 | 21.875 | 4.57M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
