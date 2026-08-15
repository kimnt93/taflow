# InsideBar benchmark (`inside bar relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 346.26M | 0.001 | 751.15M | 0.021 | 7.29× | 15.82× |
| 10,000 | 0.010 | 1.02G | 0.007 | 1.49G | 0.040 | 4.04× | 5.93× |
| 100,000 | 0.091 | 1.10G | 0.067 | 1.49G | 0.215 | 2.36× | 3.19× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.173 | 0.171 | 0.99× |
| 1 | 5 | 0.241 | 0.392 | 1.62× |
| 1 | 10 | 0.378 | 0.719 | 1.90× |
| 10 | 1 | 0.042 | 0.069 | 1.67× |
| 10 | 5 | 0.173 | 0.331 | 1.92× |
| 10 | 10 | 0.374 | 0.718 | 1.92× |
| 100 | 1 | 0.043 | 0.074 | 1.73× |
| 100 | 5 | 0.188 | 0.348 | 1.85× |
| 100 | 10 | 0.367 | 0.712 | 1.94× |
| 1,000 | 1 | 0.045 | 0.075 | 1.66× |
| 1,000 | 5 | 0.169 | 0.516 | 3.05× |
| 1,000 | 10 | 0.361 | 1.098 | 3.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
