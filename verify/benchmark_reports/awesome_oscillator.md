# AwesomeOscillator benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.025 | 40.42M | 0.023 | 43.00M | nan | — | — |
| 10,000 | 0.229 | 43.67M | 0.223 | 44.90M | nan | — | — |
| 100,000 | 2.370 | 42.19M | 2.248 | 44.49M | nan | — | — |
| 1,000,000 | 22.953 | 43.57M | 22.356 | 44.73M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **2.243 ms**; native kernel **2.234 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.251 | 0.215 | 4.66M | nan | — | — |
| 100,000 | 10 | 1.694 | 0.884 | 11.31M | nan | — | — |
| 100,000 | 1,000 | 25.703 | 23.893 | 41.85M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 38.02M | 40.12M | 1.00× | 2.41M | 2.38M | 1.00× | — |
| 2 | 69.93M | 74.09M | 1.85× | 2.89M | 2.86M | 1.20× | — |
| 4 | 98.99M | 122.46M | 3.05× | 2.53M | 2.70M | 1.14× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
