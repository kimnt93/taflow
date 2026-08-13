# CandleDragonflyDoji benchmark (`CDLDRAGONFLYDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.066 | 15.24M | 0.057 | 17.42M | 0.033 | 0.51× | 0.58× |
| 10,000 | 0.487 | 20.55M | 0.475 | 21.06M | 0.098 | 0.20× | 0.21× |
| 100,000 | 4.586 | 21.80M | 4.527 | 22.09M | 0.722 | 0.16× | 0.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | 0.145 | 1.20× |
| 1 | 5 | 0.449 | 0.482 | 1.07× |
| 1 | 10 | 0.698 | 0.952 | 1.36× |
| 10 | 1 | 0.073 | 0.088 | 1.21× |
| 10 | 5 | 0.298 | 0.433 | 1.45× |
| 10 | 10 | 0.676 | 0.899 | 1.33× |
| 100 | 1 | 0.073 | 0.085 | 1.16× |
| 100 | 5 | 0.314 | 0.428 | 1.36× |
| 100 | 10 | 0.670 | 0.931 | 1.39× |
| 1,000 | 1 | 0.116 | 0.102 | 0.88× |
| 1,000 | 5 | 0.319 | 0.465 | 1.46× |
| 1,000 | 10 | 0.684 | 0.980 | 1.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
