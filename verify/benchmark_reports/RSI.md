# RelativeStrengthIndex benchmark (`RSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 87.81M | 0.010 | 97.30M | 0.040 | 3.48× | 3.85× |
| 10,000 | 0.084 | 119.02M | 0.081 | 123.10M | 0.092 | 1.09× | 1.13× |
| 100,000 | 0.827 | 120.96M | 0.779 | 128.33M | 0.673 | 0.81× | 0.86× |
| 1,000,000 | 8.307 | 120.39M | 7.833 | 127.67M | 5.840 | 0.70× | 0.75× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.076 | 0.134 | 1.76× |
| 1 | 5 | 0.248 | 0.521 | 2.10× |
| 1 | 10 | 0.472 | 0.948 | 2.01× |
| 10 | 1 | 0.053 | 0.093 | 1.77× |
| 10 | 5 | 0.234 | 0.446 | 1.91× |
| 10 | 10 | 0.471 | 0.937 | 1.99× |
| 100 | 1 | 0.051 | 0.097 | 1.91× |
| 100 | 5 | 0.230 | 0.469 | 2.04× |
| 100 | 10 | 0.509 | 0.962 | 1.89× |
| 1,000 | 1 | 0.057 | 0.099 | 1.72× |
| 1,000 | 5 | 0.236 | 0.482 | 2.04× |
| 1,000 | 10 | 0.518 | 1.074 | 2.07× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | Reference full µs | vs full | vs bounded tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.242 | 0.182 | 5.49M | 617.028 | 3385.21× | 175.19× |
| 100,000 | 10 | 1.021 | 0.554 | 18.04M | 605.923 | 1093.28× | 56.08× |
| 100,000 | 1,000 | 14.112 | 8.780 | 113.90M | 636.156 | 72.46× | 4.44× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | Reference vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 94.01M | 105.64M | 1.00× | 2.85M | 3.39M | 1.00× | 128.11M |
| 5 | 285.95M | 416.21M | 3.94× | 2.68M | 3.09M | 0.91× | 134.89M |
| 10 | 409.08M | 632.42M | 5.99× | 2.86M | 2.63M | 0.78× | 137.84M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
