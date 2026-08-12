# BatPattern benchmark (`Bat` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 65.05M | 0.012 | 80.30M | 0.233 | 15.13× | 18.68× |
| 10,000 | 0.104 | 95.81M | 0.098 | 101.82M | 1.408 | 13.49× | 14.34× |
| 100,000 | 0.940 | 106.40M | 0.903 | 110.72M | 13.234 | 14.08× | 14.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.125 | 0.202 | 1.61× |
| 1 | 5 | 0.261 | 0.842 | 3.23× |
| 1 | 10 | 0.576 | 1.709 | 2.97× |
| 10 | 1 | 0.056 | 0.168 | 3.00× |
| 10 | 5 | 0.252 | 1.122 | 4.44× |
| 10 | 10 | 0.544 | 1.741 | 3.20× |
| 100 | 1 | 0.053 | 0.184 | 3.45× |
| 100 | 5 | 0.281 | 1.234 | 4.39× |
| 100 | 10 | 0.590 | 1.852 | 3.14× |
| 1,000 | 1 | 0.071 | 0.317 | 4.46× |
| 1,000 | 5 | 0.266 | 1.804 | 6.79× |
| 1,000 | 10 | 0.535 | 3.093 | 5.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
