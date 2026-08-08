# RollingKurtosis benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 48.48M | 0.019 | 51.29M | nan | — | — |
| 10,000 | 0.184 | 54.40M | 0.187 | 53.58M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.029 ms**; native kernel **0.028 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.484 | 0.387 | 2.58M | nan | — | — |
| 1,500 | 10 | 1.190 | 0.689 | 14.52M | nan | — | — |
| 1,500 | 100 | 4.021 | 3.965 | 25.22M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
