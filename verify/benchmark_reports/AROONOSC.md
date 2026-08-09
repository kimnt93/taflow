# AroonOscillator benchmark (`AROONOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 81.70M | 0.011 | 91.62M | 0.038 | 3.09× | 3.46× |
| 10,000 | 0.129 | 77.73M | 0.120 | 83.56M | 0.129 | 1.00× | 1.08× |
| 100,000 | 1.175 | 85.12M | 1.153 | 86.71M | 1.017 | 0.87× | 0.88× |
| 1,000,000 | 12.291 | 81.36M | 11.749 | 85.11M | 10.253 | 0.83× | 0.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.125 | 1.55× |
| 1 | 5 | 0.285 | 0.514 | 1.80× |
| 1 | 10 | 0.494 | 0.983 | 1.99× |
| 10 | 1 | 0.056 | 0.094 | 1.68× |
| 10 | 5 | 0.238 | 0.465 | 1.95× |
| 10 | 10 | 0.508 | 1.010 | 1.99× |
| 100 | 1 | 0.052 | 0.094 | 1.81× |
| 100 | 5 | 0.244 | 0.476 | 1.95× |
| 100 | 10 | 0.546 | 0.995 | 1.82× |
| 1,000 | 1 | 0.065 | 0.106 | 1.63× |
| 1,000 | 5 | 0.321 | 0.672 | 2.09× |
| 1,000 | 10 | 0.525 | 1.076 | 2.05× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.331 | 0.229 | 4.37M | 1021.983 | 4462.08× | 139.03× |
| 100,000 | 10 | 1.875 | 1.129 | 8.86M | 1058.714 | 937.70× | 27.15× |
| 100,000 | 1,000 | 30.170 | 30.988 | 32.27M | 1017.826 | 32.85× | 1.23× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 72.33M | 74.14M | 1.00× | 2.55M | 3.02M | 1.00× | 82.43M |
| 5 | 260.72M | 330.34M | 4.46× | 2.20M | 2.30M | 0.76× | 80.89M |
| 10 | 349.52M | 421.27M | 5.68× | 1.99M | 2.40M | 0.79× | 83.00M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
