# ExponentiallyWeightedSum benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 173.23M | 0.005 | 215.88M | nan | — | — |
| 10,000 | 0.042 | 240.85M | 0.039 | 254.06M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.007 ms**; native kernel **0.007 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.282 | 0.178 | 5.62M | nan | — | — |
| 1,500 | 10 | 1.019 | 0.529 | 18.90M | nan | — | — |
| 1,500 | 100 | 2.468 | 1.955 | 51.15M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
