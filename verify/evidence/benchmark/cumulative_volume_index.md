# CumulativeVolumeIndex benchmark (`CumulativeVolumeIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 146.93M | 0.006 | 180.28M | 4.316 | 634.15× | 778.08× |
| 10,000 | 0.031 | 324.08M | 0.027 | 367.06M | 42.110 | 1364.70× | 1545.69× |
| 100,000 | 0.267 | 374.43M | 0.364 | 274.57M | 429.056 | 1606.53× | 1178.08× |
| 1,000,000 | 2.805 | 356.50M | 2.485 | 402.39M | 4212.951 | 1501.93× | 1695.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.124 | 0.299 | 2.41× |
| 1 | 5 | 0.332 | 0.963 | 2.90× |
| 1 | 10 | 0.491 | 2.039 | 4.16× |
| 10 | 1 | 0.047 | 0.240 | 5.12× |
| 10 | 5 | 0.233 | 1.159 | 4.98× |
| 10 | 10 | 0.467 | 2.644 | 5.66× |
| 100 | 1 | 0.057 | 0.615 | 10.73× |
| 100 | 5 | 0.262 | 3.100 | 11.82× |
| 100 | 10 | 0.496 | 6.542 | 13.19× |
| 1,000 | 1 | 0.056 | 4.654 | 82.86× |
| 1,000 | 5 | 0.330 | 33.009 | 100.11× |
| 1,000 | 10 | 1.116 | 58.074 | 52.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
