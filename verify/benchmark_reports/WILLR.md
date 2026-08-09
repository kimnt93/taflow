# WilliamsPercentR benchmark (`WILLR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 123.69M | 0.007 | 148.77M | 0.035 | 4.28× | 5.15× |
| 10,000 | 0.083 | 120.37M | 0.078 | 128.51M | 0.110 | 1.32× | 1.41× |
| 100,000 | 0.827 | 120.86M | 0.793 | 126.06M | 0.802 | 0.97× | 1.01× |
| 1,000,000 | 9.448 | 105.84M | 8.918 | 112.13M | 7.960 | 0.84× | 0.89× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.826 ms**; native kernel **0.791 ms**; TA-Lib 0.783 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.336 | 0.233 | 4.29M | 813.278 | 3493.01× | 124.18× |
| 100,000 | 10 | 2.148 | 1.155 | 8.66M | 797.257 | 690.42× | 25.38× |
| 100,000 | 1,000 | 28.458 | 26.800 | 37.31M | 800.164 | 29.86× | 1.33× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 98.40M | 100.49M | 1.00× | 2.07M | 2.41M | 1.00× | 102.60M |
| 2 | 186.37M | 203.44M | 2.02× | 2.10M | 2.45M | 1.02× | 101.86M |
| 4 | 273.10M | 362.62M | 3.61× | 2.11M | 2.46M | 1.02× | 101.68M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
