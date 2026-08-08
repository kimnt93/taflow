# CommodityChannelIndex benchmark (`CCI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.52M | 0.020 | 49.04M | 0.053 | 2.40× | 2.58× |
| 10,000 | 0.194 | 51.57M | 0.200 | 49.95M | 0.250 | 1.29× | 1.25× |
| 100,000 | 1.934 | 51.72M | 1.944 | 51.44M | 2.282 | 1.18× | 1.17× |
| 1,000,000 | 20.738 | 48.22M | 20.079 | 49.80M | 21.328 | 1.03× | 1.06× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.927 ms**; native kernel **1.962 ms**; TA-Lib 2.176 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.340 | 0.247 | 4.06M | 2152.325 | 8728.26× | 125.94× |
| 100,000 | 10 | 2.153 | 1.139 | 8.78M | 2119.045 | 1860.21× | 26.93× |
| 100,000 | 1,000 | 23.979 | 23.646 | 42.29M | 2201.421 | 93.10× | 2.22× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 41.66M | 45.90M | 1.00× | 2.16M | 2.33M | 1.00× | 41.47M |
| 2 | 79.32M | 88.15M | 1.92× | 2.00M | 2.65M | 1.14× | 39.40M |
| 4 | 151.80M | 152.65M | 3.33× | 1.90M | 2.29M | 0.99× | 42.14M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
