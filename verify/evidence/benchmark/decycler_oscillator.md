# DecyclerOscillator benchmark (`DecyclerOscillator` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 87.46M | 0.010 | 103.39M | 0.183 | 16.03× | 18.95× |
| 10,000 | 0.090 | 110.51M | 0.084 | 119.68M | 0.538 | 5.95× | 6.44× |
| 100,000 | 0.825 | 121.25M | 0.822 | 121.66M | 4.054 | 4.92× | 4.93× |
| 1,000,000 | 8.504 | 117.60M | 8.197 | 122.00M | 37.905 | 4.46× | 4.62× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.116 | 0.307 | 2.65× |
| 1 | 5 | 0.249 | 1.095 | 4.40× |
| 1 | 10 | 0.468 | 2.359 | 5.04× |
| 10 | 1 | 0.060 | 0.223 | 3.71× |
| 10 | 5 | 0.224 | 1.057 | 4.73× |
| 10 | 10 | 0.484 | 2.421 | 5.00× |
| 100 | 1 | 0.061 | 0.210 | 3.44× |
| 100 | 5 | 0.239 | 1.085 | 4.54× |
| 100 | 10 | 0.509 | 2.375 | 4.67× |
| 1,000 | 1 | 0.081 | 0.271 | 3.33× |
| 1,000 | 5 | 0.263 | 1.329 | 5.06× |
| 1,000 | 10 | 0.534 | 2.741 | 5.13× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
