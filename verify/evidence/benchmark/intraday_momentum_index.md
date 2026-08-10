# IntradayMomentumIndex benchmark (`IMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 59.57M | 0.016 | 63.81M | 0.082 | 4.86× | 5.21× |
| 10,000 | 0.133 | 75.39M | 0.123 | 81.39M | 0.595 | 4.48× | 4.84× |
| 100,000 | 1.202 | 83.18M | 1.218 | 82.12M | 5.564 | 4.63× | 4.57× |
| 1,000,000 | 12.509 | 79.94M | 12.608 | 79.32M | 57.138 | 4.57× | 4.53× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.145 | 0.177 | 1.22× |
| 1 | 5 | 0.348 | 0.479 | 1.38× |
| 1 | 10 | 0.496 | 0.937 | 1.89× |
| 10 | 1 | 0.050 | 0.096 | 1.92× |
| 10 | 5 | 0.232 | 0.446 | 1.93× |
| 10 | 10 | 0.492 | 0.958 | 1.95× |
| 100 | 1 | 0.050 | 0.105 | 2.09× |
| 100 | 5 | 0.254 | 0.484 | 1.91× |
| 100 | 10 | 0.507 | 0.990 | 1.95× |
| 1,000 | 1 | 0.063 | 0.153 | 2.45× |
| 1,000 | 5 | 0.235 | 0.747 | 3.18× |
| 1,000 | 10 | 0.507 | 1.826 | 3.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
