# SwingHighLow benchmark (`causal confirmed swing pivots` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.036 | 27.68M | 0.033 | 30.69M | 3.779 | 104.61× | 115.95× |
| 10,000 | 0.367 | 27.23M | 0.352 | 28.38M | 39.498 | 107.55× | 112.12× |
| 100,000 | 4.089 | 24.45M | 3.459 | 28.91M | 396.410 | 96.94× | 114.61× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.139 | 0.160 | 1.15× |
| 1 | 5 | 0.320 | 0.468 | 1.46× |
| 1 | 10 | 0.404 | 0.897 | 2.22× |
| 10 | 1 | 0.042 | 0.083 | 1.96× |
| 10 | 5 | 0.193 | 0.424 | 2.20× |
| 10 | 10 | 0.416 | 0.901 | 2.17× |
| 100 | 1 | 0.053 | 0.450 | 8.45× |
| 100 | 5 | 0.216 | 2.278 | 10.57× |
| 100 | 10 | 0.472 | 4.420 | 9.36× |
| 1,000 | 1 | 0.089 | 4.101 | 46.23× |
| 1,000 | 5 | 0.322 | 23.241 | 72.22× |
| 1,000 | 10 | 0.649 | 48.768 | 75.19× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
