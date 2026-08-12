# RollingPainIndex benchmark (`PainIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.051 | 19.44M | 0.050 | 20.00M | 0.178 | 3.45× | 3.55× |
| 10,000 | 0.484 | 20.67M | 0.494 | 20.22M | 0.722 | 1.49× | 1.46× |
| 100,000 | 4.832 | 20.70M | 4.835 | 20.68M | 5.986 | 1.24× | 1.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.256 | 2.84× |
| 1 | 5 | 0.285 | 1.029 | 3.61× |
| 1 | 10 | 0.471 | 2.135 | 4.53× |
| 10 | 1 | 0.053 | 0.195 | 3.64× |
| 10 | 5 | 0.232 | 1.005 | 4.33× |
| 10 | 10 | 0.500 | 2.231 | 4.46× |
| 100 | 1 | 0.060 | 0.218 | 3.63× |
| 100 | 5 | 0.263 | 1.031 | 3.92× |
| 100 | 10 | 0.538 | 2.149 | 4.00× |
| 1,000 | 1 | 0.109 | 0.260 | 2.39× |
| 1,000 | 5 | 0.233 | 1.277 | 5.48× |
| 1,000 | 10 | 0.534 | 2.785 | 5.21× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
