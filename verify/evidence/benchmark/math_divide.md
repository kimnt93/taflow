# MathDivide benchmark (`DIV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 361.14M | 0.001 | 776.06M | 0.031 | 11.25× | 24.18× |
| 10,000 | 0.009 | 1.08G | 0.006 | 1.66G | 0.035 | 3.80× | 5.84× |
| 100,000 | 0.073 | 1.38G | 0.050 | 2.01G | 0.080 | 1.11× | 1.61× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.112 | 1.36× |
| 1 | 5 | 0.211 | 0.428 | 2.03× |
| 1 | 10 | 0.402 | 0.981 | 2.44× |
| 10 | 1 | 0.043 | 0.094 | 2.19× |
| 10 | 5 | 0.181 | 0.418 | 2.31× |
| 10 | 10 | 0.401 | 0.889 | 2.21× |
| 100 | 1 | 0.043 | 0.090 | 2.11× |
| 100 | 5 | 0.213 | 0.460 | 2.15× |
| 100 | 10 | 0.439 | 0.950 | 2.16× |
| 1,000 | 1 | 0.042 | 0.091 | 2.17× |
| 1,000 | 5 | 0.180 | 0.451 | 2.51× |
| 1,000 | 10 | 0.429 | 0.938 | 2.18× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
