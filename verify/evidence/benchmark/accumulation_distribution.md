# AccumulationDistribution benchmark (`AD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 235.53M | 0.002 | 406.69M | 0.028 | 6.54× | 11.29× |
| 10,000 | 0.020 | 504.74M | 0.016 | 610.72M | 0.040 | 2.00× | 2.41× |
| 100,000 | 0.178 | 562.81M | 0.156 | 640.56M | 0.171 | 0.96× | 1.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.113 | 1.67× |
| 1 | 5 | 0.277 | 0.478 | 1.72× |
| 1 | 10 | 0.354 | 0.869 | 2.45× |
| 10 | 1 | 0.044 | 0.080 | 1.82× |
| 10 | 5 | 0.174 | 0.405 | 2.32× |
| 10 | 10 | 0.381 | 0.852 | 2.23× |
| 100 | 1 | 0.040 | 0.085 | 2.14× |
| 100 | 5 | 0.184 | 0.407 | 2.21× |
| 100 | 10 | 0.364 | 0.839 | 2.31× |
| 1,000 | 1 | 0.041 | 0.083 | 2.01× |
| 1,000 | 5 | 0.195 | 0.404 | 2.08× |
| 1,000 | 10 | 0.395 | 0.827 | 2.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
