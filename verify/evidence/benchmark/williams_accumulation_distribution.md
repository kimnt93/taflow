# WilliamsAccumulationDistribution benchmark (`Wad` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 137.72M | 0.005 | 204.21M | 0.199 | 27.44× | 40.68× |
| 10,000 | 0.066 | 151.90M | 0.059 | 168.49M | 1.092 | 16.59× | 18.40× |
| 100,000 | 0.636 | 157.32M | 0.611 | 163.53M | 9.637 | 15.16× | 15.76× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.101 | 0.206 | 2.04× |
| 1 | 5 | 0.303 | 1.191 | 3.94× |
| 1 | 10 | 0.409 | 2.080 | 5.09× |
| 10 | 1 | 0.047 | 0.174 | 3.73× |
| 10 | 5 | 0.189 | 0.779 | 4.12× |
| 10 | 10 | 0.409 | 1.956 | 4.78× |
| 100 | 1 | 0.048 | 0.180 | 3.79× |
| 100 | 5 | 0.205 | 0.853 | 4.15× |
| 100 | 10 | 0.403 | 2.113 | 5.25× |
| 1,000 | 1 | 0.051 | 0.267 | 5.23× |
| 1,000 | 5 | 0.199 | 1.289 | 6.47× |
| 1,000 | 10 | 0.478 | 2.645 | 5.53× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
