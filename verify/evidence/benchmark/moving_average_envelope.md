# MovingAverageEnvelope benchmark (`MaEnvelope` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.026 | 38.94M | 0.025 | 40.57M | 0.635 | 24.72× | 25.75× |
| 10,000 | 0.244 | 40.99M | 0.198 | 50.54M | 3.800 | 15.58× | 19.21× |
| 100,000 | 2.147 | 46.59M | 1.907 | 52.45M | 44.223 | 20.60× | 23.19× |
| 1,000,000 | 21.519 | 46.47M | 23.147 | 43.20M | 450.906 | 20.95× | 19.48× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.302 | 4.13× |
| 1 | 5 | 0.374 | 1.506 | 4.02× |
| 1 | 10 | 0.521 | 2.730 | 5.23× |
| 10 | 1 | 0.063 | 0.264 | 4.16× |
| 10 | 5 | 0.228 | 1.456 | 6.39× |
| 10 | 10 | 0.488 | 2.959 | 6.07× |
| 100 | 1 | 0.058 | 0.286 | 4.97× |
| 100 | 5 | 0.241 | 1.755 | 7.29× |
| 100 | 10 | 0.531 | 3.156 | 5.94× |
| 1,000 | 1 | 0.072 | 0.848 | 11.84× |
| 1,000 | 5 | 0.287 | 3.532 | 12.30× |
| 1,000 | 10 | 0.570 | 6.889 | 12.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
