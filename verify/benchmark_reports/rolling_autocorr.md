# RollingAutocorr benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.100 | 9.97M | 0.079 | 12.74M | nan | — | — |
| 10,000 | 0.793 | 12.61M | 0.780 | 12.82M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.121 ms**; native kernel **0.118 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.372 | 0.270 | 3.70M | nan | — | — |
| 1,500 | 10 | 1.873 | 1.362 | 7.34M | nan | — | — |
| 1,500 | 100 | 10.589 | 9.280 | 10.78M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
