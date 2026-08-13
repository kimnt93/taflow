# PositiveVolumeIndex benchmark (`PVI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.038 | 26.31M | 0.030 | 32.92M | 0.182 | 4.80× | 6.01× |
| 10,000 | 0.235 | 42.57M | 0.230 | 43.49M | 0.735 | 3.13× | 3.20× |
| 100,000 | 2.189 | 45.68M | 2.158 | 46.35M | 6.073 | 2.77× | 2.81× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.253 | 2.53× |
| 1 | 5 | 0.397 | 1.008 | 2.54× |
| 1 | 10 | 0.610 | 2.175 | 3.57× |
| 10 | 1 | 0.073 | 0.206 | 2.83× |
| 10 | 5 | 0.283 | 1.282 | 4.54× |
| 10 | 10 | 0.619 | 2.136 | 3.45× |
| 100 | 1 | 0.074 | 0.198 | 2.67× |
| 100 | 5 | 0.290 | 1.368 | 4.73× |
| 100 | 10 | 0.578 | 2.245 | 3.89× |
| 1,000 | 1 | 0.100 | 0.261 | 2.61× |
| 1,000 | 5 | 0.305 | 1.565 | 5.13× |
| 1,000 | 10 | 0.612 | 2.784 | 4.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
