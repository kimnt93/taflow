# CandleDoji benchmark (`CDLDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 179.91M | 0.002 | 442.07M | 0.030 | 5.44× | 13.36× |
| 10,000 | 0.019 | 522.57M | 0.014 | 692.86M | 0.050 | 2.60× | 3.44× |
| 100,000 | 0.144 | 694.71M | 0.132 | 755.43M | 0.237 | 1.64× | 1.79× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.136 | 1.94× |
| 1 | 5 | 0.276 | 0.455 | 1.65× |
| 1 | 10 | 0.366 | 0.879 | 2.40× |
| 10 | 1 | 0.042 | 0.088 | 2.10× |
| 10 | 5 | 0.173 | 0.404 | 2.34× |
| 10 | 10 | 0.424 | 0.897 | 2.12× |
| 100 | 1 | 0.048 | 0.096 | 2.02× |
| 100 | 5 | 0.183 | 0.416 | 2.28× |
| 100 | 10 | 0.385 | 0.961 | 2.49× |
| 1,000 | 1 | 0.044 | 0.104 | 2.38× |
| 1,000 | 5 | 0.184 | 0.426 | 2.31× |
| 1,000 | 10 | 0.392 | 0.901 | 2.30× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
