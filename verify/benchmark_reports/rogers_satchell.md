# RogersSatchell benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.031 | 32.18M | 0.029 | 34.27M | nan | — | — |
| 10,000 | 0.275 | 36.34M | 0.271 | 36.90M | nan | — | — |
| 100,000 | 2.746 | 36.41M | 2.684 | 37.26M | nan | — | — |
| 1,000,000 | 27.239 | 36.71M | 26.937 | 37.12M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **2.767 ms**; native kernel **2.723 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.351 | 0.290 | 3.45M | nan | — | — |
| 100,000 | 10 | 2.876 | 1.425 | 7.02M | nan | — | — |
| 100,000 | 1,000 | 30.817 | 33.638 | 29.73M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 34.03M | 35.33M | 1.00× | 2.14M | 2.25M | 1.00× | — |
| 2 | 65.61M | 66.84M | 1.89× | 2.06M | 2.43M | 1.08× | — |
| 4 | 118.77M | 129.53M | 3.67× | 2.11M | 2.26M | 1.01× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
