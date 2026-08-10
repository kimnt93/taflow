# TripleExponentialAverage benchmark (`T3` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 117.47M | 0.008 | 130.26M | 0.041 | 4.76× | 5.28× |
| 10,000 | 0.044 | 226.21M | 0.043 | 232.02M | 0.083 | 1.87× | 1.92× |
| 100,000 | 0.399 | 250.67M | 0.365 | 273.76M | 0.501 | 1.26× | 1.37× |
| 1,000,000 | 4.889 | 204.56M | 4.148 | 241.11M | 4.820 | 0.99× | 1.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.071 | 0.140 | 1.96× |
| 1 | 5 | 0.277 | 0.617 | 2.22× |
| 1 | 10 | 0.523 | 1.062 | 2.03× |
| 10 | 1 | 0.061 | 0.108 | 1.76× |
| 10 | 5 | 0.277 | 0.547 | 1.97× |
| 10 | 10 | 0.524 | 1.010 | 1.93× |
| 100 | 1 | 0.063 | 0.099 | 1.58× |
| 100 | 5 | 0.292 | 0.554 | 1.90× |
| 100 | 10 | 0.577 | 1.006 | 1.74× |
| 1,000 | 1 | 0.062 | 0.113 | 1.81× |
| 1,000 | 5 | 0.281 | 0.541 | 1.92× |
| 1,000 | 10 | 0.590 | 1.095 | 1.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
