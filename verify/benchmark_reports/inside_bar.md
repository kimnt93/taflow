# InsideBar benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 183.99M | 0.004 | 255.43M | nan | — | — |
| 10,000 | 0.035 | 288.61M | 0.031 | 320.68M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.007 ms**; native kernel **0.005 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.296 | 0.207 | 4.82M | nan | — | — |
| 1,500 | 10 | 1.506 | 0.727 | 13.75M | nan | — | — |
| 1,500 | 100 | 2.873 | 2.114 | 47.30M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
