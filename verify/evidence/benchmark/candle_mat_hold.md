# CandleMatHold benchmark (`CDLMATHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 162.28M | 0.003 | 326.73M | 0.035 | 5.76× | 11.59× |
| 10,000 | 0.061 | 163.11M | 0.052 | 192.65M | 0.112 | 1.83× | 2.16× |
| 100,000 | 0.826 | 121.11M | 0.822 | 121.65M | 0.810 | 0.98× | 0.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | 0.131 | 1.27× |
| 1 | 5 | 0.274 | 0.520 | 1.90× |
| 1 | 10 | 0.398 | 0.922 | 2.31× |
| 10 | 1 | 0.043 | 0.091 | 2.12× |
| 10 | 5 | 0.176 | 0.439 | 2.49× |
| 10 | 10 | 0.371 | 0.949 | 2.56× |
| 100 | 1 | 0.044 | 0.090 | 2.04× |
| 100 | 5 | 0.183 | 0.434 | 2.37× |
| 100 | 10 | 0.373 | 0.913 | 2.45× |
| 1,000 | 1 | 0.057 | 0.098 | 1.72× |
| 1,000 | 5 | 0.189 | 0.481 | 2.54× |
| 1,000 | 10 | 0.409 | 0.998 | 2.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
