# RollingLeadLagCrossCorrelation benchmark (`LeadLagCrossCorrelation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.941 | 1.06M | 0.944 | 1.06M | 1.536 | 1.63× | 1.63× |
| 10,000 | 9.786 | 1.02M | 9.700 | 1.03M | 13.569 | 1.39× | 1.40× |
| 100,000 | 97.002 | 1.03M | 109.214 | 915.64K | 135.926 | 1.40× | 1.24× |
| 1,000,000 | 977.838 | 1.02M | 982.498 | 1.02M | 1419.164 | 1.45× | 1.44× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.312 | 4.08× |
| 1 | 5 | 0.267 | 1.533 | 5.75× |
| 1 | 10 | 0.529 | 2.754 | 5.21× |
| 10 | 1 | 0.054 | 0.262 | 4.84× |
| 10 | 5 | 0.245 | 1.604 | 6.54× |
| 10 | 10 | 0.537 | 2.953 | 5.50× |
| 100 | 1 | 0.118 | 0.380 | 3.22× |
| 100 | 5 | 0.313 | 1.982 | 6.34× |
| 100 | 10 | 0.614 | 8.343 | 13.59× |
| 1,000 | 1 | 1.691 | 2.996 | 1.77× |
| 1,000 | 5 | 4.392 | 18.699 | 4.26× |
| 1,000 | 10 | 2.226 | 19.612 | 8.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
