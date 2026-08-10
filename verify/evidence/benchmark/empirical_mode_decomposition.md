# EmpiricalModeDecomposition benchmark (`EmpiricalModeDecomposition` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.042 | 23.67M | 0.041 | 24.64M | 0.240 | 5.67× | 5.91× |
| 10,000 | 0.395 | 25.33M | 0.383 | 26.13M | 0.936 | 2.37× | 2.45× |
| 100,000 | 4.190 | 23.87M | 3.903 | 25.62M | 7.414 | 1.77× | 1.90× |
| 1,000,000 | 39.551 | 25.28M | 39.750 | 25.16M | 72.697 | 1.84× | 1.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.110 | 0.342 | 3.12× |
| 1 | 5 | 0.329 | 1.450 | 4.41× |
| 1 | 10 | 0.524 | 2.735 | 5.22× |
| 10 | 1 | 0.058 | 0.271 | 4.65× |
| 10 | 5 | 0.229 | 1.421 | 6.20× |
| 10 | 10 | 0.484 | 2.629 | 5.43× |
| 100 | 1 | 0.062 | 0.257 | 4.15× |
| 100 | 5 | 0.260 | 1.447 | 5.56× |
| 100 | 10 | 0.546 | 3.221 | 5.90× |
| 1,000 | 1 | 0.099 | 0.315 | 3.17× |
| 1,000 | 5 | 0.273 | 1.890 | 6.91× |
| 1,000 | 10 | 0.523 | 3.565 | 6.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
