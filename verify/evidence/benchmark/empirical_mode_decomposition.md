# EmpiricalModeDecomposition benchmark (`EmpiricalModeDecomposition` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.041 | 24.10M | 0.040 | 25.09M | 0.315 | 7.59× | 7.90× |
| 10,000 | 0.384 | 26.01M | 0.383 | 26.08M | 0.857 | 2.23× | 2.23× |
| 100,000 | 3.887 | 25.73M | 3.852 | 25.96M | 7.126 | 1.83× | 1.85× |
| 1,000,000 | 39.093 | 25.58M | 41.211 | 24.27M | 70.316 | 1.80× | 1.71× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.180 | 0.299 | 1.66× |
| 1 | 5 | 0.287 | 1.378 | 4.79× |
| 1 | 10 | 0.491 | 3.487 | 7.10× |
| 10 | 1 | 0.068 | 0.301 | 4.41× |
| 10 | 5 | 0.274 | 1.485 | 5.43× |
| 10 | 10 | 0.478 | 2.524 | 5.28× |
| 100 | 1 | 0.055 | 0.248 | 4.52× |
| 100 | 5 | 0.245 | 1.434 | 5.85× |
| 100 | 10 | 0.522 | 2.723 | 5.22× |
| 1,000 | 1 | 0.088 | 0.317 | 3.58× |
| 1,000 | 5 | 0.248 | 1.758 | 7.09× |
| 1,000 | 10 | 0.531 | 3.270 | 6.16× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
