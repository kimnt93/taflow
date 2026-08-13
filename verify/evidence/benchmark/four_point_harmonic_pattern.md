# FourPointHarmonicPattern benchmark (`Abcd` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.061 | 16.50M | 0.052 | 19.08M | 0.221 | 3.64× | 4.21× |
| 10,000 | 0.403 | 24.82M | 0.394 | 25.35M | 1.408 | 3.49× | 3.57× |
| 100,000 | 4.081 | 24.50M | 3.996 | 25.03M | 14.697 | 3.60× | 3.68× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.122 | 0.196 | 1.61× |
| 1 | 5 | 0.439 | 0.817 | 1.86× |
| 1 | 10 | 0.633 | 1.646 | 2.60× |
| 10 | 1 | 0.069 | 0.164 | 2.38× |
| 10 | 5 | 0.315 | 1.082 | 3.43× |
| 10 | 10 | 0.668 | 1.673 | 2.50× |
| 100 | 1 | 0.073 | 0.178 | 2.43× |
| 100 | 5 | 0.316 | 1.124 | 3.55× |
| 100 | 10 | 0.660 | 1.783 | 2.70× |
| 1,000 | 1 | 0.124 | 0.300 | 2.42× |
| 1,000 | 5 | 0.321 | 1.751 | 5.45× |
| 1,000 | 10 | 0.691 | 3.006 | 4.35× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
