# ExponentiallyWeightedSum benchmark (`exponentially weighted sum` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 159.55M | 0.006 | 177.36M | 0.196 | 31.23× | 34.71× |
| 10,000 | 0.037 | 268.07M | 0.035 | 287.78M | 1.722 | 46.15× | 49.54× |
| 100,000 | 0.332 | 301.46M | 0.318 | 314.79M | 17.206 | 51.87× | 54.16× |
| 1,000,000 | 3.433 | 291.28M | 3.110 | 321.54M | 167.332 | 48.74× | 53.80× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.119 | 1.21× |
| 1 | 5 | 0.344 | 0.430 | 1.25× |
| 1 | 10 | 0.463 | 0.855 | 1.85× |
| 10 | 1 | 0.047 | 0.093 | 1.97× |
| 10 | 5 | 0.211 | 0.407 | 1.93× |
| 10 | 10 | 0.457 | 0.880 | 1.92× |
| 100 | 1 | 0.051 | 0.104 | 2.05× |
| 100 | 5 | 0.236 | 0.493 | 2.09× |
| 100 | 10 | 0.477 | 1.009 | 2.12× |
| 1,000 | 1 | 0.064 | 0.256 | 4.01× |
| 1,000 | 5 | 0.233 | 1.330 | 5.72× |
| 1,000 | 10 | 0.474 | 2.786 | 5.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
