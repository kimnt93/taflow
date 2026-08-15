# MathSin benchmark (`SIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 87.22M | 0.010 | 95.54M | 0.038 | 3.36× | 3.68× |
| 10,000 | 0.154 | 64.83M | 0.153 | 65.24M | 0.185 | 1.20× | 1.21× |
| 100,000 | 1.496 | 66.83M | 1.518 | 65.89M | 1.576 | 1.05× | 1.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.061 | 0.101 | 1.67× |
| 1 | 5 | 0.242 | 0.471 | 1.95× |
| 1 | 10 | 0.374 | 0.889 | 2.38× |
| 10 | 1 | 0.043 | 0.087 | 2.04× |
| 10 | 5 | 0.217 | 0.567 | 2.61× |
| 10 | 10 | 0.386 | 0.942 | 2.44× |
| 100 | 1 | 0.042 | 0.093 | 2.18× |
| 100 | 5 | 0.192 | 0.441 | 2.30× |
| 100 | 10 | 0.472 | 0.983 | 2.08× |
| 1,000 | 1 | 0.063 | 0.115 | 1.84× |
| 1,000 | 5 | 0.209 | 0.508 | 2.43× |
| 1,000 | 10 | 0.460 | 1.134 | 2.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
