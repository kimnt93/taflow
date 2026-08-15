# MathAbs benchmark (`numpy.abs` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 576.25M | 0.001 | 1.26G | 0.012 | 7.07× | 15.46× |
| 10,000 | 0.005 | 1.86G | 0.003 | 3.15G | 0.014 | 2.70× | 4.56× |
| 100,000 | 0.049 | 2.02G | 0.027 | 3.73G | 0.039 | 0.79× | 1.46× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.086 | 1.01× |
| 1 | 5 | 0.226 | 0.302 | 1.34× |
| 1 | 10 | 0.427 | 0.653 | 1.53× |
| 10 | 1 | 0.044 | 0.061 | 1.36× |
| 10 | 5 | 0.179 | 0.272 | 1.52× |
| 10 | 10 | 0.387 | 0.579 | 1.49× |
| 100 | 1 | 0.040 | 0.053 | 1.34× |
| 100 | 5 | 0.188 | 0.276 | 1.47× |
| 100 | 10 | 0.404 | 0.569 | 1.41× |
| 1,000 | 1 | 0.039 | 0.057 | 1.47× |
| 1,000 | 5 | 0.173 | 0.264 | 1.53× |
| 1,000 | 10 | 0.371 | 0.604 | 1.63× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
