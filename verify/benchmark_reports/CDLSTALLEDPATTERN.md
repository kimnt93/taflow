# CandleStalledPattern benchmark (`CDLSTALLEDPATTERN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 125.20M | 0.006 | 166.88M | 0.039 | 4.94× | 6.59× |
| 10,000 | 0.083 | 120.61M | 0.072 | 139.43M | 0.156 | 1.89× | 2.18× |
| 100,000 | 0.818 | 122.22M | 0.799 | 125.17M | 1.315 | 1.61× | 1.65× |
| 1,000,000 | 8.405 | 118.98M | 8.494 | 117.73M | 13.523 | 1.61× | 1.59× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.812 ms**; native kernel **0.800 ms**; TA-Lib 1.335 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.343 | 0.274 | 3.64M | 1308.580 | 4768.48× | 102.33× |
| 100,000 | 10 | 2.640 | 1.375 | 7.27M | 1312.202 | 954.54× | 20.47× |
| 100,000 | 1,000 | 34.940 | 31.751 | 31.49M | 1367.796 | 43.08× | 1.29× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 101.43M | 107.31M | 1.00× | 2.31M | 2.40M | 1.00× | 68.40M |
| 2 | 203.48M | 217.27M | 2.02× | 2.28M | 2.69M | 1.12× | 68.65M |
| 4 | 371.84M | 329.93M | 3.07× | 2.30M | 2.55M | 1.06× | 66.15M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
