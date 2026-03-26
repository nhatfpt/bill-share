#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, token, Address, Env, String,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bill {
    pub payer: Address,           // Người đã thanh toán hóa đơn
    pub token: Address,           // Token Stellar được dùng để thanh toán (e.g., USDC)
    pub total_amount: i128,       // Tổng giá trị hóa đơn
    pub amount_per_person: i128,  // Số tiền mỗi người tham gia cần trả
    pub total_participants: u32,  // Tổng số người tham gia (không tính người thanh toán)
    pub resolved_participants: u32, // Số người đã thanh toán tiền nợ
    pub is_resolved: bool,        // Trạng thái hóa đơn đã được thanh toán xong chưa
    pub description: String,      // Mô tả hóa đơn (e.g., "Ăn trưa quán A")
}

#[contract]
pub struct BillShareContract;

#[contractimpl]
impl BillShareContract {
    /// Hàm tạo hóa đơn mới
    /// `payer`: Người đã thanh toán ban đầu và sẽ được nhận tiền hoàn lại.
    /// `token`: Token tiền tệ (vd; USDC, XLM) được sử dụng để thanh toán.
    /// `total_amount`: Tổng số tiền.
    /// `total_participants`: Bao nhiêu người phải trả tiền cho payer (không tính payer).
    /// `description`: Thông tin hóa đơn.
    pub fn create_bill(
        env: Env,
        bill_id: u32,
        payer: Address,
        token: Address,
        total_amount: i128,
        total_participants: u32,
        description: String,
    ) {
        payer.require_auth();
        
        // Tính toán số tiền mỗi người phải chịu giả sử chia đều.
        // Tổng số người chia = payer + participants
        let total_people = total_participants as i128 + 1;
        let amount_per_person = total_amount / total_people;

        let bill = Bill {
            payer,
            token,
            total_amount,
            amount_per_person,
            total_participants,
            resolved_participants: 0,
            is_resolved: false,
            description,
        };

        // Lưu bản ghi hóa đơn
        env.storage().instance().set(&bill_id, &bill);
    }

    /// Một người tham gia thanh toán phần tiền ăn của họ cho payer.
    /// Contract sẽ sử dụng Stellar Token để thực hiện giao dịch (vd: USDC).
    pub fn pay_bill(env: Env, bill_id: u32, participant: Address) {
        participant.require_auth();

        // Lấy thông tin hóa đơn
        let mut bill: Bill = env.storage().instance().get(&bill_id).unwrap();
        
        if bill.is_resolved {
            panic!("Hóa đơn này đã được thanh toán đày đủ.");
        }

        // Thực hiện giao dịch token Stellar từ participant -> payer
        let token_client = token::Client::new(&env, &bill.token);
        token_client.transfer(&participant, &bill.payer, &bill.amount_per_person);

        // Cập nhật trạng thái thanh toán
        bill.resolved_participants += 1;
        
        // Đánh dấu hoàn tất khi tất cả mọi người đã thanh toán xong
        if bill.resolved_participants == bill.total_participants {
            bill.is_resolved = true;
        }

        // Lưu lại dữ liệu lên chain
        env.storage().instance().set(&bill_id, &bill);
    }

    /// Lấy thông tin chi tiết hóa đơn
    pub fn get_bill(env: Env, bill_id: u32) -> Bill {
        env.storage().instance().get(&bill_id).unwrap()
    }
}
