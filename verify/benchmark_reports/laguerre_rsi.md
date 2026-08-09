# LaguerreRelativeStrengthIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 89.72M | 0.010 | 97.45M | 0.106 | 9.48× | 10.30× |
| 10,000 | 0.085 | 117.20M | 0.084 | 119.49M | 0.229 | 2.69× | 2.74× |
| 100,000 | 0.834 | 119.86M | 0.776 | 128.86M | 1.543 | 1.85× | 1.99× |
| 1,000,000 | 12.789 | 78.19M | 7.773 | 128.65M | 45.508 | 3.56× | 5.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.222 | 3.28× |
| 1 | 5 | 0.246 | 0.912 | 3.71× |
| 1 | 10 | 0.516 | 1.680 | 3.26× |
| 10 | 1 | 0.050 | 0.157 | 3.13× |
| 10 | 5 | 0.225 | 0.783 | 3.48× |
| 10 | 10 | 0.509 | 1.670 | 3.28× |
| 100 | 1 | 0.049 | 0.162 | 3.29× |
| 100 | 5 | 0.230 | 0.787 | 3.42× |
| 100 | 10 | 0.548 | 1.679 | 3.07× |
| 1,000 | 1 | 0.058 | 0.182 | 3.12× |
| 1,000 | 5 | 0.235 | 1.129 | 4.81× |
| 1,000 | 10 | 0.488 | 2.701 | 5.54× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | Reference full µs | vs full |
|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.300 | 0.194 | 5.16M | 1542.354 | 7966.19× |
| 100,000 | 10 | 1.215 | 0.831 | 12.04M | 1581.270 | 1903.17× |
| 100,000 | 1,000 | 35.228 | 42.431 | 23.57M | 1558.378 | 36.73× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | Reference vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 82.23M | 111.07M | 1.00× | 2.84M | 3.04M | 1.00× | 35.39M |
| 5 | 175.09M | 212.97M | 1.92× | 2.43M | 2.37M | 0.78× | 45.96M |
| 10 | 269.77M | 301.82M | 2.72× | 2.22M | 2.37M | 0.78× | 41.02M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
