# KlingerVolumeOscillator benchmark (`KVO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 51.83M | 0.018 | 54.35M | 0.302 | 15.63× | 16.39× |
| 10,000 | 0.150 | 66.80M | 0.142 | 70.37M | 1.419 | 9.48× | 9.99× |
| 100,000 | 1.389 | 72.02M | 1.358 | 73.64M | 12.861 | 9.26× | 9.47× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.305 | 2.61× |
| 1 | 5 | 0.314 | 1.472 | 4.68× |
| 1 | 10 | 0.420 | 2.660 | 6.33× |
| 10 | 1 | 0.050 | 0.254 | 5.11× |
| 10 | 5 | 0.190 | 1.596 | 8.41× |
| 10 | 10 | 0.411 | 2.831 | 6.89× |
| 100 | 1 | 0.051 | 0.257 | 5.01× |
| 100 | 5 | 0.220 | 1.570 | 7.12× |
| 100 | 10 | 0.427 | 2.744 | 6.43× |
| 1,000 | 1 | 0.063 | 0.382 | 6.05× |
| 1,000 | 5 | 0.210 | 2.181 | 10.39× |
| 1,000 | 10 | 0.431 | 4.168 | 9.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
