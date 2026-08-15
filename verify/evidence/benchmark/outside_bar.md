# OutsideBar benchmark (`outside bar relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 200.31M | 0.004 | 268.82M | 0.023 | 4.55× | 6.10× |
| 10,000 | 0.031 | 325.02M | 0.028 | 363.46M | 0.042 | 1.35× | 1.51× |
| 100,000 | 0.276 | 362.71M | 0.253 | 394.68M | 0.229 | 0.83× | 0.90× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.111 | 0.104 | 0.93× |
| 1 | 5 | 0.254 | 0.370 | 1.46× |
| 1 | 10 | 0.401 | 0.745 | 1.86× |
| 10 | 1 | 0.040 | 0.076 | 1.89× |
| 10 | 5 | 0.184 | 0.354 | 1.92× |
| 10 | 10 | 0.406 | 0.814 | 2.00× |
| 100 | 1 | 0.052 | 0.084 | 1.62× |
| 100 | 5 | 0.212 | 0.387 | 1.82× |
| 100 | 10 | 0.416 | 0.765 | 1.84× |
| 1,000 | 1 | 0.044 | 0.079 | 1.79× |
| 1,000 | 5 | 0.187 | 0.523 | 2.79× |
| 1,000 | 10 | 0.430 | 1.139 | 2.65× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
