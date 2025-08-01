# Performance Improvement PR Template

<!--
This template is designed for PRs focused on performance improvements.
Fill in relevant sections and delete any that don't apply to your specific changes.
Provide metrics where possible to clearly demonstrate the impact of your changes.
-->

## Performance Summary
<!-- Provide a brief description of the performance improvements implemented -->

## Performance Issue
<!-- What performance issue does this PR address? -->

### Current Metrics

- Response time: ___ ms
- Throughput: ___ requests/second
- Memory usage: ___ MB
- CPU utilization: ___%
- Other relevant metrics: ___

### User/System Impact
<!-- Describe how this issue affects users or system operations -->

## Optimization Approach
<!-- Check all strategies that apply to your changes -->
- [ ] Algorithm optimization
- [ ] Database query optimization
- [ ] Caching implementation
- [ ] Resource loading optimization
- [ ] Memory usage optimization
- [ ] Network request reduction
- [ ] Code execution optimization
- [ ] Infrastructure scaling
- [ ] Other: ___

## Implementation Details
<!-- Briefly explain the key changes made and how they improve performance -->

## Performance Results

| Metric | Before | After | Improvement |
|--------|--------|-------|-------------|
| Response time | ___ ms | ___ ms | ___% |
| Throughput | ___ req/sec | ___ req/sec | ___% |
| Memory usage | ___ MB | ___ MB | ___% |
| CPU utilization | ___% | ___% | ___% |
| Database query time | ___ ms | ___ ms | ___% |
| Page load time | ___ sec | ___ sec | ___% |

## Testing & Validation

- [ ] Load testing completed
- [ ] Performance regression testing done
- [ ] All functionality preserved (no regressions)
- [ ] Tested in production-like environment

## Trade-offs & Considerations
<!-- Note any trade-offs made or potential side effects -->

## Monitoring Plan
<!-- How will we track the impact of these changes in production? -->

## Rollback Plan
<!-- Steps to revert these changes if needed -->

## Future Optimizations
<!-- Any follow-up performance work identified (optional) -->
