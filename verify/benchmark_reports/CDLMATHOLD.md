# CandleMatHold benchmark (`CDLMATHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 88.85M | 0.009 | 111.61M | 0.041 | 3.64× | 4.57× |
| 10,000 | 0.096 | 104.68M | 0.090 | 110.92M | 0.121 | 1.27× | 1.34× |
| 100,000 | 0.930 | 107.53M | 0.934 | 107.03M | 0.881 | 0.95× | 0.94× |
| 1,000,000 | 9.404 | 106.33M | 9.209 | 108.58M | 8.450 | 0.90× | 0.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.121 | 0.181 | 1.50× |
| 1 | 5 | 0.289 | 0.559 | 1.93× |
| 1 | 10 | 0.525 | 0.984 | 1.88× |
| 10 | 1 | 0.053 | 0.096 | 1.80× |
| 10 | 5 | 0.237 | 0.479 | 2.03× |
| 10 | 10 | 0.527 | 0.968 | 1.84× |
| 100 | 1 | 0.057 | 0.101 | 1.79× |
| 100 | 5 | 0.246 | 0.471 | 1.92× |
| 100 | 10 | 0.540 | 1.003 | 1.86× |
| 1,000 | 1 | 0.069 | 0.111 | 1.61× |
| 1,000 | 5 | 0.245 | 0.517 | 2.11× |
| 1,000 | 10 | 0.532 | 1.095 | 2.06× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
