# Work Order — Conv1/2/3d depthwise weight shape fix + hi-alice patch wire

## Order Summary

(1) `build_conv{1,2,3}d` weight shape `input_channels / groups` fix → fork commit + push (완료, b744c60).
(2) hi-alice `rust/air-m5-dora/Cargo.toml`에 `[patch.crates-io] mlx-rs = { git = ".../mlx-rs" }` 추가하여 우리 fork 사용.
(3) AL-1 sanity test를 builder 패턴으로 재작성 + 검증.

## Checklist

- [x] build_conv1d
- [x] build_conv2d
- [x] build_conv3d
- [x] fork git commit + push (b744c60)
- [ ] hi-alice Cargo.toml `[patch.crates-io]` 블록 추가
- [ ] AL-1 builder 패턴 재작성 + cargo test PASS

## Karpathy Applied

### Assumptions
- `Param::new` weight shape는 `[out, K..., in/groups]` (mlx 공식 spec)
- groups 기본값 1 → 기존 행동 호환

### Smallest Scope
- 3 함수 weight 배열만 수정

### Files Likely To Change
- `mlx-rs/src/nn/convolution.rs` (fork)
- `../hi-alice/rust/air-m5-dora/Cargo.toml` (patch.crates-io 블록 추가)
- `../hi-alice/rust/air-m5-dora/crates/mlx-rs-proof/tests/lfm2_conv1d_depthwise_sanity.rs` (builder 패턴 재작성)

### Verification
- diff stat 1 file
- git log 1 commit

### Non-goals
- 다른 op 변경 안 함
- upstream PR (별도)

## Karpathy Verification

(commit 후 채움)
