# Work Order — Conv1/2/3d depthwise weight shape fix

## Order Summary

`build_conv{1,2,3}d` weight uniform shape 마지막 dim을 `input_channels / groups`로 수정. groups>1일 때 mlx-c가 reject하는 버그 fix.

## Checklist

- [x] build_conv1d
- [x] build_conv2d
- [x] build_conv3d
- [ ] git commit + push

## Karpathy Applied

### Assumptions
- `Param::new` weight shape는 `[out, K..., in/groups]` (mlx 공식 spec)
- groups 기본값 1 → 기존 행동 호환

### Smallest Scope
- 3 함수 weight 배열만 수정

### Files Likely To Change
- `mlx-rs/src/nn/convolution.rs`

### Verification
- diff stat 1 file
- git log 1 commit

### Non-goals
- 다른 op 변경 안 함
- upstream PR (별도)

## Karpathy Verification

(commit 후 채움)
