## Event

`oxideai/mlx-rs` fork (cndwl0528-crypto/mlx-rs)에 Conv1/2/3d depthwise weight shape fix 1줄씩 적용. Hi Alice LFM2 mlx-rs port에서 발견된 upstream 버그.

## Function

`mlx-rs/mlx-rs/src/nn/convolution.rs::build_conv{1,2,3}d`의 weight uniform shape 마지막 dim에 `/ groups` 적용.

## Steps

1. 코드 fix (완료, 3개 함수)
2. git commit + push (origin main)
3. hi-alice Cargo.toml patch는 옆 세션 iMac soak 종료 후 별도 진행

## Verify

- 3 함수 fix 확인 (build_conv1d/2d/3d)
- git push 성공
- diff stat: convolution.rs 1 file changed

## Closeout

- fork main에 fix 보존
- 향후 upstream PR 별도 packet
- iMac soak 종료 후 hi-alice patch 적용
