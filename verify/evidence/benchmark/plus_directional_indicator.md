# PlusDirectionalIndicator benchmark (`PLUS_DI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.050 | 20.14M | 0.041 | 24.37M | 0.037 | 0.74× | 0.90× |
| 10,000 | 0.325 | 30.78M | 0.315 | 31.72M | 0.095 | 0.29× | 0.30× |
| 100,000 | 3.086 | 32.41M | 3.147 | 31.78M | 0.663 | 0.21× | 0.21× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.125 | 0.159 | 1.27× |
| 1 | 5 | 0.481 | 0.485 | 1.01× |
| 1 | 10 | 0.614 | 0.926 | 1.51× |
| 10 | 1 | 0.065 | 0.094 | 1.44× |
| 10 | 5 | 0.296 | 0.458 | 1.54× |
| 10 | 10 | 0.604 | 0.940 | 1.56× |
| 100 | 1 | 0.072 | 0.089 | 1.23× |
| 100 | 5 | 0.298 | 0.436 | 1.46× |
| 100 | 10 | 0.646 | 0.933 | 1.44× |
| 1,000 | 1 | 0.110 | 0.102 | 0.93× |
| 1,000 | 5 | 0.323 | 0.488 | 1.51× |
| 1,000 | 10 | 0.664 | 1.028 | 1.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
