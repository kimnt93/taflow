# MathAbs benchmark (`numpy.abs` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 534.14M | 0.001 | 1.18G | 0.012 | 6.66× | 14.76× |
| 10,000 | 0.006 | 1.81G | 0.003 | 3.19G | 0.016 | 2.83× | 4.97× |
| 100,000 | 0.059 | 1.68G | 0.029 | 3.39G | 0.044 | 0.74× | 1.50× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.137 | 0.067 | 0.49× |
| 1 | 5 | 0.241 | 0.281 | 1.16× |
| 1 | 10 | 0.380 | 0.570 | 1.50× |
| 10 | 1 | 0.040 | 0.062 | 1.54× |
| 10 | 5 | 0.209 | 0.280 | 1.34× |
| 10 | 10 | 0.394 | 0.571 | 1.45× |
| 100 | 1 | 0.042 | 0.056 | 1.33× |
| 100 | 5 | 0.176 | 0.258 | 1.47× |
| 100 | 10 | 0.368 | 0.623 | 1.69× |
| 1,000 | 1 | 0.046 | 0.063 | 1.37× |
| 1,000 | 5 | 0.178 | 0.276 | 1.55× |
| 1,000 | 10 | 0.401 | 0.582 | 1.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
