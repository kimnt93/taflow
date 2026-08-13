# RollingLeadLagCrossCorrelation benchmark (`LeadLagCrossCorrelation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 8.425 | 118.70K | 8.421 | 118.75K | 1.389 | 0.16× | 0.16× |
| 10,000 | 91.876 | 108.84K | 97.539 | 102.52K | 12.359 | 0.13× | 0.13× |
| 100,000 | 876.866 | 114.04K | 866.273 | 115.44K | 126.181 | 0.14× | 0.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.123 | 0.298 | 2.42× |
| 1 | 5 | 0.422 | 7.841 | 18.57× |
| 1 | 10 | 0.742 | 2.595 | 3.50× |
| 10 | 1 | 0.079 | 0.253 | 3.21× |
| 10 | 5 | 0.313 | 1.384 | 4.43× |
| 10 | 10 | 0.621 | 2.584 | 4.16× |
| 100 | 1 | 0.639 | 0.345 | 0.54× |
| 100 | 5 | 0.793 | 1.862 | 2.35× |
| 100 | 10 | 1.557 | 3.702 | 2.38× |
| 1,000 | 1 | 8.932 | 1.710 | 0.19× |
| 1,000 | 5 | 12.638 | 8.416 | 0.67× |
| 1,000 | 10 | 17.859 | 16.449 | 0.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
