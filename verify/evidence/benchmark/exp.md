# MathExp benchmark (`EXP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 157.83M | 0.006 | 180.67M | 0.031 | 4.96× | 5.67× |
| 10,000 | 0.052 | 191.72M | 0.050 | 200.10M | 0.073 | 1.41× | 1.47× |
| 100,000 | 0.506 | 197.63M | 0.489 | 204.63M | 0.491 | 0.97× | 1.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.104 | 1.43× |
| 1 | 5 | 0.298 | 0.427 | 1.43× |
| 1 | 10 | 0.382 | 0.911 | 2.38× |
| 10 | 1 | 0.047 | 0.104 | 2.24× |
| 10 | 5 | 0.208 | 0.439 | 2.11× |
| 10 | 10 | 0.390 | 0.875 | 2.25× |
| 100 | 1 | 0.044 | 0.082 | 1.88× |
| 100 | 5 | 0.214 | 0.542 | 2.54× |
| 100 | 10 | 0.416 | 0.961 | 2.31× |
| 1,000 | 1 | 0.048 | 0.087 | 1.79× |
| 1,000 | 5 | 0.187 | 0.430 | 2.30× |
| 1,000 | 10 | 0.480 | 0.970 | 2.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
