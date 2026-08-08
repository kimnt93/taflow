# ArnaudLegouxMovingAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 78.84M | 0.012 | 85.92M | nan | — | — |
| 10,000 | 0.108 | 92.86M | 0.105 | 95.56M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.018 ms**; native kernel **0.017 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.280 | 0.195 | 5.12M | nan | — | — |
| 1,500 | 10 | 1.143 | 0.623 | 16.05M | nan | — | — |
| 1,500 | 100 | 3.241 | 2.596 | 38.52M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
