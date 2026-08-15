# RollingPainIndex benchmark (`PainIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.043 | 23.37M | 0.040 | 24.95M | 0.167 | 3.90× | 4.16× |
| 10,000 | 0.427 | 23.43M | 0.395 | 25.34M | 0.651 | 1.52× | 1.65× |
| 100,000 | 3.906 | 25.60M | 4.002 | 24.99M | 5.883 | 1.51× | 1.47× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.062 | 0.275 | 4.44× |
| 1 | 5 | 0.311 | 0.977 | 3.14× |
| 1 | 10 | 0.398 | 2.144 | 5.39× |
| 10 | 1 | 0.049 | 0.190 | 3.87× |
| 10 | 5 | 0.188 | 0.944 | 5.01× |
| 10 | 10 | 0.376 | 2.141 | 5.69× |
| 100 | 1 | 0.052 | 0.189 | 3.61× |
| 100 | 5 | 0.216 | 0.961 | 4.44× |
| 100 | 10 | 0.424 | 2.177 | 5.13× |
| 1,000 | 1 | 0.092 | 0.244 | 2.66× |
| 1,000 | 5 | 0.213 | 1.226 | 5.75× |
| 1,000 | 10 | 0.424 | 2.745 | 6.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
