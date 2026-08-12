# MinusDirectionalMovement benchmark (`MINUS_DM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 108.43M | 0.008 | 122.26M | 0.037 | 4.01× | 4.52× |
| 10,000 | 0.057 | 176.69M | 0.058 | 173.28M | 0.086 | 1.52× | 1.49× |
| 100,000 | 0.541 | 184.89M | 0.496 | 201.44M | 0.567 | 1.05× | 1.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.105 | 0.111 | 1.06× |
| 1 | 5 | 0.247 | 0.503 | 2.04× |
| 1 | 10 | 0.463 | 0.931 | 2.01× |
| 10 | 1 | 0.051 | 0.088 | 1.73× |
| 10 | 5 | 0.242 | 0.473 | 1.95× |
| 10 | 10 | 0.480 | 0.975 | 2.03× |
| 100 | 1 | 0.055 | 0.093 | 1.70× |
| 100 | 5 | 0.222 | 0.449 | 2.02× |
| 100 | 10 | 0.503 | 0.965 | 1.92× |
| 1,000 | 1 | 0.053 | 0.098 | 1.85× |
| 1,000 | 5 | 0.252 | 0.462 | 1.83× |
| 1,000 | 10 | 0.524 | 1.005 | 1.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
