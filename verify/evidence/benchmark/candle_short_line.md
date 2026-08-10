# CandleShortLine benchmark (`CDLSHORTLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 57.87M | 0.014 | 70.30M | 0.034 | 1.94× | 2.36× |
| 10,000 | 0.153 | 65.28M | 0.151 | 66.43M | 0.201 | 1.31× | 1.34× |
| 100,000 | 1.520 | 65.80M | 1.597 | 62.61M | 1.721 | 1.13× | 1.08× |
| 1,000,000 | 16.590 | 60.28M | 15.896 | 62.91M | 16.911 | 1.02× | 1.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.108 | 1.16× |
| 1 | 5 | 0.375 | 0.479 | 1.28× |
| 1 | 10 | 0.566 | 0.913 | 1.61× |
| 10 | 1 | 0.054 | 0.083 | 1.54× |
| 10 | 5 | 0.245 | 0.429 | 1.75× |
| 10 | 10 | 0.521 | 0.913 | 1.75× |
| 100 | 1 | 0.064 | 0.096 | 1.50× |
| 100 | 5 | 0.266 | 0.445 | 1.67× |
| 100 | 10 | 0.544 | 0.908 | 1.67× |
| 1,000 | 1 | 0.074 | 0.110 | 1.49× |
| 1,000 | 5 | 0.275 | 0.512 | 1.86× |
| 1,000 | 10 | 0.565 | 1.109 | 1.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
