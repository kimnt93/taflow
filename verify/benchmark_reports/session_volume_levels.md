# SessionVolumeLevels benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.064 | 15.60M | 0.062 | 16.05M | nan | — | — |
| 10,000 | 0.686 | 14.58M | 0.710 | 14.08M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.100 ms**; native kernel **0.099 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.690 | 0.783 | 1.28M | nan | — | — |
| 1,500 | 10 | 3.220 | 2.305 | 4.34M | nan | — | — |
| 1,500 | 100 | 12.655 | 11.346 | 8.81M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
