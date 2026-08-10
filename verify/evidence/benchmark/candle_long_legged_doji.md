# CandleLongLeggedDoji benchmark (`CDLLONGLEGGEDDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 77.12M | 0.010 | 100.67M | 0.033 | 2.54× | 3.32× |
| 10,000 | 0.071 | 141.17M | 0.068 | 147.33M | 0.090 | 1.27× | 1.33× |
| 100,000 | 0.763 | 131.04M | 0.757 | 132.12M | 0.666 | 0.87× | 0.88× |
| 1,000,000 | 7.627 | 131.12M | 7.105 | 140.75M | 6.558 | 0.86× | 0.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.138 | 0.144 | 1.04× |
| 1 | 5 | 0.363 | 0.461 | 1.27× |
| 1 | 10 | 0.656 | 1.068 | 1.63× |
| 10 | 1 | 0.057 | 0.092 | 1.61× |
| 10 | 5 | 0.259 | 0.434 | 1.68× |
| 10 | 10 | 0.536 | 0.916 | 1.71× |
| 100 | 1 | 0.057 | 0.091 | 1.61× |
| 100 | 5 | 0.248 | 0.417 | 1.68× |
| 100 | 10 | 0.581 | 0.908 | 1.56× |
| 1,000 | 1 | 0.068 | 0.102 | 1.50× |
| 1,000 | 5 | 0.253 | 0.475 | 1.88× |
| 1,000 | 10 | 0.559 | 0.997 | 1.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
