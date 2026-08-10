# MathSin benchmark (`SIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 78.87M | 0.012 | 86.40M | 0.035 | 2.73× | 2.99× |
| 10,000 | 0.145 | 68.95M | 0.140 | 71.24M | 0.170 | 1.17× | 1.21× |
| 100,000 | 1.434 | 69.73M | 1.613 | 62.00M | 1.453 | 1.01× | 0.90× |
| 1,000,000 | 15.173 | 65.91M | 15.213 | 65.73M | 15.077 | 0.99× | 0.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.139 | 1.58× |
| 1 | 5 | 0.301 | 0.451 | 1.50× |
| 1 | 10 | 0.456 | 0.858 | 1.88× |
| 10 | 1 | 0.048 | 0.098 | 2.03× |
| 10 | 5 | 0.215 | 0.410 | 1.91× |
| 10 | 10 | 0.461 | 0.880 | 1.91× |
| 100 | 1 | 0.048 | 0.087 | 1.79× |
| 100 | 5 | 0.211 | 0.402 | 1.90× |
| 100 | 10 | 0.461 | 0.833 | 1.81× |
| 1,000 | 1 | 0.059 | 0.100 | 1.68× |
| 1,000 | 5 | 0.218 | 0.506 | 2.32× |
| 1,000 | 10 | 0.503 | 1.030 | 2.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
