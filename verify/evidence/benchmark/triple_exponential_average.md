# TripleExponentialAverage benchmark (`T3` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 131.68M | 0.008 | 132.81M | 0.040 | 5.31× | 5.36× |
| 10,000 | 0.042 | 236.97M | 0.039 | 254.48M | 0.086 | 2.03× | 2.18× |
| 100,000 | 0.415 | 240.91M | 0.365 | 273.78M | 0.449 | 1.08× | 1.23× |
| 1,000,000 | 4.474 | 223.49M | 3.771 | 265.21M | 4.212 | 0.94× | 1.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.064 | 0.126 | 1.98× |
| 1 | 5 | 0.272 | 0.478 | 1.76× |
| 1 | 10 | 0.488 | 0.946 | 1.94× |
| 10 | 1 | 0.055 | 0.103 | 1.86× |
| 10 | 5 | 0.251 | 0.481 | 1.91× |
| 10 | 10 | 0.478 | 0.962 | 2.01× |
| 100 | 1 | 0.046 | 0.090 | 1.95× |
| 100 | 5 | 0.238 | 0.500 | 2.10× |
| 100 | 10 | 0.533 | 1.015 | 1.90× |
| 1,000 | 1 | 0.051 | 0.102 | 1.98× |
| 1,000 | 5 | 0.243 | 0.480 | 1.97× |
| 1,000 | 10 | 0.527 | 1.077 | 2.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
