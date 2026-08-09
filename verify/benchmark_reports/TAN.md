# MathTan benchmark (`TAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 54.48M | 0.017 | 57.45M | 0.044 | 2.37× | 2.50× |
| 10,000 | 0.203 | 49.27M | 0.197 | 50.88M | 0.218 | 1.08× | 1.11× |
| 100,000 | 2.004 | 49.91M | 1.956 | 51.12M | 1.928 | 0.96× | 0.99× |
| 1,000,000 | 20.429 | 48.95M | 20.081 | 49.80M | 19.049 | 0.93× | 0.95× |

## Warm-up

Construct + canonical extend over 100,000 bars: **2.016 ms**; native kernel **1.969 ms**; TA-Lib 1.933 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.238 | 0.170 | 5.88M | 1911.720 | 11240.10× | 149.69× |
| 100,000 | 10 | 1.077 | 0.690 | 14.49M | 1938.607 | 2809.07× | 37.24× |
| 100,000 | 1,000 | 23.427 | 22.154 | 45.14M | 1976.291 | 89.21× | 1.94× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 44.14M | 45.55M | 1.00× | 2.78M | 3.52M | 1.00× | 44.07M |
| 2 | 83.04M | 88.96M | 1.95× | 2.95M | 3.41M | 0.97× | 46.67M |
| 4 | 143.47M | 146.09M | 3.21× | 2.63M | 2.77M | 0.79× | 45.25M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
