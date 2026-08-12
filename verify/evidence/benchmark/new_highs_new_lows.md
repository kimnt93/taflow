# NewHighsNewLows benchmark (`NewHighsNewLows` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 140.57M | 0.006 | 170.24M | 12.328 | 1732.94× | 2098.71× |
| 10,000 | 0.031 | 327.83M | 0.026 | 379.08M | 92.453 | 3030.85× | 3504.74× |
| 100,000 | 0.250 | 399.42M | 0.230 | 435.38M | 862.344 | 3444.40× | 3754.51× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.177 | 0.288 | 1.63× |
| 1 | 5 | 0.344 | 1.424 | 4.14× |
| 1 | 10 | 0.543 | 2.128 | 3.92× |
| 10 | 1 | 0.053 | 0.296 | 5.54× |
| 10 | 5 | 0.241 | 1.825 | 7.56× |
| 10 | 10 | 0.514 | 3.056 | 5.94× |
| 100 | 1 | 0.060 | 1.107 | 18.61× |
| 100 | 5 | 0.235 | 5.923 | 25.19× |
| 100 | 10 | 0.501 | 11.227 | 22.39× |
| 1,000 | 1 | 0.062 | 9.067 | 145.32× |
| 1,000 | 5 | 0.268 | 46.124 | 172.35× |
| 1,000 | 10 | 0.577 | 98.207 | 170.12× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
