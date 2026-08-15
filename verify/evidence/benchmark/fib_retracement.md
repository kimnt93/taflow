# FibonacciRetracement benchmark (`rolling Fibonacci levels` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.026 | 38.00M | 0.022 | 45.32M | 11.728 | 445.71× | 531.53× |
| 10,000 | 0.298 | 33.52M | 0.284 | 35.24M | 116.123 | 389.28× | 409.20× |
| 100,000 | 2.916 | 34.29M | 2.752 | 36.34M | 1168.200 | 400.59× | 424.50× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.147 | 0.176 | 1.20× |
| 1 | 5 | 0.342 | 0.605 | 1.77× |
| 1 | 10 | 0.395 | 1.151 | 2.92× |
| 10 | 1 | 0.047 | 0.227 | 4.88× |
| 10 | 5 | 0.192 | 1.117 | 5.81× |
| 10 | 10 | 0.403 | 2.299 | 5.71× |
| 100 | 1 | 0.048 | 1.312 | 27.53× |
| 100 | 5 | 0.205 | 6.590 | 32.07× |
| 100 | 10 | 0.466 | 13.133 | 28.19× |
| 1,000 | 1 | 0.080 | 12.035 | 150.34× |
| 1,000 | 5 | 0.405 | 100.855 | 249.19× |
| 1,000 | 10 | 0.877 | 144.513 | 164.74× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
