# TripleExponentialRateOfChange benchmark (`TRIX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 213.28M | 0.004 | 269.80M | 0.039 | 8.37× | 10.59× |
| 10,000 | 0.026 | 377.47M | 0.023 | 434.19M | 0.116 | 4.37× | 5.03× |
| 100,000 | 0.235 | 426.18M | 0.218 | 459.49M | 0.886 | 3.78× | 4.07× |
| 1,000,000 | 2.554 | 391.60M | 2.171 | 460.72M | 8.961 | 3.51× | 4.13× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.241 ms**; native kernel **0.212 ms**; TA-Lib 0.913 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.206 | 0.145 | 6.91M | 889.858 | 6150.43× | 201.44× |
| 100,000 | 10 | 0.819 | 0.507 | 19.72M | 909.392 | 1793.41× | 61.92× |
| 100,000 | 1,000 | 4.697 | 3.710 | 269.56M | 928.278 | 250.23× | 10.80× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 247.73M | 333.29M | 1.00× | 3.36M | 3.34M | 1.00× | 90.89M |
| 2 | 534.32M | 597.85M | 1.79× | 3.00M | 3.86M | 1.16× | 89.51M |
| 4 | 687.36M | 1.09G | 3.26× | 2.84M | 3.17M | 0.95× | 91.25M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
