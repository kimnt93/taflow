# RollingMode benchmark (`rolling mode` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 3.621 | 276.20K | 3.628 | 275.63K | 0.045 | 0.01× | 0.01× |
| 10,000 | 36.873 | 271.20K | 36.825 | 271.56K | 0.110 | 0.00× | 0.00× |
| 100,000 | 369.492 | 270.64K | 373.006 | 268.09K | 0.963 | 0.00× | 0.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.145 | 0.114 | 0.79× |
| 1 | 5 | 0.458 | 0.421 | 0.92× |
| 1 | 10 | 0.644 | 0.816 | 1.27× |
| 10 | 1 | 0.072 | 0.088 | 1.22× |
| 10 | 5 | 0.304 | 0.403 | 1.33× |
| 10 | 10 | 0.677 | 0.839 | 1.24× |
| 100 | 1 | 0.408 | 0.113 | 0.28× |
| 100 | 5 | 0.578 | 0.545 | 0.94× |
| 100 | 10 | 1.043 | 1.088 | 1.04× |
| 1,000 | 1 | 3.855 | 0.120 | 0.03× |
| 1,000 | 5 | 4.059 | 0.795 | 0.20× |
| 1,000 | 10 | 7.216 | 1.468 | 0.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
