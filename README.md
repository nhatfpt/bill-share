# Tên dự án

**Bill Share (Chia Hóa Đơn)**

---

# Description

Bill Share là một Smart Contract đơn giản trên nền tảng **Stellar (Soroban)** được thiết kế để giải quyết bài toán chia tiền ăn uống (hoặc chi phí chung) giữa một nhóm bạn bè một cách tự động và minh bạch.

**Mục đích dự án**: 
- Giảm thiểu các tranh chấp về chia tiền trong nhóm
- Tự động hóa quy trình thanh toán bằng blockchain
- Tích hợp công nghệ Stellar để thực hiện chuyển tiền an toàn bằng stablecoin (USDC) hoặc token XLM
- Đảm bảo tính minh bạch và bất biến của các giao dịch

**Tại sao idea này**:
- Nhiều người cảm thấy khó khăn khi phải theo dõi ai nợ ai bao nhiêu tiền
- Các ứng dụng truyền thống không đảm bảo thanh toán tự động
- Blockchain cung cấp giải pháp tin cậy, minh bạch và không thể chối cãi

---

# Tính năng

1. **`create_bill`** - Tạo hóa đơn mới
   - Người thanh toán (`payer`) tạo một hóa đơn mới
   - Contract tự động tính toán số tiền mỗi người nợ (`amount_per_person` = `total_amount` / số người chia)
   - Dữ liệu hóa đơn được lưu trên Stellar với một `bill_id` duy nhất

2. **`pay_bill`** - Thanh toán phần nợ của bạn
   - Các thành viên nợ tiền gọi hàm này để thanh toán nợ cho người đã trả tiền
   - Contract tự động cấp quyền và chuyển token (USDC hoặc XLM) từ ví người thanh toán → ví người đã trả tiền
   - Tự động cập nhật trạng thái khi tất cả mọi người đã thanh toán (is_resolved = true)

3. **`get_bill`** - Xem thông tin hóa đơn
   - Truy xuất toàn bộ thông tin về số tiền còn nợ, ai đã thanh toán, chi tiết giao dịch, v.v.

---

# Contract

[https://stellar.expert/explorer/testnet/contract/CADUHNINBS6LKDC2VT3DEB4MSGYVMTET3O764S6BUZK6PDDCPDBGCF5G](https://stellar.expert/explorer/testnet/contract/CADUHNINBS6LKDC2VT3DEB4MSGYVMTET3O764S6BUZK6PDDCPDBGCF5G)

**Ảnh chụp màn hình Contract:**  
<img width="1911" height="1025" alt="image" src="https://github.com/user-attachments/assets/c596c151-55bf-4d54-8328-40838d434d36" />

---

# Future scopes

1. **Frontend dApp** - Xây dựng giao diện người dùng
   - Tạo web UI bằng React + TypeScript
   - Tích hợp ví Freighter để xác thực người dùng
   - Hiển thị danh sách hóa đơn, trạng thái thanh toán, và lịch sử giao dịch

2. **Mở rộng tính năng**
   - Hỗ trợ nhiều loại token (USDC, XLM, và các token khác)
   - Thêm chức năng chia tiền theo tỷ lệ khác nhau (không bằng nhau)
   - Thêm chức năng xóa/hủy hóa đơn trong trường hợp cần thiết

3. **Advanced features**
   - Lịch sử giao dịch chi tiết và xuất báo cáo
   - Nhắc nhở thanh toán qua notification
   - Hỗ trợ thanh toán lặp lại (recurring bills)
   - Dashboard analytics để xem chi tiêu

4. **Production ready**
   - Triển khai lên Mainnet
   - Tích hợp với các dịch vụ thanh toán khác
   - Cộng đồng người dùng và marketplace

---

# Profile

**Nickname / Tên:** nhat nguyen

**Kỹ năng:**
- Rust & Soroban Smart Contract Development
- Blockchain & Stellar Protocol
- Full-stack development (Frontend + Backend)
- TypeScript / JavaScript
- Smart Contract Testing & Deployment

---

## Cấu trúc Repository

Toàn bộ logic mã nguồn được gộp trong **1 file duy nhất** để đơn giản:

- `contracts/src/libs.rs` - Toàn bộ smart contract logic
- `contracts/Cargo.toml` - Cấu hình dự án Rust

## Build & Deploy

### Build sang WASM:

```bash
cd contracts
cargo build --target wasm32-unknown-unknown --release
```

### Deploy lên Testnet:

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/bill_share.wasm \
  --source-account <your-identity> \
  --network testnet
```
