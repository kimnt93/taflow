# FracDiff benchmark (`fixed-width fractional differencing` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.071 | 13.99M | 0.068 | 14.81M | 0.294 | 4.12× | 4.36× |
| 10,000 | 7.394 | 1.35M | 7.040 | 1.42M | 7.685 | 1.04× | 1.09× |
| 100,000 | 78.189 | 1.28M | 79.090 | 1.26M | 87.088 | 1.11× | 1.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.143 | 0.394 | 2.75× |
| 1 | 5 | 0.318 | 1.456 | 4.58× |
| 1 | 10 | 0.470 | 2.783 | 5.92× |
| 10 | 1 | 0.054 | 0.289 | 5.35× |
| 10 | 5 | 0.224 | 1.381 | 6.16× |
| 10 | 10 | 0.471 | 2.767 | 5.87× |
| 100 | 1 | 0.049 | 0.278 | 5.64× |
| 100 | 5 | 0.225 | 1.377 | 6.11× |
| 100 | 10 | 0.500 | 2.803 | 5.60× |
| 1,000 | 1 | 0.119 | 0.392 | 3.30× |
| 1,000 | 5 | 0.269 | 1.890 | 7.01× |
| 1,000 | 10 | 0.546 | 3.816 | 6.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
