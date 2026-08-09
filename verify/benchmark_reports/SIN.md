# MathSin benchmark (`SIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 84.47M | 0.011 | 94.13M | 0.036 | 3.06× | 3.41× |
| 10,000 | 0.152 | 65.95M | 0.147 | 68.12M | 0.171 | 1.13× | 1.17× |
| 100,000 | 1.490 | 67.13M | 1.499 | 66.72M | 1.483 | 1.00× | 0.99× |
| 1,000,000 | 15.861 | 63.05M | 16.158 | 61.89M | 14.825 | 0.93× | 0.92× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.494 ms**; native kernel **1.470 ms**; TA-Lib 1.469 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.242 | 0.165 | 6.06M | 1479.349 | 8967.49× | 148.19× |
| 100,000 | 10 | 1.038 | 0.693 | 14.43M | 1467.753 | 2118.62× | 36.33× |
| 100,000 | 1,000 | 16.783 | 16.331 | 61.23M | 1483.518 | 90.84× | 2.24× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 55.69M | 56.90M | 1.00× | 2.28M | 2.55M | 1.00× | 59.02M |
| 2 | 108.32M | 115.97M | 2.04× | 2.75M | 3.25M | 1.28× | 58.86M |
| 4 | 164.55M | 210.88M | 3.71× | 2.52M | 2.81M | 1.10× | 58.14M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
