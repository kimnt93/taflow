# MathFloor benchmark (`FLOOR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 199.29M | 0.004 | 224.76M | 0.027 | 5.33× | 6.01× |
| 10,000 | 0.025 | 397.50M | 0.022 | 446.55M | 0.040 | 1.58× | 1.77× |
| 100,000 | 0.257 | 389.50M | 0.206 | 484.35M | 0.153 | 0.59× | 0.74× |
| 1,000,000 | 2.686 | 372.26M | 2.348 | 425.93M | 1.726 | 0.64× | 0.73× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.064 | 0.092 | 1.43× |
| 1 | 5 | 0.267 | 0.442 | 1.65× |
| 1 | 10 | 0.559 | 0.976 | 1.75× |
| 10 | 1 | 0.048 | 0.087 | 1.80× |
| 10 | 5 | 0.230 | 0.429 | 1.87× |
| 10 | 10 | 0.503 | 0.968 | 1.93× |
| 100 | 1 | 0.049 | 0.092 | 1.87× |
| 100 | 5 | 0.263 | 0.495 | 1.88× |
| 100 | 10 | 0.533 | 0.972 | 1.83× |
| 1,000 | 1 | 0.057 | 0.089 | 1.57× |
| 1,000 | 5 | 0.267 | 0.471 | 1.76× |
| 1,000 | 10 | 0.557 | 1.001 | 1.80× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
