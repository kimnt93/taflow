# CandleSeparatingLines benchmark (`CDLSEPARATINGLINES` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 69.40M | 0.011 | 94.08M | 0.035 | 2.43× | 3.29× |
| 10,000 | 0.126 | 79.52M | 0.131 | 76.32M | 0.124 | 0.98× | 0.94× |
| 100,000 | 1.262 | 79.25M | 1.228 | 81.46M | 1.006 | 0.80× | 0.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.137 | 1.55× |
| 1 | 5 | 0.286 | 0.469 | 1.64× |
| 1 | 10 | 0.410 | 0.992 | 2.42× |
| 10 | 1 | 0.045 | 0.088 | 1.95× |
| 10 | 5 | 0.189 | 0.434 | 2.29× |
| 10 | 10 | 0.383 | 0.923 | 2.41× |
| 100 | 1 | 0.046 | 0.095 | 2.04× |
| 100 | 5 | 0.209 | 0.453 | 2.17× |
| 100 | 10 | 0.393 | 0.922 | 2.35× |
| 1,000 | 1 | 0.054 | 0.098 | 1.82× |
| 1,000 | 5 | 0.191 | 0.482 | 2.53× |
| 1,000 | 10 | 0.491 | 0.990 | 2.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
