# RollingCov benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 46.86M | 0.020 | 50.41M | nan | — | — |
| 10,000 | 0.193 | 51.82M | 0.193 | 51.84M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.031 ms**; native kernel **0.030 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.343 | 0.246 | 4.06M | nan | — | — |
| 1,500 | 10 | 1.718 | 0.934 | 10.70M | nan | — | — |
| 1,500 | 100 | 4.725 | 3.595 | 27.82M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
