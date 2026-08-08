# AwesomeOscillator benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.026 | 38.15M | 0.025 | 40.12M | nan | — | — |
| 10,000 | 0.246 | 40.61M | 0.240 | 41.69M | nan | — | — |
| 100,000 | 2.451 | 40.80M | 2.857 | 35.01M | nan | — | — |
| 1,000,000 | 25.985 | 38.48M | 25.957 | 38.53M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **2.474 ms**; native kernel **2.390 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.307 | 0.236 | 4.24M | nan | — | — |
| 100,000 | 10 | 1.678 | 0.937 | 10.67M | nan | — | — |
| 100,000 | 1,000 | 26.337 | 25.331 | 39.48M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 37.68M | 36.37M | 1.00× | 2.37M | 2.53M | 1.00× | — |
| 2 | 72.23M | 73.55M | 2.02× | 2.48M | 2.70M | 1.07× | — |
| 4 | 131.81M | 116.48M | 3.20× | 2.53M | 2.59M | 1.03× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
