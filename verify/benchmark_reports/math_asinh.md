# MathAsinh benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 71.61M | 0.014 | 70.54M | 0.012 | 0.84× | 0.82× |
| 10,000 | 0.107 | 93.41M | 0.108 | 92.82M | 0.106 | 0.99× | 0.98× |
| 100,000 | 1.079 | 92.64M | 1.053 | 95.00M | 1.020 | 0.95× | 0.97× |
| 1,000,000 | 11.400 | 87.72M | 10.513 | 95.12M | 10.641 | 0.93× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.064 | 0.052 | 0.81× |
| 1 | 5 | 0.272 | 0.207 | 0.76× |
| 1 | 10 | 0.455 | 0.403 | 0.89× |
| 10 | 1 | 0.057 | 0.046 | 0.81× |
| 10 | 5 | 0.260 | 0.201 | 0.77× |
| 10 | 10 | 0.483 | 0.399 | 0.83× |
| 100 | 1 | 0.045 | 0.042 | 0.92× |
| 100 | 5 | 0.212 | 0.187 | 0.88× |
| 100 | 10 | 0.534 | 0.468 | 0.88× |
| 1,000 | 1 | 0.063 | 0.059 | 0.93× |
| 1,000 | 5 | 0.219 | 0.202 | 0.92× |
| 1,000 | 10 | 0.531 | 0.508 | 0.96× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.345 | 0.178 | 5.61M | nan | — | — |
| 100,000 | 10 | 1.152 | 0.661 | 15.12M | nan | — | — |
| 100,000 | 1,000 | 15.756 | 15.084 | 66.29M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 65.06M | 71.82M | 1.00× | 2.26M | 3.61M | 1.00× | — |
| 5 | 298.08M | 334.67M | 4.66× | 2.06M | 2.55M | 0.71× | — |
| 10 | 341.40M | 407.98M | 5.68× | 1.99M | 2.59M | 0.72× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
