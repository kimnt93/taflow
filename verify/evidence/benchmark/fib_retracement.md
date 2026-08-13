# FibonacciRetracement benchmark (`rolling Fibonacci levels` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.151 | 6.62M | 0.132 | 7.57M | 10.629 | 70.40× | 80.48× |
| 10,000 | 1.363 | 7.34M | 1.269 | 7.88M | 105.033 | 77.04× | 82.75× |
| 100,000 | 13.787 | 7.25M | 12.679 | 7.89M | 1066.953 | 77.39× | 84.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.182 | 0.163 | 0.90× |
| 1 | 5 | 0.435 | 0.547 | 1.26× |
| 1 | 10 | 0.616 | 1.079 | 1.75× |
| 10 | 1 | 0.068 | 0.220 | 3.23× |
| 10 | 5 | 0.300 | 1.047 | 3.49× |
| 10 | 10 | 0.616 | 2.150 | 3.49× |
| 100 | 1 | 0.084 | 1.225 | 14.54× |
| 100 | 5 | 0.298 | 6.205 | 20.80× |
| 100 | 10 | 0.668 | 12.515 | 18.74× |
| 1,000 | 1 | 0.217 | 11.485 | 52.92× |
| 1,000 | 5 | 0.528 | 64.007 | 121.18× |
| 1,000 | 10 | 0.966 | 252.528 | 261.40× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
