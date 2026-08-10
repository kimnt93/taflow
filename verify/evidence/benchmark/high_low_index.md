# HighLowIndex benchmark (`HighLowIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 100.13M | 0.009 | 115.89M | 8.599 | 860.96× | 996.55× |
| 10,000 | 0.062 | 162.16M | 0.059 | 170.07M | 84.567 | 1371.35× | 1438.23× |
| 100,000 | 0.569 | 175.82M | 0.553 | 180.67M | 842.110 | 1480.56× | 1521.48× |
| 1,000,000 | 5.998 | 166.72M | 5.739 | 174.24M | 8159.193 | 1360.32× | 1421.62× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.150 | 0.271 | 1.81× |
| 1 | 5 | 0.398 | 1.524 | 3.83× |
| 1 | 10 | 0.495 | 2.563 | 5.18× |
| 10 | 1 | 0.048 | 0.324 | 6.74× |
| 10 | 5 | 0.244 | 1.566 | 6.42× |
| 10 | 10 | 0.495 | 3.334 | 6.74× |
| 100 | 1 | 0.050 | 1.069 | 21.21× |
| 100 | 5 | 0.242 | 5.476 | 22.67× |
| 100 | 10 | 0.494 | 11.077 | 22.42× |
| 1,000 | 1 | 0.065 | 8.559 | 132.18× |
| 1,000 | 5 | 0.304 | 46.780 | 154.06× |
| 1,000 | 10 | 0.644 | 90.540 | 140.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
