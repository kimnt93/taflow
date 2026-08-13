# Momentum benchmark (`MOM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.035 | 28.28M | 0.029 | 34.70M | 0.031 | 0.87× | 1.06× |
| 10,000 | 0.240 | 41.75M | 0.215 | 46.56M | 0.037 | 0.15× | 0.17× |
| 100,000 | 2.326 | 42.99M | 2.149 | 46.54M | 0.063 | 0.03× | 0.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.140 | 0.161 | 1.15× |
| 1 | 5 | 0.439 | 0.489 | 1.11× |
| 1 | 10 | 0.602 | 0.954 | 1.58× |
| 10 | 1 | 0.065 | 0.089 | 1.37× |
| 10 | 5 | 0.293 | 0.441 | 1.51× |
| 10 | 10 | 0.585 | 0.935 | 1.60× |
| 100 | 1 | 0.070 | 0.095 | 1.37× |
| 100 | 5 | 0.295 | 0.430 | 1.46× |
| 100 | 10 | 0.605 | 0.909 | 1.50× |
| 1,000 | 1 | 0.084 | 0.097 | 1.15× |
| 1,000 | 5 | 0.291 | 0.459 | 1.58× |
| 1,000 | 10 | 0.633 | 0.921 | 1.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
