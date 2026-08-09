# RollingMaximumDrawdown focused comparison

Correctness: **MATCH** against independent pandas and Wickra 0.9.9. TA-Lib: **N/A** (no `MAXDRAWDOWN` function).

| Bars | TAFlow ms | pandas ms | Wickra ms | vs pandas | vs Wickra |
|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 7.958 | 0.112 | 169.83× | 2.40× |
| 10,000 | 0.422 | 77.132 | 1.101 | 182.70× | 2.61× |
| 100,000 | 4.632 | 756.901 | 12.039 | 163.42× | 2.60× |
| 1,000,000 | 45.755 | 7771.536 | 123.824 | 169.85× | 2.71× |

Times are median fresh-state class/batch calls after one timing warm-up over deterministic positive equity data; speedup is reference time divided by TAFlow time.
