# SchaffTrendCycle benchmark (`stc` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.051 | 19.58M | 0.048 | 20.98M | 29.993 | 587.36× | 629.14× |
| 10,000 | 0.596 | 16.77M | 0.588 | 17.00M | 304.238 | 510.22× | 517.32× |
| 100,000 | 6.085 | 16.43M | 5.860 | 17.07M | 2864.788 | 470.77× | 488.88× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.229 | 2.12× |
| 1 | 5 | 0.302 | 1.030 | 3.41× |
| 1 | 10 | 0.423 | 1.929 | 4.56× |
| 10 | 1 | 0.043 | 0.196 | 4.53× |
| 10 | 5 | 0.184 | 0.948 | 5.15× |
| 10 | 10 | 0.414 | 1.900 | 4.59× |
| 100 | 1 | 0.054 | 5.070 | 93.30× |
| 100 | 5 | 0.236 | 25.146 | 106.43× |
| 100 | 10 | 0.508 | 49.398 | 97.17× |
| 1,000 | 1 | 0.165 | 29.531 | 178.67× |
| 1,000 | 5 | 0.375 | 218.930 | 584.53× |
| 1,000 | 10 | 1.238 | 457.711 | 369.68× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
