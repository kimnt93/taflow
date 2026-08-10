# MathSqrt benchmark (`SQRT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 265.08M | 0.003 | 351.82M | 0.028 | 7.31× | 9.70× |
| 10,000 | 0.012 | 844.84M | 0.009 | 1.09G | 0.040 | 3.37× | 4.34× |
| 100,000 | 0.096 | 1.04G | 0.070 | 1.42G | 0.165 | 1.71× | 2.34× |
| 1,000,000 | 1.465 | 682.72M | 0.803 | 1.25G | 1.516 | 1.03× | 1.89× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.103 | 1.04× |
| 1 | 5 | 0.367 | 0.421 | 1.15× |
| 1 | 10 | 0.471 | 0.868 | 1.84× |
| 10 | 1 | 0.047 | 0.090 | 1.93× |
| 10 | 5 | 0.210 | 0.407 | 1.93× |
| 10 | 10 | 0.466 | 0.870 | 1.87× |
| 100 | 1 | 0.046 | 0.090 | 1.98× |
| 100 | 5 | 0.222 | 0.420 | 1.89× |
| 100 | 10 | 0.453 | 0.877 | 1.93× |
| 1,000 | 1 | 0.047 | 0.093 | 1.98× |
| 1,000 | 5 | 0.224 | 0.407 | 1.82× |
| 1,000 | 10 | 0.452 | 0.871 | 1.93× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
