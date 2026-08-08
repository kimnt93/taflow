# CommodityChannelIndex benchmark (`CCI` oracle)

Correctness: **MISMATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.12M | 0.018 | 54.09M | 0.051 | 2.52× | 2.78× |
| 10,000 | 0.211 | 47.47M | 0.176 | 56.96M | 0.240 | 1.14× | 1.36× |
| 100,000 | 1.777 | 56.29M | 1.800 | 55.56M | 2.085 | 1.17× | 1.16× |
| 1,000,000 | 19.362 | 51.65M | 18.651 | 53.62M | 21.950 | 1.13× | 1.18× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.775 ms**; native kernel **1.710 ms**; TA-Lib 2.084 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.357 | 0.224 | 4.46M | 2023.438 | 9031.54× | 130.92× |
| 100,000 | 10 | 2.087 | 1.127 | 8.87M | 2049.968 | 1818.69× | 26.34× |
| 100,000 | 1,000 | 24.964 | 36.677 | 27.26M | 2193.204 | 59.80× | 1.38× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 49.66M | 52.20M | 1.00× | 1.96M | 2.59M | 1.00× | 39.58M |
| 2 | 83.28M | 93.59M | 1.79× | 2.12M | 2.21M | 0.85× | 40.83M |
| 4 | 153.92M | 156.69M | 3.00× | 1.84M | 2.25M | 0.87× | 40.20M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
