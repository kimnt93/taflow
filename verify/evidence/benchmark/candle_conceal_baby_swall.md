# CandleConcealBabySwall benchmark (`CDLCONCEALBABYSWALL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 73.80M | 0.011 | 88.64M | 0.035 | 2.60× | 3.12× |
| 10,000 | 0.119 | 83.94M | 0.124 | 80.93M | 0.092 | 0.77× | 0.74× |
| 100,000 | 1.264 | 79.13M | 1.243 | 80.43M | 0.635 | 0.50× | 0.51× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.118 | 1.00× |
| 1 | 5 | 0.209 | 0.451 | 2.16× |
| 1 | 10 | 0.395 | 0.965 | 2.44× |
| 10 | 1 | 0.045 | 0.087 | 1.93× |
| 10 | 5 | 0.191 | 0.423 | 2.21× |
| 10 | 10 | 0.399 | 0.893 | 2.24× |
| 100 | 1 | 0.048 | 0.096 | 1.99× |
| 100 | 5 | 0.217 | 0.464 | 2.14× |
| 100 | 10 | 0.399 | 0.962 | 2.41× |
| 1,000 | 1 | 0.067 | 0.111 | 1.66× |
| 1,000 | 5 | 0.227 | 0.510 | 2.25× |
| 1,000 | 10 | 0.452 | 1.023 | 2.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
