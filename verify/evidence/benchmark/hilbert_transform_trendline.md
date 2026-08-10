# HilbertTransformTrendline benchmark (`HT_TRENDLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.077 | 12.95M | 0.079 | 12.66M | 0.088 | 1.14× | 1.11× |
| 10,000 | 0.783 | 12.77M | 0.754 | 13.26M | 0.657 | 0.84× | 0.87× |
| 100,000 | 7.874 | 12.70M | 7.711 | 12.97M | 6.323 | 0.80× | 0.82× |
| 1,000,000 | 77.400 | 12.92M | 76.066 | 13.15M | 63.680 | 0.82× | 0.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.154 | 1.42× |
| 1 | 5 | 0.291 | 0.506 | 1.74× |
| 1 | 10 | 0.502 | 0.936 | 1.87× |
| 10 | 1 | 0.046 | 0.089 | 1.96× |
| 10 | 5 | 0.255 | 0.545 | 2.14× |
| 10 | 10 | 0.550 | 0.911 | 1.66× |
| 100 | 1 | 0.056 | 0.092 | 1.62× |
| 100 | 5 | 0.239 | 0.508 | 2.12× |
| 100 | 10 | 0.630 | 1.119 | 1.78× |
| 1,000 | 1 | 0.124 | 0.165 | 1.33× |
| 1,000 | 5 | 0.297 | 0.764 | 2.58× |
| 1,000 | 10 | 0.843 | 1.681 | 1.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
