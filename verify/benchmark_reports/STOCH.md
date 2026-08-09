# StochasticOscillator benchmark (`STOCH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 73.61M | 0.013 | 75.37M | 0.050 | 3.70× | 3.79× |
| 10,000 | 0.137 | 72.73M | 0.139 | 72.00M | 0.160 | 1.16× | 1.15× |
| 100,000 | 1.358 | 73.64M | 1.327 | 75.37M | 1.171 | 0.86× | 0.88× |
| 1,000,000 | 15.263 | 65.52M | 14.873 | 67.24M | 11.820 | 0.77× | 0.79× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.316 ms**; native kernel **1.299 ms**; TA-Lib 1.161 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.392 | 0.322 | 3.10M | 1179.196 | 3660.43× | 129.24× |
| 100,000 | 10 | 2.218 | 1.812 | 5.52M | 1153.230 | 636.52× | 23.26× |
| 100,000 | 1,000 | 97.938 | 91.954 | 10.88M | 1188.829 | 12.93× | 0.54× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 57.19M | 65.90M | 1.00× | 1.65M | 1.78M | 1.00× | 63.91M |
| 2 | 111.43M | 127.04M | 1.93× | 1.74M | 1.97M | 1.11× | 70.17M |
| 4 | 171.99M | 197.35M | 2.99× | 1.81M | 1.82M | 1.03× | 68.16M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
