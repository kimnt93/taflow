# MathTan benchmark (`TAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 48.58M | 0.018 | 54.80M | 0.047 | 2.30× | 2.60× |
| 10,000 | 0.222 | 45.00M | 0.202 | 49.41M | 0.229 | 1.03× | 1.13× |
| 100,000 | 1.972 | 50.71M | 1.955 | 51.14M | 1.983 | 1.01× | 1.01× |
| 1,000,000 | 21.917 | 45.63M | 19.563 | 51.12M | 19.736 | 0.90× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.169 | 1.44× |
| 1 | 5 | 0.247 | 0.489 | 1.98× |
| 1 | 10 | 0.489 | 0.890 | 1.82× |
| 10 | 1 | 0.052 | 0.086 | 1.64× |
| 10 | 5 | 0.237 | 0.445 | 1.88× |
| 10 | 10 | 0.454 | 0.908 | 2.00× |
| 100 | 1 | 0.048 | 0.086 | 1.81× |
| 100 | 5 | 0.239 | 0.447 | 1.87× |
| 100 | 10 | 0.506 | 0.918 | 1.81× |
| 1,000 | 1 | 0.073 | 0.109 | 1.49× |
| 1,000 | 5 | 0.249 | 0.542 | 2.17× |
| 1,000 | 10 | 0.535 | 1.108 | 2.07× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.318 | 0.172 | 5.82M | 2035.939 | 11853.09× | 150.96× |
| 100,000 | 10 | 1.368 | 0.789 | 12.67M | 2025.628 | 2566.31× | 32.05× |
| 100,000 | 1,000 | 23.534 | 29.887 | 33.46M | 2125.065 | 71.10× | 1.50× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 46.26M | 47.84M | 1.00× | 2.35M | 3.08M | 1.00× | 46.32M |
| 5 | 164.52M | 167.59M | 3.50× | 2.25M | 2.49M | 0.81× | 44.31M |
| 10 | 221.75M | 263.80M | 5.51× | 1.69M | 2.18M | 0.71× | 44.08M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
