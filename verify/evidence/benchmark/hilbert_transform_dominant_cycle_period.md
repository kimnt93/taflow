# HilbertTransformDominantCyclePeriod benchmark (`HT_DCPERIOD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.052 | 19.12M | 0.057 | 17.66M | 0.085 | 1.62× | 1.49× |
| 10,000 | 0.511 | 19.56M | 0.511 | 19.57M | 0.525 | 1.03× | 1.03× |
| 100,000 | 5.308 | 18.84M | 5.011 | 19.96M | 4.922 | 0.93× | 0.98× |
| 1,000,000 | 53.919 | 18.55M | 51.568 | 19.39M | 47.899 | 0.89× | 0.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.155 | 1.45× |
| 1 | 5 | 0.325 | 0.548 | 1.69× |
| 1 | 10 | 0.594 | 1.303 | 2.19× |
| 10 | 1 | 0.066 | 0.162 | 2.46× |
| 10 | 5 | 0.379 | 0.600 | 1.59× |
| 10 | 10 | 0.682 | 1.343 | 1.97× |
| 100 | 1 | 0.060 | 0.140 | 2.31× |
| 100 | 5 | 0.335 | 0.676 | 2.02× |
| 100 | 10 | 0.671 | 1.340 | 2.00× |
| 1,000 | 1 | 0.113 | 0.199 | 1.76× |
| 1,000 | 5 | 0.349 | 0.870 | 2.49× |
| 1,000 | 10 | 0.669 | 1.748 | 2.61× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
