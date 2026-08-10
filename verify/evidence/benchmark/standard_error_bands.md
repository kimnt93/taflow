# StandardErrorBands benchmark (`StandardErrorBands` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.077 | 12.98M | 0.073 | 13.71M | 0.631 | 8.19× | 8.66× |
| 10,000 | 0.738 | 13.54M | 0.698 | 14.32M | 4.501 | 6.10× | 6.45× |
| 100,000 | 6.870 | 14.56M | 7.181 | 13.92M | 44.140 | 6.43× | 6.15× |
| 1,000,000 | 77.802 | 12.85M | 70.316 | 14.22M | 553.814 | 7.12× | 7.88× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.352 | 4.92× |
| 1 | 5 | 0.281 | 1.815 | 6.45× |
| 1 | 10 | 0.519 | 2.897 | 5.58× |
| 10 | 1 | 0.051 | 0.283 | 5.51× |
| 10 | 5 | 0.245 | 1.508 | 6.15× |
| 10 | 10 | 0.565 | 3.218 | 5.69× |
| 100 | 1 | 0.083 | 0.328 | 3.94× |
| 100 | 5 | 0.254 | 1.689 | 6.66× |
| 100 | 10 | 0.535 | 3.380 | 6.32× |
| 1,000 | 1 | 0.131 | 0.873 | 6.66× |
| 1,000 | 5 | 0.314 | 4.231 | 13.48× |
| 1,000 | 10 | 0.590 | 9.040 | 15.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
