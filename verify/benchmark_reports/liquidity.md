# Liquidity benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.39M | 0.033 | 30.35M | nan | — | — |
| 10,000 | 0.482 | 20.73M | 0.380 | 26.32M | nan | — | — |
| 100,000 | 4.923 | 20.31M | 4.045 | 24.72M | nan | — | — |
| 1,000,000 | 65.884 | 15.18M | 44.437 | 22.50M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **4.883 ms**; native kernel **4.010 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.319 | 0.277 | 3.61M | nan | — | — |
| 100,000 | 10 | 1.882 | 1.125 | 8.89M | nan | — | — |
| 100,000 | 1,000 | 48.487 | 42.554 | 23.50M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 22.26M | 22.81M | 1.00× | 1.93M | 2.12M | 1.00× | — |
| 2 | 29.01M | 28.71M | 1.26× | 2.26M | 2.08M | 0.98× | — |
| 4 | 43.65M | 40.57M | 1.78× | 2.21M | 2.25M | 1.06× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
