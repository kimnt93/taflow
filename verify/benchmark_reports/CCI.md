# CommodityChannelIndex benchmark (`CCI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 48.03M | 0.019 | 52.45M | 0.051 | 2.45× | 2.68× |
| 10,000 | 0.188 | 53.18M | 0.188 | 53.13M | 0.230 | 1.22× | 1.22× |
| 100,000 | 1.882 | 53.15M | 1.813 | 55.15M | 2.021 | 1.07× | 1.11× |
| 1,000,000 | 20.161 | 49.60M | 19.212 | 52.05M | 20.375 | 1.01× | 1.06× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.875 ms**; native kernel **1.815 ms**; TA-Lib 2.022 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.365 | 0.225 | 4.45M | 2020.698 | 8988.25× | 130.29× |
| 100,000 | 10 | 2.033 | 1.078 | 9.27M | 2037.611 | 1889.77× | 27.26× |
| 100,000 | 1,000 | 24.122 | 20.355 | 49.13M | 2022.855 | 99.38× | 2.43× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 46.32M | 48.10M | 1.00× | 2.32M | 2.66M | 1.00× | 42.61M |
| 2 | 90.54M | 92.69M | 1.93× | 2.08M | 2.44M | 0.91× | 42.46M |
| 4 | 137.20M | 147.27M | 3.06× | 1.98M | 2.41M | 0.90× | 42.80M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
