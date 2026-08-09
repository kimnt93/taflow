# Parkinson benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.92M | 0.017 | 58.35M | nan | — | — |
| 10,000 | 0.179 | 55.96M | 0.142 | 70.34M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.027 ms**; native kernel **0.024 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.288 | 0.211 | 4.74M | nan | — | — |
| 1,500 | 10 | 2.073 | 0.905 | 11.05M | nan | — | — |
| 1,500 | 100 | 3.719 | 2.876 | 34.77M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.76M | 11.95M | 1.00× | 1.16M | 1.14M | 1.00× | — |
| 2 | 11.90M | 15.61M | 1.31× | 1.16M | 1.45M | 1.27× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
