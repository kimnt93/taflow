# Falling benchmark (`period-over-period falling` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 124.48M | 0.007 | 138.77M | 0.033 | 4.06× | 4.53× |
| 10,000 | 0.051 | 197.01M | 0.048 | 208.37M | 0.044 | 0.87× | 0.92× |
| 100,000 | 0.478 | 209.38M | 0.456 | 219.11M | 0.149 | 0.31× | 0.33× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.144 | 0.120 | 0.83× |
| 1 | 5 | 0.269 | 0.473 | 1.76× |
| 1 | 10 | 0.491 | 1.041 | 2.12× |
| 10 | 1 | 0.060 | 0.088 | 1.47× |
| 10 | 5 | 0.273 | 0.520 | 1.91× |
| 10 | 10 | 0.549 | 1.044 | 1.90× |
| 100 | 1 | 0.052 | 0.092 | 1.76× |
| 100 | 5 | 0.240 | 0.506 | 2.11× |
| 100 | 10 | 0.541 | 1.039 | 1.92× |
| 1,000 | 1 | 0.056 | 0.096 | 1.72× |
| 1,000 | 5 | 0.278 | 0.555 | 1.99× |
| 1,000 | 10 | 0.557 | 1.202 | 2.16× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
