# SuperSmoother benchmark (`SuperSmoother` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 127.75M | 0.007 | 141.04M | 0.139 | 17.72× | 19.56× |
| 10,000 | 0.049 | 204.81M | 0.048 | 207.55M | 0.452 | 9.25× | 9.37× |
| 100,000 | 0.514 | 194.73M | 0.433 | 230.73M | 3.224 | 6.28× | 7.44× |
| 1,000,000 | 4.689 | 213.25M | 4.255 | 235.03M | 33.103 | 7.06× | 7.78× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.227 | 2.43× |
| 1 | 5 | 0.263 | 0.991 | 3.78× |
| 1 | 10 | 0.495 | 2.049 | 4.14× |
| 10 | 1 | 0.052 | 0.189 | 3.61× |
| 10 | 5 | 0.216 | 0.917 | 4.25× |
| 10 | 10 | 0.479 | 2.045 | 4.27× |
| 100 | 1 | 0.053 | 0.193 | 3.65× |
| 100 | 5 | 0.218 | 0.932 | 4.27× |
| 100 | 10 | 0.515 | 2.139 | 4.15× |
| 1,000 | 1 | 0.059 | 0.220 | 3.76× |
| 1,000 | 5 | 0.239 | 1.110 | 4.65× |
| 1,000 | 10 | 0.503 | 2.414 | 4.80× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
