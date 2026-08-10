# IntradayVolatilityProfile benchmark (`IntradayVolatilityProfile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.067 | 14.95M | 0.061 | 16.28M | 1.721 | 25.72× | 28.02× |
| 10,000 | 0.599 | 16.71M | 0.537 | 18.61M | 14.905 | 24.90× | 27.74× |
| 100,000 | 6.522 | 15.33M | 5.157 | 19.39M | 186.621 | 28.61× | 36.19× |
| 1,000,000 | 165.371 | 6.05M | 100.780 | 9.92M | 1859.632 | 11.25× | 18.45× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.320 | 3.32× |
| 1 | 5 | 0.452 | 1.507 | 3.33× |
| 1 | 10 | 0.586 | 2.891 | 4.93× |
| 10 | 1 | 0.066 | 0.284 | 4.29× |
| 10 | 5 | 0.271 | 1.596 | 5.90× |
| 10 | 10 | 0.598 | 3.101 | 5.19× |
| 100 | 1 | 0.063 | 0.424 | 6.68× |
| 100 | 5 | 0.312 | 2.233 | 7.17× |
| 100 | 10 | 0.596 | 4.436 | 7.44× |
| 1,000 | 1 | 0.122 | 1.988 | 16.27× |
| 1,000 | 5 | 0.332 | 9.637 | 28.99× |
| 1,000 | 10 | 0.667 | 19.545 | 29.29× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
