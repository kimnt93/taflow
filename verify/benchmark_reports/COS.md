# MathCos benchmark (`COS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 72.73M | 0.012 | 81.20M | 0.041 | 2.97× | 3.31× |
| 10,000 | 0.155 | 64.53M | 0.152 | 65.66M | 0.186 | 1.20× | 1.22× |
| 100,000 | 1.525 | 65.56M | 1.501 | 66.64M | 1.749 | 1.15× | 1.17× |
| 1,000,000 | 15.847 | 63.10M | 15.399 | 64.94M | 15.713 | 0.99× | 1.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.110 | 0.119 | 1.08× |
| 1 | 5 | 0.301 | 0.529 | 1.76× |
| 1 | 10 | 0.528 | 1.048 | 1.99× |
| 10 | 1 | 0.063 | 0.107 | 1.69× |
| 10 | 5 | 0.241 | 0.516 | 2.14× |
| 10 | 10 | 0.480 | 0.949 | 1.98× |
| 100 | 1 | 0.059 | 0.099 | 1.68× |
| 100 | 5 | 0.239 | 0.474 | 1.98× |
| 100 | 10 | 0.524 | 0.998 | 1.90× |
| 1,000 | 1 | 0.067 | 0.109 | 1.62× |
| 1,000 | 5 | 0.260 | 0.539 | 2.07× |
| 1,000 | 10 | 0.577 | 1.214 | 2.11× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.309 | 0.178 | 5.61M | 1772.009 | 9947.57× | 161.11× |
| 100,000 | 10 | 1.196 | 0.719 | 13.91M | 1534.962 | 2134.70× | 41.94× |
| 100,000 | 1,000 | 18.160 | 19.134 | 52.26M | 1511.011 | 78.97× | 2.14× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 60.55M | 58.82M | 1.00× | 2.24M | 2.18M | 1.00× | 57.32M |
| 5 | 193.12M | 208.81M | 3.55× | 2.07M | 2.22M | 1.02× | 52.72M |
| 10 | 294.98M | 236.42M | 4.02× | 2.08M | 2.49M | 1.14× | 56.44M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
