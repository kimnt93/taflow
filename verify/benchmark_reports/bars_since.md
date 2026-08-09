# BarsSince benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 273.91M | 0.003 | 337.97M | nan | — | — |
| 10,000 | 0.025 | 405.49M | 0.023 | 439.13M | nan | — | — |
| 100,000 | 0.242 | 414.06M | 0.216 | 463.99M | nan | — | — |
| 1,000,000 | 2.432 | 411.14M | 2.223 | 449.81M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.242 ms**; native kernel **0.220 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.310 | 0.240 | 4.17M | nan | — | — |
| 100,000 | 10 | 0.621 | 0.480 | 20.85M | nan | — | — |
| 100,000 | 1,000 | 3.707 | 5.373 | 186.12M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 248.81M | 311.02M | 1.00× | 2.49M | 2.44M | 1.00× | — |
| 2 | 527.94M | 485.89M | 1.56× | 2.61M | 2.74M | 1.13× | — |
| 4 | 651.36M | 1.07G | 3.43× | 2.74M | 2.87M | 1.18× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
