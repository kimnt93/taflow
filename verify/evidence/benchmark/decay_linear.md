# DecayLinear benchmark (`linear decay weighted mean` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 259.46M | 0.003 | 328.05M | 0.086 | 22.23× | 28.11× |
| 10,000 | 0.025 | 400.11M | 0.022 | 450.24M | 0.279 | 11.15× | 12.55× |
| 100,000 | 0.235 | 425.04M | 0.219 | 456.59M | 2.179 | 9.26× | 9.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.071 | 0.184 | 2.58× |
| 1 | 5 | 0.330 | 0.596 | 1.80× |
| 1 | 10 | 0.381 | 1.162 | 3.05× |
| 10 | 1 | 0.051 | 0.105 | 2.05× |
| 10 | 5 | 0.196 | 0.592 | 3.02× |
| 10 | 10 | 0.457 | 2.406 | 5.27× |
| 100 | 1 | 0.068 | 0.226 | 3.32× |
| 100 | 5 | 0.279 | 1.088 | 3.90× |
| 100 | 10 | 0.512 | 2.281 | 4.46× |
| 1,000 | 1 | 0.070 | 0.244 | 3.47× |
| 1,000 | 5 | 0.298 | 1.197 | 4.02× |
| 1,000 | 10 | 0.665 | 2.296 | 3.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
