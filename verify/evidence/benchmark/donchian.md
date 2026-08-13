# Donchian benchmark (`Donchian` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.108 | 9.23M | 0.100 | 9.98M | 0.565 | 5.21× | 5.64× |
| 10,000 | 0.887 | 11.27M | 0.934 | 10.71M | 4.280 | 4.82× | 4.58× |
| 100,000 | 8.706 | 11.49M | 8.613 | 11.61M | 45.710 | 5.25× | 5.31× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.288 | 2.69× |
| 1 | 5 | 0.407 | 1.119 | 2.75× |
| 1 | 10 | 0.655 | 2.375 | 3.62× |
| 10 | 1 | 0.076 | 0.226 | 2.99× |
| 10 | 5 | 0.315 | 1.406 | 4.47× |
| 10 | 10 | 0.646 | 2.452 | 3.80× |
| 100 | 1 | 0.092 | 0.273 | 2.95× |
| 100 | 5 | 0.321 | 1.699 | 5.30× |
| 100 | 10 | 0.690 | 2.922 | 4.24× |
| 1,000 | 1 | 0.177 | 0.937 | 5.30× |
| 1,000 | 5 | 0.375 | 3.682 | 9.82× |
| 1,000 | 10 | 0.687 | 7.584 | 11.03× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
