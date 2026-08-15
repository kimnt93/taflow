# SchaffTrendCycle benchmark (`stc` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.057 | 17.49M | 0.053 | 18.81M | 31.515 | 551.20× | 592.92× |
| 10,000 | 0.648 | 15.42M | 0.606 | 16.51M | 317.289 | 489.41× | 523.82× |
| 100,000 | 6.528 | 15.32M | 6.349 | 15.75M | 2992.867 | 458.49× | 471.42× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.229 | 2.54× |
| 1 | 5 | 0.327 | 0.989 | 3.02× |
| 1 | 10 | 0.408 | 2.033 | 4.98× |
| 10 | 1 | 0.048 | 0.202 | 4.17× |
| 10 | 5 | 0.196 | 0.966 | 4.94× |
| 10 | 10 | 0.398 | 2.046 | 5.14× |
| 100 | 1 | 0.053 | 5.010 | 94.56× |
| 100 | 5 | 0.215 | 25.106 | 116.98× |
| 100 | 10 | 0.537 | 49.654 | 92.54× |
| 1,000 | 1 | 0.140 | 30.960 | 220.40× |
| 1,000 | 5 | 0.345 | 166.179 | 481.94× |
| 1,000 | 10 | 0.591 | 354.103 | 599.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
