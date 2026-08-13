# AccumulationDistribution benchmark (`AD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.053 | 18.90M | 0.044 | 22.62M | 0.029 | 0.54× | 0.64× |
| 10,000 | 0.341 | 29.33M | 0.330 | 30.31M | 0.040 | 0.12× | 0.12× |
| 100,000 | 3.181 | 31.43M | 3.157 | 31.67M | 0.159 | 0.05× | 0.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.152 | 0.146 | 0.96× |
| 1 | 5 | 0.491 | 0.493 | 1.00× |
| 1 | 10 | 0.623 | 0.899 | 1.44× |
| 10 | 1 | 0.078 | 0.090 | 1.15× |
| 10 | 5 | 0.307 | 0.426 | 1.39× |
| 10 | 10 | 0.637 | 0.868 | 1.36× |
| 100 | 1 | 0.069 | 0.088 | 1.28× |
| 100 | 5 | 0.320 | 0.412 | 1.29× |
| 100 | 10 | 0.640 | 0.898 | 1.40× |
| 1,000 | 1 | 0.108 | 0.092 | 0.85× |
| 1,000 | 5 | 0.302 | 0.416 | 1.38× |
| 1,000 | 10 | 0.651 | 0.917 | 1.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
