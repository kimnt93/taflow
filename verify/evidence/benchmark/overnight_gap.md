# OvernightGap benchmark (`OvernightGap` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 65.82M | 0.012 | 86.17M | 0.360 | 23.68× | 31.00× |
| 10,000 | 0.052 | 192.45M | 0.046 | 216.65M | 2.317 | 44.58× | 50.19× |
| 100,000 | 0.450 | 222.35M | 0.394 | 253.79M | 22.641 | 50.34× | 57.46× |
| 1,000,000 | 5.923 | 168.84M | 6.120 | 163.40M | 270.268 | 45.63× | 44.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.507 | 5.81× |
| 1 | 5 | 0.370 | 1.201 | 3.25× |
| 1 | 10 | 0.590 | 2.710 | 4.59× |
| 10 | 1 | 0.076 | 0.244 | 3.19× |
| 10 | 5 | 0.292 | 1.236 | 4.24× |
| 10 | 10 | 0.672 | 2.849 | 4.24× |
| 100 | 1 | 0.067 | 0.273 | 4.06× |
| 100 | 5 | 0.315 | 1.646 | 5.23× |
| 100 | 10 | 0.675 | 2.990 | 4.43× |
| 1,000 | 1 | 0.074 | 0.495 | 6.73× |
| 1,000 | 5 | 0.313 | 2.644 | 8.45× |
| 1,000 | 10 | 0.677 | 5.099 | 7.53× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
