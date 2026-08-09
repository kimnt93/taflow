# MathAcosh benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 83.17M | 0.011 | 91.43M | nan | — | — |
| 10,000 | 0.101 | 98.77M | 0.098 | 102.26M | nan | — | — |
| 100,000 | 1.009 | 99.11M | 0.995 | 100.48M | nan | — | — |
| 1,000,000 | 10.821 | 92.42M | 10.357 | 96.55M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.986 ms**; native kernel **0.969 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.227 | 0.153 | 6.55M | nan | — | — |
| 100,000 | 10 | 0.929 | 0.581 | 17.22M | nan | — | — |
| 100,000 | 1,000 | 11.904 | 11.296 | 88.53M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 73.90M | 87.32M | 1.00× | 2.53M | 3.04M | 1.00× | — |
| 2 | 150.16M | 178.27M | 2.04× | 2.98M | 3.40M | 1.12× | — |
| 4 | 228.38M | 295.75M | 3.39× | 2.71M | 2.98M | 0.98× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
