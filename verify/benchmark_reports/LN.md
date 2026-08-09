# MathLn benchmark (`LN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 125.15M | 0.007 | 145.42M | 0.034 | 4.27× | 4.97× |
| 10,000 | 0.050 | 198.06M | 0.045 | 224.53M | 0.070 | 1.38× | 1.57× |
| 100,000 | 0.447 | 223.75M | 0.432 | 231.74M | 0.430 | 0.96× | 1.00× |
| 1,000,000 | 4.731 | 211.38M | 4.530 | 220.75M | 4.787 | 1.01× | 1.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.102 | 1.15× |
| 1 | 5 | 0.276 | 0.556 | 2.01× |
| 1 | 10 | 0.501 | 0.966 | 1.93× |
| 10 | 1 | 0.056 | 0.089 | 1.57× |
| 10 | 5 | 0.218 | 0.429 | 1.97× |
| 10 | 10 | 0.536 | 1.022 | 1.91× |
| 100 | 1 | 0.049 | 0.088 | 1.79× |
| 100 | 5 | 0.235 | 0.419 | 1.78× |
| 100 | 10 | 0.553 | 0.990 | 1.79× |
| 1,000 | 1 | 0.055 | 0.096 | 1.74× |
| 1,000 | 5 | 0.245 | 0.493 | 2.01× |
| 1,000 | 10 | 0.529 | 0.988 | 1.87× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.319 | 0.163 | 6.14M | 416.099 | 2555.16× | 157.80× |
| 100,000 | 10 | 1.013 | 0.541 | 18.48M | 430.571 | 795.58× | 50.03× |
| 100,000 | 1,000 | 6.772 | 5.859 | 170.67M | 433.194 | 73.93× | 5.45× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 181.69M | 148.10M | 1.00× | 2.51M | 3.50M | 1.00× | 171.57M |
| 5 | 474.10M | 562.97M | 3.80× | 2.33M | 3.07M | 0.88× | 193.34M |
| 10 | 572.09M | 827.43M | 5.59× | 1.89M | 2.57M | 0.73× | 189.68M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
