# RateOfChangeRatio benchmark (`ROCR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 269.46M | 0.003 | 365.13M | 0.030 | 8.20× | 11.11× |
| 10,000 | 0.022 | 464.60M | 0.020 | 507.62M | 0.040 | 1.84× | 2.01× |
| 100,000 | 0.193 | 517.45M | 0.171 | 583.22M | 0.124 | 0.64× | 0.72× |
| 1,000,000 | 2.138 | 467.72M | 1.844 | 542.30M | 1.074 | 0.50× | 0.58× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.194 ms**; native kernel **0.172 ms**; TA-Lib 0.123 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.212 | 0.157 | 6.35M | 122.507 | 778.06× | 182.67× |
| 100,000 | 10 | 0.880 | 0.513 | 19.49M | 123.393 | 240.45× | 55.55× |
| 100,000 | 1,000 | 4.159 | 3.032 | 329.80M | 124.492 | 41.06× | 9.84× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 339.62M | 400.91M | 1.00× | 3.48M | 4.02M | 1.00× | 391.60M |
| 2 | 578.27M | 779.78M | 1.94× | 3.28M | 3.91M | 0.97× | 486.65M |
| 4 | 750.49M | 1.18G | 2.95× | 3.31M | 3.55M | 0.88× | 485.02M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
