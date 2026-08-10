# MovingAverage benchmark (`MA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 162.17M | 0.005 | 192.28M | 0.040 | 6.45× | 7.64× |
| 10,000 | 0.029 | 347.27M | 0.024 | 418.51M | 0.059 | 2.04× | 2.46× |
| 100,000 | 0.243 | 411.50M | 0.213 | 468.56M | 0.251 | 1.03× | 1.18× |
| 1,000,000 | 2.865 | 349.05M | 2.532 | 395.02M | 2.525 | 0.88× | 1.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.230 | 2.36× |
| 1 | 5 | 0.640 | 2.164 | 3.38× |
| 1 | 10 | 0.792 | 1.224 | 1.55× |
| 10 | 1 | 0.067 | 0.102 | 1.52× |
| 10 | 5 | 0.307 | 0.646 | 2.11× |
| 10 | 10 | 0.974 | 1.308 | 1.34× |
| 100 | 1 | 0.084 | 0.117 | 1.39× |
| 100 | 5 | 0.335 | 0.623 | 1.86× |
| 100 | 10 | 0.585 | 1.554 | 2.65× |
| 1,000 | 1 | 0.074 | 0.123 | 1.67× |
| 1,000 | 5 | 0.323 | 0.615 | 1.90× |
| 1,000 | 10 | 0.685 | 1.247 | 1.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
