# MathMultiply benchmark (`MULT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 406.21M | 0.001 | 1.02G | 0.032 | 13.01× | 32.52× |
| 10,000 | 0.007 | 1.43G | 0.004 | 2.52G | 0.036 | 5.18× | 9.17× |
| 100,000 | 0.062 | 1.60G | 0.035 | 2.84G | 0.067 | 1.08× | 1.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.150 | 1.44× |
| 1 | 5 | 0.239 | 0.472 | 1.98× |
| 1 | 10 | 0.448 | 0.927 | 2.07× |
| 10 | 1 | 0.040 | 0.087 | 2.17× |
| 10 | 5 | 0.187 | 0.413 | 2.22× |
| 10 | 10 | 0.371 | 1.002 | 2.70× |
| 100 | 1 | 0.047 | 0.093 | 1.98× |
| 100 | 5 | 0.172 | 0.435 | 2.54× |
| 100 | 10 | 0.374 | 0.895 | 2.39× |
| 1,000 | 1 | 0.040 | 0.090 | 2.25× |
| 1,000 | 5 | 0.210 | 0.482 | 2.30× |
| 1,000 | 10 | 0.399 | 0.921 | 2.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
