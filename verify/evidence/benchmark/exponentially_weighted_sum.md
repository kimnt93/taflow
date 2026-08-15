# ExponentiallyWeightedSum benchmark (`exponentially weighted sum` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 176.45M | 0.005 | 185.85M | 0.210 | 37.07× | 39.04× |
| 10,000 | 0.037 | 270.48M | 0.034 | 293.84M | 1.748 | 47.29× | 51.38× |
| 100,000 | 0.354 | 282.39M | 0.332 | 301.33M | 17.431 | 49.22× | 52.53× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.111 | 1.34× |
| 1 | 5 | 0.303 | 0.438 | 1.45× |
| 1 | 10 | 0.378 | 0.882 | 2.33× |
| 10 | 1 | 0.043 | 0.093 | 2.19× |
| 10 | 5 | 0.187 | 0.422 | 2.26× |
| 10 | 10 | 0.380 | 0.878 | 2.31× |
| 100 | 1 | 0.042 | 0.102 | 2.45× |
| 100 | 5 | 0.190 | 0.550 | 2.90× |
| 100 | 10 | 0.429 | 1.011 | 2.35× |
| 1,000 | 1 | 0.047 | 0.266 | 5.63× |
| 1,000 | 5 | 0.193 | 1.306 | 6.78× |
| 1,000 | 10 | 0.439 | 2.609 | 5.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
