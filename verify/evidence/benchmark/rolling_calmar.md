# RollingCalmar benchmark (`rolling calmar on equity` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.050 | 19.97M | 0.046 | 21.66M | 0.263 | 5.26× | 5.70× |
| 10,000 | 0.459 | 21.80M | 0.457 | 21.88M | 1.845 | 4.02× | 4.04× |
| 100,000 | 5.163 | 19.37M | 4.659 | 21.47M | 17.895 | 3.47× | 3.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.104 | 1.26× |
| 1 | 5 | 0.336 | 0.441 | 1.31× |
| 1 | 10 | 0.528 | 1.084 | 2.06× |
| 10 | 1 | 0.056 | 0.091 | 1.64× |
| 10 | 5 | 0.224 | 0.414 | 1.85× |
| 10 | 10 | 0.465 | 0.854 | 1.84× |
| 100 | 1 | 0.066 | 0.214 | 3.22× |
| 100 | 5 | 0.246 | 1.047 | 4.26× |
| 100 | 10 | 0.531 | 2.221 | 4.18× |
| 1,000 | 1 | 0.116 | 0.410 | 3.54× |
| 1,000 | 5 | 0.258 | 1.354 | 5.24× |
| 1,000 | 10 | 0.638 | 2.849 | 4.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
