# CandleUpsideGapTwoCrows benchmark (`CDLUPSIDEGAP2CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.09M | 0.016 | 60.66M | 0.034 | 1.76× | 2.04× |
| 10,000 | 0.137 | 73.16M | 0.129 | 77.43M | 0.122 | 0.89× | 0.94× |
| 100,000 | 1.288 | 77.64M | 1.279 | 78.17M | 1.028 | 0.80× | 0.80× |
| 1,000,000 | 13.564 | 73.72M | 13.508 | 74.03M | 9.991 | 0.74× | 0.74× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.168 | 0.144 | 0.86× |
| 1 | 5 | 0.429 | 0.460 | 1.07× |
| 1 | 10 | 0.534 | 0.935 | 1.75× |
| 10 | 1 | 0.056 | 0.091 | 1.60× |
| 10 | 5 | 0.288 | 0.446 | 1.55× |
| 10 | 10 | 0.603 | 0.911 | 1.51× |
| 100 | 1 | 0.054 | 0.089 | 1.65× |
| 100 | 5 | 0.260 | 0.451 | 1.73× |
| 100 | 10 | 0.558 | 0.918 | 1.65× |
| 1,000 | 1 | 0.065 | 0.096 | 1.47× |
| 1,000 | 5 | 0.273 | 0.496 | 1.82× |
| 1,000 | 10 | 0.609 | 1.082 | 1.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
