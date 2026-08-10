# CandleDragonflyDoji benchmark (`CDLDRAGONFLYDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 65.04M | 0.011 | 87.97M | 0.038 | 2.49× | 3.36× |
| 10,000 | 0.083 | 121.20M | 0.080 | 124.54M | 0.103 | 1.25× | 1.28× |
| 100,000 | 0.887 | 112.79M | 0.897 | 111.54M | 0.868 | 0.98× | 0.97× |
| 1,000,000 | 9.503 | 105.23M | 8.863 | 112.83M | 8.557 | 0.90× | 0.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | 0.337 | 2.81× |
| 1 | 5 | 0.319 | 0.510 | 1.60× |
| 1 | 10 | 0.564 | 0.941 | 1.67× |
| 10 | 1 | 0.052 | 0.088 | 1.68× |
| 10 | 5 | 0.302 | 0.496 | 1.64× |
| 10 | 10 | 0.618 | 0.998 | 1.62× |
| 100 | 1 | 0.062 | 0.094 | 1.53× |
| 100 | 5 | 0.262 | 0.429 | 1.64× |
| 100 | 10 | 0.584 | 1.119 | 1.92× |
| 1,000 | 1 | 0.082 | 0.120 | 1.47× |
| 1,000 | 5 | 0.316 | 0.499 | 1.58× |
| 1,000 | 10 | 0.559 | 1.157 | 2.07× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
