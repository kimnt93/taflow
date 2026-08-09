# DoubleExponentialMovingAverage benchmark (`DEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 103.20M | 0.010 | 102.26M | 0.038 | 3.91× | 3.87× |
| 10,000 | 0.064 | 155.82M | 0.060 | 167.37M | 0.092 | 1.44× | 1.55× |
| 100,000 | 0.594 | 168.42M | 0.558 | 179.15M | 0.653 | 1.10× | 1.17× |
| 1,000,000 | 6.116 | 163.51M | 5.902 | 169.42M | 11.742 | 1.92× | 1.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.076 | 0.129 | 1.71× |
| 1 | 5 | 0.332 | 0.551 | 1.66× |
| 1 | 10 | 0.592 | 1.070 | 1.81× |
| 10 | 1 | 0.059 | 0.100 | 1.68× |
| 10 | 5 | 0.266 | 0.501 | 1.88× |
| 10 | 10 | 0.548 | 1.090 | 1.99× |
| 100 | 1 | 0.054 | 0.100 | 1.86× |
| 100 | 5 | 0.243 | 0.464 | 1.91× |
| 100 | 10 | 0.534 | 1.085 | 2.03× |
| 1,000 | 1 | 0.063 | 0.107 | 1.71× |
| 1,000 | 5 | 0.247 | 0.503 | 2.03× |
| 1,000 | 10 | 0.509 | 1.104 | 2.17× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
