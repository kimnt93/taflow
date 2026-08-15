# CandleSpinningTop benchmark (`CDLSPINNINGTOP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 160.74M | 0.003 | 328.24M | 0.031 | 5.01× | 10.23× |
| 10,000 | 0.082 | 121.53M | 0.077 | 129.34M | 0.116 | 1.41× | 1.50× |
| 100,000 | 0.927 | 107.82M | 0.926 | 108.01M | 0.953 | 1.03× | 1.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.131 | 1.86× |
| 1 | 5 | 0.273 | 0.455 | 1.66× |
| 1 | 10 | 0.410 | 0.916 | 2.23× |
| 10 | 1 | 0.041 | 0.085 | 2.08× |
| 10 | 5 | 0.178 | 0.431 | 2.42× |
| 10 | 10 | 0.376 | 0.888 | 2.36× |
| 100 | 1 | 0.046 | 0.088 | 1.92× |
| 100 | 5 | 0.194 | 0.427 | 2.20× |
| 100 | 10 | 0.391 | 0.902 | 2.30× |
| 1,000 | 1 | 0.050 | 0.098 | 1.94× |
| 1,000 | 5 | 0.193 | 0.480 | 2.48× |
| 1,000 | 10 | 0.406 | 0.976 | 2.40× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
