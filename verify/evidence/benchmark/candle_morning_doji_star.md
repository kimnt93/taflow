# CandleMorningDojiStar benchmark (`CDLMORNINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 65.74M | 0.012 | 85.73M | 0.041 | 2.70× | 3.53× |
| 10,000 | 0.149 | 66.92M | 0.151 | 66.13M | 0.127 | 0.85× | 0.84× |
| 100,000 | 1.624 | 61.58M | 1.786 | 56.00M | 0.896 | 0.55× | 0.50× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.113 | 0.149 | 1.31× |
| 1 | 5 | 0.284 | 0.476 | 1.67× |
| 1 | 10 | 0.397 | 1.043 | 2.63× |
| 10 | 1 | 0.047 | 0.092 | 1.98× |
| 10 | 5 | 0.184 | 0.485 | 2.63× |
| 10 | 10 | 0.394 | 1.029 | 2.61× |
| 100 | 1 | 0.048 | 0.100 | 2.08× |
| 100 | 5 | 0.195 | 0.475 | 2.44× |
| 100 | 10 | 0.427 | 0.962 | 2.25× |
| 1,000 | 1 | 0.060 | 0.105 | 1.77× |
| 1,000 | 5 | 0.200 | 0.548 | 2.74× |
| 1,000 | 10 | 0.441 | 1.038 | 2.36× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
