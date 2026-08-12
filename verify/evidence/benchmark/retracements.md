# Retracements benchmark (`causal swing retracements` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.34M | 0.061 | 16.26M | 5.080 | 103.36× | 82.63× |
| 10,000 | 0.437 | 22.88M | 0.432 | 23.13M | 52.509 | 120.16× | 121.44× |
| 100,000 | 4.404 | 22.71M | 4.299 | 23.26M | 496.088 | 112.65× | 115.40× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.131 | 0.142 | 1.09× |
| 1 | 5 | 0.314 | 0.564 | 1.80× |
| 1 | 10 | 0.569 | 0.993 | 1.75× |
| 10 | 1 | 0.061 | 0.101 | 1.65× |
| 10 | 5 | 0.269 | 0.590 | 2.19× |
| 10 | 10 | 0.560 | 1.049 | 1.87× |
| 100 | 1 | 0.063 | 0.534 | 8.42× |
| 100 | 5 | 0.283 | 2.800 | 9.90× |
| 100 | 10 | 0.601 | 5.383 | 8.95× |
| 1,000 | 1 | 0.106 | 4.751 | 44.66× |
| 1,000 | 5 | 0.307 | 29.675 | 96.57× |
| 1,000 | 10 | 1.105 | 50.307 | 45.53× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
