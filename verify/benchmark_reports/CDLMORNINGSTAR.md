# CandleMorningStar benchmark (`CDLMORNINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 148.61M | 0.005 | 207.62M | 0.038 | 5.63× | 7.86× |
| 10,000 | 0.084 | 119.54M | 0.079 | 126.00M | 0.115 | 1.38× | 1.45× |
| 100,000 | 0.876 | 114.09M | 0.856 | 116.79M | 0.940 | 1.07× | 1.10× |
| 1,000,000 | 9.382 | 106.59M | 9.958 | 100.42M | 8.645 | 0.92× | 0.87× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.887 ms**; native kernel **0.916 ms**; TA-Lib 0.903 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.368 | 0.294 | 3.40M | 888.645 | 3020.55× | 118.64× |
| 100,000 | 10 | 4.364 | 2.696 | 3.71M | 885.693 | 328.55× | 12.15× |
| 100,000 | 1,000 | 43.264 | 38.272 | 26.13M | 906.400 | 23.68× | 1.07× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 99.10M | 102.01M | 1.00× | 2.29M | 2.31M | 1.00× | 95.82M |
| 2 | 190.10M | 187.34M | 1.84× | 2.38M | 2.51M | 1.09× | 98.09M |
| 4 | 346.17M | 371.18M | 3.64× | 2.40M | 2.53M | 1.09× | 100.54M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
