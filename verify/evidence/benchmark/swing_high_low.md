# SwingHighLow benchmark (`causal confirmed swing pivots` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.035 | 28.98M | 0.036 | 27.64M | 3.956 | 114.65× | 109.35× |
| 10,000 | 0.385 | 26.01M | 0.371 | 26.98M | 41.523 | 107.99× | 112.04× |
| 100,000 | 3.871 | 25.83M | 3.637 | 27.49M | 407.295 | 105.22× | 111.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.148 | 0.143 | 0.97× |
| 1 | 5 | 0.214 | 0.485 | 2.27× |
| 1 | 10 | 0.466 | 0.871 | 1.87× |
| 10 | 1 | 0.043 | 0.089 | 2.05× |
| 10 | 5 | 0.192 | 0.450 | 2.34× |
| 10 | 10 | 0.451 | 0.884 | 1.96× |
| 100 | 1 | 0.049 | 0.455 | 9.20× |
| 100 | 5 | 0.212 | 2.275 | 10.74× |
| 100 | 10 | 0.456 | 4.631 | 10.15× |
| 1,000 | 1 | 0.086 | 4.098 | 47.41× |
| 1,000 | 5 | 0.282 | 21.941 | 77.92× |
| 1,000 | 10 | 0.605 | 46.764 | 77.28× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
