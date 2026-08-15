# TriangularMovingAverage benchmark (`TRIMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 173.51M | 0.005 | 201.78M | 0.035 | 6.08× | 7.07× |
| 10,000 | 0.043 | 233.87M | 0.041 | 245.76M | 0.062 | 1.46× | 1.53× |
| 100,000 | 0.524 | 190.97M | 0.399 | 250.47M | 0.321 | 0.61× | 0.80× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.179 | 0.108 | 0.60× |
| 1 | 5 | 0.213 | 0.451 | 2.12× |
| 1 | 10 | 0.399 | 0.945 | 2.37× |
| 10 | 1 | 0.047 | 0.094 | 2.00× |
| 10 | 5 | 0.196 | 0.453 | 2.31× |
| 10 | 10 | 0.376 | 0.945 | 2.51× |
| 100 | 1 | 0.043 | 0.090 | 2.09× |
| 100 | 5 | 0.184 | 0.479 | 2.60× |
| 100 | 10 | 0.409 | 0.919 | 2.24× |
| 1,000 | 1 | 0.043 | 0.097 | 2.24× |
| 1,000 | 5 | 0.191 | 0.443 | 2.32× |
| 1,000 | 10 | 0.484 | 1.131 | 2.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
