# RollingSharpe benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 48.10M | 0.020 | 50.69M | nan | — | — |
| 10,000 | 0.184 | 54.20M | 0.184 | 54.31M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.030 ms**; native kernel **0.029 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.931 | 0.209 | 4.78M | nan | — | — |
| 1,500 | 10 | 1.167 | 0.652 | 15.33M | nan | — | — |
| 1,500 | 100 | 3.875 | 3.080 | 32.47M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
