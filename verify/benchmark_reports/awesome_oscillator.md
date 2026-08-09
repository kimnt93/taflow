# AwesomeOscillator benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.030 | 33.62M | 0.028 | 35.72M | nan | — | — |
| 10,000 | 0.271 | 36.86M | 0.356 | 28.13M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.043 ms**; native kernel **0.042 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.292 | 0.233 | 4.30M | nan | — | — |
| 1,500 | 10 | 1.747 | 1.288 | 7.76M | nan | — | — |
| 1,500 | 100 | 5.484 | 4.205 | 23.78M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.71M | 8.16M | 1.00× | 1.03M | 1.34M | 1.00× | — |
| 2 | 16.41M | 14.63M | 1.79× | 1.38M | 1.29M | 0.96× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
