# MathExp benchmark (`EXP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 122.53M | 0.007 | 135.99M | 0.035 | 4.26× | 4.72× |
| 10,000 | 0.054 | 185.11M | 0.052 | 191.54M | 0.077 | 1.43× | 1.48× |
| 100,000 | 0.535 | 186.76M | 0.506 | 197.56M | 0.543 | 1.01× | 1.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.120 | 1.36× |
| 1 | 5 | 0.244 | 0.454 | 1.86× |
| 1 | 10 | 0.469 | 0.917 | 1.95× |
| 10 | 1 | 0.049 | 0.085 | 1.72× |
| 10 | 5 | 0.289 | 0.500 | 1.73× |
| 10 | 10 | 0.466 | 0.873 | 1.87× |
| 100 | 1 | 0.048 | 0.086 | 1.78× |
| 100 | 5 | 0.207 | 0.412 | 1.99× |
| 100 | 10 | 0.559 | 0.898 | 1.61× |
| 1,000 | 1 | 0.053 | 0.093 | 1.77× |
| 1,000 | 5 | 0.221 | 0.430 | 1.94× |
| 1,000 | 10 | 0.490 | 0.999 | 2.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
