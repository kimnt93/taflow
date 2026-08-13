# McGinleyDynamic benchmark (`McGinleyDynamic` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.039 | 25.74M | 0.037 | 27.07M | 0.178 | 4.58× | 4.81× |
| 10,000 | 0.270 | 37.06M | 0.264 | 37.83M | 0.534 | 1.98× | 2.02× |
| 100,000 | 2.569 | 38.92M | 2.669 | 37.47M | 4.139 | 1.61× | 1.55× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.181 | 0.263 | 1.45× |
| 1 | 5 | 0.429 | 1.397 | 3.25× |
| 1 | 10 | 0.583 | 2.412 | 4.14× |
| 10 | 1 | 0.073 | 0.225 | 3.08× |
| 10 | 5 | 0.284 | 1.380 | 4.85× |
| 10 | 10 | 0.592 | 2.335 | 3.94× |
| 100 | 1 | 0.074 | 0.219 | 2.97× |
| 100 | 5 | 0.299 | 1.503 | 5.03× |
| 100 | 10 | 0.594 | 2.352 | 3.96× |
| 1,000 | 1 | 0.100 | 0.258 | 2.57× |
| 1,000 | 5 | 0.350 | 1.641 | 4.69× |
| 1,000 | 10 | 0.595 | 2.785 | 4.68× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
