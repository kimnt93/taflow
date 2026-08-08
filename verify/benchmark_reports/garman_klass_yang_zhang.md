# GarmanKlassYangZhang benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.025 | 39.33M | 0.024 | 42.16M | nan | — | — |
| 10,000 | 0.222 | 45.11M | 0.214 | 46.80M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.037 ms**; native kernel **0.034 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.433 | 0.314 | 3.18M | nan | — | — |
| 1,500 | 10 | 2.716 | 1.357 | 7.37M | nan | — | — |
| 1,500 | 100 | 6.228 | 4.681 | 21.36M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
