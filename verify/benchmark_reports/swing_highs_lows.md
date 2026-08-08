# SwingHighLow benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.039 | 25.76M | 0.035 | 28.62M | nan | — | — |
| 10,000 | 0.398 | 25.11M | 0.383 | 26.09M | nan | — | — |
| 100,000 | 4.089 | 24.46M | 3.794 | 26.36M | nan | — | — |
| 1,000,000 | 49.983 | 20.01M | 39.192 | 25.52M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **3.979 ms**; native kernel **3.738 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.386 | 0.284 | 3.52M | nan | — | — |
| 100,000 | 10 | 1.945 | 1.157 | 8.64M | nan | — | — |
| 100,000 | 1,000 | 45.515 | 42.686 | 23.43M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 25.13M | 26.94M | 1.00× | 2.32M | 2.63M | 1.00× | — |
| 2 | 40.37M | 48.71M | 1.81× | 2.30M | 2.33M | 0.89× | — |
| 4 | 75.74M | 78.21M | 2.90× | 2.15M | 2.26M | 0.86× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
