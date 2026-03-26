# Bill Share (Chia Hóa Đơn) - Stellar Smart Contract

Đây là một Smart Contract đơn giản trên nền tảng **Stellar (Soroban)** dùng để giải quyết bài toán chia tiền ăn uống (hoặc chi phí chung) giữa một nhóm bạn bè.

Dự án này tích hợp tính năng **Stellar chuyển đổi tiền tệ / thanh toán** (ví dụ: thanh toán bằng stablecoin USDC hoặc token XLM native trên Stellar) để tự động thanh toán nợ cho người đã trả tiền hóa đơn tổng.

## Tính Năng Của Hợp Đồng

1. **`create_bill`**:
   - Người thanh toán toàn bộ (`payer`) sẽ tạo một hóa đơn mới.
   - Contract sẽ tự động tính toán toán số tiền mỗi người nợ (`amount_per_person` = `total_amount` / tổng số người chia).
   - Dữ liệu hóa đơn được lưu trên chain của Stellar với một `bill_id` duy nhất.

2. **`pay_bill`**:
   - Các thành viên nợ tiền (`participant`) gọi hàm này để thanh toán phần nợ của họ cho người đã trả tiền.
   - Hợp đồng thông minh sẽ tự động cấp quyền và chuyển token (ví dụ: USDC) từ ví của `participant` qua ví của `payer` thông qua interface `token::Client`.
   - Lưu trữ lại số lượng người đã trả, tự động gán nhãn `is_resolved = true` trên hóa đơn nếu như tất cả nhóm đã thanh toán hết nợ.

3. **`get_bill`**:
   - Hàm query dùng để truy xuất toàn bộ thông tin về lượng tiền còn nợ, ai đã trả hóa đơn, thông tin nhà hàng v.v.

## Cấu trúc Contract

Theo đúng yêu cầu, để giữ mọi thứ thật đơn giản dễ hiểu, toàn bộ logic mã nguồn đã được gộp lại vào **1 file duy nhất**:

- `src/libs.rs`

Ngoài ra, `contracts/Cargo.toml` cũng đã được tuỳ chỉnh `name = "bill-share"` và trỏ thẳng entry point vào `libs.rs`.

## Hướng dẫn thao tác

### Build DAPP sang WebAssembly (WASM):

Truy cập vào thư mục `contracts` và chạy:

```bash
cargo build --target wasm32-unknown-unknown --release
```

### Mở rộng sau này:

Bạn có thể kết nối Contract này trên một giao diện (Frontend) dùng `soroban-react` hay `freighter-api` để tương tác trực tiếp với ví điện tử.
