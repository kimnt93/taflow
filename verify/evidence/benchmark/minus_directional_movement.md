# MinusDirectionalMovement benchmark (`MINUS_DM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.048 | 20.82M | 0.047 | 21.07M | 0.038 | 0.78× | 0.79× |
| 10,000 | 0.341 | 29.35M | 0.343 | 29.12M | 0.080 | 0.23× | 0.23× |
| 100,000 | 3.131 | 31.94M | 3.168 | 31.56M | 0.522 | 0.17× | 0.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.114 | 0.110 | 0.97× |
| 1 | 5 | 0.420 | 0.484 | 1.15× |
| 1 | 10 | 0.584 | 0.931 | 1.59× |
| 10 | 1 | 0.065 | 0.101 | 1.57× |
| 10 | 5 | 0.297 | 0.448 | 1.50× |
| 10 | 10 | 0.623 | 1.005 | 1.61× |
| 100 | 1 | 0.071 | 0.091 | 1.28× |
| 100 | 5 | 0.308 | 0.437 | 1.42× |
| 100 | 10 | 0.635 | 0.953 | 1.50× |
| 1,000 | 1 | 0.111 | 0.100 | 0.90× |
| 1,000 | 5 | 0.301 | 0.501 | 1.66× |
| 1,000 | 10 | 0.652 | 1.028 | 1.58× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
